/*
 * Copyright © 2016 Quentin "Sardem FF7" Glidic
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */

#include "helpers.h"

#include <sys/stat.h>
#include <fcntl.h>
#include <sys/mman.h>
#include <poll.h>

#include <wayland-cursor.h>
#include <cairo.h>
#include <pango/pango.h>
#include <pango/pangocairo.h>
#include "dock-manager-unstable-v2-client-protocol.h"

/* Supported interface versions */
#define WL_COMPOSITOR_INTERFACE_VERSION 3
#define WW_DOCK_MANAGER_INTERFACE_VERSION 1
#define WL_SHM_INTERFACE_VERSION 1
#define WL_SEAT_INTERFACE_VERSION 5
#define WL_OUTPUT_INTERFACE_VERSION 2

typedef enum {
    WW_DOCK_GLOBAL_COMPOSITOR,
    WW_DOCK_GLOBAL_DOCK_MANAGER,
    WW_DOCK_GLOBAL_SHM,
    _WW_DOCK_GLOBAL_SIZE,
} WwDockGlobalName;

typedef struct {
    char runtime_dir[PATH_MAX];
    struct wl_display *display;
    struct wl_registry *registry;
    uint32_t global_names[_WW_DOCK_GLOBAL_SIZE];
    struct wl_compositor *compositor;
    struct zww_dock_manager_v2 *dock_manager;
    struct wl_shm *shm;
    size_t buffer_count;
    struct {
        char *theme_name;
        char **name;
        struct wl_cursor_theme *theme;
        struct wl_cursor *cursor;
        struct wl_cursor_image *image;
        struct wl_surface *surface;
        struct wl_callback *frame_cb;
    } cursor;
    struct wl_list seats;
    struct wl_list outputs;
    WwColour background_colour;
    WwColour text_colour;
} WwDockContext;

typedef struct {
    WwDockContext *context;
    struct wl_list link;
    uint32_t global_name;
    struct wl_seat *seat;
    struct wl_pointer *pointer;
} WwDockSeat;

typedef struct {
    WwDockContext *context;
    struct wl_list link;
    uint32_t global_name;
    struct wl_output *output;
    int32_t scale;
} WwDockOutput;

typedef struct {
    struct wl_buffer *buffer;
    uint8_t *data;
    bool released;
} WwBuffer;
typedef struct {
    WwDockContext *context;
    uint8_t *data;
    size_t size;
    bool to_free;
    WwBuffer *buffers;
} WwBufferPool;

typedef struct {
    WwDockContext *context;
    struct wl_list link;
    struct wl_surface *surface;
    struct zww_dock_v2 *dock;
    PangoLayout *text;
    WwBufferPool *pool;
    int32_t width;
    int32_t height;
    size_t scales[3];
    int32_t scale;
    int32_t text_width;
    int32_t text_height;
    time_t time;
    struct wl_callback *frame_cb;
} WwDock;

static WwDockOutput *
_ww_dock_get_output(WwDockContext *self, struct wl_output *wl_output)
{
    WwDockOutput *output;
    wl_list_for_each(output, &self->outputs, link)
    {
        if ( output->output == wl_output )
            return output;
    }
    return NULL;
}

static void
_ww_dock_buffer_cleanup(WwBufferPool *self)
{
    if ( ! self->to_free )
        return;

    size_t i, count = 0;
    for ( i = 0 ; i < self->context->buffer_count ; ++i )
    {
        if ( ( self->buffers[i].released ) && ( self->buffers[i].buffer != NULL ) )
        {
            wl_buffer_destroy(self->buffers[i].buffer);
            self->buffers[i].buffer = NULL;
        }
        if ( self->buffers[i].buffer == NULL )
            ++count;
    }

    if ( count < self->context->buffer_count )
        return;

    munmap(self->data, self->size);
    free(self);
}

