use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "ssr")] {

use brianryall_xyz::app::App;
use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes, handle_server_fns_with_context};
//use tower_http::compression::CompressionLayer;
use brianryall_xyz::fileserv::file_and_error_handler;
use brianryall_xyz::state::AppState;
use axum::{
        response::{Response, IntoResponse},
        routing::get,
        extract::{Path, State, RawQuery},
        http::{Request, header::HeaderMap},
        body::Body as AxumBody,
        Router,
    };
use leptos::logging::log;

#[tracing::instrument(level = "info", fields(error))]
    async fn server_fn_handler(State(app_state): State<AppState>, path: Path<String>, headers: HeaderMap, raw_query: RawQuery,
    request: Request<AxumBody>) -> impl IntoResponse {

        log!("{:?}", path);
         handle_server_fns_with_context(
        move || {
            provide_context(app_state.posts.clone());
        },
        request,
    )
    .await;
    }
    #[tracing::instrument(level = "info", fields(error))]
 async fn leptos_routes_handler(State(app_state): State<AppState>, req: Request<AxumBody>) -> Response{
            let handler = leptos_axum::render_route_with_context(app_state.leptos_options.clone(), app_state.routes.clone(),
            move || {
                provide_context( app_state.posts.clone());
            },
            || view! {  <App/> }
        );
        handler(req).await.into_response()
    }

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

    let app_state= AppState::new_with_posts(leptos_options, routes.clone()).await.expect("Failed to create App State");

    // build our application with a route
    let app = Router::new()
        //.leptos_routes(&leptos_options, routes, App)
        .fallback(file_and_error_handler)
        .route("/api/*fn_name", get(server_fn_handler).post(server_fn_handler))
        .leptos_routes_with_handler(routes, get(leptos_routes_handler) )
        //.layer(CompressionLayer::new())
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    logging::log!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

}
}}
