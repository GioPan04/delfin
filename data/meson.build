gnome = import('gnome')

subdir('icons')

ascli_exe = find_program('appstreamcli')

metainfo_file = configure_file(
  input: 'cafe.avery.Delfin.metainfo.xml.in',
  output: '@0@.metainfo.xml'.format(app_id),
  configuration: {
    'APP_ID': app_id,
  },
)

if ascli_exe.found()
  test('validate metainfo file',
    ascli_exe,
    args: ['validate',
      '--no-net',
      '--strict',
      metainfo_file]
  )
endif

install_data(
  metainfo_file,
  install_dir: metainfodir,
)

desktop_file = configure_file(
  input: 'cafe.avery.Delfin.desktop.in',
  output: '@0@.desktop'.format(app_id),
  configuration: {
    'APP_ID': app_id,
  },
)

install_data(
  desktop_file,
  install_dir: appsdir,
)

resources_file = configure_file(
  input: 'cafe.avery.Delfin.gresource.xml.in',
  output: '@0@.gresource.xml'.format(app_id),
  configuration: {
    'APP_ID': app_id,
  },
)

# Compile and install resources
gnome.compile_resources(
  'resources',
  resources_file,
  gresource_bundle: true,
  source_dir: meson.current_build_dir(),
  install: true,
  install_dir: pkgdatadir,
)
