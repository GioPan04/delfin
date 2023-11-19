project(
  'delfin',
  'rust',
  version: '0.1.0',
  # bad hack to compile libvideo-player-mpv
  subproject_dir: 'video_player_mpv',
)

dependency('gtk4', version: '>= 4.0.0')
dependency('libadwaita-1', version: '>= 1.4')
dependency('mpv', version: '>= 2.1.0')

base_id = 'cafe.avery.Delfin'

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
bindir = prefix / get_option('bindir')
datadir = prefix / get_option('datadir')
pkgdatadir = datadir / meson.project_name()
appsdir = datadir / 'applications'
iconsdir = datadir / 'icons'

subproject('sys')
subdir('delfin')
subdir('data')

gnome = import('gnome')
gnome.post_install(
  gtk_update_icon_cache: true,
  update_desktop_database: true,
)
