#[derive(Debug)]
pub enum KeyState {
    /// Represents that the key has been pressed since the last
    /// call to update()
    JustPressed,
    /// Represents that the key is held down
    Down,
    /// Represents that the key has been released since the
    /// last call to update()
    JustReleased,
    /// Represents that the key is not held down
    Up,
}

impl KeyState {
    pub fn update(&self) -> KeyState {
        match self {
            KeyState::JustPressed => KeyState::Down,
            KeyState::Down => KeyState::Down,
            KeyState::JustReleased => KeyState::Up,
            KeyState::Up => KeyState::Up,
        }
    }

    /// Similar to how is js integer is `truthy` this return if the
    /// key is `downy` - in a state where the player has the key down.
    /// This is includes both the edge and steady state.
    pub fn active(&self) -> bool {
        match self {
            KeyState::JustPressed => true,
            KeyState::Down => true,
            KeyState::JustReleased => false,
            KeyState::Up => false,
        }
    }
}

#[derive(Debug)]
pub struct KeyMap {
    pub forwards: KeyState,
    pub backwards: KeyState,
    pub turn_left: KeyState,
    pub turn_right: KeyState,
}

impl KeyMap {
    pub fn new() -> Self {
        Self {
            forwards: KeyState::Up,
            backwards: KeyState::Up,
            turn_left: KeyState::Up,
            turn_right: KeyState::Up,
        }
    }

    pub fn update(&mut self) {
        self.forwards = self.forwards.update();
        self.backwards = self.backwards.update();
        self.turn_left = self.turn_left.update();
        self.turn_right = self.turn_right.update();
    }

    pub fn set_state_from_str(&mut self, code: &str, new_state: KeyState) {
        match code {
            "KeyW" => self.forwards = new_state,
            "KeyS" => self.backwards = new_state,
            "KeyA" => self.turn_left = new_state,
            "KeyD" => self.turn_right = new_state,
            _ => (),
        }
    }
}
