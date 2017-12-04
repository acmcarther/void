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

#include <wayland-cursor.h>
#ifdef ENABLE_IMAGES
#include <gdk-pixbuf/gdk-pixbuf.h>
#if ( ( GDK_PIXBUF_MAJOR < 2 ) || ( ( GDK_PIXBUF_MAJOR == 2 ) && ( GDK_PIXBUF_MINOR < 32 ) ) )
static inline const guchar *gdk_pixbuf_read_pixels(GdkPixbuf *pixbuf) { return gdk_pixbuf_get_pixels(pixbuf); }
#endif /* gdk-pixbux < 2.32 */
#endif /* ENABLE_IMAGES */
#include "viewporter-client-protocol.h"
#include "background-unstable-v2-client-protocol.h"

/* Supported interface versions */
#define WL_COMPOSITOR_INTERFACE_VERSION 3
#define WL_SUBCOMPOSITOR_INTERFACE_VERSION 1
#define WW_BACKGROUND_INTERFACE_VERSION 1
#define WL_SHM_INTERFACE_VERSION 1
#define WL_SEAT_INTERFACE_VERSION 5
#define WL_OUTPUT_INTERFACE_VERSION 2
#define WP_VIEWPORTER_INTERFACE_VERSION 1

typedef enum {
    WW_BACKGROUND_GLOBAL_COMPOSITOR,
    WW_BACKGROUND_GLOBAL_SUBCOMPOSITOR,
    WW_BACKGROUND_GLOBAL_BACKGROUND,
    WW_BACKGROUND_GLOBAL_SHM,
    WW_BACKGROUND_GLOBAL_VIEWPORTER,
    _WW_BACKGROUND_GLOBAL_SIZE,
} WwBackgroundGlobalName;

typedef struct {
    bool to_free;
    struct wl_buffer *buffer;
    bool released;
#ifdef ENABLE_IMAGES
    struct wl_buffer *image_buffer;
    bool image_released;
#endif /* ENABLE_IMAGES */
} WwBackgroundBuffer;

typedef struct {
    char runtime_dir[PATH_MAX];
    struct wl_display *display;
    struct wl_registry *registry;
    uint32_t global_names[_WW_BACKGROUND_GLOBAL_SIZE];
    struct wl_compositor *compositor;
    struct wl_subcompositor *subcompositor;
    struct zww_background_v2 *background;
    struct wl_shm *shm;
    struct wp_viewporter *viewporter;
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
#ifdef ENABLE_IMAGES
    char *image;
    bool image_scalable;
    GdkPixbuf *pixbuf;
#endif /* ENABLE_IMAGES */
    int32_t width;
    int32_t height;
    WwColour colour;
    WwBackgroundBuffer *buffer;
} WwBackgroundContext;

typedef struct _WwBackgroundOutput WwBackgroundOutput;
typedef struct  {
    WwBackgroundOutput *output;
    int32_t width;
    int32_t height;
    struct wl_surface *surface;
    struct wp_viewport *viewport;
#ifdef ENABLE_IMAGES
    struct wl_surface *image_surface;
    struct wl_subsurface *image_subsurface;
    struct wp_viewport *image_viewport;
#endif /* ENABLE_IMAGES */
} WwBackgroundSurface;

typedef struct {
    WwBackgroundContext *context;
    struct wl_list link;
    uint32_t global_name;
    struct wl_seat *seat;
    struct wl_pointer *pointer;
} WwBackgroundSeat;

struct _WwBackgroundOutput {
    WwBackgroundContext *context;
    struct wl_list link;
    uint32_t global_name;
    struct wl_output *output;
    int32_t width;
    int32_t height;
    int32_t scale;
    WwBackgroundSurface *surface;
};

