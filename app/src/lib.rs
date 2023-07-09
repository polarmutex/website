#![feature(result_flattening)]

pub mod components;
pub mod errors;
pub mod functions;
pub mod layouts;
pub mod providers;
pub mod routes;

use crate::error_template::ErrorTemplate;
use crate::layouts::Default;
use crate::providers::provide_color_scheme;
use crate::routes::IdeaPage;
use crate::routes::Ideas;
use crate::routes::Index;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub mod error_template;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    _ = provide_color_scheme(cx);
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        <Router>
            <Routes>
                <Route
                    path="minimal"
                    view=move |cx| {
                        view! { cx, <Index/> }
                    }
                />
               <Route
                    path=""
                    view=|cx| {
                        view! { cx,
                            <Default>
                                <ErrorBoundary fallback=|cx, errors| {
                                    view! { cx, <ErrorTemplate errors=errors/> }
                                }>
                                    <Outlet/>
                                </ErrorBoundary>
                            </Default>
                        }
                    }
                >*/
                    <Route
                        path=""
                        view=move |cx| {
                            view! { cx, <Index/> }
                        }
                    />
                    <Route
                        path="/ideas"
                        view=move |cx| {
                            view! { cx, <Ideas/> }
                        }
                        ssr=SsrMode::Async
                    />
                    <Route
                        path="/ideas/:slug"
                        view=move |cx| {
                            view! { cx, <IdeaPage/> }
                        }
                        ssr=SsrMode::Async
                    />
                </Route>
            </Routes>
        </Router>
    }
}
