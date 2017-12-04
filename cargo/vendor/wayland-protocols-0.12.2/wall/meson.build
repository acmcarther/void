project('wayland-wall', 'c',
    version: '1',
    meson_version: '>=0.39.1',
    license: 'MIT',
    default_options: [
        'c_std=gnu11',
        'warning_level=2',
    ],
)

c_compiler = meson.get_compiler('c')
wayland_scanner = find_program('wayland-scanner', required: get_option('enable-clients'))

unstable_protocols = [
    # [ 'name', [ 'v1', 'v2' ] ],
    [ 'background', [ 'v1', 'v2' ] ],
    [ 'dock-manager', [ 'v1', 'v2' ] ],
    [ 'launcher-menu', [ 'v1' ] ],
    [ 'notification-area', [ 'v1' ] ],
    [ 'window-switcher', [ 'v1' ] ],
]

stable_protocols = [
    # 'name',
]

foreach p : unstable_protocols
    name = p[0]
    foreach version : p[1]
        file = join_paths('unstable', name, '@0@-unstable-@1@.xml'.format(name, version))
        install_data(
            file,
            install_dir: join_paths(get_option('datadir'), meson.project_name(), 'unstable', name),
        )
        if wayland_scanner.found()
            test('@0@ @1@ client header'.format(name, version), wayland_scanner,
                args: [ 'client-header', join_paths(meson.source_root(), file), '/dev/null' ]
            )
            test('@0@ @1@ server header'.format(name, version), wayland_scanner,
                args: [ 'server-header', join_paths(meson.source_root(), file), '/dev/null' ]
            )
            test('@0@ @1@ code'.format(name, version), wayland_scanner,
                args: [ 'code', join_paths(meson.source_root(), file), '/dev/null' ]
            )
        endif
    endforeach
endforeach

foreach name : stable_protocols
    file = join_paths('stable', name, '@0@.xml'.format(name))
    install_data(
        file,
        install_dir: join_paths(get_option('datadir'), meson.project_name(), 'stable', name),
    )
    if wayland_scanner.found()
        test('@0@ client header'.format(name), wayland_scanner,
            args: [ 'client-header', join_paths(meson.source_root(), file), '/dev/null' ]
        )
        test('@0@ server header'.format(name), wayland_scanner,
            args: [ 'server-header', join_paths(meson.source_root(), file), '/dev/null' ]
        )
        test('@0@ code'.format(name), wayland_scanner,
            args: [ 'code', join_paths(meson.source_root(), file), '/dev/null' ]
        )
    endif
endforeach

header_conf = configuration_data()
other_conf = configuration_data()
header_conf.set_quoted('PACKAGE_NAME', meson.project_name())
header_conf.set_quoted('PACKAGE_VERSION', meson.project_version())
header_conf.set_quoted('GETTEXT_PACKAGE', meson.project_name())
other_conf.set('PACKAGE_NAME', meson.project_name())
other_conf.set('PACKAGE_VERSION', meson.project_version())

header_conf.set_quoted('EVENTD_SYSCONFDIR', join_paths(get_option('prefix'), get_option('sysconfdir')))
header_conf.set_quoted('EVENTD_BINDIR', join_paths(get_option('prefix'), get_option('bindir')))
header_conf.set_quoted('EVENTD_LIBDIR', join_paths(get_option('prefix'), get_option('libdir')))
header_conf.set_quoted('EVENTD_DATADIR', join_paths(get_option('prefix'), get_option('datadir')))
header_conf.set_quoted('EVENTD_LOCALEDIR', join_paths(get_option('prefix'), get_option('localedir')))
other_conf.set('pkgdatadir', join_paths(get_option('prefix'), get_option('datadir'), meson.project_name()))

config_h = configure_file(output: 'config.h', configuration: header_conf)
configure_file(
    input: 'wayland-wall.pc.in',
    output: 'wayland-wall.pc',
    configuration: other_conf,
    install: true,
    install_dir: join_paths(get_option('datadir'), 'pkgconfig'),
)

flags = [
    '-Wformat=2',
    '-Wno-unused-parameter',
]
foreach f : flags
    if c_compiler.has_argument(f)
        add_project_arguments(f, language: 'c')
    endif
endforeach

if get_option('enable-clients')
    wayland_min_version='1.9.91'

    headers = [
        'locale.h',
        'limits.h',
        'stdio.h',
        'stdint.h',
        'stdbool.h',
        'fcntl.h',
        'errno.h',
        'assert.h',
        'sys/mman.h',
    ]
    foreach h : headers
        if not c_compiler.has_header(h)
            error('Header @0@ was not found, but is required'.format(h))
        endif
    endforeach
    dependencies = [
        dependency('wayland-client', version: '>=@0@'.format(wayland_min_version)),
        dependency('wayland-cursor'),
        dependency('cairo'),
    ]

    wayland_protocols = dependency('wayland-protocols')
    wp_protocol_dir = wayland_protocols.get_pkgconfig_variable('pkgdatadir')

    wayland_scanner_client = generator(wayland_scanner, output: '@BASENAME@-client-protocol.h', arguments: ['client-header', '@INPUT@', '@OUTPUT@'])
    wayland_scanner_server = generator(wayland_scanner, output: '@BASENAME@-server-protocol.h', arguments: ['server-header', '@INPUT@', '@OUTPUT@'])
    wayland_scanner_code = generator(wayland_scanner, output: '@BASENAME@-protocol.c', arguments: ['code', '@INPUT@', '@OUTPUT@'])

    if get_option('enable-images') != 'false'
        gdk_pixbuf = dependency('gdk-pixbuf-2.0', required: get_option('enable-images') == 'true')
        if gdk_pixbuf.found()
            add_project_arguments('-DENABLE_IMAGES', language: 'c')
            dependencies += gdk_pixbuf
        endif
    endif

    executable('ww-background', [
            'src/background.c',
            wayland_scanner_client.process(join_paths(meson.source_root(), 'unstable', 'background', 'background-unstable-v2.xml')),
            wayland_scanner_code.process(join_paths(meson.source_root(), 'unstable', 'background', 'background-unstable-v2.xml')),
            wayland_scanner_client.process(join_paths(wp_protocol_dir, 'stable', 'viewporter', 'viewporter.xml')),
            wayland_scanner_code.process(join_paths(wp_protocol_dir, 'stable', 'viewporter', 'viewporter.xml')),
        ],
        dependencies: dependencies,
        install: true,
    )

    if get_option('enable-text') != 'false'
        pango = dependency('pango', required: get_option('enable-text') == 'true')
        if pango.found()
            text_dependencies = [ pango, dependency('pangocairo') ]

            executable('ww-dock', [
                    'src/dock.c',
                    wayland_scanner_client.process(join_paths(meson.source_root(), 'unstable', 'dock-manager', 'dock-manager-unstable-v2.xml')),
                    wayland_scanner_code.process(join_paths(meson.source_root(), 'unstable', 'dock-manager', 'dock-manager-unstable-v2.xml')),
                ],
                dependencies: dependencies + text_dependencies,
                install: true,
            )
        endif
    endif
endif
