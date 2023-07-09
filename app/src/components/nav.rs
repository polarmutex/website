use crate::components::DarkModeToggle;
use leptos::*;

#[component]
pub fn Nav(cx: Scope) -> impl IntoView {
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
                <DarkModeToggle/>
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
