#[component(transparent)]
pub fn Umami() -> impl IntoView {
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
