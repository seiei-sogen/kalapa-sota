mod commit_staged_changes;
mod parse_porcelain_status;
mod read_changed_files;
mod toggle_changed_file_in_commit;

#[cfg(test)]
mod changed_files_round_trip_tests;
#[cfg(test)]
mod commit_staged_changes_round_trip_tests;
#[cfg(test)]
mod parse_porcelain_status_tests;

pub use commit_staged_changes::commit_staged_changes;
pub use read_changed_files::read_changed_files;
pub use toggle_changed_file_in_commit::toggle_changed_file_in_commit;
