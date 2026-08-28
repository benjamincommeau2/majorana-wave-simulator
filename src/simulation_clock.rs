// src/simulation_clock.rs

pub struct SimulationClock { // Stores the timing state used to separate browser rendering timestamps from fixed physics steps.

  physics_dt: f64, // Stores how much simulation time one numerical physics step advances.

  playback_rate: f64, // Stores how many simulation-time units should advance during one real-world second.

  max_steps_per_frame: usize, // Limits how many fixed physics steps may be scheduled before one rendered browser frame so delayed frames cannot create unbounded catch-up work.

  accumulated_simulation_time: f64, // Stores simulation time earned from browser elapsed time that has not yet been consumed by physics steps.

  previous_frame_timestamp_ms: Option<f64>, // Remembers the previous browser timestamp, or None before the first browser frame establishes a timing reference.

} // Finishes the simulation-clock state.

impl SimulationClock { // Defines the behavior of the pure simulation clock.

  pub fn new( // Creates a simulation clock with a fixed numerical timestep and independent playback rate.

    physics_dt: f64, // Receives how much simulation time one physics step advances.

    playback_rate: f64, // Receives how many simulation-time units should advance per real-world second.

    max_steps_per_frame: usize, // Receives the maximum number of fixed physics steps that may be scheduled before one rendered browser frame.

  ) -> Self { // Returns a newly initialized simulation clock.

    Self { // Creates the initial clock state.

      physics_dt, // Stores the fixed numerical timestep independently from browser frame timing.

      playback_rate, // Stores the requested relationship between real elapsed time and displayed simulation time.

      max_steps_per_frame, // Stores the catch-up limit independently from the numerical physics timestep.

      accumulated_simulation_time: 0.0, // Starts with no unconsumed simulation time because no browser time has elapsed yet.

      previous_frame_timestamp_ms: None, // Starts without a browser timing reference because no requestAnimationFrame timestamp has been observed yet.

    } // Finishes constructing the initial clock state.

  } // Finishes creating the simulation clock.

  pub fn steps_for_frame( // Determines how many fixed physics steps should occur before the current browser frame is rendered.

    &mut self, // Mutably borrows the clock because processing a frame changes its timing state.

    timestamp_ms: f64, // Receives the current requestAnimationFrame timestamp measured in browser milliseconds.

  ) -> usize { // Returns the number of fixed physics steps that should run before this render.

    let previous_timestamp_ms = match self.previous_frame_timestamp_ms { // Checks whether a previous browser timestamp exists.

      Some(previous_timestamp_ms) => previous_timestamp_ms, // Uses the remembered timestamp when this is not the first browser frame.

      None => { // Handles the first browser frame, where no elapsed interval can yet be calculated.

        self.previous_frame_timestamp_ms = Some(timestamp_ms); // Remembers the first timestamp so the next frame can measure elapsed time.

        return 0; // Schedules no physics because the first timestamp only establishes the timing reference.

      } // Finishes handling the first browser frame.

    }; // Finishes retrieving the previous browser timestamp.

    self.previous_frame_timestamp_ms = Some(timestamp_ms); // Updates the remembered timestamp so the next frame measures time from this frame.

    let elapsed_real_seconds = (timestamp_ms - previous_timestamp_ms) / 1000.0; // Converts browser elapsed milliseconds into real-world seconds.

    let elapsed_simulation_time = elapsed_real_seconds * self.playback_rate; // Converts real elapsed time into the amount of simulation time that playback wants to advance.

    self.accumulated_simulation_time += elapsed_simulation_time; // Adds this frame's earned simulation time to any leftover amount from earlier frames.

    let due_steps = ( // Calculates how many complete fixed physics timesteps became due from the accumulated simulation time.

      self.accumulated_simulation_time / self.physics_dt // Expresses accumulated simulation time as a number of complete numerical timesteps.

    ).floor() as usize; // Keeps fractional timestep time separate so it can remain available for a later browser frame.

    let scheduled_steps = due_steps.min( // Applies the responsiveness limit without changing the numerical physics timestep.

      self.max_steps_per_frame, // Prevents one delayed browser frame from scheduling an excessive amount of catch-up work.

    ); // Finishes limiting the amount of physics work scheduled for this rendered frame.

    self.accumulated_simulation_time -= // Removes all complete timesteps that became due, including any excess catch-up work deliberately discarded by the cap.

      due_steps as f64 * self.physics_dt; // Leaves only a fractional remainder smaller than one fixed physics timestep.

    scheduled_steps // Returns only the number of physics steps that the engine is allowed to execute before this render.

  } // Finishes processing one browser frame timestamp.

} // Finishes the simulation-clock implementation.