use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "ssr")] {

use axum::Router;
use brianryall_xyz::app::App;
use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
//use tower_http::compression::CompressionLayer;
use brianryall_xyz::fileserv::file_and_error_handler;


#[tokio::main]
async fn main()  {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or(
        tracing_subscriber::EnvFilter::new("info,site_server=debug,site_app=debug"),
    );

    #[cfg(not(feature = "chrome-tracing"))]
    {
        tracing_subscriber::fmt().with_env_filter(filter).init();
    }
    #[cfg(feature = "chrome-tracing")]
    let guard = {
        use tracing_subscriber::prelude::*;

        let (chrome_layer, guard) = tracing_chrome::ChromeLayerBuilder::new().build();
        tracing_subscriber::registry().with(chrome_layer).init();
        guard
    };

    // Setting get_configuration(None) means we'll be using cargo-leptos's env
    // values For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to
    // deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    // build our application with a route
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, App)
        .fallback(file_and_error_handler)
        //.layer(CompressionLayer::new())
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    logging::log!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

}
}}
