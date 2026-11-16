use gpui::{Bounds, Canvas, PathBuilder, canvas, fill, point, px, size};

use crate::commit_graph::{CommitGraphLine, CommitGraphRow};
use crate::history_window::appearance::branch_lane_color;
use crate::history_window::appearance::history_window_sizes::{
    COMMIT_DOT_RADIUS, GRAPH_LANE_WIDTH, GRAPH_LINE_WIDTH,
};

pub fn paint_commit_graph_cell(graph_row: CommitGraphRow) -> Canvas<()> {
    canvas(
        |_, _, _| {},
        move |bounds, (), window, _| {
            let lane_center_x =
                |lane: usize| bounds.origin.x + px(GRAPH_LANE_WIDTH * (lane as f32 + 0.5));
            let (top, middle, bottom) = (bounds.top(), bounds.center().y, bounds.bottom());
            let commit_point = point(lane_center_x(graph_row.commit_lane), middle);

            for line in &graph_row.lines {
                let (from, to, lane) = match *line {
                    CommitGraphLine::PassThrough { lane } => (
                        point(lane_center_x(lane), top),
                        point(lane_center_x(lane), bottom),
                        lane,
                    ),
                    CommitGraphLine::IntoCommit { from_lane } => (
                        point(lane_center_x(from_lane), top),
                        commit_point,
                        from_lane,
                    ),
                    CommitGraphLine::OutOfCommit { to_lane } => {
                        (commit_point, point(lane_center_x(to_lane), bottom), to_lane)
                    }
                };
                let mut path = PathBuilder::stroke(px(GRAPH_LINE_WIDTH));
                path.move_to(from);
                path.line_to(to);
                if let Ok(path) = path.build() {
                    window.paint_path(path, branch_lane_color(lane));
                }
            }

            let dot_diameter = px(COMMIT_DOT_RADIUS * 2.);
            window.paint_quad(
                fill(
                    Bounds::centered_at(commit_point, size(dot_diameter, dot_diameter)),
                    branch_lane_color(graph_row.commit_lane),
                )
                .corner_radii(px(COMMIT_DOT_RADIUS)),
            );
        },
    )
}
