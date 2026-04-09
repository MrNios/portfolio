use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use tower_http::{compression::CompressionLayer, services::ServeDir, trace::TraceLayer};

mod projects;

use projects::{all_projects, Project};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    projects: Vec<Project>,
    project_count: usize,
    engineering_count: usize,
    software_count: usize,
}

#[derive(Template)]
#[template(path = "404.html")]
struct NotFoundTemplate;

async fn index() -> impl IntoResponse {
    let projects = all_projects();
    let engineering_count = projects
        .iter()
        .filter(|p| p.categories.contains(&"Engineering"))
        .count();
    let software_count = projects
        .iter()
        .filter(|p| p.categories.contains(&"Software"))
        .count();
    let project_count = projects.len();

    let tpl = IndexTemplate {
        projects,
        project_count,
        engineering_count,
        software_count,
    };
    HtmlTemplate(tpl)
}

async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, HtmlTemplate(NotFoundTemplate))
}

struct HtmlTemplate<T>(T);

impl<T: Template> IntoResponse for HtmlTemplate<T> {
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Template render error: {}", err),
            )
                .into_response(),
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "portfolio=info,tower_http=info".into()),
        )
        .init();

    let app = Router::new()
        .route("/", get(index))
        .nest_service("/static", ServeDir::new("static"))
        .fallback(not_found)
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http());

    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080u16);
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::info!("portfolio listening on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}
