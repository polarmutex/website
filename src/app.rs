use crate::components::nav::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    let toggle_dark_class = |_| {
        let document = document();
        //document.body().expect("reason").class_list().toggle("dark");
        document
            .document_element()
            .expect("reason")
            .class_list()
            .toggle("dark");
    };

    view! {
        cx,
        <Body class="vsc-initialized bg-white text-white dark:bg-gray-900 dark:text-black"/>

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/website.css"/>

        <div class="flex flex-col justify-center bg-gray-50 px-4 dark:bg-gray-900 sm:px-8">
            <Nav />
        </div>

        <Router>
            <main class="flex flex-col justify-center bg-gray-50 px-4 dark:bg-gray-900 sm:px-8">
                // sets the document title
                <Title text="Welcome to Leptos"/>

                //<DarkModeToggle/>
                <h1>"Welcome to Leptos!"</h1>
                //<div class="bg-blue dark:bg-white">
                <button class="text-black dark:text-blue-400" on:click=on_click>"Click Me: " {count}</button>
                <button on:click=toggle_dark_class>"Toggle dark mode"</button>
                //</div>
            </main>
        </Router>

        <footer class="mx-auto mb-8 flex w-full max-w-2xl flex-col items-start justify-center">
            <hr class="border-1 mb-8 w-full border-gray-200 dark:border-gray-800" />
            <div class="grid w-full max-w-2xl grid-cols-1 gap-4 px-4 pb-16 sm:grid-cols-2 sm:px-8">
                <div class="flex flex-col space-y-4">
                    <a class="text-gray-500 transition hover:text-gray-300" href="/">"Home"</a>
                    <a class="text-gray-500 transition hover:text-gray-300" href="/about">"About"</a>
                    <a class="text-gray-500 transition hover:text-gray-300" href="/rss.xml" rel="external">"RSS"</a>
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