static void
_ww_background_buffer_cleanup(WwBackgroundBuffer *self)
{
    if ( ! self->to_free )
        return;

    int count = 1;

#ifdef ENABLE_IMAGES
    ++count;
    if ( self->image_released )
    {
        if ( self->image_buffer != NULL )
            wl_buffer_destroy(self->image_buffer);
        --count;
    }
#endif /* ENABLE_IMAGES */

    if ( self->released )
    {
        wl_buffer_destroy(self->buffer);
        --count;
    }

    if ( count > 0 )
        return;

    free(self);
}

static void
_ww_background_buffer_release(void *data, struct wl_buffer *buf)
{
    WwBackgroundBuffer *self = data;

    if ( self->buffer == buf )
        self->released = true;
#ifdef ENABLE_IMAGES
    if ( self->image_buffer == buf )
        self->image_released = true;
#endif /* ENABLE_IMAGES */
    _ww_background_buffer_cleanup(self);
}

static void
_ww_background_buffer_free(WwBackgroundBuffer *self)
{
    self->to_free = true;
    _ww_background_buffer_cleanup(self);
}

static const struct wl_buffer_listener _ww_background_buffer_listener = {
    _ww_background_buffer_release
};

static void
_ww_background_surface_update(WwBackgroundSurface *self, WwBackgroundBuffer *buffer)
{
    struct wl_region *region;

    wl_surface_attach(self->surface, buffer->buffer, 0, 0);
    if ( wl_surface_get_version(self->surface) >= WL_SURFACE_SET_BUFFER_SCALE_SINCE_VERSION )
        wl_surface_set_buffer_scale(self->surface, self->output->scale);
    region = wl_compositor_create_region(self->output->context->compositor);
    wl_region_add(region, 0, 0, self->output->width, self->output->height);
    wl_surface_set_opaque_region(self->surface, region);

#ifdef ENABLE_IMAGES
    if ( self->output->context->pixbuf != NULL )
    {
        int image_width, image_height;
        image_width = gdk_pixbuf_get_width(self->output->context->pixbuf);
        image_height = gdk_pixbuf_get_height(self->output->context->pixbuf);

        if ( self->image_viewport != NULL )
        {
            double sx, sy, s;
            sx = (double) image_width / self->output->width;
            sy = (double) image_height / self->output->height;
            s = MAX(sx, sy);
            image_width /= s;
            image_height /= s;
            wp_viewport_set_destination(self->image_viewport, image_width, image_height);
        }

        wl_surface_attach(self->image_surface, buffer->image_buffer, 0, 0);
        if ( wl_surface_get_version(self->image_surface) >= WL_SURFACE_SET_BUFFER_SCALE_SINCE_VERSION )
            wl_surface_set_buffer_scale(self->image_surface, self->output->scale);

        wl_subsurface_set_position(self->image_subsurface, self->output->width / 2 - image_width / 2, self->output->height / 2 - image_height / 2);

        wl_surface_commit(self->image_surface);
    }
#endif /* ENABLE_IMAGES */

    if ( self->viewport != NULL )
        wp_viewport_set_source(self->viewport, 0, 0, wl_fixed_from_int(self->output->width), wl_fixed_from_int(self->output->height));

    wl_surface_commit(self->surface);
    wl_region_destroy(region);

    zww_background_v2_set_background(self->output->context->background, self->surface, self->output->output);
}

#if BYTE_ORDER == BIG_ENDIAN
#define RED_BYTE 1
#define GREEN_BYTE 2
#define BLUE_BYTE 3
#define ALPHA_BYTE 0
#else
#define RED_BYTE 2
#define GREEN_BYTE 1
#define BLUE_BYTE 0
#define ALPHA_BYTE 3
#endif

