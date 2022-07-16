use comrak::{markdown_to_html, ComrakOptions};
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;
use tinytemplate::{format_unescaped, TinyTemplate};

#[derive(Clone, Deserialize, Serialize, Debug)]
enum Tag {
    A,
    B,
    C,
    D,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
struct Post {
    title: String,
    published: String,
    slug: String,
    tags: Vec<Tag>,
    content: String,
}


#[derive(Clone, Serialize, Debug)]
struct Index<'a> {
    posts: &'a Vec<Post>
}

fn main() -> Result<(), Box<dyn Error>> {
    let build_dir = env::var("BUILDDIR")?;
    if Path::new(&build_dir).exists() {
        fs::remove_dir_all(&build_dir)?;
    }
    fs::create_dir(&build_dir)?;

    let mut tt = TinyTemplate::new();
    tt.set_default_formatter(&format_unescaped);

    let s = fs::read_to_string("./bisenum/post.html")?;
    tt.add_template("post", &s)?;

    let i = fs::read_to_string("./bisenum/index.html")?;
    tt.add_template("index", &i)?;


    let mut options = ComrakOptions::default();
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.header_ids = Some("".to_string());
    options.extension.footnotes = true;
    options.render.github_pre_lang = true;
    options.render.escape = true;

    let paths = fs::read_dir("./hord")?;

    let mut posts: Vec<Post> = Vec::new();
    for path in paths {
        let mut b: Post = serde_dhall::from_file(path.unwrap().path()).parse()?;
        b.content = markdown_to_html(&b.content, &ComrakOptions::default());
        posts.push(b);
    }

    let index = Index { posts: &posts };
    let rendered_index = tt.render("index", &index)?;
    fs::write(format!("{}/index.html", build_dir), rendered_index)?;

    fs::create_dir(format!("{}/gewritu", &build_dir))?;
    for post in posts {
        let rendered = tt.render("post", &post)?;
        fs::write(format!("{}/gewritu/{}.html", build_dir, post.slug), rendered)?;
    }

    Ok(())
}
