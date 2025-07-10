use std::fmt::Display;
use evdev_rs::enums::EV_KEY;

use super::keymapper::KEYMAPPER;

pub struct KeyBuffer {
    pub keys: [i32; 6],
    pub grabbed: bool,
    pub ungrab_shortcut: [i32; 6],
}

impl KeyBuffer {
    pub fn new() -> Self {
        KeyBuffer {
            keys: [0; 6],
            grabbed: false,
            ungrab_shortcut: [
                EV_KEY::KEY_LEFTCTRL as i32,
                EV_KEY::KEY_LEFTSHIFT as i32,
                EV_KEY::KEY_EQUAL as i32,
                0,
                0,
                0,
            ],
        }
    }

    /// Checks if the ungrab skortcut is pressed
    pub fn is_ungrab_pressed(&self) -> bool {
        for ungrab_key in self.ungrab_shortcut {
            let mut result = false;
            if ungrab_key != 0 {
                for pressed_key in self.keys {
                    if pressed_key == ungrab_key {
                        result = true;
                        break;
                    }
                }
                if !result {
                    return false;
                }
            }
        }
        true
    }

    /// Adds a key to the buffer if there is space (and it isn't already there). Returns whether or not the key was added.
    pub fn press_key(&mut self, key: i32) -> bool {
        for k in self.keys.iter_mut() {
            if *k == key {
                return false;
            } else if *k == 0 {
                *k = key;
                return true;
            }
        }
        false
    }

    /// removes a key from the buffer if it isn't already there. Returns whether or not the key was removed.
    pub fn release_key(&mut self, key: i32) -> bool {
        for k in self.keys.iter_mut() {
            if *k == key {
                *k = 0;
                return true;
            }
        }
        false
    }

    pub fn to_hid(&self) -> [u8; 6] {
        self.keys.map(|k| KEYMAPPER[k as usize])
    }
}

impl Display for KeyBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pressed: Vec<i32> = self.keys.into_iter().filter(|&k| k != 0).collect();
        write!(f, "{:?}", pressed)
    }
}

