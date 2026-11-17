mod appearance;
mod commit_graph;
mod git_cli;
mod repository;
mod repository_window;
mod single_line_text_input;

use std::path::PathBuf;

use gpui::{
    App, Application, Bounds, TitlebarOptions, WindowBounds, WindowOptions, prelude::*, px, size,
};

use git_cli::read_repository_snapshot;
use repository_window::RepositoryWindow;
use single_line_text_input::bind_single_line_text_input_keys;

fn main() {
    let repository_path = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    let window_title = format!("kalapa - {}", repository_path.display());
    let snapshot = match read_repository_snapshot(&repository_path) {
        Ok(snapshot) => snapshot,
        Err(error) => {
            eprintln!("{}", error.message);
            std::process::exit(1);
        }
    };

    Application::new().run(move |cx: &mut App| {
        bind_single_line_text_input_keys(cx);
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
            |window, cx| cx.new(|cx| RepositoryWindow::new(repository_path, snapshot, window, cx)),
        )
        .expect("open the repository window");
        cx.on_window_closed(|cx| cx.quit()).detach();
    });
}
