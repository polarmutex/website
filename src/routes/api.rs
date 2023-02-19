use cfg_if::cfg_if;
use leptos::*;
use serde::{Deserialize, Serialize};

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

#[server(GetPosts, "/api")]
pub async fn get_posts(_cx: Scope) -> Result<Vec<Post>, ServerFnError> {
    use futures::TryStreamExt;
    let mut posts: Vec<Post> = Vec::new();
    posts.push(Post {
        id: 20,
        title: String::from("hey I am here"),
        slug: String::from("This is a slug"),
        date: String::from("2023-02-02"),
    });
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
