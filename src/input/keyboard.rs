use winit::VirtualKeyCode;

use std::collections::HashSet;

#[derive(Default)]
pub struct KeyboardState {
    pub pressed_keys: HashSet<VirtualKeyCode>,
    pub released_keys: HashSet<VirtualKeyCode>,
    pub held_keys: HashSet<VirtualKeyCode>,
}

impl KeyboardState {
    pub fn is_pressed(&self, vk: VirtualKeyCode) -> bool {
        self.pressed_keys.contains(&vk)
    }

    pub fn is_released(&self, vk: VirtualKeyCode) -> bool {
        self.released_keys.contains(&vk)
    }

    pub fn is_held(&self, vk: VirtualKeyCode) -> bool {
        self.held_keys.contains(&vk)
    }
}

pub trait KeyboardUpdate {
    fn frame_start(&mut self);
    fn key_pressed(&mut self, vk: VirtualKeyCode);
    fn key_released(&mut self, vk: VirtualKeyCode);
    fn focus_lost(&mut self);
}

impl KeyboardUpdate for KeyboardState {
    fn frame_start(&mut self) {
        self.pressed_keys.clear();
        self.released_keys.clear();
    }

    fn key_pressed(&mut self, vk: VirtualKeyCode) {
        if !self.held_keys.contains(&vk) {
            self.pressed_keys.insert(vk);
        }
        self.held_keys.insert(vk);
    }

    fn key_released(&mut self, vk: VirtualKeyCode) {
        self.released_keys.insert(vk);
        self.held_keys.remove(&vk);
    }

    fn focus_lost(&mut self) {
        self.held_keys.clear();
    }
}
