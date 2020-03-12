// DO NOT redefine doomlib module/namespace !

// declaration only; definition goes to creaturelib.rs
pub mod creaturelib;

pub fn map(episode: i32, level: i32) -> String {
    format!("e{}m{}", episode, level)
}
