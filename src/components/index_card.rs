use crate::routes::api::Post;
use leptos::*;

#[component]
pub fn IndexCard(cx: Scope, post: Post) -> impl IntoView {
    view! {cx,
        <a class="w-full text-gray-900 hover:text-yellow-600 dark:text-gray-100 dark:hover:text-yellow-100 hover:no-underline" href="href">
        </a>
    }
}
