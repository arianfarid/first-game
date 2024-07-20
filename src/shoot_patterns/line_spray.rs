use bevy::{math::Vec2, time::{Timer, TimerMode}, };

use crate::{basic_enemy::BEAM_SPEED, beam::{Beam, BeamType, ShootPattern}};

pub struct LineSpray;
impl LineSpray {
    pub fn new() -> ShootPattern {
        ShootPattern { 
            beam: vec![
                Beam::new(&BeamType::FireBall, Vec2::new(0., -1.)).speed(BEAM_SPEED),
                Beam::new(&BeamType::FireBall, Vec2::new(0., -1.)).speed(BEAM_SPEED - 20.),
                Beam::new(&BeamType::FireBall, Vec2::new(0., -1.)).speed(BEAM_SPEED - 40.),
                Beam::new(&BeamType::FireBall, Vec2::new(0., -1.)).speed(BEAM_SPEED - 60.),
                Beam::new(&BeamType::FireBall, Vec2::new(0., -1.)).speed(BEAM_SPEED - 80.),
                Beam::new(&BeamType::FireBall, Vec2::new(0., -1.)).speed(BEAM_SPEED - 100.),
                Beam::new(&BeamType::FireBall, Vec2::new(0., -1.)).speed(BEAM_SPEED - 120.),
                Beam::new(&BeamType::FireBall, Vec2::new(0., -1.)).speed(BEAM_SPEED - 140.),
                Beam::new(&BeamType::FireBall, Vec2::new(0., -1.)).speed(BEAM_SPEED - 160.),
                Beam::new(&BeamType::FireBall, Vec2::new(0., -1.)).speed(BEAM_SPEED - 180.),
                Beam::new(&BeamType::FireBall, Vec2::new(0., -1.)).speed(BEAM_SPEED - 200.),
            ],
            timer: Timer::from_seconds(0.25, TimerMode::Repeating)
        }
    }
}