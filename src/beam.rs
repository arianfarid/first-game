use bevy::prelude::*;
use flat_spray::FlatSpray;
use line_spray::LineSpray;

use crate::{basic_enemy::BEAM_SPEED, shoot_patterns::*, GameState};

pub struct BeamPlugin;
impl Plugin for BeamPlugin {
    fn build(&self, app: &mut App) {
       app
       .add_systems(Update, (update_beam).run_if(in_state(GameState::Playing)));
    }
}

#[derive(Clone, Component, Debug)]
pub struct Beam {
    pub beam_type: BeamType,
    pub lifetime: f32,
    pub speed: f32,
    pub direction: Vec2,
    pub power: f32,
}

#[derive(Clone, Debug, Default)]
pub enum ShootType {
    #[default]
    Basic,
    TestHell(ShootPattern)
}
#[derive(Clone, Debug)]
pub struct ShootPattern {
    pub beam: Vec<Beam>,
    pub timer: Timer
}
impl ShootPattern {
    pub fn flat_spray() -> ShootPattern {
        FlatSpray::new()
    }
    pub fn line_spray() -> ShootPattern {
        LineSpray::new()
    }
}

impl Beam {
    pub fn new(beam_type: &BeamType, direction: Vec2) -> Beam {
        match beam_type {
            BeamType::Proton => {
                Beam {
                    beam_type: beam_type.clone(),
                    lifetime: 4.,
                    speed: BEAM_SPEED,
                    direction: direction,
                    power: 20.,
                }
            }
            BeamType::Laser => {
                Beam {
                    beam_type: beam_type.clone(),
                    lifetime: 4.,
                    speed: BEAM_SPEED,
                    direction: direction,
                    power: 30.,
                }
            }
            BeamType::Wave => {
                Beam {
                    beam_type: beam_type.clone(),
                    lifetime: 4.,
                    speed: BEAM_SPEED + 300.,
                    direction: direction,
                    power: 50.,
                }
            }
            BeamType::PlasmaOrb => {
                Beam {
                    beam_type: beam_type.clone(),
                    lifetime: 4.,
                    speed: BEAM_SPEED,
                    direction: direction,
                    power: 50.,
                }
            }
            BeamType::FireBall => {
                Beam {
                    beam_type: beam_type.clone(),
                    lifetime: 4.,
                    speed: 600.,
                    direction: direction,
                    power: 10.,
                }
            }
        }
    }

    pub fn speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }
}

#[derive(Clone, Debug)]
pub enum BeamType {
    PlasmaOrb,
    Proton,
    Laser,
    Wave,
    FireBall,
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
