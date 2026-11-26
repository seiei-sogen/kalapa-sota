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

use git_cli::{read_repository_snapshot, resolve_worktree_root_path};
use repository_window::{RepositoryWindow, repository_window_title};
use single_line_text_input::bind_single_line_text_input_keys;

fn main() {
    let launch_argument_path = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    let opened_worktree =
        resolve_worktree_root_path(&launch_argument_path).and_then(|opened_worktree_root_path| {
            let snapshot = read_repository_snapshot(&opened_worktree_root_path.0)?;
            Ok((opened_worktree_root_path, snapshot))
        });
    let (opened_worktree_root_path, snapshot) = match opened_worktree {
        Ok(opened_worktree) => opened_worktree,
        Err(error) => {
            eprintln!("{}", error.message);
            std::process::exit(1);
        }
    };
    let window_title = repository_window_title(&opened_worktree_root_path);

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
            |window, cx| {
                cx.new(|cx| RepositoryWindow::new(opened_worktree_root_path, snapshot, window, cx))
            },
        )
        .expect("open the repository window");
        cx.on_window_closed(|cx| cx.quit()).detach();
    });
}
