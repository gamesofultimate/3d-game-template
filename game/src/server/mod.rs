mod network_controller;

use crate::server::network_controller::NetworkController;
use crate::shared::animations::AnimationTransitions;
use crate::shared::game_input::GameInput;
use crate::shared::{components, ui_components};
use engine::systems::hdr::HdrPipeline;
use engine::systems::Scheduler;

const FRAMES_PER_SECOND: u64 = 60;

pub async fn main() {
  dotenv::dotenv().ok();
  env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

  log::info!("Number of cpus: {:}", num_cpus::get());

  let rpc_address = {
    let address = dotenv::var("RPC_ADDRESS").unwrap();
    let port = dotenv::var("RPC_PORT").unwrap();

    format!("{}:{}", address, port).parse().unwrap()
  };

  let session_address = {
    let address = dotenv::var("GAME_ADDRESS").unwrap();
    let port = dotenv::var("GAME_PORT").unwrap();

    format!("{}:{}", address, port).parse().unwrap()
  };

  let (hdr, _) =
    HdrPipeline::<NetworkController, GameInput>::new("resources", rpc_address, session_address);
  let mut runner = Scheduler::new(FRAMES_PER_SECOND);
  runner.attach_plugin(hdr);
  runner.attach_registry::<components::GameComponents>();
  runner.attach_registry::<ui_components::UIComponents>();
  runner.attach_registry::<AnimationTransitions>();

  #[cfg(feature = "multiplayer")]
  {
    use crate::shared::{
      audio_control, checkpoint, collision, developer, door, generator, grenade, health, inputs,
      interactable, level, lifecycle, lift, light, lockdown, mission_timer, oxygen, pickup,
      platform, shooting, spawn,
    };

    runner.attach_system::<inputs::InputsSystem>();
    runner.attach_system::<collision::CollisionSystem>();

    runner.attach_system::<developer::DeveloperSystem>();
  }

  runner.run().await;
}
