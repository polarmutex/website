use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Post {
    id: u16,
    pub title: String,
    pub slug: String,
    pub date: String,
    pub content: String,
}

const _404_PAGE: &str = "---\ntitle = 404\n---\n404 Not found.";

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FrontMatter {
    pub title: String,
    pub description: String,
    pub published: String,
    pub featured: bool,
    pub draft: bool,
    pub category: String,
    //pub slug: String,
}

#[server(GetPosts, "/api")]
pub async fn get_posts() -> Result<Vec<Post>, ServerFnError> {
    use glob::glob;
    use gray_matter;
    use std::fs;

    let posts: Vec<Post> = glob("content/ideas/*.md")
        .unwrap()
        .filter_map(Result::ok)
        .map(|filename| {
            let page = &fs::read_to_string(&filename).unwrap_or(_404_PAGE.to_string());
            let front_matter: gray_matter::Matter<gray_matter::engine::YAML> =
                gray_matter::Matter::new();
            let front_matter: gray_matter::ParsedEntity = front_matter.parse(page);
            let front_matter: FrontMatter = front_matter.data.unwrap().deserialize().unwrap();

            Post {
                id: 20,
                title: front_matter.title,
                slug: filename
                    .with_extension("")
                    .into_os_string()
                    .into_string()
                    .unwrap_or("not-found".to_string())
                    .replace("content/ideas/", ""),
                date: front_matter.published,
                content: "".to_string(),
            }
        })
        .collect();
    //std::thread::sleep(std::time::Duration::from_millis(1000));
    Ok(posts)
}

#[server(GetPost, "/api")]
pub async fn get_post(slug: String) -> Result<Option<Post>, ServerFnError> {
    use comrak::ComrakOptions;
    use std::fs;

    let page = &fs::read_to_string(format!("content/ideas/{}.md", slug));

    if page.is_ok() {
        let front_matter: gray_matter::Matter<gray_matter::engine::YAML> =
            gray_matter::Matter::new();
        let front_matter: gray_matter::ParsedEntity = front_matter.parse(page.as_ref().unwrap());
        let front_matter: FrontMatter = front_matter.data.unwrap().deserialize().unwrap();

        let mut options = ComrakOptions::default();
        options.extension.front_matter_delimiter = Some("---".to_owned());
        options.extension.footnotes = true;
        options.extension.header_ids = Some("heading-".to_string());
        options.extension.tasklist = true;
        options.extension.autolink = true;
        options.extension.table = true;

        let post = Post {
            id: 20,
            title: front_matter.title,
            slug,
            date: front_matter.published,
            content: comrak::markdown_to_html(page.as_ref().unwrap(), &options),
        };
        Ok(Some(post))
    } else {
        Ok(None)
    }
}
