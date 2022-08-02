let Config = ./dhall/types/Config.dhall

let drv = env:DRV as Text ? ""

let build_dir = env:BUILDDIR as Text ? "./build"

in  { drv, build_dir, repo = "https://github.com/autophagy/wordhord" } : Config
