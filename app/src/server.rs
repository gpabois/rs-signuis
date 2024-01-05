use std::{collections::HashMap, path::Path};
use include_dir::{include_dir, Dir};

use app::ServerAppProps;
use axum::{
    extract::{Query, State}, 
    response::{Response, IntoResponse}, 
    Router, 
    handler::HandlerWithoutStateExt
};
use hyper::{Uri, header::HeaderValue};

mod app;

// Embark all dist assets in the executable.
const DIST: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/dist");

struct PageRenderer {
    frame: &'static str
}

impl PageRenderer {
    pub fn new() -> Self {
        Self{
            frame: DIST.get_file("index.html").unwrap().contents_utf8().unwrap()
        }
    }

    pub fn render(&self, rendered: &str) -> String {
        self.frame.replace("{render}", rendered)
    }
}

#[derive(Clone, Default)]
struct Executor(yew::platform::Runtime);

impl<F> hyper::rt::Executor<F> for Executor
where
    F: futures::future::Future + Send + 'static,
{
    fn execute(&self, fut: F) {
        self.0.spawn_pinned(move || async move {
            fut.await;
        });
    }
}

async fn render_page(
    url: Uri, 
    Query(queries): Query<HashMap<String, String>>, 
    State(page_renderer): State<PageRenderer>,
) -> impl IntoResponse {

    let url = url.to_string();

    let renderer = yew::ServerRenderer::<app::ServerApp>::with_props(move || ServerAppProps {
        url: url.into(),
        queries
    });

    page_renderer.render(
        &renderer.render().await
    )
}

/// Route assets stored in the dist, except index.html
async fn route_assets(router: &mut Router) {
    DIST
    .files()
    .filter(|f| f.path() != Path::new("index.html"))
    .for_each(|f| {
        let mime_type = mime_guess::from_path(f.path()).first_or_text_plain();

        router.route(
            &f.path().to_str().unwrap(), 
            axum::routing::get(|_| Box::pin(async {
                Response::builder()
                .header(                
                    "Content-Type",
                    HeaderValue::from_str(mime_type.as_ref()).unwrap()
                )
                .body(f.contents())
            }))
        );
    })
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let executor = Executor::default();

    let page_renderer = PageRenderer::new();
    let handle_error = |e| async move {
        (
            500,
            format!("error occurred: {e}"),
        )
    };

    let app = Router::new()
        .fallback(
            axum::routing::get(render_page)
            .with_state(page_renderer)
            .into_service()
            .map_err(|err| -> std::io::Error { match err {} }),
        );
    
    route_assets(&mut app);

    println!("You can view the website at: http://localhost:8080/");
    Server::bind(&"127.0.0.1:8080".parse().unwrap())
        .executor(executor)
        .serve(app.into_make_service())
        .await
        .unwrap();
}