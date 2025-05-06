use engine::{
  application::scene::Scene,
  systems::{Backpack, Initializable, Inventory, System},
};

pub struct UITimerSystem {}

impl Initializable for UITimerSystem {
  fn initialize(_: &Inventory) -> Self {
    Self {}
  }
}

impl UITimerSystem {}

impl System for UITimerSystem {
  fn get_name(&self) -> &'static str {
    "UITimerSystem"
  }

  fn run(&mut self, _scene: &mut Scene, _backpack: &mut Backpack) {}
}
