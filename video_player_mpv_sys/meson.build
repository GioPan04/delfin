project('video-player-mpv', 'c')


depgtk = dependency('gtk4')
depepoxy = dependency('epoxy')

lib = static_library(
  'video-player-mpv',
  'video-player-mpv/video-player-mpv.c',
  dependencies: [depgtk, depepoxy],
)

generate_gir = get_option('generate_gir')
if generate_gir
  gnome = import('gnome')
  gir_files = gnome.generate_gir(
    lib,
    sources: [
      'video-player-mpv/video-player-mpv.c',
      'video-player-mpv/video-player-mpv.h',
    ],
    dependencies: [depgtk, depepoxy],
    includes: ['Gtk-4.0'],
    namespace: 'Vpm',
    nsversion: '0.0',
    identifier_prefix: ['vpm', 'Vpm'],
    header: 'video-player-mpv/video-player-mpv.h',
    install: false,
  )
endif
