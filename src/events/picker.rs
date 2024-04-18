use bevy_ecs::entity::Entity;


#[derive(Debug, Event)]
pub struct Open;

#[derive(Debug, Event)]
pub struct Close;

#[derive(Debug, Event)]
pub struct Accept(Entity);