static bool
_ww_background_create_buffer(WwBackgroundContext *self, int32_t width, int32_t height)
{
    struct wl_shm_pool *pool;
    struct wl_buffer *buffer;
    int fd;
    uint8_t *data;
    int32_t stride;
    size_t size;

    stride = 4 * width;
    size = stride * height;

#ifdef ENABLE_IMAGES
    struct wl_buffer *image_buffer;
    const uint8_t *pdata = NULL;
    int image_width, image_height;
    int cstride, bytes;
    if ( self->pixbuf != NULL )
    {
        image_width = gdk_pixbuf_get_width(self->pixbuf);
        image_height = gdk_pixbuf_get_height(self->pixbuf);
        cstride = gdk_pixbuf_get_rowstride(self->pixbuf);
        bytes = gdk_pixbuf_get_has_alpha(self->pixbuf) ? 4 : 3;
        pdata = gdk_pixbuf_read_pixels(self->pixbuf);

        size += image_height * image_width * 4;
    }
#endif /* ENABLE_IMAGES */

    char filename[PATH_MAX];
    snprintf(filename, PATH_MAX, "%s/%s", self->runtime_dir, "wayland-surface");
    fd = open(filename, O_CREAT | O_RDWR | O_CLOEXEC, 0);
    unlink(filename);
    if ( fd < 0 )
    {
        ww_warning("creating a buffer file for %zu B failed: %s\n", size, strerror(errno));
        return false;
    }
    if ( ftruncate(fd, size) < 0 )
    {
        close(fd);
        return false;
    }

    data = mmap(NULL, size, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
    if ( data == MAP_FAILED )
    {
        ww_warning("mmap failed: %s\n", strerror(errno));
        close(fd);
        return false;
    }

    for ( int32_t y = 0 ; y < height ; ++y )
    {
        uint8_t *line = data + y * stride;
        for ( int32_t x = 0 ; x < width ; ++x )
        {
            uint8_t *pixel = line + x * 4;
            {
                pixel[ALPHA_BYTE] = 0xff;
                pixel[RED_BYTE]   = self->colour.r * 0xff;
                pixel[GREEN_BYTE] = self->colour.g * 0xff;
                pixel[BLUE_BYTE]  = self->colour.b * 0xff;
            }
        }
    }

#ifdef ENABLE_IMAGES
    if ( pdata != NULL )
    {
        uint8_t *image_data = data + height * stride;
        for ( int y = 0 ; y < image_height ; ++y )
        {
            uint8_t *line = image_data + y * image_width * 4;
            for ( int32_t x = 0 ; x < image_width ; ++x )
            {
                uint8_t *pixel = line + x * 4;
                const uint8_t *ppixel = pdata + y * cstride + x * bytes;

                pixel[ALPHA_BYTE] = 0xff;
                pixel[RED_BYTE]   = ppixel[0];
                pixel[GREEN_BYTE] = ppixel[1];
                pixel[BLUE_BYTE]  = ppixel[2];
            }
        }
    }
#endif /* ENABLE_IMAGES */

    munmap(data, size);

    pool = wl_shm_create_pool(self->shm, fd, size);
    buffer = wl_shm_pool_create_buffer(pool, 0, width, height, stride, WL_SHM_FORMAT_XRGB8888);
#ifdef ENABLE_IMAGES
    if ( pdata != NULL )
        image_buffer = wl_shm_pool_create_buffer(pool, height * stride, image_width, image_height, image_width * 4, WL_SHM_FORMAT_XRGB8888);
#endif /* ENABLE_IMAGES */
    wl_shm_pool_destroy(pool);
    close(fd);

    if ( self->buffer != NULL )
        _ww_background_buffer_free(self->buffer);

    self->buffer = ww_new0(WwBackgroundBuffer, 1);
    self->buffer->buffer = buffer;
    wl_buffer_add_listener(buffer, &_ww_background_buffer_listener, self->buffer);
#ifdef ENABLE_IMAGES
    if ( pdata != NULL )
    {
        self->buffer->image_buffer = image_buffer;
        wl_buffer_add_listener(image_buffer, &_ww_background_buffer_listener, self->buffer);
    }
    else
        self->buffer->image_released = true;
#endif /* ENABLE_IMAGES */

    WwBackgroundOutput *output;
    wl_list_for_each(output, &self->outputs, link)
    {
        if ( output->surface != NULL )
            _ww_background_surface_update(output->surface, self->buffer);
    }

    return true;
}

static void
_ww_background_check_buffer(WwBackgroundContext *self, WwBackgroundSurface *surface)
{
    int32_t width, height;

    width = surface->width;
    height = surface->height;
    if ( ( self->width < width ) || ( self->height < height ) )
    {
        int32_t new_width = MAX(self->width, width), new_height = MAX(self->height, height);

#ifdef ENABLE_IMAGES
        GdkPixbuf *pixbuf = self->pixbuf;
        if ( pixbuf != NULL )
        {
            int pw, ph;
            pw = gdk_pixbuf_get_width(pixbuf);
            ph = gdk_pixbuf_get_height(pixbuf);
            if ( self->image_scalable && ( ( pw < new_width ) || ( ph < new_height ) ) )
            {
                GError *error = NULL;

                /*
                 * If the image is scalable, we already loaded it at the biggest size we need
                 * so we use MAX() to get the biggest size again
                 */
                self->pixbuf = gdk_pixbuf_new_from_file_at_size(self->image, MAX(pw, new_width), MAX(ph, new_height), &error);
                if ( self->pixbuf == NULL )
                {
                    self->pixbuf = pixbuf;
                    ww_warning("Couldn’t reload the pixbuf: %s", error->message);
                    g_error_free(error);
                }
            }
        }
#endif /* ENABLE_IMAGES */

        if ( _ww_background_create_buffer(self, new_width, new_height) )
        {
            self->width = new_width;
            self->height = new_height;
            return;
        }
#ifdef ENABLE_IMAGES
        else if ( pixbuf != self->pixbuf )
        {
            g_object_unref(self->pixbuf);
            self->pixbuf = pixbuf;
        }
#endif /* ENABLE_IMAGES */
    }
    _ww_background_surface_update(surface, self->buffer);
}

static WwBackgroundSurface *
_ww_background_surface_new(WwBackgroundOutput *output)
{
    WwBackgroundSurface *self;

    self = ww_new0(WwBackgroundSurface, 1);
    self->output = output;
    self->width = self->output->width * self->output->scale;
    self->height = self->output->height * self->output->scale;

    self->surface = wl_compositor_create_surface(self->output->context->compositor);
#ifdef ENABLE_IMAGES
    self->image_surface = wl_compositor_create_surface(self->output->context->compositor);
    self->image_subsurface = wl_subcompositor_get_subsurface(self->output->context->subcompositor, self->image_surface, self->surface);
#endif /* ENABLE_IMAGES */
    if ( self->output->context->viewporter != NULL )
    {
        self->viewport = wp_viewporter_get_viewport(self->output->context->viewporter, self->surface);
#ifdef ENABLE_IMAGES
        self->image_viewport = wp_viewporter_get_viewport(self->output->context->viewporter, self->image_surface);
#endif /* ENABLE_IMAGES */
    }
    wl_surface_set_user_data(self->surface, self);

    return self;
}

static void
_ww_background_surface_free(WwBackgroundSurface *self)
{
    if ( self->output->context->viewporter != NULL )
    {
#ifdef ENABLE_IMAGES
        wp_viewport_destroy(self->image_viewport);
#endif /* ENABLE_IMAGES */
        wp_viewport_destroy(self->viewport);
    }

#ifdef ENABLE_IMAGES
    wl_subsurface_destroy(self->image_subsurface);
    wl_surface_destroy(self->image_surface);
#endif /* ENABLE_IMAGES */
    wl_surface_destroy(self->surface);

    free(self);
}

static void
_ww_background_cursor_set_image(WwBackgroundContext *self, int i)
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

static void _ww_background_cursor_frame_callback(void *data, struct wl_callback *callback, uint32_t time);

static const struct wl_callback_listener _ww_background_cursor_frame_wl_callback_listener = {
    .done = _ww_background_cursor_frame_callback,
};

static void
_ww_background_cursor_frame_callback(void *data, struct wl_callback *callback, uint32_t time)
{
    WwBackgroundContext *self = data;
    int i;

    if ( self->cursor.frame_cb != NULL )
        wl_callback_destroy(self->cursor.frame_cb);
    self->cursor.frame_cb = wl_surface_frame(self->cursor.surface);
    wl_callback_add_listener(self->cursor.frame_cb, &_ww_background_cursor_frame_wl_callback_listener, self);

    i = wl_cursor_frame(self->cursor.cursor, time);
    _ww_background_cursor_set_image(self, i);
}

static void
_ww_background_pointer_enter(void *data, struct wl_pointer *pointer, uint32_t serial, struct wl_surface *surface, wl_fixed_t x, wl_fixed_t y)
{
    WwBackgroundSeat *self = data;
    WwBackgroundContext *context = self->context;

    if ( context->cursor.surface == NULL )
        return;

    if ( context->cursor.cursor->image_count < 2 )
        _ww_background_cursor_set_image(context, 0);
    else
        _ww_background_cursor_frame_callback(context, context->cursor.frame_cb, 0);

    wl_pointer_set_cursor(self->pointer, serial, context->cursor.surface, context->cursor.image->hotspot_x, context->cursor.image->hotspot_y);
}

static void
_ww_background_pointer_leave(void *data, struct wl_pointer *pointer, uint32_t serial, struct wl_surface *surface)
{
    WwBackgroundSeat *self = data;
    WwBackgroundContext *context = self->context;

    if ( context->cursor.frame_cb != NULL )
        wl_callback_destroy(context->cursor.frame_cb);
}

static void
_ww_background_pointer_motion(void *data, struct wl_pointer *pointer, uint32_t time, wl_fixed_t x, wl_fixed_t y)
{
}

static void
_ww_background_pointer_button(void *data, struct wl_pointer *pointer, uint32_t serial, uint32_t time, uint32_t button, enum wl_pointer_button_state state)
{
}

static void
_ww_background_pointer_axis(void *data, struct wl_pointer *pointer, uint32_t time, enum wl_pointer_axis axis, wl_fixed_t value)
{
}

static void
_ww_background_pointer_frame(void *data, struct wl_pointer *pointer)
{
}

static void
_ww_background_pointer_axis_source(void *data, struct wl_pointer *pointer, enum wl_pointer_axis_source axis_source)
{
}

static void
_ww_background_pointer_axis_stop(void *data, struct wl_pointer *pointer, uint32_t time, enum wl_pointer_axis axis)
{
}

static void
_ww_background_pointer_axis_discrete(void *data, struct wl_pointer *pointer, enum wl_pointer_axis axis, int32_t discrete)
{
}

static const struct wl_pointer_listener _ww_background_pointer_listener = {
    .enter = _ww_background_pointer_enter,
    .leave = _ww_background_pointer_leave,
    .motion = _ww_background_pointer_motion,
    .button = _ww_background_pointer_button,
    .axis = _ww_background_pointer_axis,
    .frame = _ww_background_pointer_frame,
    .axis_source = _ww_background_pointer_axis_source,
    .axis_stop = _ww_background_pointer_axis_stop,
    .axis_discrete = _ww_background_pointer_axis_discrete,
};

static void
_ww_background_pointer_release(WwBackgroundSeat *self)
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
_ww_background_seat_release(WwBackgroundSeat *self)
{
    _ww_background_pointer_release(self);

    if ( wl_seat_get_version(self->seat) >= WL_SEAT_RELEASE_SINCE_VERSION )
        wl_seat_release(self->seat);
    else
        wl_seat_destroy(self->seat);

    wl_list_remove(&self->link);

    free(self);
}

static void
_ww_background_seat_capabilities(void *data, struct wl_seat *seat, uint32_t capabilities)
{
    WwBackgroundSeat *self = data;
    if ( ( capabilities & WL_SEAT_CAPABILITY_POINTER ) && ( self->pointer == NULL ) )
    {
        self->pointer = wl_seat_get_pointer(self->seat);
        wl_pointer_add_listener(self->pointer, &_ww_background_pointer_listener, self);
    }
    else if ( ( ! ( capabilities & WL_SEAT_CAPABILITY_POINTER ) ) && ( self->pointer != NULL ) )
        _ww_background_pointer_release(self);
}

static void
_ww_background_seat_name(void *data, struct wl_seat *seat, const char *name)
{
}

static const struct wl_seat_listener _ww_background_seat_listener = {
    .capabilities = _ww_background_seat_capabilities,
    .name = _ww_background_seat_name,
};

static void
_ww_background_output_release(WwBackgroundOutput *self)
{
    _ww_background_surface_free(self->surface);

    if ( wl_output_get_version(self->output) >= WL_OUTPUT_RELEASE_SINCE_VERSION )
        wl_output_release(self->output);
    else
        wl_output_destroy(self->output);

    wl_list_remove(&self->link);

    free(self);
}

static void
_ww_background_output_done(void *data, struct wl_output *output)
{
    WwBackgroundOutput *self = data;

    if ( self->surface == NULL )
        self->surface = _ww_background_surface_new(self);
    _ww_background_check_buffer(self->context, self->surface);
}

static void
_ww_background_output_geometry(void *data, struct wl_output *output, int32_t x, int32_t y, int32_t width, int32_t height, int32_t subpixel, const char *make, const char *model, int32_t transform)
{
}

static void
_ww_background_output_mode(void *data, struct wl_output *output, enum wl_output_mode flags, int32_t width, int32_t height, int32_t refresh)
{
    WwBackgroundOutput *self = data;

    if ( ! ( flags & WL_OUTPUT_MODE_CURRENT ) )
        return;

    self->width = width;
    self->height = height;

    if ( wl_output_get_version(self->output) < WL_OUTPUT_DONE_SINCE_VERSION )
        _ww_background_output_done(self, self->output);
}

static void
_ww_background_output_scale(void *data, struct wl_output *output, int32_t scale)
{
    WwBackgroundOutput *self = data;

    self->scale = scale;
}

static const struct wl_output_listener _ww_background_output_listener = {
    .geometry = _ww_background_output_geometry,
    .mode = _ww_background_output_mode,
    .scale = _ww_background_output_scale,
    .done = _ww_background_output_done,
};

static const char * const _ww_background_cursor_names[] = {
    "left_ptr",
    "default",
    "top_left_arrow",
    "left-arrow",
    NULL
};

static void
_ww_background_registry_handle_global(void *data, struct wl_registry *registry, uint32_t name, const char *interface, uint32_t version)
{
    WwBackgroundContext *self = data;

    if ( strcmp0(interface, "wl_compositor") == 0 )
    {
        self->global_names[WW_BACKGROUND_GLOBAL_COMPOSITOR] = name;
        self->compositor = wl_registry_bind(registry, name, &wl_compositor_interface, MIN(version, WL_COMPOSITOR_INTERFACE_VERSION));
    }
    else if ( strcmp0(interface, "wl_subcompositor") == 0 )
    {
        self->global_names[WW_BACKGROUND_GLOBAL_SUBCOMPOSITOR] = name;
        self->subcompositor = wl_registry_bind(registry, name, &wl_subcompositor_interface, MIN(version, WL_SUBCOMPOSITOR_INTERFACE_VERSION));
    }
    else if ( strcmp0(interface, "zww_background_v2") == 0 )
    {
        self->global_names[WW_BACKGROUND_GLOBAL_BACKGROUND] = name;
        self->background = wl_registry_bind(registry, name, &zww_background_v2_interface, WW_BACKGROUND_INTERFACE_VERSION);
    }
    else if ( strcmp0(interface, "wl_shm") == 0 )
    {
        self->global_names[WW_BACKGROUND_GLOBAL_SHM] = name;
        self->shm = wl_registry_bind(registry, name, &wl_shm_interface, MIN(version, WL_SHM_INTERFACE_VERSION));
    }
    else if ( strcmp0(interface, "wp_viewporter") == 0 )
    {
        self->global_names[WW_BACKGROUND_GLOBAL_VIEWPORTER] = name;
        self->viewporter = wl_registry_bind(registry, name, &wp_viewporter_interface, MIN(version, WP_VIEWPORTER_INTERFACE_VERSION));
    }
    else if ( strcmp0(interface, "wl_seat") == 0 )
    {
        WwBackgroundSeat *seat = ww_new0(WwBackgroundSeat, 1);
        seat->context = self;
        seat->global_name = name;
        seat->seat = wl_registry_bind(registry, name, &wl_seat_interface, MIN(version, WL_SEAT_INTERFACE_VERSION));

        wl_list_insert(&self->seats, &seat->link);

        wl_seat_add_listener(seat->seat, &_ww_background_seat_listener, seat);
    }
    else if ( strcmp0(interface, "wl_output") == 0 )
    {
        WwBackgroundOutput *output = ww_new0(WwBackgroundOutput, 1);
        output->context = self;
        output->global_name = name;
        output->output = wl_registry_bind(registry, name, &wl_output_interface, MIN(version, WL_OUTPUT_INTERFACE_VERSION));
        output->scale = 1;

        wl_list_insert(&self->outputs, &output->link);

        wl_output_add_listener(output->output, &_ww_background_output_listener, output);
    }

    if ( ( self->cursor.theme == NULL ) && ( self->compositor != NULL ) && ( self->shm != NULL ) )
    {
        self->cursor.theme = wl_cursor_theme_load(self->cursor.theme_name, 32, self->shm);
        if ( self->cursor.theme != NULL )
        {
            const char * const *cname = (const char * const *) self->cursor.name;
            for ( cname = ( cname != NULL ) ? cname : _ww_background_cursor_names ; ( self->cursor.cursor == NULL ) && ( *cname != NULL ) ; ++cname )
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
_ww_background_registry_handle_global_remove(void *data, struct wl_registry *registry, uint32_t name)
{
    WwBackgroundContext *self = data;

    WwBackgroundGlobalName i;
    for ( i = 0 ; i < _WW_BACKGROUND_GLOBAL_SIZE ; ++i )
    {
        if ( self->global_names[i] != name )
            continue;
        self->global_names[i] = 0;

        switch ( i )
        {
        case WW_BACKGROUND_GLOBAL_COMPOSITOR:
            wl_compositor_destroy(self->compositor);
            self->compositor = NULL;
        break;
        case WW_BACKGROUND_GLOBAL_SUBCOMPOSITOR:
            wl_subcompositor_destroy(self->subcompositor);
            self->subcompositor = NULL;
        break;
        case WW_BACKGROUND_GLOBAL_BACKGROUND:
            zww_background_v2_destroy(self->background);
            self->background = NULL;
        break;
        case WW_BACKGROUND_GLOBAL_SHM:
            wl_shm_destroy(self->shm);
            self->shm = NULL;
        break;
        case WW_BACKGROUND_GLOBAL_VIEWPORTER:
            wp_viewporter_destroy(self->viewporter);
            self->viewporter = NULL;
        break;
        case _WW_BACKGROUND_GLOBAL_SIZE:
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

    WwBackgroundSeat *seat, *tmp_seat;
    wl_list_for_each_safe(seat, tmp_seat, &self->seats, link)
    {
        if ( seat->global_name != name )
            continue;

        _ww_background_seat_release(seat);
        return;
    }

    WwBackgroundOutput *output, *tmp_output;
    wl_list_for_each_safe(output, tmp_output, &self->outputs, link)
    {
        if ( output->global_name != name )
            continue;

        _ww_background_output_release(output);
        return;
    }
}

static const struct wl_registry_listener _ww_background_registry_listener = {
    .global = _ww_background_registry_handle_global,
    .global_remove = _ww_background_registry_handle_global_remove,
};


static void
_ww_background_disconnect(WwBackgroundContext *self)
{
    WwBackgroundSeat *seat, *tmp;
    wl_list_for_each_safe(seat, tmp, &self->seats, link)
        _ww_background_seat_release(seat);

    WwBackgroundGlobalName i;
    for ( i = 0 ; i < _WW_BACKGROUND_GLOBAL_SIZE ; ++i )
    {
        if ( self->global_names[i] != 0 )
            _ww_background_registry_handle_global_remove(self, self->registry, self->global_names[i]);
    }

    wl_registry_destroy(self->registry);
    self->registry = NULL;

    wl_display_disconnect(self->display);
    self->display = NULL;
}

int
main(int argc, char *argv[])
{
    static WwBackgroundContext self_;
    WwBackgroundContext *self = &self_;

    self->width = 1920;
    self->height = 1080;

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

    wl_list_init(&self->seats);
    wl_list_init(&self->outputs);

    int arg;
    while ( ( arg = getopt(argc, argv, "c:w:h:f:C:") ) != -1 )
    {
        bool good = false;
        switch ( arg )
        {
        case 'c':
            if ( _ww_parse_colour(optarg, &self->colour) )
                good = true;
        break;
        case 'w':
        {
            char *e;
            errno = 0;
            self->width = strtoul(optarg, &e, 10);
            if ( ( e != optarg ) && ( errno == 0 ) )
                good = true;
        }
        break;
        case 'h':
        {
            char *e;
            errno = 0;
            self->height = strtoul(optarg, &e, 10);
            if ( ( e != optarg ) && ( errno == 0 ) )
                good = true;
        }
        break;
#ifdef ENABLE_IMAGES
        case 'f':
        {
            GError *error = NULL;
            GdkPixbufFormat *format;
            format = gdk_pixbuf_get_file_info(optarg, NULL, NULL);
            if ( format != NULL )
            {
                self->image = optarg;
                self->image_scalable = gdk_pixbuf_format_is_scalable(format);
                self->pixbuf = gdk_pixbuf_new_from_file(self->image, &error);
                if ( self->pixbuf == NULL )
                {
                    ww_warning("Couldn’t load image: %s", error->message);
                    g_error_free(error);
                }
                good = true;
            }
        }
#endif /* ENABLE_IMAGES */
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
                "\n    %s [OPTION...] - Demo client for Wayland Wall background protocol"
                "\n"
                "\nOptions:"
                "\n    -c <colour>      Colour to use as background, defaults to #000000"
                "\n    -w <size>        Width of the buffer to create"
                "\n    -h <size>        Height of the buffer to create"
#ifdef ENABLE_IMAGES
                "\n    -f <file>        File to use as background image"
#endif /* ENABLE_IMAGES */
                "\n    -C <name>        The cursor theme to use"
                "\n"
                "\nFormats:"
                "\n    Colours options supports #RRGGBB(AA) and #RGB(A) formats"
                "\n\n", argv[0]);
            return 3;
        }
    }

    self->registry = wl_display_get_registry(self->display);
    wl_registry_add_listener(self->registry, &_ww_background_registry_listener, self);
    wl_display_roundtrip(self->display);

    if ( self->shm == NULL )
    {
        _ww_background_disconnect(self);
        ww_warning("No wl_shm interface provided by the compositor");
        return 3;
    }
    if ( self->background == NULL )
    {
        _ww_background_disconnect(self);
        ww_warning("No ww_background interface provided by the compositor");
        return 3;
    }

    if ( ! _ww_background_create_buffer(self, self->width, self->height) )
        return 4;

    int ret;
    do
        ret = wl_display_dispatch(self->display);
    while ( ret > 0 );
    if ( ret < 0 )
        ww_warning("Couldn’t dispatch events: %s", strerror(errno));

    return 0;
}
