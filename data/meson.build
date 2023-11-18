subdir('icons')

ascli_exe = find_program('appstreamcli')

metainfo_file = 'cafe.avery.Delfin.metainfo.xml'

# Validate MetaInfo file
if ascli_exe.found()
  test('validate metainfo file',
        ascli_exe,
        args: ['validate',
               '--no-net',
               '--strict',
               metainfo_file]
  )
endif

# Create desktop-entry file from metainfo
custom_target('gen-desktop-entry',
    input : [metainfo_file],
    output : ['cafe.avery.Delfin.desktop'],
    command : [ascli_exe, 'make-desktop-file', '@INPUT@', '@OUTPUT@'],
    install: true,
    install_dir: join_paths (get_option ('datadir'), 'applications')
)
