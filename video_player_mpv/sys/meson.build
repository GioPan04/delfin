project('video-player-mpv', 'c')

depgtk = dependency('gtk4')
depepoxy = dependency('epoxy')
depmpv = dependency('mpv')

lib = static_library(
  'video-player-mpv',
  'video-player-mpv/video-player-mpv.c',
  'video-player-mpv/track-list.c',
  'video-player-mpv/track.c',
  dependencies: [depgtk, depepoxy, depmpv],
)

generate_gir = get_option('generate_gir')
if generate_gir
  gnome = import('gnome')
  gir_files = gnome.generate_gir(
    lib,
    sources: [
      'video-player-mpv/video-player-mpv.c',
      'video-player-mpv/video-player-mpv.h',
      'video-player-mpv/track-list.c',
      'video-player-mpv/track-list.h',
      'video-player-mpv/track.c',
      'video-player-mpv/track.h',
    ],
    dependencies: [depgtk, depepoxy, depmpv],
    includes: ['Gtk-4.0'],
    namespace: 'Vpm',
    nsversion: '0.0',
    identifier_prefix: ['vpm', 'Vpm'],
    header: 'video-player-mpv/video-player-mpv.h',
    install: false,
  )
endif
