use cfg_if::cfg_if;
use comrak::ComrakOptions;
use glob::glob;
use gray_matter;
use leptos::*;
use serde::{Deserialize, Serialize};
use std::fs;

cfg_if! {

    if #[cfg(feature = "ssr")] {
         pub fn register_server_functions() {
               // Silence clippy with the _
            _ = GetPosts::register();
            _ = GetPost::register();
            _ = ToggleDarkMode::register();
         }

        #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
        pub struct Post {
            id: u16,
            pub title: String,
            pub slug: String,
            pub date: String,
            pub content: String,
        }
    } else {
        #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
        pub struct Post {
            id: u16,
            pub title: String,
            pub slug: String,
            pub date: String,
            pub content: String,
        }
    }
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
pub async fn get_posts(_cx: Scope) -> Result<Vec<Post>, ServerFnError> {
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

#[server(ToggleDarkMode, "/api")]
pub async fn toggle_dark_mode(cx: Scope, prefers_dark: bool) -> Result<bool, ServerFnError> {
    use axum::http::header::{HeaderMap, HeaderValue, SET_COOKIE};
    use leptos_axum::{ResponseOptions, ResponseParts};

    let response =
        use_context::<ResponseOptions>(cx).expect("to have leptos_actix::ResponseOptions provided");
    let mut response_parts = ResponseParts::default();
    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        HeaderValue::from_str(&format!("darkmode={prefers_dark}; Path=/"))
            .expect("to create header value"),
    );
    response_parts.headers = headers;

    std::thread::sleep(std::time::Duration::from_millis(250));

    response.overwrite(response_parts);
    Ok(prefers_dark)
}
