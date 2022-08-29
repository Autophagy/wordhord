let Post = ./Post.dhall

in  { hord : List Post
    , drv : Text
    , build_dir : Text
    , hord_dir : Text
    , tag_dir : Text
    , repo : Text
    , url : Text
    }
