use engine::{
  application::{animation::AnimationTransition, scene::Scene},
  systems::{Backpack, Registry},
  Entity,
};
use rand::Rng;
use serde::{Deserialize, Serialize};
use tagged::{Duplicate, Registerable, Schema};

use super::components::Health;

pub struct AnimationTransitions {}

impl Registry for AnimationTransitions {
  fn register() {
    use engine::application::animation::animation_transition_registry::Access;

    DeathTransition::register();
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct DeathTransition {}

impl AnimationTransition for DeathTransition {
  fn should_transition(&self, entity: Entity, scene: &mut Scene, _: &Backpack) -> bool {
    let is_dead = match scene.get_components_mut::<&Health>(entity) {
      Some(health) => health.current_health <= 0.0,
      None => return false,
    };

    // Support for multiple death animations
    if is_dead {
      let mut rng = rand::thread_rng();
      let random_value = rng.gen_range(0.0..1.0);

      return random_value < 0.2;
    }

    false
  }
}
