use cfg_if::cfg_if;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde::Serialize;

cfg_if! {
if #[cfg(feature = "ssr")] {
    use octocrab::{models::repos::Content, Octocrab};
    use femark::{process_markdown_to_html_with_frontmatter, HTMLOutput};
    use parking_lot::RwLock;
    use crate::errors::AppError;
    use std::sync::Arc;
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Post {
    pub title: String,
    pub slug: String,
    //pub hero: Option<String>,
    //pub hero_caption: Option<String>,
    //pub hero_alt: Option<String>,
    //pub excerpt: Option<String>,
    pub content: String,
    //pub toc: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub published: bool,
    //pub preview: bool,
    //pub links: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PostFrontmatter {
    title: String,
    slug: String,
    //hero: Option<String>,
    //hero_alt: Option<String>,
    //hero_caption: Option<String>,
    //excerpt: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    published: bool,
    //preview: bool,
    tags: Vec<String>,
}

cfg_if! {
if #[cfg(feature = "ssr")] {

impl TryFrom<String> for Post {
    type Error = AppError;

    fn try_from(post_string: String) -> Result<Self, Self::Error> {
        let HTMLOutput {
            content,
            //toc,
            frontmatter,
            ..
        } = process_markdown_to_html_with_frontmatter(&post_string, true).unwrap_or_default();

        let frontmatter = match frontmatter {
                    Some(f) => f,
                    None => return Err(AppError::MissingOrInvalidFrontmatter)
                };
        let code_block = frontmatter.code_block.ok_or(AppError::MissingOrInvalidFrontmatter)?;
        let fm: PostFrontmatter = toml::from_str(&code_block.source)?;

        Ok(Self {
            title: fm.title,
            slug: fm.slug,
            //hero: fm.hero,
            //hero_caption: fm.hero_caption,
            //hero_alt: fm.hero_alt,
            //excerpt: Some(fm.excerpt),
            content,
            //toc,
            created_at: fm.created_at,
            updated_at: fm.updated_at,
            published: fm.published,
            //preview: fm.preview,
            //links: None,
            tags: fm.tags,
        })
    }
}
}}
