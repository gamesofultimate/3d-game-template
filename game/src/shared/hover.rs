use engine::{
  application::{
    components::PhysicsComponent,
    scene::{Scene, TransformComponent},
  },
  systems::{physics::PhysicsController, Backpack, Initializable, Inventory, System},
  utils::{easing::ease, units::Seconds},
};

use super::components::Hover;

pub struct HoverSystem {
  physics: PhysicsController,
}

impl Initializable for HoverSystem {
  fn initialize(inventory: &Inventory) -> Self {
    let physics = inventory.get::<PhysicsController>().clone();
    Self { physics }
  }
}

impl System for HoverSystem {
  fn get_name(&self) -> &'static str {
    "HoverSystem"
  }

  fn run(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    self.hover(scene, backpack);
  }
}

impl HoverSystem {
  fn hover(&self, scene: &mut Scene, backpack: &mut Backpack) -> Option<()> {
    let delta_time = backpack.get::<Seconds>()?;

    for (_, (transform, physics, hover)) in
      scene.query_mut::<(&mut TransformComponent, &PhysicsComponent, &mut Hover)>()
    {
      if hover.starting_y.is_none() {
        hover.starting_y = Some(transform.translation.y);
      }

      let mut translation = transform.translation;
      translation.y = calculate_hover_height(hover, *delta_time);
      self.physics.set_translation(physics, translation)
    }

    Some(())
  }
}

fn calculate_hover_height(hover: &mut Hover, delta_time: Seconds) -> f32 {
  hover.current_time += delta_time;

  if hover.current_time >= hover.loop_duration {
    hover.current_time = Seconds::new(0.0);
  }

  if hover.current_time < hover.loop_duration / 2.0 {
    let t = *hover.current_time / (*hover.loop_duration / 2.0);
    let t = ease(hover.easing, t);
    return hover.starting_y.unwrap() + *hover.amplitude * t;
  } else {
    let t = (*hover.current_time - (*hover.loop_duration / 2.0)) / (*hover.loop_duration / 2.0);
    let t = ease(hover.easing, 1.0 - t);

    return hover.starting_y.unwrap() + *hover.amplitude * t;
  }
}
