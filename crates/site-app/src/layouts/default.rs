use leptos::*;
use leptos_meta::*;

//use crate::components::{Footer, FooterProps, Nav, NavProps};
use crate::components::Footer;
use crate::components::Nav;

#[component]
pub fn Default(children: Children) -> impl IntoView {
    view! {
        <Html/>
        <Body class="vsc-initialized bg-white text-white dark:bg-gray-900 dark:text-black"/>
        <Stylesheet id="leptos" href="/pkg/brianryall-xyz.css" />
        <div class="flex flex-col justify-center bg-gray-50 px-4 dark:bg-gray-900 sm:px-8">
            <Nav/>
        </div>
        <main class="flex flex-col justify-center bg-gray-50 px-4 dark:bg-gray-900 sm:px-8">
            {children()}
        </main>
        <Footer/>
    }
}
