use bevy::{prelude::*, utils::HashMap};

use crate::{basic_enemy, basic_enemy_move_patterns::EnemyMovePattern, level::Wave};

pub struct EnemyCorePlugin;
impl Plugin for EnemyCorePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, init_assets)
        .add_event::<SpawnEnemyEvent>()
        .add_systems(Update, spawn_enemy)
        ;
    }
}

#[derive(Component)]
pub struct EnemyCore {
    pub x_direction: f32,
    pub y_direction: f32,
    pub health: f32,
    move_pattern: EnemyMovePattern,
    state: EnemyState,
    shoot: bool,
}
#[derive(Bundle)]
pub struct EnemyCoreBundle {
    pub enemy_core: EnemyCore,
}
impl Default for EnemyCoreBundle {
    fn default() -> Self {
       EnemyCoreBundle { 
        enemy_core: EnemyCore {
            x_direction: 0.,
            y_direction: 0.,
            health: 100.,
            move_pattern: EnemyMovePattern::Basic,
            state: EnemyState::Active,
            shoot: false,
            }
        }
    }
}

#[derive(Default)]
pub enum EnemyType {
    #[default]
    Basic,
    EnemyB,
}

#[derive(Event)]
pub struct SpawnEnemyEvent(
    pub (
        EnemyCore, 
        EnemyType,
        Transform,
        Wave,
    )
);

#[derive(Resource)]
struct ShootTimer(Timer);

pub enum EnemyState {
    Active,
    Dead,
}
#[derive(Resource)]
pub struct EnemyHandles {
    basic_enemy: Handle<Image>,
    enemy_b: Handle<Image>,
}

fn init_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let basic_enemy = asset_server.load("enemy_test.png");
    let enemy_b = asset_server.load("enemy_b.png");
    commands.insert_resource(
        EnemyHandles {
            basic_enemy: basic_enemy,
            enemy_b: enemy_b,
        }
    );
}

fn spawn_enemy(
    mut commands: Commands,
    mut events: EventReader<SpawnEnemyEvent>,
    mut enemy_handles: ResMut<EnemyHandles>,
) {
    if !events.is_empty() {
        for event in events.read() {
            let (core, enemy_type, transform, wave) = &event.0;
            let texture = get_enemy_texture(&enemy_type, enemy_handles.as_ref());
            let mut core = core;
            commands.spawn((
                EnemyCoreBundle {
                    enemy_core: core.clone()
                },
                SpriteBundle {
                    texture: texture,
                    transform: transform.clone(),
                    ..default()
                    },
                wave.clone()
            ));
        }
    }
}

fn get_enemy_texture(
    enemy_type: &EnemyType,
    enemy_handles: &EnemyHandles
) -> Handle<Image> {
    match enemy_type {
        EnemyType::Basic => enemy_handles.basic_enemy.clone(),
        EnemyType::EnemyB => enemy_handles.enemy_b.clone(),
    }
}