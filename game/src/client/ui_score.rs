use engine::{
  application::{components::TextComponent, scene::Scene},
  systems::{Backpack, Initializable, Inventory, System},
};

use crate::shared::{pickup::PickupSystemData, ui_components::UIScore};

pub struct UIScoreSystem {}

impl Initializable for UIScoreSystem {
  fn initialize(_: &Inventory) -> Self {
    Self {}
  }
}

impl System for UIScoreSystem {
  fn get_name(&self) -> &'static str {
    "UIScoreSystem"
  }

  fn run(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    self.show_ball_speed(scene, backpack);
  }
}

impl UIScoreSystem {
  fn show_ball_speed(&self, scene: &mut Scene, backpack: &mut Backpack) -> Option<()> {
    let pickup_data = backpack.get_mut::<PickupSystemData>()?;

    for (_, (text, _)) in scene.query_mut::<(&mut TextComponent, &UIScore)>() {
      text.text = format!("Score: {}", pickup_data.player_score);
    }

    Some(())
  }
}
