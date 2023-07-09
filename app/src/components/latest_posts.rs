use crate::functions::posts::get_posts;
use leptos::*;

#[component]
pub fn LatestPosts(cx: Scope) -> impl IntoView {
    let posts = create_resource(cx, move || (), move |_| get_posts(cx));

    view! {
       cx,
        <section class="mb-8 w-full">
            <h3 id="latest" class="mb-6 text-2xl font-bold tracking-tight text-black dark:text-white md:text-4xl">
                "Latest Posts"
            </h3>
            <Suspense fallback=move || view! {cx, <p>"Loading..."</p> }>
                <ul class="space-y-2 text-white">
                { move || {
                    posts.read(cx).map(move |posts| match posts {
                        Err(e) => {
                            vec![view! { cx, <pre class="error">"Server Error: " {e.to_string()}</pre>}.into_any()]
                        }
                        Ok(posts) => {
                            posts.into_iter().map(move |post| {
                                view! { cx,
                                <li>
                                    <a class="font-bold" data-sveltekit-preload-data href={format!("/ideas/{}",post.slug)}>{post.title}</a> //data-sveltekit-preload
                                    <span class="hidden text-xs text-black dark:text-gray-400 sm:inline">
                                        {format!(" {}", post.date)}
                                    </span>
                                </li>
                                }.into_any()
                            }).collect::<Vec<_>>()
                        }
                    }).unwrap_or_default()
                }}
                </ul>
            </Suspense>
            <a class="mt-2 flex h-6 rounded-lg leading-7 text-gray-600 transition-all dark:text-gray-400 dark:hover:text-gray-200" href="/ideas">
                "Search and see all posts"
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="ml-1 h-6 w-6">
                    <path
                        stroke="currentColor"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M17.5 12h-15m11.667-4l3.333 4-3.333-4zm3.333 4l-3.333 4 3.333-4z"
                    />
                </svg>
            </a>
        </section>
    }
}
