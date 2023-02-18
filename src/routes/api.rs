use cfg_if::cfg_if;
use leptos::*;
use serde::{Deserialize, Serialize};

cfg_if! {

    if #[cfg(feature = "ssr")] {
         pub fn register_server_functions() {
               // Silence clippy with the _
            _ = GetPosts::register();
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
