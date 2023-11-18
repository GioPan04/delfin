project(
  'delfin',
  'rust',
  version: '0.1.0',
  # bad hack to compile libvideo-player-mpv
  subproject_dir: 'video_player_mpv',
)

gnome = import('gnome')

base_id = 'cafe.avery.Delfin'

cargo = find_program('cargo', required: true)

dependency('gtk4', version: '>= 4.0.0')
dependency('libadwaita-1', version: '>= 1.4')
dependency('mpv', version: '>= 2.1.0')

if get_option('profile') == 'dev'
  app_id = '@0@.@1@'.format(base_id, 'Devel')
else
  app_id = base_id
endif

if get_option('flatpak')
  meson.add_dist_script(
    'build-aux/dist-vendor',
    meson.project_build_root() / 'meson-dist' / meson.project_name() + '-' + meson.project_version(),
    meson.project_source_root()
  )
endif

prefix = get_option('prefix')
datadir = prefix / get_option('datadir')
iconsdir = datadir / 'icons'

subproject('sys')
subdir('delfin')
subdir('data')

gnome.post_install(
  gtk_update_icon_cache: true,
)
