use crate::components::nav::*;
use crate::errors::AppError;
use crate::routes::homepage::*;
use crate::routes::idea_page::*;
use crate::routes::ideas::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
//use leptos_reactive::scope::Scope;
use crate::error_template::{ErrorTemplate, ErrorTemplateProps};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    let (prefers_dark, set_prefers_dark) = create_signal(cx, true);

    let color_scheme = move || {
        if prefers_dark() {
            "dark".to_string()
        } else {
            "".to_string()
        }
    };

    view! {
        cx,
        <Html class=color_scheme />
        <Body class="vsc-initialized bg-white text-white dark:bg-gray-900 dark:text-black"/>

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/brianryall_xyz.css"/>
        <Umami />

        <Router fallback=|cx| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { cx,
                <ErrorTemplate outside_errors/>
            }
            .into_view(cx)
        }>
        <div class="flex flex-col justify-center bg-gray-50 px-4 dark:bg-gray-900 sm:px-8">
            <Nav setter=set_prefers_dark />
        </div>
            <main class="flex flex-col justify-center bg-gray-50 px-4 dark:bg-gray-900 sm:px-8">
                <Routes>
                    <Route path="" view=|cx| view! {cx, <Homepage/> }/>
                    <Route path="/ideas" view=|cx| view! { cx, <Ideas /> } ssr=SsrMode::Async/>
                    <Route path="/ideas/:slug" view=|cx| view! { cx, <IdeaPage /> } ssr=SsrMode::Async/>
                </Routes>
            </main>
        </Router>

        <footer class="mx-auto mb-8 flex w-full max-w-2xl flex-col items-start justify-center">
            <hr class="border-1 mb-8 w-full border-gray-200 dark:border-gray-800" />
            <div class="grid w-full max-w-2xl grid-cols-1 gap-4 px-4 pb-16 sm:grid-cols-2 sm:px-8">
                <div class="flex flex-col space-y-4">
                    <a class="text-gray-500 transition hover:text-gray-300" href="/">"Home"</a>
                    //<a class="text-gray-500 transition hover:text-gray-300" href="/about">"About"</a>
                    //<a class="text-gray-500 transition hover:text-gray-300" href="/rss.xml" rel="external">"RSS"</a>
                </div>
                <div class="flex flex-col space-y-4">
                    <a
                        class="text-gray-500 transition hover:text-gray-300"
                        target="_blank"
                        rel="noopener noreferrer"
                        href="https://twitter.com/intent/follow?screen_name=polarmutex"
                    >
                        "Twitter"
                    </a>
                    <a
                        class="text-gray-500 transition hover:text-gray-300"
                        target="_blank"
                        rel="noopener noreferrer"
                        href="https://github.com/polarmutex"
                    >
                        "GitHub"
                    </a>
                </div>
            </div>
            <p class="prose px-4 dark:prose-invert sm:px-8">
                "This blog is based on the "
                <a href="https://swyxkit.netlify.app/">"swyxkit"</a>
                " template."
            </p>
        </footer>
    }
}

#[component(transparent)]
pub fn Umami(cx: Scope) -> impl IntoView {
    let meta = use_head(cx);
    let id = "polar-link-1".to_string();

    let builder_el = leptos::leptos_dom::html::script(cx)
        .attr("async", true)
        .attr("defer", true)
        .attr("data-website-id", "e067c6cf-e744-4a39-b69c-fe471533edf5")
        .attr("src", "https://umami.brianryall.xyz/umami.js");

    //if cfg!(not(debug_assertions)) {
    if cfg!(not(debug_assertions)) {
        meta.tags.register(cx, id, builder_el.into_any());
    }
}
