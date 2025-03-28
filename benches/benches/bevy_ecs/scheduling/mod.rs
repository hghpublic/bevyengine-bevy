mod run_condition;
mod running_systems;
mod schedule;

use criterion::criterion_group;
use run_condition::*;
use running_systems::*;
use schedule::*;

criterion_group!(
    benches,
    run_condition_yes,
    run_condition_no,
    run_condition_yes_with_query,
    run_condition_yes_with_resource,
    empty_systems,
    busy_systems,
    contrived,
    schedule,
    build_schedule,
    empty_schedule_run,
);
