let Post = ../dhall/types/Post.dhall
let Tag = ../dhall/types/Tag.dhall

in
  { title = "Wes Hāl, Wordhord"
  , published = "2022-07-17"
  , slug = "wes-hal-wordhord"
  , tags = [ ] : List Tag
  , content = "./texts/wes-hal-wordhord.md"
  } : Post
