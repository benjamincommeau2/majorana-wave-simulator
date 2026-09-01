// src/runtime_diagnostics.rs

#[derive(Clone, Copy, Debug)]

pub struct RuntimeDiagnosticsSnapshot {

  pub frames_per_second: f64,

  pub frame_time_ms: f64,

  pub physics_steps_this_frame: usize,

  pub total_physics_steps: usize,

  pub total_dropped_steps: usize,

  pub simulation_time: f64,

}


pub struct RuntimeDiagnostics {

  physics_dt: f64,

  update_interval_ms: f64,

  previous_frame_timestamp_ms: Option<f64>,

  interval_start_timestamp_ms: Option<f64>,

  interval_frame_count: usize,

  total_physics_steps: usize,

  total_dropped_steps: usize,

}


impl RuntimeDiagnostics {

  pub fn new(

    physics_dt: f64,

    update_interval_ms: f64,

  ) -> Self {

    Self {

      physics_dt,

      update_interval_ms,

      previous_frame_timestamp_ms:
        None,

      interval_start_timestamp_ms:
        None,

      interval_frame_count:
        0,

      total_physics_steps:
        0,

      total_dropped_steps:
        0,

    }

  }


  pub fn record_frame(

    &mut self,

    timestamp_ms: f64,

    physics_steps_this_frame: usize,

    dropped_steps_this_frame: usize,

  ) -> Option<RuntimeDiagnosticsSnapshot> {

    self.total_physics_steps +=
      physics_steps_this_frame;


    self.total_dropped_steps +=
      dropped_steps_this_frame;


    let frame_time_ms =

      match self.previous_frame_timestamp_ms {

        Some(previous_timestamp_ms) =>

          timestamp_ms
          - previous_timestamp_ms,

        None =>
          0.0,

      };


    self.previous_frame_timestamp_ms =
      Some(
        timestamp_ms,
      );


    let interval_start_timestamp_ms =

      match self.interval_start_timestamp_ms {

        Some(interval_start_timestamp_ms) =>
          interval_start_timestamp_ms,

        None => {

          self.interval_start_timestamp_ms =
            Some(
              timestamp_ms,
            );


          return None;

        }

      };


    self.interval_frame_count +=
      1;


    let interval_elapsed_ms =

      timestamp_ms
      - interval_start_timestamp_ms;


    if interval_elapsed_ms
      < self.update_interval_ms
    {

      return None;

    }


    let frames_per_second =

      if interval_elapsed_ms > 0.0 {

        self.interval_frame_count as f64

        / (

          interval_elapsed_ms
          / 1000.0

        )

      } else {

        0.0

      };


    let simulation_time =

      self.total_physics_steps as f64

      * self.physics_dt;


    let snapshot = RuntimeDiagnosticsSnapshot {

      frames_per_second,

      frame_time_ms,

      physics_steps_this_frame,

      total_physics_steps:
        self.total_physics_steps,

      total_dropped_steps:
        self.total_dropped_steps,

      simulation_time,

    };


    self.interval_start_timestamp_ms =
      Some(
        timestamp_ms,
      );


    self.interval_frame_count =
      0;


    Some(
      snapshot,
    )

  }

}