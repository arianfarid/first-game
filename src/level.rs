use bevy::prelude::*;

use crate::{basic_enemy::BasicEnemy, basic_enemy_move_patterns::EnemyMovePattern, enemy_core::{EnemyCore, EnemyCoreBuilder, EnemyCoreBundle, EnemyType, SpawnEnemyEvent}};

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
       app
        .init_state::<Wave>()
        .init_state::<WaveState>()
        .insert_resource(SpawnTimer(Timer::from_seconds(0.1, TimerMode::Repeating)))
        .add_systems(OnTransition {from: WaveState::Active, to: WaveState::Completed}, increment_state)
        .add_systems(Update, check_wave_complete)
        .add_systems(OnEnter(Wave::One), wave_one)
        .add_systems(OnEnter(Wave::Two), wave_two)
        .add_systems(OnEnter(Wave::Three), wave_three)
        .add_systems(OnEnter(Wave::Four), wave_four)
        .add_systems(Update, (spawn_horde).run_if(in_state(Wave::Three)))
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
    Initialized,
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
        _ => {
            next_state.set(state.next());
            println!("STATE: {:?}", state);
        }
    }
}
fn check_wave_complete(
    query: Query<&Wave>,
    mut next_wave_state: ResMut<NextState<WaveState>>
) {
    //check enemy count
    if query.iter().count() < 1 {
        next_wave_state.set(WaveState::Completed);
    }
}


fn wave_one(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    mut next_wave_state: ResMut<NextState<WaveState>>,
    mut spawn_enemy_event_writer: EventWriter<SpawnEnemyEvent>
) {
    spawn_enemy_event_writer.send(SpawnEnemyEvent((
        EnemyCoreBundle {
            enemy_core: EnemyCore::builder().direction(1., 0.).build()
        },
        EnemyType::Basic,
        Transform::from_xyz(0., 300., 0.),
        Wave::One,
    )));
    // commands.spawn((
    //     SpriteBundle {
    //         texture: asset_server.load("enemy_test.png"),
    //         transform: Transform::from_xyz(60., 300., 0.),
    //         ..default()
    //     },
    //     BasicEnemy::new(EnemyMovePattern::Basic),
    //     Wave::One,
    // ));
    // commands.spawn((
    //     SpriteBundle {
    //         texture: asset_server.load("enemy_test.png"),
    //         transform: Transform::from_xyz(-60., 300., 0.),
    //         ..default()
    //     },
    //     BasicEnemy::new(EnemyMovePattern::Basic),
    //     Wave::One,
    // ));
    next_wave_state.set(WaveState::Active); //could schedule this

}
fn wave_two (
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    mut next_wave_state: ResMut<NextState<WaveState>>,
    mut spawn_enemy_event_writer: EventWriter<SpawnEnemyEvent>
) {
    // next_wave_state.set(WaveState::Active); //could schedule this
    // commands.spawn((
    //     SpriteBundle {
    //         texture: asset_server.load("enemy_test.png"),
    //         transform: Transform::from_xyz(0., 300., 0.),
    //         ..default()
    //     },
    //     BasicEnemy::new(EnemyMovePattern::Basic),
    //     Wave::Two,
    // ));
    // spawn_enemy_event_writer.send(SpawnEnemyEvent((
    //     EnemyCoreBundle {
    //         enemy_core: EnemyCore::builder().direction(1., 0.).build()
    //     },
    //     EnemyType::Basic,
    //     Transform::from_xyz(0., 300., 0.),
    //     Wave::Two,
    // )));
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


#[derive(Resource)]
pub struct SpawnTimer(Timer);
#[derive(Component, Debug)]
pub struct WaveHordeCount {
    count: i32,
    max: i32,
}
impl WaveHordeCount {
    pub fn new(max: i32) -> Self {
        WaveHordeCount {
            count: 0,
            max: max,
        }
    }
}
fn wave_three (
    mut commands: Commands, 
    mut spawn_timer: ResMut<SpawnTimer>, 
    mut next_wave_state: ResMut<NextState<WaveState>>
) {
    next_wave_state.set(WaveState::Active); //could schedule this
    commands.spawn(WaveHordeCount::new(100));
    spawn_timer.0.reset();
}

pub fn spawn_horde(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    mut spawn_timer: ResMut<SpawnTimer>,
    mut horde_count_query: Query<&mut WaveHordeCount>,
    time: Res<Time>,
) {
    let horde_count = horde_count_query.get_single_mut();
    match horde_count {
        Ok(mut horde_count) => {
            if horde_count.count < horde_count.max {
                if spawn_timer.0.tick(time.delta()).finished() {
                    commands.spawn((
                        SpriteBundle {
                            texture: asset_server.load("enemy_test.png"),
                            transform: Transform::from_xyz(0., 400., 0.),
                            ..default()
                        },
                        BasicEnemy::new(EnemyMovePattern::Down).shoot(false).health(1.),
                        Wave::Three,
                    ));
                    spawn_timer.0.reset();
                }
                horde_count.count += 1;
            } else {
                
            }
        }
        Err(err) => {
            eprintln!("err: {:?}", err);
        }
    }
}

fn wave_four() {
    //
}