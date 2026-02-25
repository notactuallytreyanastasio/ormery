package = 'ormery'
version = 'dev-1'
rockspec_format = '3.0'
description = {
}
build = {
    type = 'builtin',
    copy_directories = {'.'},
    modules = {},
}
dependencies = {
    'lua >= 5.1',
    'temper-core == 0.6.0',
    'std == 0.6.0',
}
source = {
    url = 'file:///Users/robertgrayson/code/ormery/temper.out/lua/ormery/ormery.zip',
}
