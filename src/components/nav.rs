use crate::routes::api;
use leptos::*;
use leptos_router::ToHref;
use leptos_router::{ActionForm, ActionFormProps};
use wasm_bindgen::JsCast;

#[cfg(not(feature = "ssr"))]
fn initial_prefers_dark(_cx: Scope) -> bool {
    let doc = document().unchecked_into::<web_sys::HtmlDocument>();
    let cookie = doc.cookie().unwrap_or_default();
    if cookie.contains("darkmode=") {
        cookie.contains("darkmode=true")
    } else {
        true
    }
}

#[cfg(feature = "ssr")]
fn initial_prefers_dark(cx: Scope) -> bool {
    use axum::http::header::COOKIE;
    use axum_extra::extract::cookie::Cookie;
    let req = match use_context::<leptos_axum::RequestParts>(cx) {
        Some(req) => req,
        None => return true,
    };
    //use_context::<http::Request>(cx)
    req.headers
        .get_all(COOKIE)
        .iter()
        .filter_map(|cookie_header| cookie_header.to_str().ok())
        .flat_map(|cookie_header| cookie_header.split(';'))
        .filter_map(|cookie_header| Cookie::parse_encoded(cookie_header.trim()).ok())
        .filter(|cookie| cookie.name() == "darkmode")
        .any(|cookie| cookie.value() == "true")
}

#[component]
pub fn Nav(cx: Scope, setter: WriteSignal<bool>) -> impl IntoView {
    let initial = initial_prefers_dark(cx);

    let toggle_dark_mode_action = create_server_action::<api::ToggleDarkMode>(cx);
    // input is `Some(value)` when pending, and `None` if not pending
    let input = toggle_dark_mode_action.input();
    // value contains most recently-returned value
    let value = toggle_dark_mode_action.value();

    let prefers_dark = move || {
        match (input(), value()) {
            // if there's some current input, use that optimistically
            (Some(submission), _) => {
                let res = submission.prefers_dark;
                setter(res);
                res
            }
            // otherwise, if there was a previous value confirmed by server, use that
            (_, Some(Ok(value))) => {
                let res = value;
                setter(res);
                res
            }
            // otherwise, use the initial value
            _ => {
                let res = initial;
                setter(res);
                res
            }
        }
    };

    view! {
        cx,
        <nav class="relative mx-auto flex w-full max-w-2xl items-center justify-between border-gray-200 bg-gray-50 bg-opacity-60 py-8 text-gray-900 dark:border-gray-700 dark:bg-gray-900 dark:text-gray-100 sm:pb-16">
            //<a href="#skip" class="skip-nav">Skip to content</a>
            //<MobileMenu />
            <ul class="ml-[-0.60rem] flex">
                <li>
                    <NavLink href="/">"Home"</NavLink>
                </li>
                <li>
                    <NavLink href="/ideas">"Ideas"</NavLink>
                </li>
                /*<li>
                    <NavLink href="/about">"About"</NavLink>
                </li>*/
                /*<li>
                    <a
                        class="hidden rounded-lg p-1 text-gray-800 transition-all hover:bg-yellow-200 dark:text-gray-200 dark:hover:bg-yellow-800 sm:px-3 sm:py-2 md:inline-block"
                        rel="external"
                        href="/rss.xml"
                        ><span class="capsize">"RSS"</span>
                    </a>
                </li>*/
            </ul>
            <div class="flex items-center space-x-4">
                // <-- Github -->
                <a
                    class="rounded-lg text-gray-700 hover:bg-yellow-200 dark:text-gray-200
                    dark:hover:bg-yellow-800"
                    href="https://github.com/polarmutex"
                    aria-label="GitHub source"
                >
                    <svg aria-hidden="true" class="h-9 w-9 p-1" fill="currentColor" viewBox="0 0 24 24">
                        <path
                            fill-rule="evenodd"
                                d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483
                                0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608
                                1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088
                                2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988
                                1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112
                                6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202
                                2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566
                                4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019
                                10.019 0 0022 12.017C22 6.484 17.522 2 12 2z"
                            clip-rule="evenodd"
                        />
                    </svg>
                </a>
                <ActionForm action=toggle_dark_mode_action>
                <input
                type="hidden"
                name="prefers_dark"
                value=move || (!prefers_dark()).to_string()
            />
               <button
                    aria-label="Toggle Dark Mode"
                    class="ml-1 flex h-9 w-9 items-center justify-center rounded-lg bg-yellow-400 ring-yellow-400
                    transition-all hover:ring-2 dark:bg-yellow-800"
                >
                {move || if prefers_dark() {
                                       view! {cx,
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    class="h-5 w-5 text-gray-800 dark:text-yellow-100"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728
                        0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"
                    />
                </svg>
                                       }
                     } else {
                                       view! {cx,
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    class="h-5 w-5 text-gray-800 dark:text-gray-200"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z"
                    />
                </svg>
                                       }
                     }}
        </button>
        </ActionForm>
            </div>
        </nav>
    }
}

#[component]
pub fn NavLink(cx: Scope, href: &'static str, children: Children) -> impl IntoView {
    view! {
            cx,
            <a
                class="hidden rounded-lg p-1 text-gray-800 transition-all hover:bg-yellow-200 dark:text-gray-200 dark:hover:bg-yellow-800 sm:px-3 sm:py-2 md:inline-block"
                //class:font-semibold={isActive}
                href=href
            ><span class="capsize">
            {children(cx)}
            </span>
    </a>
        }
}
