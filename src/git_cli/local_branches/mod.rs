mod parse_upstream_tracking;
mod push_current_branch;
mod read_local_branches;
mod switch_local_branch;

#[cfg(test)]
mod parse_upstream_tracking_tests;
#[cfg(test)]
mod push_current_branch_round_trip_tests;

pub use push_current_branch::push_current_branch;
pub use read_local_branches::read_local_branches;
pub use switch_local_branch::switch_local_branch;
