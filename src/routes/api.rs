use cfg_if::cfg_if;
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
            _ = ToggleDarkMode::register();
         }

        #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
        pub struct Post {
            id: u16,
            pub title: String,
            pub slug: String,
            pub date: String,
        }
    } else {
        #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
        pub struct Post {
            id: u16,
            pub title: String,
            pub slug: String,
            pub date: String,
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
pub async fn get_posts(_cx: Scope, filter: String) -> Result<Vec<Post>, ServerFnError> {
    let mut posts: Vec<Post> = glob("content/posts/*.md")
        .unwrap()
        .filter_map(Result::ok)
        .map(|filename| {
            let page = &fs::read_to_string(filename).unwrap_or(_404_PAGE.to_string());
            let front_matter: gray_matter::Matter<gray_matter::engine::YAML> =
                gray_matter::Matter::new();
            let front_matter: gray_matter::ParsedEntity = front_matter.parse(&page);
            let front_matter: FrontMatter = front_matter.data.unwrap().deserialize().unwrap();

            Post {
                id: 20,
                title: front_matter.title,
                slug: "my-slug".to_string(),
                date: front_matter.published,
            }
        })
        .collect();
    //std::thread::sleep(std::time::Duration::from_millis(1000));
    Ok(posts)
}

#[server(ToggleDarkMode, "/api")]
pub async fn toggle_dark_mode(cx: Scope, prefers_dark: bool) -> Result<bool, ServerFnError> {
    use actix_web::http::header::{HeaderMap, HeaderValue, SET_COOKIE};
    use leptos_actix::{ResponseOptions, ResponseParts};

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
