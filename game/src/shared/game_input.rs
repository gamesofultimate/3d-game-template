use engine::systems::Backpack;
use engine::{
  application::devices::{ButtonState, DeviceEvent, GamepadEvent, KeyboardEvent, KeyboardKey},
  application::input::Input,
  utils::units::Seconds,
};

use bitflags::bitflags;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
enum GameButtonState {
  Down,
  Up,
  JustPressed,
  JustReleased,
}

impl Default for GameButtonState {
  fn default() -> Self {
    GameButtonState::Up
  }
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
pub struct GameButton {
  state: GameButtonState,
}

impl GameButton {
  pub fn new() -> Self {
    GameButton {
      state: GameButtonState::Up,
    }
  }

  pub fn just_pressed(&self) -> bool {
    match self.state {
      GameButtonState::JustPressed => true,
      _ => false,
    }
  }

  pub fn just_released(&self) -> bool {
    match self.state {
      GameButtonState::JustReleased => true,
      _ => false,
    }
  }

  pub fn is_down(&self) -> bool {
    match self.state {
      GameButtonState::Down | GameButtonState::JustPressed => true,
      _ => false,
    }
  }

  pub fn is_up(&self) -> bool {
    match self.state {
      GameButtonState::Up | GameButtonState::JustReleased => true,
      _ => false,
    }
  }

  pub fn press(&mut self) {
    self.state = GameButtonState::JustPressed;
  }

  pub fn release(&mut self) {
    self.state = GameButtonState::JustReleased;
  }

  pub fn tick(&mut self) {
    match self.state {
      GameButtonState::JustPressed => self.state = GameButtonState::Down,
      GameButtonState::JustReleased => self.state = GameButtonState::Up,
      _ => {}
    }
  }
}

bitflags! {
  #[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
  pub struct GameInputState: u32 {
    const Focused = 0b00000001;
    const MouseLock = 0b00000010;
    const IsFullscreen = 0b00000100;
    const KeyboardShift = 0b00001000;
    const KeyboardEscape = 0b00010000;
    const MouseLeftClick = 0b00100000;
    const MouseRightClick = 0b01000000;
    const ToggleDebugPhysics = 0b10000000_00000000_00000000;
    const ToggleDebugLines = 0b00000001_00000000_00000000_00000000;
    const ToggleDebugPerformance = 0b00000010_00000000_00000000_00000000;
    const ToggleDebugGoaps = 0b00000100_00000000_00000000_00000000;
    const HasMouse = 0b00001000_00000000_00000000_00000000;
    const HasJoystick = 0b00010000_00000000_00000000_00000000;
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameInput {
  pub forward: f32,
  pub right: f32,
  state: GameInputState,

  pub developer_reset_ball: GameButton,
}

impl GameInput {
  pub fn new() -> Self {
    Self {
      forward: 0.0,
      right: 0.0,
      state: GameInputState::default(),

      developer_reset_ball: GameButton::new(),
    }
  }

  fn tick(&mut self) {
    self.developer_reset_ball.tick();
  }

  pub fn check(&self, state: GameInputState) -> bool {
    self.state.contains(state)
  }

  fn handle_joystick(&mut self, event: DeviceEvent) {
    match event {
      DeviceEvent::Gamepad(_, GamepadEvent::Joystick { left, .. }) => {
        const MIN_EPSILON: f32 = 0.0 - 0.02;
        const MAX_EPSILON: f32 = 0.0 + 0.02;

        if left.x > MAX_EPSILON || left.x < MIN_EPSILON {
          self.right += left.x;
        }
        if left.y > MAX_EPSILON || left.y < MIN_EPSILON {
          self.forward += -left.y;
        }
      }
      _ => {}
    }
  }

  fn handle_keyboard(&mut self, event: DeviceEvent) {
    match event {
      DeviceEvent::Keyboard(KeyboardEvent::Button(
        ButtonState::Down,
        KeyboardKey::D | KeyboardKey::Right,
      )) => {
        self.right += 1.0;
      }
      DeviceEvent::Keyboard(KeyboardEvent::Button(
        ButtonState::Down,
        KeyboardKey::A | KeyboardKey::Left,
      )) => {
        self.right -= 1.0;
      }
      DeviceEvent::Keyboard(KeyboardEvent::Button(
        ButtonState::Down,
        KeyboardKey::W | KeyboardKey::Up,
      )) => {
        self.forward += 1.0;
      }
      DeviceEvent::Keyboard(KeyboardEvent::Button(
        ButtonState::Down,
        KeyboardKey::S | KeyboardKey::Down,
      )) => {
        self.forward -= 1.0;
      }

      DeviceEvent::Keyboard(KeyboardEvent::Button(
        ButtonState::Up,
        KeyboardKey::D | KeyboardKey::Right,
      )) => {
        self.right -= 1.0;
      }
      DeviceEvent::Keyboard(KeyboardEvent::Button(
        ButtonState::Up,
        KeyboardKey::A | KeyboardKey::Left,
      )) => {
        self.right += 1.0;
      }
      DeviceEvent::Keyboard(KeyboardEvent::Button(
        ButtonState::Up,
        KeyboardKey::W | KeyboardKey::Up,
      )) => {
        self.forward -= 1.0;
      }
      DeviceEvent::Keyboard(KeyboardEvent::Button(
        ButtonState::Up,
        KeyboardKey::S | KeyboardKey::Down,
      )) => {
        self.forward += 1.0;
      }

      DeviceEvent::Keyboard(KeyboardEvent::Button(ButtonState::Down, KeyboardKey::O)) => {
        self.state |= GameInputState::ToggleDebugPhysics;
      }
      DeviceEvent::Keyboard(KeyboardEvent::Button(ButtonState::Up, KeyboardKey::O)) => {
        self.state -= GameInputState::ToggleDebugPhysics;
      }

      DeviceEvent::Keyboard(KeyboardEvent::Button(ButtonState::Down, KeyboardKey::P)) => {
        self.state |= GameInputState::ToggleDebugPerformance;
      }
      DeviceEvent::Keyboard(KeyboardEvent::Button(ButtonState::Up, KeyboardKey::P)) => {
        self.state -= GameInputState::ToggleDebugPerformance;
      }

      DeviceEvent::Keyboard(KeyboardEvent::Button(
        ButtonState::Down,
        KeyboardKey::Key0 | KeyboardKey::Numpad0,
      )) => {
        self.developer_reset_ball.press();
      }
      DeviceEvent::Keyboard(KeyboardEvent::Button(
        ButtonState::Up,
        KeyboardKey::Key0 | KeyboardKey::Numpad0,
      )) => {
        self.developer_reset_ball.release();
      }
      _ => {}
    }
  }

  fn handle_mouse(&mut self, _event: DeviceEvent) {}

  fn handle_window(&mut self, _event: DeviceEvent) {}
}

impl Default for GameInput {
  fn default() -> Self {
    Self::new()
  }
}

impl Input for GameInput {
  fn from_backpack(&mut self, _: &mut Backpack) {}

  fn reset(&mut self) {
    // Uncomment this to only allow one direction at a time
    // self.forward = 0.0;
    // self.right = 0.0;
  }

  fn from_devices(&mut self, event: DeviceEvent, _: Seconds) {
    self.tick();

    self.handle_joystick(event);
    self.handle_keyboard(event);
    self.handle_mouse(event);
    self.handle_window(event);

    self.forward = self.forward.clamp(-1.0, 1.0);
    self.right = self.right.clamp(-1.0, 1.0);
  }
}
