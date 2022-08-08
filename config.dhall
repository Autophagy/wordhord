let Config = ./dhall/types/Config.dhall

let Tag = ./dhall/types/Tag.dhall

let drv = env:DRV as Text ? ""

let build_dir = env:BUILDDIR as Text ? "./build"

let hord =
      [ { title = "Wes Hāl, Wordhord"
        , published = "2022-07-17"
        , slug = "wes-hal-wordhord"
        , tags = [] : List Tag
        , content = "./hord/wes-hal-wordhord.md"
        }
      , { title = "From Arch to NixOS"
        , published = "2022-07-18"
        , slug = "arch-to-nixos"
        , tags = [ Tag.Nix ]
        , content = "./hord/from-arch-to-nixos.md"
        }
      ]

in    { hord
      , drv
      , build_dir
      , hord_dir = "gewritu"
      , tag_dir = "tags"
      , repo = "https://github.com/autophagy/wordhord"
      }
    : Config
