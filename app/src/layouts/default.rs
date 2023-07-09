use leptos::*;
use leptos_meta::*;

//use crate::components::{Footer, FooterProps, Nav, NavProps};
use crate::components::Footer;
use crate::components::Nav;
use crate::providers::color_scheme::ColorScheme;

#[component]
pub fn Default(cx: Scope, children: Children) -> impl IntoView {
    let color_scheme = use_context::<ColorScheme>(cx).expect("Failed to find ColorScheme");

    view! { cx,
        <Html class=move || {
            let theme = match color_scheme.prefers_dark.get() {
                true => "dark",
                false => "",
            };
            format!("{}", theme)
        }/>
        <Body class="vsc-initialized bg-white text-white dark:bg-gray-900 dark:text-black"/>
        <Stylesheet id="leptos" href="/pkg/brianryall-xyz.css" />
        <div class="flex flex-col justify-center bg-gray-50 px-4 dark:bg-gray-900 sm:px-8">
            <Nav/>
        </div>
        <main class="flex flex-col justify-center bg-gray-50 px-4 dark:bg-gray-900 sm:px-8">
            {children(cx)}
        </main>
        <Footer/>
    }
}
