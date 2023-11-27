config = configure_file(
  input: 'src/meson_config.rs.in',
  output: 'meson_config.rs',
  configuration: {
    'APP_ID': app_id,
    'BUILDDIR': meson.project_build_root(),
    'PKGDATADIR': pkgdatadir,
    'VERSION': version,
  },
)

run_command(
  'cp',
  meson.project_build_root() / 'delfin' / 'meson_config.rs',
  meson.project_source_root() / 'delfin' / 'src' / 'meson_config.rs',
  check: true
)

cargo = find_program('cargo', required: true)

cargo_options = ['--target-dir', meson.project_build_root() / 'delfin']
rust_target = 'debug'

if get_option('profile') == 'release'
  cargo_options += ['--release']
  rust_target = 'release'
endif

custom_target(
  'cargo-build',
  build_by_default: true,
  build_always_stale: true,
  output: meson.project_name(),
  console: true,
  command: [
    cargo, 'build', cargo_options,
    '&&', 'cp', 'delfin' / rust_target / meson.project_name(), '@OUTPUT@',
  ],
  install: true,
  install_dir: bindir,
  env: {
    'MESON_BUILD_ROOT': meson.project_build_root(),
    'FLATPAK': get_option('flatpak') ? 'true' : 'false',
  },
)
