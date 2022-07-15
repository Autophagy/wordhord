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
    let paths = fs::read_dir("./hord")?;

    let mut posts: Vec<Post> = Vec::new();
    for path in paths {
        let mut b: Post = serde_dhall::from_file(path.unwrap().path()).parse()?;
        b.content = markdown_to_html(&b.content, &ComrakOptions::default());
        posts.push(b);
    }

    for post in posts {
        let rendered = tt.render("post", &post)?;
        fs::write(format!("{}/{}.html", build_dir, post.slug), rendered)?;
    }

    Ok(())
}
