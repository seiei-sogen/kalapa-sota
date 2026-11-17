use gpui::{Rgba, rgb};

const BRANCH_LANE_PALETTE: [u32; 8] = [
    0x4fc1ff, 0xf0a35e, 0x89d185, 0xc586c0, 0xf14c4c, 0xdcdcaa, 0x4ec9b0, 0xd16d9e,
];

/// グラフの線とブランチ名のラベルは、どちらもこの関数で色を決める。
pub fn branch_lane_color(lane: usize) -> Rgba {
    rgb(BRANCH_LANE_PALETTE[lane % BRANCH_LANE_PALETTE.len()])
}
