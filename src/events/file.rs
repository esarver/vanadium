use std::path::PathBuf;

use bevy_ecs::{event::Event, system::Commands};


#[derive(Debug, Event)]
pub struct Open(PathBuf);
