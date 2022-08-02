let Post = ../dhall/types/Post.dhall
let Tag = ../dhall/types/Tag.dhall

in
  { title = "From Arch to NixOS"
  , published = "2022-07-18"
  , slug = "arch-to-nixos"
  , tags = [ Tag.Nix ]
  , content = "./texts/from-arch-to-nixos.md"
  } : Post