static void
_ww_dock_buffer_release(void *data, struct wl_buffer *buffer)
{
    WwBufferPool *self = data;

    size_t i;
    for ( i = 0 ; i < self->context->buffer_count ; ++i )
    {
        if ( self->buffers[i].buffer == buffer )
            self->buffers[i].released = true;
    }

    _ww_dock_buffer_cleanup(self);
}

static void
_ww_dock_buffer_pool_free(WwBufferPool *self)
{
    self->to_free = true;
    _ww_dock_buffer_cleanup(self);
}

static const struct wl_buffer_listener _ww_dock_buffer_listener = {
    _ww_dock_buffer_release
};

static WwBufferPool *
_ww_dock_create_buffer_pool(WwDock *dock)
{
    struct wl_shm_pool *pool;
    struct wl_buffer *buffer;
    int fd;
    uint8_t *data;
    int32_t width = dock->width * dock->scale;
    int32_t height = dock->height * dock->scale;
    int32_t stride;
    size_t size;
    size_t pool_size;

    stride = cairo_format_stride_for_width(CAIRO_FORMAT_ARGB32, width);
    size = stride * height;
    pool_size = size * dock->context->buffer_count;

    char filename[PATH_MAX];
    snprintf(filename, PATH_MAX, "%s/%s", dock->context->runtime_dir, "wayland-surface");
    fd = open(filename, O_CREAT | O_RDWR | O_CLOEXEC, 0);
    unlink(filename);
    if ( fd < 0 )
    {
        ww_warning("creating a buffer file for %zu B failed: %s\n", pool_size, strerror(errno));
        return NULL;
    }
    if ( ftruncate(fd, pool_size) < 0 )
    {
        close(fd);
        return NULL;
    }

    data = mmap(NULL, pool_size, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
    if ( data == MAP_FAILED )
    {
        ww_warning("mmap failed: %s\n", strerror(errno));
        close(fd);
        return NULL;
    }

    WwBufferPool *self;
    self = ww_new0(WwBufferPool, 1);
    if ( self == NULL )
    {
        wl_buffer_destroy(buffer);
        return NULL;
    }

    self->context = dock->context;
    self->buffers = ww_new0(WwBuffer, self->context->buffer_count);
    if ( self->buffers == NULL )
    {
        wl_buffer_destroy(buffer);
        free(self);
        return NULL;
    }

    pool = wl_shm_create_pool(dock->context->shm, fd, pool_size);
    size_t i;
    for ( i = 0 ; i < self->context->buffer_count ; ++i )
    {
        self->buffers[i].buffer = wl_shm_pool_create_buffer(pool, size * i, width, height, stride, WL_SHM_FORMAT_ARGB8888);
        self->buffers[i].data = data + size * i;
        self->buffers[i].released = true;
        wl_buffer_add_listener(self->buffers[i].buffer, &_ww_dock_buffer_listener, self);
    }
    wl_shm_pool_destroy(pool);
    close(fd);

    return self;
}

static void
_ww_dock_surface_protocol_enter(void *data, struct wl_surface *wl_surface, struct wl_output *wl_output)
{
    WwDock *self = data;
    WwDockOutput *output;

    output = _ww_dock_get_output(self->context, wl_output);
    if ( output == NULL )
        return;
    if ( ( output->scale < 1 ) || ( output->scale > 3 ) )
        return;

    ++self->scales[output->scale - 1];
    if ( self->scale < output->scale )
    {
        WwBufferPool *pool;
        self->scale = output->scale;
        pool = _ww_dock_create_buffer_pool(self);
        if ( pool != NULL )
        {
            _ww_dock_buffer_pool_free(self->pool);
            self->pool = pool;
        }
    }
}

static void
_ww_dock_surface_protocol_leave(void *data, struct wl_surface *wl_surface, struct wl_output *wl_output)
{
    WwDock *self = data;
    WwDockOutput *output;

    output = _ww_dock_get_output(self->context, wl_output);
    if ( output == NULL )
        return;
    if ( ( output->scale < 1 ) || ( output->scale > 3 ) )
        return;

    if ( ( --self->scales[output->scale - 1] < 1 ) && ( self->scale == output->scale ) )
    {
        int32_t i;
        for ( i = 0 ; i < 3 ; ++i )
        {
            if ( self->scales[i] > 0 )
                self->scale = i + 1;
        }
    }

}

static void
_ww_dock_dock_protocol_configure(void *data, struct zww_dock_v2 *dock, int32_t min_width, int32_t min_height, int32_t max_width, int32_t max_height, enum zww_dock_manager_v2_position position)
{
    WwDock *self = data;

    switch ( position )
    {
    case ZWW_DOCK_MANAGER_V2_POSITION_TOP:
    case ZWW_DOCK_MANAGER_V2_POSITION_BOTTOM:
        self->width = max_width;
        self->height = MAX(min_height, self->text_height + 10);
    break;
    case ZWW_DOCK_MANAGER_V2_POSITION_LEFT:
    case ZWW_DOCK_MANAGER_V2_POSITION_RIGHT:
        self->width = MAX(min_width, self->text_width + 10);
        self->height = max_height;
    break;
    case ZWW_DOCK_MANAGER_V2_POSITION_DEFAULT:
        assert_not_reached();
    }
}

static const struct wl_surface_listener _ww_dock_surface_interface = {
    .enter = _ww_dock_surface_protocol_enter,
    .leave = _ww_dock_surface_protocol_leave,
};

static const struct zww_dock_v2_listener _ww_dock_dock_interface = {
    .configure = _ww_dock_dock_protocol_configure,
};

static PangoLayout *
_ww_dock_create_text(WwDock *self)
{
    PangoContext *pango_context;
    PangoFontDescription *font;
    PangoLayout *text;

    pango_context = pango_context_new();
    pango_context_set_font_map(pango_context, pango_cairo_font_map_get_default());

    font = pango_font_description_from_string("Sans 15");

    text = pango_layout_new(pango_context);
    pango_layout_set_font_description(text, font);
    pango_layout_set_text(text, "9999-99-99 99:99:99", -1);
    pango_layout_get_pixel_size(text, &self->text_width, &self->text_height);

    pango_font_description_free(font);
    g_object_unref(pango_context);

    return text;
}

static void _ww_dock_frame_callback(void *data, struct wl_callback *callback, uint32_t time);

static const struct wl_callback_listener _ww_dock_frame_wl_callback_listener = {
    .done = _ww_dock_frame_callback,
};

static int
_ww_dock_trigger_drawing(WwDock *self, time_t t)
{
    WwBuffer *buffer = NULL;
    size_t i;
    for ( i = 0 ; ( buffer == NULL ) && ( i < self->context->buffer_count ) ; ++i )
    {
        buffer = self->pool->buffers + i;
        if ( ! buffer->released )
            buffer = NULL;
    }
    if ( buffer == NULL )
        return 1;

    struct tm *tmp;
    char text[20];
    int32_t text_width;
    int32_t text_height;

    self->time = t;
    tmp = localtime(&t);
    strftime(text, sizeof(text), "%Y-%m-%d %T", tmp);
    pango_layout_set_text(self->text, text, -1);
    pango_layout_get_pixel_size(self->text, &text_width, &text_height);

    int32_t width = self->width * self->scale;
    int32_t height = self->height * self->scale;
    int32_t stride;
    cairo_surface_t *surface;
    cairo_t *cr;

    stride = cairo_format_stride_for_width(CAIRO_FORMAT_ARGB32, width);
    surface = cairo_image_surface_create_for_data(buffer->data, CAIRO_FORMAT_ARGB32, width, height, stride);
    cairo_surface_set_device_scale(surface, self->scale, self->scale);
    cr = cairo_create(surface);

    cairo_set_source_rgba(cr, self->context->background_colour.r, self->context->background_colour.g, self->context->background_colour.b, self->context->background_colour.a);
    cairo_set_operator(cr, CAIRO_OPERATOR_SOURCE);
    cairo_paint(cr);

    cairo_set_source_rgba(cr, self->context->text_colour.r, self->context->text_colour.g, self->context->text_colour.b, self->context->text_colour.a);
    cairo_set_operator(cr, CAIRO_OPERATOR_OVER);
    cairo_move_to(cr, self->width / 2 - text_width / 2, self->height / 2 - text_height / 2);
    pango_cairo_layout_path(cr, self->text);
    cairo_fill(cr);

    cairo_destroy(cr);
    cairo_surface_destroy(surface);

    wl_surface_damage(self->surface, 0, 0, self->width, self->height);
    wl_surface_attach(self->surface, buffer->buffer, 0, 0);
    if ( wl_surface_get_version(self->surface) >= WL_SURFACE_SET_BUFFER_SCALE_SINCE_VERSION )
        wl_surface_set_buffer_scale(self->surface, self->scale);
    buffer->released = false;

    return 1;
}

static void
_ww_dock_frame_callback(void *data, struct wl_callback *callback, uint32_t timestamp)
{
    WwDock *self = data;

    if ( self->frame_cb != NULL )
        wl_callback_destroy(self->frame_cb);
    self->frame_cb = wl_surface_frame(self->surface);
    wl_callback_add_listener(self->frame_cb, &_ww_dock_frame_wl_callback_listener, self);

    time_t current;
    current = time(NULL);
    if ( current > self->time )
        _ww_dock_trigger_drawing(self, current);

    wl_surface_commit(self->surface);
}

static WwDock *
_ww_dock_create(WwDockContext *context, time_t t)
{
    WwDock *self;
    self = ww_new0(WwDock, 1);
    if ( self == NULL )
        return NULL;

    self->context = context;
    self->surface = wl_compositor_create_surface(self->context->compositor);
    if ( self->surface == NULL )
    {
        free(self);
        return NULL;
    }

    self->dock = zww_dock_manager_v2_create_dock(self->context->dock_manager, self->surface, NULL, ZWW_DOCK_MANAGER_V2_POSITION_DEFAULT);
    if ( self->dock == NULL )
    {
        wl_surface_destroy(self->surface);
        free(self);
        return NULL;
    }

    self->scale = 1;
    self->text = _ww_dock_create_text(self);

    wl_surface_add_listener(self->surface, &_ww_dock_surface_interface, self);
    zww_dock_v2_add_listener(self->dock, &_ww_dock_dock_interface, self);
    wl_display_roundtrip(self->context->display);

    if ( ( self->width < 1 ) || ( self->height < 1 ) )
    {
        zww_dock_v2_destroy(self->dock);
        wl_surface_destroy(self->surface);
        free(self);
        return NULL;
    }

    self->pool = _ww_dock_create_buffer_pool(self);
    if ( self->pool == NULL )
    {
        zww_dock_v2_destroy(self->dock);
        wl_surface_destroy(self->surface);
        free(self);
        return NULL;
    }

    _ww_dock_frame_callback(self, self->frame_cb, 0);

    return self;
}

static void
_ww_dock_free(WwDock *self)
{
    zww_dock_v2_destroy(self->dock);
    wl_surface_destroy(self->surface);
    _ww_dock_buffer_pool_free(self->pool);
    free(self);
}

static void
_ww_dock_cursor_set_image(WwDockContext *self, int i)
{
    struct wl_buffer *buffer;
    struct wl_cursor_image *image;
    image = self->cursor.cursor->images[i];

    self->cursor.image = image;
    buffer = wl_cursor_image_get_buffer(self->cursor.image);
    wl_surface_attach(self->cursor.surface, buffer, 0, 0);
    wl_surface_damage(self->cursor.surface, 0, 0, self->cursor.image->width, self->cursor.image->height);
    wl_surface_commit(self->cursor.surface);
}

static void _ww_dock_cursor_frame_callback(void *data, struct wl_callback *callback, uint32_t time);

static const struct wl_callback_listener _ww_dock_cursor_frame_wl_callback_listener = {
    .done = _ww_dock_cursor_frame_callback,
};

static void
_ww_dock_cursor_frame_callback(void *data, struct wl_callback *callback, uint32_t time)
{
    WwDockContext *self = data;
    int i;

    if ( self->cursor.frame_cb != NULL )
        wl_callback_destroy(self->cursor.frame_cb);
    self->cursor.frame_cb = wl_surface_frame(self->cursor.surface);
    wl_callback_add_listener(self->cursor.frame_cb, &_ww_dock_cursor_frame_wl_callback_listener, self);

    i = wl_cursor_frame(self->cursor.cursor, time);
    _ww_dock_cursor_set_image(self, i);
}

static void
_ww_dock_pointer_enter(void *data, struct wl_pointer *pointer, uint32_t serial, struct wl_surface *surface, wl_fixed_t x, wl_fixed_t y)
{
    WwDockSeat *self = data;
    WwDockContext *context = self->context;

    if ( context->cursor.surface == NULL )
        return;

    if ( context->cursor.cursor->image_count < 2 )
        _ww_dock_cursor_set_image(context, 0);
    else
        _ww_dock_cursor_frame_callback(context, context->cursor.frame_cb, 0);

    wl_pointer_set_cursor(self->pointer, serial, context->cursor.surface, context->cursor.image->hotspot_x, context->cursor.image->hotspot_y);
}

static void
_ww_dock_pointer_leave(void *data, struct wl_pointer *pointer, uint32_t serial, struct wl_surface *surface)
{
    WwDockSeat *self = data;
    WwDockContext *context = self->context;

    if ( context->cursor.frame_cb != NULL )
        wl_callback_destroy(context->cursor.frame_cb);
}

static void
_ww_dock_pointer_motion(void *data, struct wl_pointer *pointer, uint32_t time, wl_fixed_t x, wl_fixed_t y)
{
}

static void
_ww_dock_pointer_button(void *data, struct wl_pointer *pointer, uint32_t serial, uint32_t time, uint32_t button, enum wl_pointer_button_state state)
{
}

static void
_ww_dock_pointer_axis(void *data, struct wl_pointer *pointer, uint32_t time, enum wl_pointer_axis axis, wl_fixed_t value)
{
}

static void
_ww_dock_pointer_frame(void *data, struct wl_pointer *pointer)
{
}

static void
_ww_dock_pointer_axis_source(void *data, struct wl_pointer *pointer, enum wl_pointer_axis_source axis_source)
{
}

static void
_ww_dock_pointer_axis_stop(void *data, struct wl_pointer *pointer, uint32_t time, enum wl_pointer_axis axis)
{
}

static void
_ww_dock_pointer_axis_discrete(void *data, struct wl_pointer *pointer, enum wl_pointer_axis axis, int32_t discrete)
{
}

static const struct wl_pointer_listener _ww_dock_pointer_listener = {
    .enter = _ww_dock_pointer_enter,
    .leave = _ww_dock_pointer_leave,
    .motion = _ww_dock_pointer_motion,
    .button = _ww_dock_pointer_button,
    .axis = _ww_dock_pointer_axis,
    .frame = _ww_dock_pointer_frame,
    .axis_source = _ww_dock_pointer_axis_source,
    .axis_stop = _ww_dock_pointer_axis_stop,
    .axis_discrete = _ww_dock_pointer_axis_discrete,
};

static void
_ww_dock_pointer_release(WwDockSeat *self)
{
    if ( self->pointer == NULL )
        return;

    if ( wl_pointer_get_version(self->pointer) >= WL_POINTER_RELEASE_SINCE_VERSION )
        wl_pointer_release(self->pointer);
    else
        wl_pointer_destroy(self->pointer);

    self->pointer = NULL;
}

static void
_ww_dock_seat_release(WwDockSeat *self)
{
    _ww_dock_pointer_release(self);

    if ( wl_seat_get_version(self->seat) >= WL_SEAT_RELEASE_SINCE_VERSION )
        wl_seat_release(self->seat);
    else
        wl_seat_destroy(self->seat);

    wl_list_remove(&self->link);

    free(self);
}

static void
_ww_dock_seat_capabilities(void *data, struct wl_seat *seat, uint32_t capabilities)
{
    WwDockSeat *self = data;
    if ( ( capabilities & WL_SEAT_CAPABILITY_POINTER ) && ( self->pointer == NULL ) )
    {
        self->pointer = wl_seat_get_pointer(self->seat);
        wl_pointer_add_listener(self->pointer, &_ww_dock_pointer_listener, self);
    }
    else if ( ( ! ( capabilities & WL_SEAT_CAPABILITY_POINTER ) ) && ( self->pointer != NULL ) )
        _ww_dock_pointer_release(self);
}

static void
_ww_dock_seat_name(void *data, struct wl_seat *seat, const char *name)
{
}

static const struct wl_seat_listener _ww_dock_seat_listener = {
    .capabilities = _ww_dock_seat_capabilities,
    .name = _ww_dock_seat_name,
};

static void
_ww_dock_output_release(WwDockOutput *self)
{
    if ( wl_output_get_version(self->output) >= WL_OUTPUT_RELEASE_SINCE_VERSION )
        wl_output_release(self->output);
    else
        wl_output_destroy(self->output);

    wl_list_remove(&self->link);

    free(self);
}

static void
_ww_dock_output_done(void *data, struct wl_output *output)
{
}

static void
_ww_dock_output_geometry(void *data, struct wl_output *output, int32_t x, int32_t y, int32_t width, int32_t height, int32_t subpixel, const char *make, const char *model, int32_t transform)
{
}

static void
_ww_dock_output_mode(void *data, struct wl_output *output, enum wl_output_mode flags, int32_t width, int32_t height, int32_t refresh)
{
}

static void
_ww_dock_output_scale(void *data, struct wl_output *output, int32_t scale)
{
    WwDockOutput *self = data;

    self->scale = scale;
}

static const struct wl_output_listener _ww_dock_output_listener = {
    .geometry = _ww_dock_output_geometry,
    .mode = _ww_dock_output_mode,
    .scale = _ww_dock_output_scale,
    .done = _ww_dock_output_done,
};

static const char * const _ww_dock_cursor_names[] = {
    "left_ptr",
    "default",
    "top_left_arrow",
    "left-arrow",
    NULL
};

static void
_ww_dock_registry_handle_global(void *data, struct wl_registry *registry, uint32_t name, const char *interface, uint32_t version)
{
    WwDockContext *self = data;

    if ( strcmp0(interface, "wl_compositor") == 0 )
    {
        self->global_names[WW_DOCK_GLOBAL_COMPOSITOR] = name;
        self->compositor = wl_registry_bind(registry, name, &wl_compositor_interface, MIN(version, WL_COMPOSITOR_INTERFACE_VERSION));
    }
    else if ( strcmp0(interface, "zww_dock_manager_v2") == 0 )
    {
        self->global_names[WW_DOCK_GLOBAL_DOCK_MANAGER] = name;
        self->dock_manager = wl_registry_bind(registry, name, &zww_dock_manager_v2_interface, WW_DOCK_MANAGER_INTERFACE_VERSION);
    }
    else if ( strcmp0(interface, "wl_shm") == 0 )
    {
        self->global_names[WW_DOCK_GLOBAL_SHM] = name;
        self->shm = wl_registry_bind(registry, name, &wl_shm_interface, MIN(version, WL_SHM_INTERFACE_VERSION));
    }
    else if ( strcmp0(interface, "wl_seat") == 0 )
    {
        WwDockSeat *seat = ww_new0(WwDockSeat, 1);
        seat->context = self;
        seat->global_name = name;
        seat->seat = wl_registry_bind(registry, name, &wl_seat_interface, MIN(version, WL_SEAT_INTERFACE_VERSION));

        wl_list_insert(&self->seats, &seat->link);

        wl_seat_add_listener(seat->seat, &_ww_dock_seat_listener, seat);
    }
    else if ( strcmp0(interface, "wl_output") == 0 )
    {
        WwDockOutput *output = ww_new0(WwDockOutput, 1);
        output->context = self;
        output->global_name = name;
        output->output = wl_registry_bind(registry, name, &wl_output_interface, MIN(version, WL_OUTPUT_INTERFACE_VERSION));
        output->scale = 1;

        wl_list_insert(&self->outputs, &output->link);

        wl_output_add_listener(output->output, &_ww_dock_output_listener, output);
    }

    if ( ( self->cursor.theme == NULL ) && ( self->compositor != NULL ) && ( self->shm != NULL ) )
    {
        self->cursor.theme = wl_cursor_theme_load(self->cursor.theme_name, 32, self->shm);
        if ( self->cursor.theme != NULL )
        {
            const char * const *cname = (const char * const *) self->cursor.name;
            for ( cname = ( cname != NULL ) ? cname : _ww_dock_cursor_names ; ( self->cursor.cursor == NULL ) && ( *cname != NULL ) ; ++cname )
                self->cursor.cursor = wl_cursor_theme_get_cursor(self->cursor.theme, *cname);
            if ( self->cursor.cursor == NULL )
            {
                wl_cursor_theme_destroy(self->cursor.theme);
                self->cursor.theme = NULL;
            }
            else
                self->cursor.surface = wl_compositor_create_surface(self->compositor);
        }
    }
}

static void
_ww_dock_registry_handle_global_remove(void *data, struct wl_registry *registry, uint32_t name)
{
    WwDockContext *self = data;

    WwDockGlobalName i;
    for ( i = 0 ; i < _WW_DOCK_GLOBAL_SIZE ; ++i )
    {
        if ( self->global_names[i] != name )
            continue;
        self->global_names[i] = 0;

        switch ( i )
        {
        case WW_DOCK_GLOBAL_COMPOSITOR:
            wl_compositor_destroy(self->compositor);
            self->compositor = NULL;
        break;
        case WW_DOCK_GLOBAL_DOCK_MANAGER:
            zww_dock_manager_v2_destroy(self->dock_manager);
            self->dock_manager = NULL;
        break;
        case WW_DOCK_GLOBAL_SHM:
            wl_shm_destroy(self->shm);
            self->shm = NULL;
        break;
        case _WW_DOCK_GLOBAL_SIZE:
            assert_not_reached();
        }
        return;
    }
    if ( ( self->cursor.theme != NULL ) && ( ( self->compositor == NULL ) || ( self->shm == NULL ) ) )
    {
        if ( self->cursor.frame_cb != NULL )
            wl_callback_destroy(self->cursor.frame_cb);
        self->cursor.frame_cb = NULL;

        wl_surface_destroy(self->cursor.surface);
        wl_cursor_theme_destroy(self->cursor.theme);
        self->cursor.surface = NULL;
        self->cursor.image = NULL;
        self->cursor.cursor = NULL;
        self->cursor.theme = NULL;
    }

    WwDockSeat *seat, *tmp_seat;
    wl_list_for_each_safe(seat, tmp_seat, &self->seats, link)
    {
        if ( seat->global_name != name )
            continue;

        _ww_dock_seat_release(seat);
        return;
    }

    WwDockOutput *output, *tmp_output;
    wl_list_for_each_safe(output, tmp_output, &self->outputs, link)
    {
        if ( output->global_name != name )
            continue;

        _ww_dock_output_release(output);
        return;
    }
}

static const struct wl_registry_listener _ww_dock_registry_listener = {
    .global = _ww_dock_registry_handle_global,
    .global_remove = _ww_dock_registry_handle_global_remove,
};


static void
_ww_dock_disconnect(WwDockContext *self)
{
    WwDockSeat *seat, *tmp;
    wl_list_for_each_safe(seat, tmp, &self->seats, link)
        _ww_dock_seat_release(seat);

    WwDockGlobalName i;
    for ( i = 0 ; i < _WW_DOCK_GLOBAL_SIZE ; ++i )
    {
        if ( self->global_names[i] != 0 )
            _ww_dock_registry_handle_global_remove(self, self->registry, self->global_names[i]);
    }

    wl_registry_destroy(self->registry);
    self->registry = NULL;

    wl_display_disconnect(self->display);
    self->display = NULL;
}

int
main(int argc, char *argv[])
{
    static WwDockContext self_;
    WwDockContext *self = &self_;

    setlocale(LC_ALL, "");

    const char *runtime_dir;
    runtime_dir = getenv("XDG_RUNTIME_DIR");
    if ( runtime_dir == NULL )
        return 1;
    snprintf(self->runtime_dir, PATH_MAX, "%s/" PACKAGE_NAME, runtime_dir);

    if ( mkdir(self->runtime_dir, 0755) < 0 )
    {
        if ( errno == EEXIST )
        {
            struct stat buf;
            if ( stat(self->runtime_dir, &buf) < 0 )
                return 1;
            if ( ! S_ISDIR(buf.st_mode) )
                return 1;
        }
        else
            return 1;
    }
    self->display = wl_display_connect(NULL);
    if ( self->display == NULL )
        return 2;


    self->buffer_count = 3;

    self->background_colour.a = 1.0;
    self->text_colour.r = 1.0;
    self->text_colour.g = 1.0;
    self->text_colour.b = 1.0;
    self->text_colour.a = 1.0;

    int arg;
    while ( ( arg = getopt(argc, argv, "b:t:c:C:") ) != -1 )
    {
        bool good = false;
        switch ( arg )
        {
        case 'b':
            if ( _ww_parse_colour(optarg, &self->background_colour) )
                good = true;
        break;
        case 't':
            if ( _ww_parse_colour(optarg, &self->text_colour) )
                good = true;
        break;
        case 'c':
        {
            char *e;
            errno = 0;
            self->buffer_count = strtoul(optarg, &e, 10);
            if ( ( e != optarg ) && ( errno == 0 ) )
                good = true;
        }
        break;
        case 'C':
            self->cursor.theme_name = optarg;
            good = true;
        break;
        default:
        break;
        }
        if ( ! good )
        {
            fprintf(stderr, ""
                "Usage:"
                "\n    %s [OPTION...] - Demo client for Wayland Wall dock protocol"
                "\n"
                "\nOptions:"
                "\n    -b <colour>      Colour to use as background, defaults to #000000"
                "\n    -t <colour>      Colour to use for the text, defaults to #FFFFFF"
                "\n    -c <count>       Number of buffers to use, defaults to 3"
                "\n    -C <name>        The cursor theme to use"
                "\n"
                "\nFormats:"
                "\n    Colours options supports #RRGGBB(AA) and #RGB(A) formats"
                "\n\n", argv[0]);
            return 3;
        }
    }


    wl_list_init(&self->seats);
    wl_list_init(&self->outputs);

    self->registry = wl_display_get_registry(self->display);
    wl_registry_add_listener(self->registry, &_ww_dock_registry_listener, self);
    wl_display_roundtrip(self->display);

    if ( self->shm == NULL )
    {
        _ww_dock_disconnect(self);
        ww_warning("No wl_shm interface provided by the compositor");
        return 4;
    }
    if ( self->dock_manager == NULL )
    {
        _ww_dock_disconnect(self);
        ww_warning("No ww_dock_manager interface provided by the compositor");
        return 4;
    }

    WwDock *dock;
    time_t current;
    current = time(NULL);
    dock = _ww_dock_create(self, current);
    if ( dock == NULL )
        return 5;

    int ret;
    do
    {
        wl_display_flush(self->display);
        ret = wl_display_dispatch(self->display);
    }
    while ( ret > 0 );
    if ( ret < 0 )
        ww_warning("Couldn’t dispatch events: %s", strerror(errno));

    return 0;
}
