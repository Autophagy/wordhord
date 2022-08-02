let Post = ./Post.dhall
in
{ hord: List Post
, drv: Text
, build_dir: Text
, repo: Text
}
