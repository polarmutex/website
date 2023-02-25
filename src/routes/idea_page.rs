use crate::errors::AppError;
use crate::routes::api;
use leptos::*;
use leptos_router::*;

#[derive(Params, Clone, Debug, PartialEq, Eq)]
pub struct PostParams {
    slug: String,
}

#[component]
pub fn IdeaPage(cx: Scope) -> impl IntoView {
    let query = use_params::<PostParams>(cx);
    let id = move || {
        query.with(|q| {
            q.as_ref()
                .map(|q| q.slug.clone())
                .map_err(|_| AppError::NotFound)
        })
    };
    let post = create_resource(cx, id, |id| async move {
        match id {
            Err(e) => Err(e),
            Ok(id) => api::get_post(id)
                .await
                .map(|data| data.ok_or(AppError::NotFound))
                .map_err(|_| AppError::InternalServerError)
                .flatten(),
        }
    });

    let post_view = move || {
        post.with(cx, |post| {
            post.clone().map(|post| {
                view! { cx,
                    // render content
                    <article class="items-start justify-center w-full mx-auto mt-16 mb-32 prose swyxcontent dark:prose-invert max-w-none">
                        <h1 class="md:text-center mb-8 text-3xl font-bold tracking-tight text-black dark:text-white md:text-5xl ">
                            {&post.title}
                        </h1>
                        <div class="flex justify-between w-full mt-2 bg border-red sm:items-start md:flex-row md:items-center">
                            <p class="flex items-center text-sm text-gray-700 dark:text-gray-300">"brian"</p>
                            <p class="flex items-center text-sm text-gray-600 dark:text-gray-400">
                                {&post.date}
                            </p>
                        </div>
                        <div class="-mx-4 my-2 flex h-1 w-[100vw] bg-gradient-to-r from-yellow-400 via-red-500 to-pink-500 sm:mx-0 sm:w-full" />
                        <section inner_html={&post.content} />
                    </article>
                    <div class="max-w-2xl mx-auto">
                        /*
                        {#if json?.tags?.length}
                            <p class="!text-slate-400 flex-auto mb-4 italic">
                                Tagged in:
                                {#each json.tags as tag}
                                    span class="px-1">
                                        <a href={`/blog?filter=hashtag-${tag}`}>#{tag}</a>
                                    </span>
                                {/each}
                            </p>
                        {/if}*/
                        <div class="max-w-full p-4 mb-12 prose border-t border-b border-blue-800 dark:prose-invert">
                            /*{#if json.ghMetadata.reactions.total_count > 0}
                              Reactions: <Reactions
                              issueUrl={json.ghMetadata.issueUrl}
                              reactions={json.ghMetadata.reactions}
                            />
                            :else}
                                <a href={json.ghMetadata.issueUrl}>Leave a reaction </a>
                                if you liked this post! 🧡
                            {/if}*/
                        </div>
                    </div>
                    //<p>{&post.content}</p>

                    // since we're using async rendering for this page,
                    // this metadata should be included in the actual HTML <head>
                    // when it's first served
                    //<Title text=post.title/>
                    //<Meta name="description" content=post.content/>
                }
            })
        })
    };

    view! { cx,
        <Suspense fallback=move || view! { cx, <p>"Loading post..."</p> }>
            <ErrorBoundary fallback=|cx, errors| {
                view! { cx,
                    <div class="error">
                        <h1>"Something went wrong."</h1>
                        <ul>
                        {move || errors.get()
                            .into_iter()
                            .map(|(_, error)| view! { cx, <li>{error.to_string()} </li> })
                            .collect::<Vec<_>>()
                        }
                        </ul>
                    </div>
                }
            }>
                {post_view}
            </ErrorBoundary>
        </Suspense>
    }
}
