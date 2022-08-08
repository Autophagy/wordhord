use chrono::NaiveDate;
use comrak::plugins::syntect::SyntectAdapter;
use comrak::{markdown_to_html_with_plugins, ComrakOptions, ComrakPlugins};
use serde::Serialize;
use std::error::Error;
use std::fs;
use std::path::Path;
use tinytemplate::{format_unescaped, TinyTemplate};

use crate::config::{Config, Tag};
use crate::templates;

#[derive(Serialize)]
struct TagPage<'a> {
    tag: &'a Tag,
    posts: &'a Vec<Post<'a>>,
    config: &'a Config,
}

#[derive(Serialize, Clone)]
struct Post<'a> {
    title: &'a str,
    published: NaiveDate,
    slug: &'a str,
    tags: &'a Vec<Tag>,
    content: String,
    read_time: usize,
}

#[derive(Serialize)]
struct PostPage<'a> {
    post: &'a Post<'a>,
    config: &'a Config,
}

#[derive(Serialize)]
struct Index<'a> {
    posts: &'a Vec<Post<'a>>,
    config: &'a Config,
}

/// Returns an estimated read time for a given string, assuming a reading speed
/// of 200 words per minute, rounded to the nearest minute.
fn estimate_read_time(s: &str) -> usize {
    let wpm = 200;
    let mut total_words = 0;
    let mut previous_char = ' ';
    for chr in s.chars() {
        if previous_char.is_ascii_whitespace()
            && (chr.is_ascii_alphabetic() || chr.is_ascii_digit() || chr.is_ascii_punctuation())
        {
            total_words += 1;
        }
        previous_char = chr;
    }

    let quotient = total_words / wpm;
    let remainder = total_words % wpm;

    if remainder < (wpm / 2) {
        quotient
    } else {
        quotient + 1
    }
}

pub fn create_build_dir(config: &Config) -> std::io::Result<()> {
    if Path::new(&config.build_dir).exists() {
        fs::remove_dir_all(&config.build_dir)?;
    }
    fs::create_dir(&config.build_dir)
}

fn create_templater<'a>() -> Result<TinyTemplate<'a>, Box<dyn Error>> {
    let mut tt = TinyTemplate::new();
    tt.set_default_formatter(&format_unescaped);
    tt.add_template("index", templates::INDEX)?;
    tt.add_template("post", templates::POST)?;
    tt.add_template("tag", templates::TAG)?;
    tt.add_template("footer", templates::FOOTER)?;
    Ok(tt)
}

fn create_markdown_options() -> ComrakOptions {
    let mut options = ComrakOptions::default();
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.header_ids = Some("".to_string());
    options.extension.footnotes = true;
    options.render.github_pre_lang = true;
    options.render.escape = true;
    options
}

fn render_posts<'a>(
    hord: &'a Vec<crate::config::Post>,
    options: &'a ComrakOptions,
    plugins: &'a ComrakPlugins,
) -> std::io::Result<Vec<Post<'a>>> {
    let mut posts: Vec<Post> = Vec::new();
    for hord_post in hord {
        let content = fs::read_to_string(&hord_post.content)?;
        posts.push(Post {
            title: &hord_post.title,
            published: hord_post.published,
            slug: &hord_post.slug,
            tags: &hord_post.tags,
            content: markdown_to_html_with_plugins(&content, options, plugins),
            read_time: estimate_read_time(&content),
        });
    }
    posts.sort_by(|a, b| b.published.cmp(&a.published));
    Ok(posts)
}

pub fn build_wordhord(config: &Config) -> Result<(), Box<dyn Error>> {
    let tt = create_templater()?;
    let options = create_markdown_options();
    let adapter = SyntectAdapter::new("base16-eighties.dark");
    let mut plugins = ComrakPlugins::default();
    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    let posts = render_posts(&config.hord, &options, &plugins)?;

    let rendered_index = tt.render(
        "index",
        &Index {
            posts: &posts,
            config,
        },
    )?;
    fs::write(format!("{}/index.html", &config.build_dir), rendered_index)?;

    let hord_path = format!("{}/{}", &config.build_dir, &config.hord_dir);
    fs::create_dir(&hord_path)?;
    for post in &posts {
        let rendered = tt.render("post", &PostPage { post, config })?;
        fs::write(format!("{}/{}.html", &hord_path, post.slug), rendered)?;
    }

    let tag_path = format!("{}/{}", &config.build_dir, &config.tag_dir);
    fs::create_dir(&tag_path)?;
    for tag in Tag::iterator() {
        let rendered_tag = tt.render(
            "tag",
            &TagPage {
                tag,
                posts: &posts
                    .clone()
                    .into_iter()
                    .filter(|p| p.tags.contains(tag))
                    .collect(),
                config,
            },
        )?;
        fs::write(format!("{}/{}.html", &tag_path, tag), rendered_tag)?;
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_time_estimation() {
        assert_eq!(estimate_read_time(&"Word ".repeat(0)), 0);
        assert_eq!(estimate_read_time(&"Word ".repeat(200)), 1);
        assert_eq!(estimate_read_time(&"Word ".repeat(2000)), 10);
        assert_eq!(estimate_read_time(&"Word ".repeat(20000)), 100);
        assert_eq!(estimate_read_time(&"Word ".repeat(299)), 1);
        assert_eq!(estimate_read_time(&"Word ".repeat(300)), 2);
        assert_eq!(estimate_read_time(&"Word ".repeat(301)), 2);
    }
}
