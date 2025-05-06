use engine::{
  application::{
    components::CameraComponent,
    scene::{Scene, TransformComponent},
  },
  nalgebra::{Unit, Vector3},
  systems::{rendering::CameraConfig, Backpack, Initializable, Inventory, Middleware, Subsystem},
};

pub struct CameraTracking {
  pub tracking: Option<bool>,
}

pub struct CameraMiddleware {}

impl Initializable for CameraMiddleware {
  fn initialize(_inventory: &Inventory) -> Self {
    Self {}
  }
}

impl CameraMiddleware {
  fn update_camera_translation(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    for (_, (transform, camera)) in scene.query_mut::<(&mut TransformComponent, &CameraComponent)>()
    {
      let camera_position = transform.world_transform();

      if let CameraComponent::Perspective {
        fovy, zfar, znear, ..
      } = camera
        && let Some(camera_config) = backpack.get_mut::<CameraConfig>()
      {
        camera_config.fovy = *fovy;
        camera_config.znear = *znear;
        camera_config.zfar = *zfar;
        camera_config.translation = camera_position.translation;
      }
    }
  }

  fn update_camera_rotation(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    for (_, (transform, camera)) in scene.query_mut::<(&mut TransformComponent, &CameraComponent)>()
    {
      if let CameraComponent::Perspective { .. } = camera
        && let Some(camera_config) = backpack.get_mut::<CameraConfig>()
      {
        camera_config.front = transform.get_forward_direction();
        camera_config.up = Unit::new_normalize(Vector3::y());
      }
    }
  }
}

impl Middleware for CameraMiddleware {
  fn get_name(&self) -> &'static str {
    "CameraMiddleware"
  }

  fn post(&mut self, scene: &mut Scene, backpack: &mut Backpack) {
    self.update_camera_translation(scene, backpack);
    self.update_camera_rotation(scene, backpack);
  }
}

pub struct CameraSubsystem;

impl Subsystem for CameraSubsystem {
  fn get_priority() -> isize {
    0_100_000
  }
}
