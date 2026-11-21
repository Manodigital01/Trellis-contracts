#![no_std]
// Compile the actual production modules without the unrelated broken root exports.
#[path = "../../../shared/src/auth.rs"]
pub mod auth;
#[path = "../../../shared/src/auth_fingerprint.rs"]
pub mod auth_fingerprint;
#[path = "../../../shared/src/batch.rs"]
pub mod batch;
#[path = "../../../shared/src/bounded_batch.rs"]
pub mod bounded_batch;
#[path = "../../../shared/src/bounded_cache.rs"]
pub mod bounded_cache;
#[path = "../../../shared/src/compat.rs"]
pub mod compat;
#[path = "../../../shared/src/config_audit.rs"]
pub mod config_audit;
#[path = "../../../shared/src/deployment_rollback_verification.rs"]
pub mod deployment_rollback_verification;
#[path = "../../../shared/src/error_taxonomy.rs"]
pub mod error_taxonomy;
#[path = "../../../shared/src/errors.rs"]
pub mod errors;
#[path = "../../../shared/src/event_pagination.rs"]
pub mod event_pagination;
#[path = "../../../shared/src/events.rs"]
pub mod events;
#[path = "../../../shared/src/evidence.rs"]
pub mod evidence;
#[path = "../../../shared/src/external_evidence.rs"]
pub mod external_evidence;
#[path = "../../../shared/src/feature_flags.rs"]
pub mod feature_flags;
#[path = "../../../shared/src/fuzz_boundary_inputs.rs"]
pub mod fuzz_boundary_inputs;
#[path = "../../../shared/src/history.rs"]
pub mod history;
#[path = "../../../shared/src/init_rollback.rs"]
pub mod init_rollback;
#[path = "../../../shared/src/input_limits.rs"]
pub mod input_limits;
#[path = "../../../shared/src/jobs.rs"]
pub mod jobs;
#[cfg(test)]
#[path = "../../../shared/src/ledger_sequence.rs"]
pub mod ledger_sequence;
#[path = "../../../shared/src/math.rs"]
pub mod math;
#[path = "../../../shared/src/ownership_history.rs"]
pub mod ownership_history;
#[path = "../../../shared/src/permission_diff_preview.rs"]
pub mod permission_diff_preview;
#[path = "../../../shared/src/problem_details_taxonomy.rs"]
pub mod problem_details_taxonomy;
#[path = "../../../shared/src/resumable_repair.rs"]
pub mod resumable_repair;
#[path = "../../../shared/src/rollout_guard.rs"]
pub mod rollout_guard;
#[path = "../../../shared/src/storage.rs"]
pub mod storage;
#[cfg(test)]
#[path = "../../../shared/src/test_boundary_properties.rs"]
mod test_boundary_properties;
#[cfg(test)]
#[path = "../../../shared/src/test_compat_fixtures.rs"]
mod test_compat_fixtures;
#[cfg(test)]
#[path = "../../../shared/src/test_concurrent_replay.rs"]
mod test_concurrent_replay;
#[cfg(test)]
#[path = "../../../shared/src/test_init_rollback.rs"]
mod test_init_rollback;
#[cfg(test)]
#[path = "../../../shared/src/test_compat_fixtures.rs"]
mod test_compat_fixtures;
#[path = "../../../shared/src/config_audit.rs"]
pub mod config_audit;
#[path = "../../../shared/src/upgrade_compat_snapshot.rs"]
pub mod upgrade_compat_snapshot;
#[path = "../../../shared/src/pagination.rs"]
pub mod pagination;
#[path = "../../../shared/src/event_pagination.rs"]
pub mod event_pagination;
#[path = "../../../shared/src/treasury_conservation.rs"]
pub mod treasury_conservation;
#[path = "../../../shared/src/auth_fingerprint.rs"]
pub mod auth_fingerprint;
#[path = "../../../shared/src/workflow_synthetic_monitor.rs"]
pub mod workflow_synthetic_monitor;
#[path = "../../../shared/src/ttl_monitor_repair.rs"]
pub mod ttl_monitor_repair;
#[path = "../../../shared/src/dependency_aware_health.rs"]
pub mod dependency_aware_health;
#[path = "../../../shared/src/unified_dedup.rs"]
pub mod unified_dedup;
#[path = "../../../shared/src/ownership_transfer_manager.rs"]
pub mod ownership_transfer_manager;
#[path = "../../../shared/src/rare_failure_fixtures.rs"]
pub mod rare_failure_fixtures;
#[path = "../../../shared/src/deterministic_sort.rs"]
pub mod deterministic_sort;
#[path = "../../../shared/src/session_invalidation.rs"]
pub mod session_invalidation;
