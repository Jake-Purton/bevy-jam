use bevy::{prelude::*, window::PrimaryWindow};



pub fn despawn_everything(
    query: Query<Entity, Without<PrimaryWindow>>,
    mut commands: Commands,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}