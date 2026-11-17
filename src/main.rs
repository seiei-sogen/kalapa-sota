mod appearance;
mod commit_graph;
mod git_cli;
mod repository;
mod repository_window;

use std::path::PathBuf;

use gpui::{
    App, Application, Bounds, TitlebarOptions, WindowBounds, WindowOptions, prelude::*, px, size,
};

use repository_window::RepositoryWindow;

fn main() {
    let repository_path = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    let window_title = format!("kalapa - {}", repository_path.display());
    let repository_window = match RepositoryWindow::load(repository_path) {
        Ok(repository_window) => repository_window,
        Err(error) => {
            eprintln!("{}", error.message);
            std::process::exit(1);
        }
    };

    Application::new().run(move |cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1100.), px(700.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some(window_title.into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_, cx| cx.new(|_| repository_window),
        )
        .expect("open the repository window");
        cx.on_window_closed(|cx| cx.quit()).detach();
    });
}
