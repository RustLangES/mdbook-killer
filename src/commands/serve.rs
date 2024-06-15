use anyhow::Error;
use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::Html,
    routing::get,
    Router,
};
use std::path::Path as StdPath;
use std::{path::PathBuf, sync::Arc};

pub struct ServeConfig {
    pub hostname: Option<String>,
    pub port: Option<u16>,
    pub open: bool,
    pub dest_dir: Option<PathBuf>,
}

pub async fn get_html(
    Extension(config): Extension<Arc<ServeConfig>>,
    Path((folder, file)): Path<(String, String)>,
) -> (StatusCode, Html<String>) {
    let dest_dir = config
        .dest_dir
        .as_deref()
        .unwrap_or(StdPath::new("book/"))
        .to_str()
        .unwrap_or("book/");
    let route = format!("{}/{}/{}", dest_dir, folder, file);
    let path = StdPath::new(route.as_str());

    if path.exists() {
        let file = std::fs::read_to_string(path).unwrap();

        (StatusCode::OK, Html(file))
    } else {
        (StatusCode::NOT_FOUND, Html(String::from("not-found")))
    }
}

pub async fn execute(config: ServeConfig) -> Result<(), Error> {
    let port = config.port.clone().unwrap();
    let hostname = config.hostname.clone().unwrap();
    let addr = format!("{}:{}", hostname, port);

    let shared_config = Arc::new(config);

    let app = Router::new()
        .route("/:folder/:file", get(get_html))
        .layer(Extension(shared_config.clone()));

    println!("Server running on port {}", port.clone());

    let server_task = tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind(addr.clone()).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    });

    let browser_task = tokio::spawn(async move {
        let url = format!("http://{}:{}", hostname, port);

        if shared_config.open && webbrowser::open(url.as_str()).is_ok() {
            println!("Browser is open");
        } else {
            println!("Browser is not open");
        }
    });

    tokio::join!(server_task, browser_task);

    Ok(())
}
