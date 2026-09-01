use majorana_wave_simulator::runtime_diagnostics::RuntimeDiagnostics;

use majorana_wave_simulator::simulation_clock::SimulationClock;


#[test]

fn simulation_clock_reports_executed_and_dropped_physics_steps() {

  let mut clock = SimulationClock::new(

    0.01,

    1.0,

    4,

  );


  let first_schedule = clock.schedule_for_frame(

    1000.0,

  );


  assert_eq!(

    first_schedule.steps_to_run,

    0,

  );


  assert_eq!(

    first_schedule.dropped_steps,

    0,

  );


  let delayed_schedule = clock.schedule_for_frame(

    1100.0,

  );


  assert_eq!(

    delayed_schedule.steps_to_run,

    4,

  );


  assert_eq!(

    delayed_schedule.dropped_steps,

    6,

  );


  let following_schedule = clock.schedule_for_frame(

    1110.0,

  );


  assert_eq!(

    following_schedule.steps_to_run,

    1,

  );


  assert_eq!(

    following_schedule.dropped_steps,

    0,

  );

}


#[test]

fn runtime_diagnostics_counts_only_executed_steps_as_simulation_time() {

  let mut diagnostics = RuntimeDiagnostics::new(

    0.01,

    250.0,

  );


  diagnostics.record_frame(

    0.0,

    0,

    0,

  );


  diagnostics.record_frame(

    100.0,

    4,

    6,

  );


  diagnostics.record_frame(

    200.0,

    4,

    0,

  );


  let snapshot = diagnostics.record_frame(

    300.0,

    2,

    3,

  )

  .expect(

    "Three hundred milliseconds should cross the diagnostics update interval.",

  );


  assert_eq!(

    snapshot.physics_steps_this_frame,

    2,

  );


  assert_eq!(

    snapshot.total_physics_steps,

    10,

  );


  assert_eq!(

    snapshot.total_dropped_steps,

    9,

  );


  assert!(

    (

      snapshot.simulation_time

      - 0.10

    )

    .abs()

      < 1.0e-12,

  );

}


#[test]

fn runtime_diagnostics_reports_average_fps_and_latest_frame_time() {

  let mut diagnostics = RuntimeDiagnostics::new(

    0.01,

    250.0,

  );


  assert!(

    diagnostics.record_frame(

      0.0,

      0,

      0,

    )

    .is_none(),

  );


  assert!(

    diagnostics.record_frame(

      50.0,

      0,

      0,

    )

    .is_none(),

  );


  assert!(

    diagnostics.record_frame(

      100.0,

      0,

      0,

    )

    .is_none(),

  );


  assert!(

    diagnostics.record_frame(

      150.0,

      0,

      0,

    )

    .is_none(),

  );


  assert!(

    diagnostics.record_frame(

      200.0,

      0,

      0,

    )

    .is_none(),

  );


  let snapshot = diagnostics.record_frame(

    250.0,

    0,

    0,

  )

  .expect(

    "Two hundred fifty milliseconds should produce a diagnostics snapshot.",

  );


  assert!(

    (

      snapshot.frames_per_second

      - 20.0

    )

    .abs()

      < 1.0e-12,

  );


  assert!(

    (

      snapshot.frame_time_ms

      - 50.0

    )

    .abs()

      < 1.0e-12,

  );

}


#[test]

fn diagnostics_update_interval_does_not_change_physics_accounting() {

  let mut diagnostics = RuntimeDiagnostics::new(

    0.01,

    250.0,

  );


  diagnostics.record_frame(

    0.0,

    1,

    0,

  );


  let first_snapshot = diagnostics.record_frame(

    250.0,

    2,

    0,

  )

  .expect(

    "First diagnostics interval should complete.",

  );


  assert_eq!(

    first_snapshot.total_physics_steps,

    3,

  );


  let second_snapshot = diagnostics.record_frame(

    500.0,

    4,

    1,

  )

  .expect(

    "Second diagnostics interval should complete.",

  );


  assert_eq!(

    second_snapshot.total_physics_steps,

    7,

  );


  assert_eq!(

    second_snapshot.total_dropped_steps,

    1,

  );


  assert!(

    (

      second_snapshot.simulation_time

      - 0.07

    )

    .abs()

      < 1.0e-12,

  );

}