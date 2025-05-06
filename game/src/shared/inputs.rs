use engine::application::scene::TransformComponent;
use engine::nalgebra::Vector3;
use engine::{
  application::{components::PhysicsComponent, scene::Scene},
  systems::{physics::PhysicsController, Backpack, Initializable, Inventory, System},
  utils::units::Seconds,
};

#[cfg(target_arch = "wasm32")]
use engine::{
  application::components::SelfComponent,
  application::input::InputsReader,
  systems::{
    ai::{AiDebugger, GoalComponent},
    input::CanvasController,
  },
};

use super::components::Ball;

use crate::shared::game_input::GameInput;

pub struct InputsSystem {
  #[cfg(target_arch = "wasm32")]
  inputs: InputsReader<GameInput>,
  physics: PhysicsController,
  #[cfg(target_arch = "wasm32")]
  canvas: CanvasController,
}

impl Initializable for InputsSystem {
  fn initialize(inventory: &Inventory) -> Self {
    #[cfg(target_arch = "wasm32")]
    let inputs = inventory.get::<InputsReader<GameInput>>().clone();
    let physics = inventory.get::<PhysicsController>().clone();

    #[cfg(target_arch = "wasm32")]
    let canvas = inventory.get::<CanvasController>().clone();
    Self {
      #[cfg(target_arch = "wasm32")]
      inputs,
      physics,
      #[cfg(target_arch = "wasm32")]
      canvas,
    }
  }
}

impl InputsSystem {
  #[cfg(target_arch = "wasm32")]
  fn capture_mouse(&mut self, input: &GameInput) {
    use super::game_input::GameInputState;

    {
      if input.check(GameInputState::MouseLeftClick) && !input.check(GameInputState::MouseLock) {
        self.canvas.capture_mouse(true);
      } else if input.check(GameInputState::KeyboardEscape)
        && input.check(GameInputState::MouseLock)
      {
        self.canvas.capture_mouse(false);
      }
    }
  }

  fn handle_move(&mut self, scene: &mut Scene, delta_time: Seconds) {
    for (_, (_transform, ball, physics, input)) in scene.query_mut::<(
      &TransformComponent,
      &mut Ball,
      &PhysicsComponent,
      &GameInput,
    )>() {
      self.physics.set_linear_damping(physics, 1.0);

      let linear_velocity = self.physics.linear_velocity(physics);
      let mut horizontal_velocity = Vector3::new(linear_velocity.x, 0.0, linear_velocity.z);

      let input_vector = Vector3::new(-input.right, 0.0, input.forward);
      horizontal_velocity += input_vector * *ball.acceleration * *delta_time;

      if horizontal_velocity.magnitude() > *ball.max_speed {
        horizontal_velocity = horizontal_velocity.normalize() * *ball.max_speed;
      }

      let linear_velocity = Vector3::new(
        horizontal_velocity.x,
        linear_velocity.y,
        horizontal_velocity.z,
      );

      self.physics.set_linear_velocity(physics, linear_velocity);
      ball.current_velocity = linear_velocity;
    }
  }
}

impl System for InputsSystem {
  fn get_name(&self) -> &'static str {
    "InputsSystem"
  }

  fn run(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    let delta_time = backpack.get::<Seconds>().cloned().unwrap();

    #[cfg(target_arch = "wasm32")]
    {
      use super::game_input::GameInputState;
      use engine::systems::world::WorldConfig;

      let input = self.inputs.read_client();

      if let Some(world) = backpack.get_mut::<WorldConfig>() {
        if input.check(GameInputState::ToggleDebugPhysics) {
          world.debug_physics = !world.debug_physics;
        }

        if input.check(GameInputState::ToggleDebugLines) {
          world.debug_lines = !world.debug_lines;
        }

        if input.check(GameInputState::ToggleDebugPerformance) {
          world.debug_performance = !world.debug_performance;
        }

        if input.check(GameInputState::ToggleDebugGoaps) {
          world.debug_ai = !world.debug_ai;
        }

        // For GOAP debugging
        if world.debug_ai {
          use engine::application::components::InputComponent;
          use engine::rapier3d::prelude::*;

          let (_, (_, transform, input)) = scene
            .query_one::<(&SelfComponent, &TransformComponent, &InputComponent)>()
            .unwrap();

          let direction = input.get_front();
          let center = transform.world_transform().translation + *direction * 1.0;
          let ray = Ray::new(center.into(), direction.into_inner());
          let max_distance = 100.0;
          let solid = true;
          let filter = QueryFilter::exclude_fixed().exclude_sensors();

          if let Some((entity, _handle, _intersection)) =
            self.physics.raycast(&ray, max_distance, solid, filter)
          {
            if let Some(_) = scene.get_components_mut::<&GoalComponent>(entity) {
              let ai_debugger = backpack.get_mut::<AiDebugger>().unwrap();
              ai_debugger.selected.clear();
              ai_debugger.selected.insert(entity);
            }
          }
        }
      }

      self.capture_mouse(&input);
    }

    self.handle_move(scene, delta_time);
  }
}
