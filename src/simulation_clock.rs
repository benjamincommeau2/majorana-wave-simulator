// src/simulation_clock.rs

#[derive(Clone, Copy, Debug, PartialEq, Eq)]

pub struct FramePhysicsSchedule {

  pub steps_to_run: usize,

  pub dropped_steps: usize,

}


pub struct SimulationClock {

  physics_dt: f64,

  playback_rate: f64,

  max_steps_per_frame: usize,

  accumulated_simulation_time: f64,

  previous_frame_timestamp_ms: Option<f64>,

}


impl SimulationClock {

  pub fn new(

    physics_dt: f64,

    playback_rate: f64,

    max_steps_per_frame: usize,

  ) -> Self {

    Self {

      physics_dt,

      playback_rate,

      max_steps_per_frame,

      accumulated_simulation_time:
        0.0,

      previous_frame_timestamp_ms:
        None,

    }

  }


  pub fn schedule_for_frame(

    &mut self,

    timestamp_ms: f64,

  ) -> FramePhysicsSchedule {

    let previous_timestamp_ms =
      match self.previous_frame_timestamp_ms {

        Some(previous_timestamp_ms) =>
          previous_timestamp_ms,

        None => {

          self.previous_frame_timestamp_ms =
            Some(
              timestamp_ms,
            );


          return FramePhysicsSchedule {

            steps_to_run:
              0,

            dropped_steps:
              0,

          };

        }

      };


    self.previous_frame_timestamp_ms =
      Some(
        timestamp_ms,
      );


    let elapsed_real_seconds =

      (
        timestamp_ms
        - previous_timestamp_ms
      )

      / 1000.0;


    let elapsed_simulation_time =

      elapsed_real_seconds
      * self.playback_rate;


    self.accumulated_simulation_time +=
      elapsed_simulation_time;


    let due_steps = (

      self.accumulated_simulation_time
      / self.physics_dt

    )

    .floor() as usize;


    let steps_to_run = due_steps.min(

      self.max_steps_per_frame,

    );


    let dropped_steps =

      due_steps
      - steps_to_run;


    self.accumulated_simulation_time -=

      due_steps as f64
      * self.physics_dt;


    FramePhysicsSchedule {

      steps_to_run,

      dropped_steps,

    }

  }


  pub fn steps_for_frame(

    &mut self,

    timestamp_ms: f64,

  ) -> usize {

    self.schedule_for_frame(

      timestamp_ms,

    )

    .steps_to_run

  }

}