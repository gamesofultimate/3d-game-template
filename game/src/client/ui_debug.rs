use engine::{
  application::{components::TextComponent, scene::Scene},
  nalgebra::Vector3,
  systems::{Backpack, Initializable, Inventory, System},
};

use crate::shared::{components::Ball, ui_components::UIBallVelocity};

pub struct UIDebugSystem {}

impl Initializable for UIDebugSystem {
  fn initialize(_: &Inventory) -> Self {
    Self {}
  }
}

impl System for UIDebugSystem {
  fn get_name(&self) -> &'static str {
    "UIDebugSystem"
  }

  fn run(&mut self, scene: &mut Scene, _backpack: &mut Backpack) {
    self.show_ball_speed(scene);
  }
}

impl UIDebugSystem {
  fn show_ball_speed(&self, scene: &mut Scene) {
    let mut ball_speed = Vector3::new(0.0, 0.0, 0.0);
    for (_, ball) in scene.query_mut::<&Ball>() {
      ball_speed = ball.current_velocity
    }

    for (_, (text, _)) in scene.query_mut::<(&mut TextComponent, &UIBallVelocity)>() {
      text.text = format!(
        "{:+3.2} {:+3.2} {:+3.2}",
        ball_speed.x, ball_speed.y, ball_speed.z
      );
    }
  }
}
