use bevy::prelude::*;

use crate::GameState;

pub struct BeamPlugin;
impl Plugin for BeamPlugin {
    fn build(&self, app: &mut App) {
       app
       .add_systems(Update, (update_beam).run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct Beam {
    pub lifetime: f32,
    pub speed: f32,
    pub direction: Vec2,
    pub power: f32,
}
impl Beam {
    pub fn new(beam_type: BeamType, direction: Vec2) -> Beam {
        match beam_type {
            BeamType::Proton => {
                Beam {
                    lifetime: 4.,
                    speed: 450.,
                    direction: direction,
                    power: 20.,
                }
            }
            BeamType::Laser => {
                Beam {
                    lifetime: 4.,
                    speed: 500.,
                    direction: direction,
                    power: 30.,
                }
            }
            BeamType::Wave => {
                Beam {
                    lifetime: 4.,
                    speed: 500.,
                    direction: direction,
                    power: 50.,
                }
            }
            BeamType::PlasmaOrb => {
                Beam {
                    lifetime: 4.,
                    speed: 500.,
                    direction: direction,
                    power: 50.,
                }
            }
        }
    }
}

pub enum BeamType {
    PlasmaOrb,
    Proton,
    Laser,
    Wave,
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
