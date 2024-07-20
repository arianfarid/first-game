use bevy::{math::Vec2, time::{Timer, TimerMode}};

use crate::beam::{Beam, BeamType, ShootPattern};

pub struct FlatSpray;
impl FlatSpray {
    pub fn new() -> ShootPattern {
        ShootPattern { 
            beam: vec![
                Beam::new(&BeamType::FireBall, Vec2::new(1., -1.)),
                Beam::new(&BeamType::FireBall, Vec2::new(0.9, -1.)),
                Beam::new(&BeamType::FireBall, Vec2::new(0.8, -1.)),
                Beam::new(&BeamType::FireBall, Vec2::new(0.7, -1.)),
                Beam::new(&BeamType::FireBall, Vec2::new(0.6, -1.)),
                Beam::new(&BeamType::FireBall, Vec2::new(0.5, -1.)),
                Beam::new(&BeamType::FireBall, Vec2::new(0.4, -1.)),
                Beam::new(&BeamType::FireBall, Vec2::new(0.3, -1.)),
                Beam::new(&BeamType::FireBall, Vec2::new(0.2, -1.)),
                Beam::new(&BeamType::FireBall, Vec2::new(0.1, -1.)),
                Beam::new(&BeamType::FireBall, Vec2::new(0.0, -1.)),
            ],
            timer: Timer::from_seconds(0.25, TimerMode::Repeating)
        }
    }
}