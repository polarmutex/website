use leptos::*;
use leptos_meta::use_head;

#[component(transparent)]
pub fn Umami() -> impl IntoView {
    let meta = use_head();
    let id = "polar-link-1".to_string();

    let builder_el = leptos::leptos_dom::html::script()
        .attr("async", true)
        //.attr("defer", true)
        .attr("data-website-id", "32d1fd9c-4c0c-4841-8c8e-1cf26d18e234")
        .attr("src", "https://umami.brianryall.xyz/script.js");

    //if cfg!(not(debug_assertions)) {
    if cfg!(not(debug_assertions)) {
        meta.tags.register(id.into(), builder_el.into_any());
    }
}
