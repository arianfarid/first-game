use bevy::prelude::*;

pub struct BeamPlugin;
impl Plugin for BeamPlugin {
    fn build(&self, app: &mut App) {
       app
       .add_systems(Update, update_beam);
    }
}

#[derive(Component)]
pub struct Beam
{
    pub lifetime: f32,
    pub speed: f32,
    pub direction: Vec2,
    pub power: f32,
}

pub fn update_beam(
    mut beam_query: Query<(&mut Beam, &mut Transform, Entity)>,
    time:Res<Time>,
    mut commands : Commands) {
        for (mut beam, mut transform, entity) in beam_query.iter_mut() {
            beam.lifetime -= time.delta_seconds();
            let moving = beam.speed * beam.direction * time.delta_seconds();
            transform.translation += Vec3::new(moving.x,moving.y,0.);
            if beam.lifetime <=0.
            {
                commands.entity(entity).despawn();
            }
        }
    }
