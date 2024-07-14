use bevy::{prelude::*};

use crate::basic_enemy::BasicEnemy;

#[derive(Clone, Debug, Default)]
pub enum EnemyMovePattern {
    #[default]
    Basic,
    Down,
    StartShootGo,
}

pub fn basic_move (transform: Mut<Transform>, mut enemy: Mut<BasicEnemy>) {
    //simply flip direction depending on bounds
    if transform.translation.x >= 600. {
        enemy.x_direction = -1.
    } else if transform.translation.x <= -600. {
        enemy.x_direction = 1.
    }
}