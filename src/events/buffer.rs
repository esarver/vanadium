use bevy_ecs::event::Event;

/// Event to open a buffer
#[derive(Debug, Event)]
pub struct Open;

/// Event to close a buffer
#[derive(Debug, Event)]
pub struct Close;

/// Event to move a buffer
#[derive(Debug, Event)]
pub struct Move;

/// Event to show a buffer
#[derive(Debug, Event)]
pub struct Show;

///Event to hide a buffer
#[derive(Debug, Event)]
pub struct Hide;
