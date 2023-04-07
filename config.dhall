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
        , hero_model = "openjourney-v4"
        }
      , { title = "From Arch to NixOS"
        , published = "2022-07-18"
        , slug = "arch-to-nixos"
        , tags = [ Tag.Nix ]
        , content = "./hord/from-arch-to-nixos.md"
        , hero_model = "openjourney-v4"
        }
      , { title = "Consistent Terraform Environments with Nix and Sops"
        , published = "2022-08-29"
        , slug = "terraform-nix-sops"
        , tags = [ Tag.Nix, Tag.DevOps ] : List Tag
        , content = "./hord/terraform-nix-sops.md"
        , hero_model = "openjourney-v4"
        }
      , { title = "Wending and Back"
        , published = "2022-12-06"
        , slug = "wending-and-back"
        , tags = [] : List Tag
        , content = "./hord/wending-and-back.md"
        , hero_model = "openjourney-v4"
        }
      , { title = "2023 Resolutions"
        , published = "2023-01-04"
        , slug = "2023-resolutions"
        , tags = [ Tag.Personal ] : List Tag
        , content = "./hord/2023-resolutions.md"
        , hero_model = "openjourney-v4"
        }
      ]

in    { hord
      , drv
      , build_dir
      , hord_dir = "gewritu"
      , tag_dir = "tags"
      , repo = "https://github.com/autophagy/wordhord"
      , url = "https://wordhord.autophagy.io"
      }
    : Config
