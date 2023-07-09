use crate::providers::ColorScheme;
use leptos::*;
use leptos_router::ActionForm;

#[component]
pub fn DarkModeToggle(cx: Scope) -> impl IntoView {
    let color_scheme = use_context::<ColorScheme>(cx).expect("Failed to find ColorSchemeProvider");

    view! { cx,
            <ActionForm action=color_scheme.action>
                <input
                    type="hidden"
                    name="prefers_dark"
                    value=move || (!(color_scheme.prefers_dark)()).to_string()
                />
                <button
                    aria-label="Toggle Dark Mode"
                    //type="submit"
                    class="ml-1 flex h-9 w-9 items-center justify-center rounded-lg bg-yellow-400 ring-yellow-400 transition-all hover:ring-2 dark:bg-yellow-800"
                    value=move || { if (color_scheme.prefers_dark)() { "dark" } else { "light" } }
                >
                    {move || if (color_scheme.prefers_dark)() {
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
    }
}