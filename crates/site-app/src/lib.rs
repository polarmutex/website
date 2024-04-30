mod components;
mod errors;
mod functions;
mod layouts;
mod routes;

use crate::layouts::Default;
use crate::routes::idea_page::IdeaPage;
use crate::routes::ideas::Ideas;
use crate::routes::Index;
use leptos::*;
use leptos_meta::*;
use leptos_router::{Outlet, Route, Router, Routes, SsrMode};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
      // <Stylesheet id="leptos" href="/pkg/site.css"/>
      //
      // <Title text="A template app"/>
      // <Html lang="en" />
      // <Meta charset="utf-8"/>
      // <Meta name="viewport" content="width=device-width, initial-scale=1"/>

      <Router>
        <Routes>
          // <Route
          //   path="minimal"
          //   view=move || {
          //       view! { <Index/> }
          //   }
          // />
          //
          <Route
            path=""
            view=|| {
                view! {
                  <Default>
                    // <ErrorBoundary fallback=|errors| {
                    //     view! { <ErrorTemplate errors=errors/> }
                    // }>
                      <Outlet/>
                    // </ErrorBoundary>
                  </Default>
                }
            }
          >

            <Route
              path=""
              view=move || {
                  view! { <Index/> }
              }
            />

            // <Route
            //   path="about"
            //   view=move || {
            //       view! { <About/> }
            //   }
            // />

            // <Route
            //   path="portfolio"
            //   view=move || {
            //       view! { <Portfolio/> }
            //   }
            // />

            <Route
              path="ideas"
              view=move || {
                  view! { <Ideas/> }
              }
            />

            <Route
              path="ideas/:slug"
              view=move || {
                  view! { <IdeaPage/> }
              }

              ssr=SsrMode::Async
            />

            // <Route
            //   path="nedry"
            //   view=move || {
            //       view! { <Nedry/> }
            //   }
            // />

            // <Route path="/*any" view=NotFound/>
          </Route>
        </Routes>
      </Router>
    }
}

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
      <p>"Hello, World!"</p>
    }
}
