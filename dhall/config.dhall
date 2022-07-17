let Config = ./types/Config.dhall

let drv = env:DRV as Text ? ""
let build_dir = env:BUILDDIR as Text ? "./build"

in { drv = drv, build_dir = build_dir } : Config
