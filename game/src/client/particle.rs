use engine::{
  application::scene::Scene,
  systems::{controller::ParticleController, Backpack, Initializable, Inventory, System},
};

pub struct ParticleSystem {
  particle_controller: ParticleController,
}

impl Initializable for ParticleSystem {
  fn initialize(inventory: &Inventory) -> Self {
    let particle_controller = inventory.get::<ParticleController>().clone();

    Self {
      particle_controller,
    }
  }
}

impl ParticleSystem {
  fn handle_pickup_particles(&mut self, _scene: &mut Scene, _backpack: &mut Backpack) {}
}

impl System for ParticleSystem {
  fn get_name(&self) -> &'static str {
    "ParticleSystem"
  }

  fn run(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    self.handle_pickup_particles(scene, backpack);
  }
}
