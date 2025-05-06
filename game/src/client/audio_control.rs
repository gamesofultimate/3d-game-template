#![cfg(target_arch = "wasm32")]
use engine::{
  application::scene::Scene,
  systems::{Backpack, Initializable, Inventory, System},
};

pub struct AudioControlSystem {}

impl Initializable for AudioControlSystem {
  fn initialize(_inventory: &Inventory) -> Self {
    Self {}
  }
}

impl AudioControlSystem {
  fn handle_pickup_audio(&mut self, _scene: &mut Scene, _backpack: &mut Backpack) {}
}

impl System for AudioControlSystem {
  fn get_name(&self) -> &'static str {
    "AudioControlSystem"
  }

  fn run(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    self.handle_pickup_audio(scene, backpack);
  }
}
