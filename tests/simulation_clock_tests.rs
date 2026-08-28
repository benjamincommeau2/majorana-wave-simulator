use majorana_wave_simulator::simulation_clock::SimulationClock; // Imports the pure scheduler that will separate browser frame timing from physics timing.

#[test]
fn first_browser_timestamp_schedules_no_physics_steps() { // Verifies that the first browser frame only establishes a timing reference.

  let mut clock = SimulationClock::new( // Creates a clock with an explicitly fixed physics timestep and playback rate.

    0.01, // Advances the numerical simulation by 0.01 simulation-time units per physics step.

    1.0, // Requests one simulation-time unit of progression per real second.

    4, // Allows at most four physics steps to be scheduled for one rendered browser frame.

  ); // Finishes creating the simulation clock.

  let steps = clock.steps_for_frame( // Gives the clock its first browser requestAnimationFrame timestamp.

    1000.0, // Uses an arbitrary timestamp of 1000 milliseconds; only differences between timestamps should matter.

  ); // Finishes processing the first browser timestamp.

  assert_eq!(steps, 0); // Confirms that establishing the initial timestamp does not advance the physics.

}

#[test]
fn elapsed_browser_time_accumulates_until_one_physics_step_is_due() { // Verifies that partial browser-frame intervals accumulate instead of forcing one physics update per render.

  let mut clock = SimulationClock::new( // Creates a clock whose fixed physics step requires ten milliseconds of real time at the selected playback rate.

    0.01, // Advances the numerical simulation by 0.01 simulation-time units per physics step.

    1.0, // Requests one simulation-time unit of progression per real second.

    4, // Allows enough physics steps per rendered frame that this small test will not encounter the future catch-up limit.

  ); // Finishes creating the simulation clock.

  assert_eq!( // Confirms that the first browser timestamp only establishes the timing reference.

    clock.steps_for_frame(1000.0),

    0,

  );

  assert_eq!( // Confirms that five elapsed milliseconds are remembered but are not yet enough for one 0.01 simulation-time step.

    clock.steps_for_frame(1005.0),

    0,

  );

  assert_eq!( // Confirms that the second five-millisecond interval combines with the first to make one complete physics step.

    clock.steps_for_frame(1010.0),

    1,

  );

} // Finishes testing accumulation across multiple rendered browser frames.

#[test]
fn physics_steps_are_capped_without_preserving_excess_catch_up_backlog() { // Verifies that a long browser delay cannot create an unbounded queue of physics work.

  let mut clock = SimulationClock::new( // Creates a clock whose numerical timestep would normally schedule ten steps after one hundred real milliseconds.

    0.01, // Advances the numerical simulation by 0.01 simulation-time units per physics step.

    1.0, // Requests one simulation-time unit of progression per real second.

    4, // Allows at most four physics steps before any one rendered browser frame.

  ); // Finishes creating the simulation clock.

  assert_eq!( // Establishes the initial browser timing reference without advancing the simulation.

    clock.steps_for_frame(1000.0),

    0,

  );

  assert_eq!( // Confirms that one hundred elapsed milliseconds are capped at four scheduled physics steps instead of ten.

    clock.steps_for_frame(1100.0),

    4,

  );

  assert_eq!( // Confirms that excess catch-up work was discarded, so the next ordinary ten-millisecond frame schedules only one new step.

    clock.steps_for_frame(1110.0),

    1,

  );

} // Finishes testing the per-frame catch-up limit.

