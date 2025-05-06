mod audio_control;
mod browser;
mod camera;
mod particle;
mod ui_debug;
mod ui_score;

use crate::client::browser::BrowserSystem;
use crate::shared::{developer, inputs, pickup};
use crate::{
  client::browser::Message,
  shared::game_input::GameInput,
  shared::{animations, collision, components, ui_components},
};
use engine::application::bus::BrowserBus;
use engine::systems::browser::BrowserActor;
use engine::systems::hdr::HdrMultiplayerPipeline;
use engine::systems::Scheduler;
use engine::utils::browser::grow_memory;

const FRAMES_PER_SECOND: u64 = 60;
const GROW_MEMORY_IN_MB: u32 = 100;

pub fn main(
  canvas_id: String,
  assets_location: String,
  bus: BrowserBus,
  session_id: String,
  connection_id: String,
  unique_id: String,
  access_token: Option<String>,
  udp_url: String,
  tcp_url: String,
  recording_url: Option<String>,
) -> Scheduler {
  wasm_logger::init(wasm_logger::Config::default());
  grow_memory(GROW_MEMORY_IN_MB);
  let mut scheduler = Scheduler::new(FRAMES_PER_SECOND, [0, 0, 0, 255], canvas_id);

  log::debug!("assets location: {:?}", &assets_location);

  let hdr = HdrMultiplayerPipeline::<GameInput>::new(
    assets_location,
    session_id,
    connection_id,
    unique_id,
    access_token,
    udp_url,
    tcp_url,
    recording_url,
  );

  scheduler.attach_plugin(hdr);

  let browser = BrowserActor::<Message, Message>::new(bus);
  scheduler.attach_actor(browser);
  scheduler.attach_system::<BrowserSystem>();

  // Register game components and transitions
  scheduler.attach_registry::<components::GameComponents>();
  scheduler.attach_registry::<ui_components::UIComponents>();
  scheduler.attach_registry::<animations::AnimationTransitions>();

  // Register game systems and middlewares
  scheduler.attach_system::<collision::CollisionSystem>();
  scheduler.attach_system::<inputs::InputsSystem>();
  scheduler.attach_system::<developer::DeveloperSystem>();

  scheduler.attach_system::<pickup::PickupSystem>();

  scheduler.attach_system::<ui_score::UIScoreSystem>();
  scheduler.attach_system::<ui_debug::UIDebugSystem>();

  scheduler.attach_middleware_with_subsystem::<camera::CameraMiddleware, camera::CameraSubsystem>();

  scheduler
}
