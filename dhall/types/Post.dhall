let Tag = ./Tag.dhall

in
{ title : Text, published : Text, slug : Text, tags : List Tag, content : Text, hero_model : Text }
