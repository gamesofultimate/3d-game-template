use engine::{
  application::scene::ProvideAssets,
  nalgebra::Vector3,
  systems::Registry,
  utils::{
    easing::Easing,
    units::{Meters, Mps, Seconds},
  },
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
    SpawnArea::register();
    Hover::register();
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

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct SpawnArea {}

impl ProvideAssets for SpawnArea {}

#[derive(Debug, Clone, Serialize, Deserialize, Registerable, Schema, Duplicate)]
pub struct Hover {
  pub amplitude: Meters,
  pub easing: Easing,
  pub loop_duration: Seconds,

  #[serde(skip)]
  pub starting_y: Option<f32>,

  #[serde(skip)]
  pub current_time: Seconds,
}

impl ProvideAssets for Hover {}
