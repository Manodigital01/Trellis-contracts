#![no_std]
// Compile the actual production modules without the unrelated broken root exports.
#[path = "../../../shared/src/errors.rs"]
pub mod errors;
#[path = "../../../shared/src/storage.rs"]
pub mod storage;
#[path = "../../../shared/src/history.rs"]
pub mod history;
#[path = "../../../shared/src/rollout_guard.rs"]
pub mod rollout_guard;
#[path = "../../../shared/src/ownership_history.rs"]
pub mod ownership_history;
#[path = "../../../shared/src/feature_flags.rs"]
pub mod feature_flags;
#[path = "../../../shared/src/auth.rs"]
pub mod auth;
#[path = "../../../shared/src/evidence.rs"]
pub mod evidence;
#[path = "../../../shared/src/external_evidence.rs"]
pub mod external_evidence;
#[path = "../../../shared/src/jobs.rs"]
pub mod jobs;
#[cfg(test)]
#[path = "../../../shared/src/test_queue_backpressure.rs"]
mod test_queue_backpressure;
#[path = "../../../shared/src/events.rs"]
pub mod events;
#[path = "../../../shared/src/batch.rs"]
pub mod batch;
#[path = "../../../shared/src/error_taxonomy.rs"]
pub mod error_taxonomy;
#[cfg(test)]
#[path = "../../../shared/src/ledger_sequence.rs"]
pub mod ledger_sequence;
#[cfg(test)]
#[path = "../../../shared/src/test_concurrent_replay.rs"]
mod test_concurrent_replay;
#[path = "../../../shared/src/resumable_repair.rs"]
pub mod resumable_repair;
#[cfg(test)]
#[path = "../../../shared/src/test_resumable_repair.rs"]
mod test_resumable_repair;
#[path = "../../../shared/src/bounded_batch.rs"]
pub mod bounded_batch;
#[path = "../../../shared/src/bounded_cache.rs"]
pub mod bounded_cache;
#[path = "../../../shared/src/workflow_failure_injection.rs"]
pub mod workflow_failure_injection;
#[path = "../../../shared/src/user_facing_degraded_status.rs"]
pub mod user_facing_degraded_status;
#[path = "../../../shared/src/math.rs"]
pub mod math;
#[path = "../../../shared/src/time_window.rs"]
pub mod time_window;
#[path = "../../../shared/src/input_limits.rs"]
pub mod input_limits;
#[cfg(test)]
#[path = "../../../shared/src/test_boundary_properties.rs"]
mod test_boundary_properties;
#[path = "../../../shared/src/init_rollback.rs"]
pub mod init_rollback;
#[cfg(test)]
#[path = "../../../shared/src/test_init_rollback.rs"]
mod test_init_rollback;
#[path = "../../../shared/src/compat.rs"]
pub mod compat;
#[cfg(test)]
#[path = "../../../shared/src/test_compat_fixtures.rs"]
mod test_compat_fixtures;
#[path = "../../../shared/src/config_audit.rs"]
pub mod config_audit;
