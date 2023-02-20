use crate::components::index_card::*;
use crate::routes::api;
use leptos::*;

#[component]
pub fn Ideas(cx: Scope) -> impl IntoView {
    //let posts_list = create_server_action::<api::GetPosts>(cx);
    //posts_list.dispatch(api::GetPosts {});
    //let value = posts_list.value();

    //let posts = move || {
    //    value.with(|value| match value {
    //        None => vec![],
    //        Some(Ok(s)) => s.to_owned(),
    //        Some(Err(_)) => vec![],
    //    })
    //};
    /*let posts = create_resource(
        cx,
        move || (),
        move |_| async move {
            match api::get_posts(cx).await {
                //None => vec![],
                Err(_e) => vec![],
                Ok(s) => s,
            }
        },
    );*/
    let posts = create_resource(
        cx,
        || (),
        move |_| async move {
            match api::get_posts(cx, "".to_string()).await {
                Ok(s) => s,
                _ => vec![],
            }
        },
    );

    let (search, set_search) = create_signal(cx, "".to_string());

    /*let filtered_posts = create_resource(
        cx,
        move || (posts(), search()),
        move |(posts, search)| async move {
            if let Some(posts) = posts {
                posts
                    .into_iter()
                    .filter(|post| post.title.contains(&search))
                    .collect::<Vec<_>>()
            } else {
                vec![]
            }
        },
    );*/
    let filtered_posts = create_local_resource(
        cx,
        move || (search(), posts()),
        move |(search, posts)| async move {
            match posts {
                None => vec![],
                Some(s) => s
                    .into_iter()
                    .filter(|post| post.title.contains(&search))
                    .collect(),
            }
        },
    );
    /* look into
          let resource1 =
        create_local_resource(cx, || (), |blah| async move { "foo" });

    let resource2 = create_local_resource(
        cx,
        move || resource1,
        |resource1| async move {
            match resource1.with(|resource1| async move { "bar" }) {
                None => async { "baz" }.await,
                Some(fut) => fut.await,
            }
        },
    );
    */

    //let posts = create_resource(cx, move || (), move |_| api::get_posts(cx));

    //<svelte:window on:keyup={focusSearch} /> ?
    view! { cx,
        <section class="mx-auto mb-16 flex max-w-2xl flex-col items-start justify-center px-4 sm:px-8">
            <h1 class="mb-4 text-3xl font-bold tracking-tight text-black dark:text-white md:text-5xl">
                "Brian Ryall's Ideas"
            </h1>
            <p class="mb-4 text-gray-600 dark:text-gray-400">
            </p>
            <div class="relative mb-4 w-full">
                <input
                  //ria-label="Search articles"
                  type="text"
                  prop:value={move || search()}
                  on:input=move |e| {
                    let val = event_target_value(&e);
                    set_search(val);
                  }
                  //bind:value={$search}
                  //bind:this={inputEl}
                  //on:focus={loadsearchFn}
                  placeholder="Hit / to search"
                  class="block w-full rounded-md border border-gray-200 bg-white px-4 py-2 text-gray-900 focus:border-blue-500 focus:ring-blue-500 dark:border-gray-900 dark:bg-gray-800 dark:text-gray-100"
                />
                <svg
                  class="absolute right-3 top-3 h-5 w-5 text-gray-400 dark:text-gray-300"
                  xmlns="http://www.w3.org/2000/svg"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke="currentColor">
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
                </svg>
            </div>
            // if you have multiple categories enabled
            //{#if POST_CATEGORIES.length > 1}
                <div class="mt-2 mb-8 flex items-center">
                    <div class="mr-2 text-gray-900 dark:text-gray-400">"Filter:"</div>
                    <div class="grid grid-cols-2 rounded-md shadow-sm sm:grid-cols-2">
                        //{#each POST_CATEGORIES as availableCategory}
                            <div>
                                <input
                                  //id="category-{availableCategory}"
                                  class="peer sr-only"
                                  type="checkbox"
                                  //bind:group={$selectedCategories}
                                  //value={availableCategory}
                                />
                                <label
                                  //for="category-{availableCategory}"
                                  class="inline-flex w-full cursor-pointer items-center justify-between border border-gray-200 bg-white px-4 py-2 text-gray-500 hover:bg-gray-100 hover:text-gray-600 peer-checked:border-purple-600 peer-checked:text-purple-600 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-300 dark:peer-checked:text-purple-500">
                                      //{availableCategory}
                                </label>
                            </div>
                        //{/each}
                    </div>
                </div>
            //{/if}

            // you can hardcode yourmost popular posts or pinned post here if you wish
            //{#if !$search && !$selectedCategories?.length}
            {move || {
                if search().is_empty() {
                    view! { cx,
                        //<MostPopular />
                        <h3 class="mt-8 mb-4 text-2xl font-bold tracking-tight text-black dark:text-white md:text-4xl">
                            "All Posts"
                        </h3>
                    }.into_any()
                } else {
                        view!{cx, <div />}.into_any()
                }
            }}
            //{/if}

            //{#if list?.length}
            <Suspense fallback=move || view! {cx, <p>"Loading..."</p> }>
                <ul class="">
                    //{#each list as item}
                    {move || {
                                 match filtered_posts() {
                                     Some(list) => {
                                        if list.is_empty() {
                                            vec![view! { cx, <div class="prose dark:prose-invert">"No blogposts found!"</div> }.into_any()]
                                        } else {
                                            list
                                                .into_iter()
                                                .map(move |post| {
                                                    view! {
                                                        cx,
                                                        <li class="mb-8 text-lg">
                                                            <IndexCard
                                                              post={post}
                                                              //ghMetadata={item.ghMetadata}
                                                            />
                                                            //{#if item.highlightedResults}
                                                            //    <span class="italic">
                                                            //        {@html item.highlightedResults}
                                                            //    </span>
                                                            //{:else}
                                                            //    {item.description}
                                                            //{/if}
                                                        </li>
                                                    }
                                                    .into_any()
                                                })
                                                .collect::<Vec<_>>()
                                        }
                                     },
                                     None => vec![view! { cx, <div class="prose dark:prose-invert">"No blogposts found!"</div>}.into_any()],
                             }
                             }}

                    //{/each}
                </ul>
                //{#if isTruncated}
                //    <div class="flex justify-center">
                //        <button
                //          //on:click={() => (isTruncated = false)}
                //          class="inline-block rounded bg-blue-100 p-4 text-lg font-bold tracking-tight text-black hover:text-yellow-900 dark:bg-blue-900 dark:text-white hover:dark:text-yellow-200 md:text-2xl"
                //        >
                //            "Load More Posts..."
                //        </button>
                //    </div>
                //{/if}
            //{:else if $search}
                //<div class="prose dark:prose-invert">
                //    "No posts found for "
                //    //<code>{$search}</code>
                //    "."
                //</div>
                //<button
                //  class="bg-slate-500 p-2"
                //  //on:click={() => ($search = '')}
                //>
                //    "Clear your search"
                //</button>
            //{:else}
                //<div class="prose dark:prose-invert">"No blogposts found!"</div>
            //{/if}
            </Suspense>
        </section>
    }
}
