use engine::{
  application::{
    components::{AudioSourceComponent, SourceState},
    scene::{Collision, Scene, TransformComponent},
  },
  nalgebra::Vector3,
  systems::{Backpack, Initializable, Inventory, System},
};
use rand::Rng;

use super::components::{Ball, Pickup, SpawnArea};

pub struct PickupSystemData {
  pub spawned: usize,
  pub max_pickups: usize,

  pub player_score: usize,
}

pub struct PickupSystem {}

impl Initializable for PickupSystem {
  fn initialize(_inventory: &Inventory) -> Self {
    Self {}
  }
}

impl System for PickupSystem {
  fn get_name(&self) -> &'static str {
    "PickupSystem"
  }

  fn attach(&mut self, _: &mut Scene, backpack: &mut Backpack) {
    backpack.insert(PickupSystemData {
      spawned: 0,
      max_pickups: 5,
      player_score: 0,
    });
  }

  fn run(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    self.handle_spawn_pickup(scene, backpack);
    self.handle_collect_pickup(scene, backpack);
  }
}

impl PickupSystem {
  fn handle_spawn_pickup(&mut self, scene: &mut Scene, backpack: &mut Backpack) -> Option<()> {
    let data = backpack.get_mut::<PickupSystemData>()?;
    if data.spawned >= data.max_pickups {
      return None;
    }

    let (_, (spawner_transform, _)) = scene
      .query_mut::<(&TransformComponent, &SpawnArea)>()
      .into_iter()
      .next()?;

    let spawner_transform = spawner_transform.clone();

    for _ in data.spawned..data.max_pickups {
      let mut rng = rand::thread_rng();
      let x = rng.gen_range(
        spawner_transform.translation.x - spawner_transform.scale.x / 2.0
          ..spawner_transform.translation.x + spawner_transform.scale.x / 2.0,
      );
      let z = rng.gen_range(
        spawner_transform.translation.z - spawner_transform.scale.z / 2.0
          ..spawner_transform.translation.z + spawner_transform.scale.z / 2.0,
      );

      scene.spawn_prefab_with("Pickup Prefab", |prefab| {
        prefab.transform.translation = Vector3::new(x, prefab.transform.translation.y, z);
      });

      data.spawned += 1;
    }

    Some(())
  }

  fn handle_collect_pickup(&mut self, scene: &mut Scene, backpack: &mut Backpack) -> Option<()> {
    let mut collected = vec![];

    for (pickup_entity, (transform, _pickup, _collision)) in
      scene.query_mut::<(&TransformComponent, &Pickup, &Collision<Ball, Pickup>)>()
    {
      collected.push((pickup_entity, transform.translation));

      let data = backpack.get_mut::<PickupSystemData>()?;
      data.spawned -= 1;
      data.player_score += 1;
    }

    for (pickup_entity, pickup_translation) in collected {
      let _ = scene.remove_entity(pickup_entity);

      scene.spawn_prefab_with("Pickup Sound Prefab", |prefab| {
        prefab.transform.translation = pickup_translation;
        prefab.get_mut::<AudioSourceComponent>().unwrap().state = SourceState::Playing;
      });
    }

    Some(())
  }
}
