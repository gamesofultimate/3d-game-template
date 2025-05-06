use engine::{
  application::{components::PhysicsComponent, scene::Scene},
  nalgebra::Vector3,
  systems::{physics::PhysicsController, Backpack, Initializable, Inventory, System},
};
use serde::{Deserialize, Serialize};

use crate::shared::game_input::GameInput;

use super::components::Ball;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeveloperMode;

pub struct DeveloperSystem {
  physics: PhysicsController,
}

impl Initializable for DeveloperSystem {
  fn initialize(inventory: &Inventory) -> Self {
    let physics = inventory.get::<PhysicsController>().clone();

    Self { physics }
  }
}

impl DeveloperSystem {}

impl System for DeveloperSystem {
  fn get_name(&self) -> &'static str {
    "DeveloperSystem"
  }

  fn run(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    if let None = backpack.get::<DeveloperMode>() {
      return;
    }

    for (_, (physics, _, input)) in scene.query_mut::<(&mut PhysicsComponent, &Ball, &GameInput)>()
    {
      if input.developer_reset_ball.just_pressed() {
        self
          .physics
          .set_angular_velocity(physics, Vector3::new(0.0, 0.0, 0.0));
        self
          .physics
          .set_linear_velocity(physics, Vector3::new(0.0, 0.0, 0.0));

        self
          .physics
          .set_translation(physics, Vector3::new(0.0, 3.0, 0.0));
      }
    }
  }
}
