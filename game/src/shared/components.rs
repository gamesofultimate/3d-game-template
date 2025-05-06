use engine::{
  application::scene::ProvideAssets, nalgebra::Vector3, systems::Registry, utils::units::Mps,
};
use tagged::{Duplicate, Registerable, Schema};

use serde::{Deserialize, Serialize};

pub struct GameComponents;

impl Registry for GameComponents {
  fn register() {
    use engine::application::scene::component_registry::Access;

    Health::register();
    Ball::register();
    Pickup::register();
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Health {
  pub current_health: f32,
  pub max_health: f32,
}

impl ProvideAssets for Health {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Ball {
  pub acceleration: Mps,
  pub max_speed: Mps,

  #[serde(skip)]
  pub current_velocity: Vector3<f32>,
}

impl ProvideAssets for Ball {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Pickup {}

impl ProvideAssets for Pickup {}
