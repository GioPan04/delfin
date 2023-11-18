config = configure_file(
  input: 'src/meson_config.rs.in',
  output: 'meson_config.rs',
  configuration: {
    'APP_ID': app_id,
  },
)

run_command(
  'cp',
  meson.project_build_root() / 'delfin' / 'meson_config.rs',
  meson.project_source_root() / 'delfin' / 'src' / 'meson_config.rs',
  check: true
)

cargo_options = []

rust_target = 'debug'

if get_option('profile') == 'release'
  cargo_options += ['--release']
  rust_target = 'release'
endif

cargo_options += ['--target-dir', meson.project_build_root() / 'delfin']

prefix = get_option('prefix')
bindir = prefix / get_option('bindir')

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
  install: get_option('flatpak'),
  install_dir: bindir,
)
