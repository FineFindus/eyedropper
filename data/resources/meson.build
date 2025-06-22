# Compile blueprints
blueprints = custom_target(
  'blueprints',
  input: files(
    'ui/color-format-row.blp',
    'ui/history-item.blp',
    'ui/placeholder-page.blp',
    'ui/preferences.blp',
    'ui/shortcuts.blp',
    'ui/window.blp',
  ),
  output: '.',
  command: [
    blueprint_compiler,
    'batch-compile',
    '@OUTPUT@',
    '@CURRENT_SOURCE_DIR@',
    '@INPUT@',
  ],
)

# Resources
resources = gnome.compile_resources(
  'resources',
  'resources.gresource.xml',
  gresource_bundle: true,
  source_dir: meson.current_build_dir(),
  install: true,
  install_dir: pkgdatadir,
  dependencies: blueprints,
)
