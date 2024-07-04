use bevy::prelude::*;

use crate::{basic_enemy::BasicEnemy, basic_enemy_move_patterns::EnemyMovePattern};

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
       app
        .init_state::<Wave>()
        .init_state::<WaveState>()
        .add_systems(OnTransition {from: WaveState::Active, to: WaveState::Completed}, increment_state)
        .add_systems(Update, check_wave_complete)
        .add_systems(OnEnter(Wave::One), wave_one)
        .add_systems(OnEnter(Wave::Two), wave_two)
       ;
    }
}

// We will want to lock out some time between waves
#[derive(Component, Resource)]
struct WaveTimer(Timer);
// Used to tell if we are currently in a state
#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum WaveState {
    #[default]
    Active,
    Completed,
    // GameOver,
}

// Todo, have struct of waves/states w/ enemy numbers (and when) to spawn,
// cooldown timer.
#[derive(Clone, Component, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum Wave {
    #[default]
    One,
    Two,
    Three,
    Four,
    Five,
    Boss,
    None
}
impl Wave {
    fn next(&self) -> Self {
        use Wave::*;
        match *self {
            One => Two,
            Two => Three,
            Three => Four,
            Four => Five,
            Five => Boss,
            Boss => None,
            None => None,
        }
    }
}

fn increment_state(
    state: Res<State<Wave>>,
    mut next_state: ResMut<NextState<Wave>>,
) {
    match state.get() {
        Wave::None => {},
        _ => next_state.set(state.next())
    }
}
fn check_wave_complete(
    state: Res<State<Wave>>,
    query: Query<&Wave>,
    mut next_wave_state: ResMut<NextState<WaveState>>
) {
    //check enemy count
    if query.iter().count() < 1 {
        next_wave_state.set(WaveState::Completed)
    }
}


fn wave_one(mut commands: Commands, asset_server: Res<AssetServer>, mut next_wave_state: ResMut<NextState<WaveState>>) {
    next_wave_state.set(WaveState::Active); //could schedule this
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("enemy_test.png"),
            transform: Transform::from_xyz(0., 300., 0.),
            ..default()
        },
        BasicEnemy::new(EnemyMovePattern::Basic),
        Wave::One,
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("enemy_test.png"),
            transform: Transform::from_xyz(60., 300., 0.),
            ..default()
        },
        BasicEnemy::new(EnemyMovePattern::Basic),
        Wave::One,
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("enemy_test.png"),
            transform: Transform::from_xyz(-60., 300., 0.),
            ..default()
        },
        BasicEnemy::new(EnemyMovePattern::Basic),
        Wave::One,
    ));
}
fn wave_two (mut commands: Commands, asset_server: Res<AssetServer>, mut next_wave_state: ResMut<NextState<WaveState>>) {
    next_wave_state.set(WaveState::Active); //could schedule this
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("enemy_test.png"),
            transform: Transform::from_xyz(0., 300., 0.),
            ..default()
        },
        BasicEnemy::new(EnemyMovePattern::Basic),
        Wave::Two,
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("enemy_test.png"),
            transform: Transform::from_xyz(60., 300., 0.),
            ..default()
        },
        BasicEnemy::new(EnemyMovePattern::Basic),
        Wave::Two,
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("enemy_test.png"),
            transform: Transform::from_xyz(-60., 300., 0.),
            ..default()
        },
        BasicEnemy::new(EnemyMovePattern::Basic),
        Wave::Two,
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("enemy_test.png"),
            transform: Transform::from_xyz(120., 300., 0.),
            ..default()
        },
        BasicEnemy::new(EnemyMovePattern::Basic),
        Wave::Two,
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("enemy_test.png"),
            transform: Transform::from_xyz(180., 300., 0.),
            ..default()
        },
        BasicEnemy::new(EnemyMovePattern::Basic),
        Wave::Two,
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("enemy_test.png"),
            transform: Transform::from_xyz(-120., 300., 0.),
            ..default()
        },
        BasicEnemy::new(EnemyMovePattern::Basic),
        Wave::Two,
    ));
}
