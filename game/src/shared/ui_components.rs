use engine::application::scene::ProvideAssets;
use engine::systems::Registry;

use tagged::{Duplicate, Registerable, Schema};

use serde::{Deserialize, Serialize};

pub struct UIComponents;

impl Registry for UIComponents {
  fn register() {
    use engine::application::scene::component_registry::Access;

    UIBallVelocity::register();
    UIScore::register();
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct UIBallVelocity {}

impl ProvideAssets for UIBallVelocity {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct UIScore {}

impl ProvideAssets for UIScore {}
