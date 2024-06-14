use anyhow::Error;
use axum::{
    response::Html,
    routing::{get, Router},
};
use webbrowser;

pub struct ServeConfig {
    pub hostname: Option<String>,
    pub port: Option<u16>,
    pub open: bool,
}

pub async fn ping() -> Html<&'static str> {
    Html("<h1>Sergio ribera is a crack</h1>")
}

pub async fn execute(config: ServeConfig) -> Result<(), Error> {
    let port = config.port.unwrap();
    let hostname = config.hostname.unwrap();
    let addr = format!("{}:{}", hostname, port);

    let app = Router::new().route("/", get(ping));

    println!("Server running on port {}", port.clone());

    let server_task = tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind(addr.clone()).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    });

    let browser_task = tokio::spawn(async move {
        let url = format!("http://{}:{}", hostname, port);

        if config.open && webbrowser::open(url.as_str()).is_ok() {
            println!("Browser is open");
        } else {
            println!("Browser is not open");
        }
    });

    tokio::join!(server_task, browser_task);

    Ok(())
}
