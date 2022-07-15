let Post = ../dhall/Post.dhall
let Tag = ../dhall/Tag.dhall

let content = ''
# h1 Hello world!

Hello!
''

in
  { title = "Hello World"
  , published = "1970-01-01"
  , slug = "hello-world"
  , tags = [ Tag.A, Tag.C ]
  , content
  } : Post
