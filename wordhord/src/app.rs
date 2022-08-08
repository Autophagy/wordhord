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
    tag: Tag,
    posts: Vec<Post>,
    config: &'a Config,
}

#[derive(Serialize, Clone)]
struct Post {
    title: String,
    published: NaiveDate,
    slug: String,
    tags: Vec<Tag>,
    content: String,
    read_time: usize,
}

#[derive(Serialize)]
struct PostPage {
    post: Post,
    config: Config,
}

#[derive(Serialize)]
struct Index<'a> {
    posts: &'a Vec<Post>,
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

pub fn build_wordhord(config: &Config) -> Result<(), Box<dyn Error>> {
    let mut tt = TinyTemplate::new();
    tt.set_default_formatter(&format_unescaped);
    tt.add_template("index", templates::INDEX)?;
    tt.add_template("post", templates::POST)?;
    tt.add_template("tag", templates::TAG)?;
    tt.add_template("footer", templates::FOOTER)?;

    let mut options = ComrakOptions::default();
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.header_ids = Some("".to_string());
    options.extension.footnotes = true;
    options.render.github_pre_lang = true;
    options.render.escape = true;

    let adapter = SyntectAdapter::new("base16-eighties.dark");
    let mut plugins = ComrakPlugins::default();
    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    let mut posts: Vec<Post> = Vec::new();
    for hord_post in &config.hord {
        let content = fs::read_to_string(&hord_post.content)?;
        posts.push(Post {
            title: hord_post.title.clone(),
            published: hord_post.published,
            slug: hord_post.slug.clone(),
            tags: hord_post.tags.clone(),
            content: markdown_to_html_with_plugins(&content, &options, &plugins),
            read_time: estimate_read_time(&content),
        });
    }
    posts.sort_by(|a, b| b.published.cmp(&a.published));

    let index = Index {
        posts: &posts,
        config,
    };
    let rendered_index = tt.render("index", &index)?;
    fs::write(format!("{}/index.html", config.build_dir), rendered_index)?;

    fs::create_dir(format!("{}/gewritu", &config.build_dir))?;
    for post in &posts {
        let post_page: PostPage = PostPage {
            post: post.clone(),
            config: config.clone(),
        };
        let rendered = tt.render("post", &post_page)?;
        fs::write(
            format!("{}/gewritu/{}.html", &config.build_dir, post.slug),
            rendered,
        )?;
    }

    fs::create_dir(format!("{}/tags", &config.build_dir))?;
    for tag in Tag::iterator() {
        let tag_page = TagPage {
            tag: tag.clone(),
            posts: posts
                .clone()
                .into_iter()
                .filter(|p| p.tags.contains(tag))
                .collect(),
            config,
        };
        let rendered_tag = tt.render("tag", &tag_page)?;
        fs::write(
            format!("{}/tags/{}.html", config.build_dir, tag),
            rendered_tag,
        )?;
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
