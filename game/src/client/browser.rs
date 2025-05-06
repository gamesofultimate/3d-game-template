#![cfg(target_arch = "wasm32")]
use crate::shared::{
  developer::DeveloperMode,
  game_input::{GameInput, GameInputState},
};
use engine::{
  application::input::InputsReader,
  application::scene::Scene,
  systems::{
    browser::{BrowserController, BrowserReceiver},
    Backpack, Initializable, Inventory, System,
  },
  tsify,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, tsify::Tsify)]
pub enum Message {
  StartGame,
  StopGame,
  PauseGame,
  TriggerSignup,
  FinishSignup,
  Login,
  TriggerInvitation,
  FinishInvitation,
  Developer(String),
}

pub struct BrowserSystem {
  inputs: InputsReader<GameInput>,
  receiver: BrowserReceiver<Message>,
  controller: BrowserController<Message>,
}

impl Initializable for BrowserSystem {
  fn initialize(inventory: &Inventory) -> Self {
    let inputs = inventory.get::<InputsReader<GameInput>>().clone();
    let receiver = inventory.get::<BrowserReceiver<Message>>().clone();
    let controller = inventory.get::<BrowserController<Message>>().clone();
    Self {
      inputs,
      receiver,
      controller,
    }
  }
}

impl System for BrowserSystem {
  fn get_name(&self) -> &'static str {
    "BrowserSystem"
  }

  fn run(&mut self, _scene: &mut Scene, backpack: &mut Backpack) {
    let input = self.inputs.read_client();
    if input.check(GameInputState::MouseLeftClick) {
      self.controller.send(Message::StartGame);
    }

    if input.check(GameInputState::KeyboardEscape) {
      self.controller.send(Message::StopGame);
    }

    let messages = self.receiver.receive();
    for message in messages {
      match message {
        Message::Developer(key) => {
          if key == "PLEASE_REPLACE_ME" {
            backpack.insert(DeveloperMode);
          }
        }
        _ => {}
      }
    }

    #[cfg(not(feature = "production"))]
    backpack.insert(DeveloperMode);
  }
}
