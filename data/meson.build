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

# Validate MetaInfo file
# Skipping this for now - we use developerr_name, which is deprecated but still used by libadwaita
# if ascli_exe.found()
#   test('validate metainfo file',
#         ascli_exe,
#         args: ['validate',
#                '--no-net',
#                '--strict',
#                metainfo_file]
#   )
# endif

install_data(
  metainfo_file,
  install_dir: metainfodir,
)

desktop_file = '@0@.desktop'.format(app_id)

# Create desktop-entry file from metainfo
custom_target('gen-desktop-entry',
    input : [metainfo_file],
    output : [desktop_file],
    command : [ascli_exe, 'make-desktop-file', '@INPUT@', '@OUTPUT@'],
    install: true,
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
resources = gnome.compile_resources(
  'resources',
  resources_file,
  gresource_bundle: true,
  source_dir: meson.current_build_dir(),
  install: true,
  install_dir: pkgdatadir,
)
