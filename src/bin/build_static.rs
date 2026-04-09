// Build a static copy of the site into ./dist
// Run with: cargo run --release --bin build_static
use std::fs;
use std::path::Path;

use askama::Template;

#[path = "../projects.rs"]
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

fn copy_dir(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let to = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir(&entry.path(), &to)?;
        } else {
            fs::copy(entry.path(), to)?;
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out = Path::new("dist");
    if out.exists() {
        fs::remove_dir_all(out)?;
    }
    fs::create_dir_all(out)?;

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

    let index = IndexTemplate {
        projects,
        project_count,
        engineering_count,
        software_count,
    }
    .render()?;
    fs::write(out.join("index.html"), index)?;

    let nf = NotFoundTemplate.render()?;
    fs::write(out.join("404.html"), nf)?;

    copy_dir(Path::new("static"), &out.join("static"))?;

    println!("Built static site to ./dist");
    println!("  index.html, 404.html, static/");
    Ok(())
}