#[test]
fn different_render_frame_rates_schedule_the_same_total_physics_steps() { // Verifies that rendering frequency does not determine how quickly the numerical simulation advances.

  let mut fifty_hz_clock = SimulationClock::new( // Creates the clock representing a display that renders every twenty milliseconds.

    0.01, // Uses the same fixed numerical physics timestep for both simulated displays.

    1.0, // Requests one simulation-time unit of progression per real-world second.

    100, // Keeps the catch-up cap far above the work required by either normal frame interval in this test.

  ); // Finishes creating the fifty-hertz clock.

  let mut one_hundred_hz_clock = SimulationClock::new( // Creates a second clock representing a display that renders every ten milliseconds.

    0.01, // Uses exactly the same numerical physics timestep as the fifty-hertz clock.

    1.0, // Uses exactly the same playback rate as the fifty-hertz clock.

    100, // Uses the same generous catch-up limit.

  ); // Finishes creating the one-hundred-hertz clock.

  let mut fifty_hz_total_steps = 0; // Counts every physics step scheduled during one real second at fifty rendered frames per second.

  for frame_index in 0..=50 { // Generates browser timestamps from zero through one thousand milliseconds in twenty-millisecond intervals.

    let timestamp_ms = frame_index as f64 * 20.0; // Converts this fifty-hertz frame number into its browser timestamp.

    fifty_hz_total_steps += fifty_hz_clock.steps_for_frame(timestamp_ms); // Adds the fixed physics steps scheduled before this rendered frame.

  } // Finishes simulating one real second of fifty-hertz rendering.

  let mut one_hundred_hz_total_steps = 0; // Counts every physics step scheduled during the same real second at one hundred rendered frames per second.

  for frame_index in 0..=100 { // Generates browser timestamps from zero through one thousand milliseconds in ten-millisecond intervals.

    let timestamp_ms = frame_index as f64 * 10.0; // Converts this one-hundred-hertz frame number into its browser timestamp.

    one_hundred_hz_total_steps += one_hundred_hz_clock.steps_for_frame(timestamp_ms); // Adds the fixed physics steps scheduled before this rendered frame.

  } // Finishes simulating one real second of one-hundred-hertz rendering.

  assert_eq!( // Confirms that changing only the display frequency does not change the total amount of scheduled physics.

    fifty_hz_total_steps,

    one_hundred_hz_total_steps,

  );

  assert_eq!( // Confirms that one real second at playback rate one schedules one hundred fixed 0.01 simulation-time steps.

    fifty_hz_total_steps,

    100,

  );

} // Finishes proving that rendering frequency is independent from numerical simulation progression.

#[test]
fn playback_rate_changes_displayed_simulation_speed_without_changing_physics_dt() { // Verifies that playback speed controls how quickly fixed physics steps are scheduled without changing their numerical timestep.

  let mut normal_speed_clock = SimulationClock::new( // Creates a clock using ordinary one-times playback speed.

    0.01, // Keeps the fixed numerical physics timestep at 0.01 simulation-time units.

    1.0, // Advances one simulation-time unit per real-world second.

    100, // Keeps the catch-up limit high enough that it does not affect this test.

  ); // Finishes creating the normal-speed clock.

  let mut double_speed_clock = SimulationClock::new( // Creates another clock with the same physics timestep but twice the playback speed.

    0.01, // Uses exactly the same fixed numerical physics timestep.

    2.0, // Advances two simulation-time units per real-world second.

    100, // Uses the same catch-up limit.

  ); // Finishes creating the double-speed clock.

  assert_eq!( // Establishes the starting browser timestamp for the normal-speed clock.

    normal_speed_clock.steps_for_frame(0.0),

    0,

  );

  assert_eq!( // Establishes the same starting browser timestamp for the double-speed clock.

    double_speed_clock.steps_for_frame(0.0),

    0,

  );

  assert_eq!( // Confirms that one hundred real milliseconds schedule ten fixed physics steps at normal playback speed.

    normal_speed_clock.steps_for_frame(100.0),

    10,

  );

  assert_eq!( // Confirms that the same real time schedules twenty of the same fixed physics steps at double playback speed.

    double_speed_clock.steps_for_frame(100.0),

    20,

  );

} // Finishes proving that playback rate changes viewing speed without redefining the numerical physics timestep.