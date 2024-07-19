use bevy::{prelude::*, utils::HashMap};

use crate::{basic_enemy, basic_enemy_move_patterns::EnemyMovePattern, beam::{Beam, ShootType}, level::Wave};

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

#[derive(Component, Clone, Debug)]
pub struct EnemyCore {
    pub x_direction: f32,
    pub y_direction: f32,
    pub health: f32,
    pub move_pattern: EnemyMovePattern,
    pub state: EnemyState,
    pub shoot: bool,
    pub shoot_type: ShootType,
    pub shoot_timer: ShootTimer,
}
impl EnemyCore {
    pub fn builder() -> EnemyCoreBuilder {
        EnemyCoreBuilder::default()
    }
}
#[derive(Default)]
pub struct EnemyCoreBuilder {
    pub x_direction: f32,
    pub y_direction: f32,
    pub health: f32,
    move_pattern: EnemyMovePattern,
    state: EnemyState,
    shoot: bool,
    shoot_type: ShootType,
    shoot_timer: ShootTimer,
}
impl EnemyCoreBuilder {
    pub fn default() -> Self {
        EnemyCoreBuilder {
            x_direction: 0.,
            y_direction: 0.,
            health: 100.,
            move_pattern: EnemyMovePattern::Basic,
            state: EnemyState::Active,
            shoot: true,
            shoot_type: ShootType::Basic,
            shoot_timer: ShootTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
        }
    }

    pub fn direction(mut self, x: f32, y: f32) -> Self {
        self.x_direction = x;
        self.y_direction = y;
        self
    }

    pub fn health(mut self, health: f32) -> Self {
        self.health = health;
        self
    }

    pub fn state(mut self, state: EnemyState) -> Self {
        self.state = state;
        self
    }

    pub fn shoot(mut self, shoot: bool) -> Self {
        self.shoot = shoot;
        self
    }

    pub fn shoot_type(mut self, shoot_type: ShootType) -> Self {
        self.shoot_type = shoot_type;
        self
    }

    pub fn shoot_timer(mut self, shoot_timer: ShootTimer) -> Self {
        self.shoot_timer = shoot_timer;
        self
    }

    pub fn move_pattern(mut self, move_pattern: EnemyMovePattern) -> Self {
        self.move_pattern = move_pattern;
        self
    }

    pub fn build(self) -> EnemyCore {
        EnemyCore {
            x_direction: self.x_direction,
            y_direction: self.y_direction,
            health: self.health,
            move_pattern: self.move_pattern,
            state: self.state,
            shoot: self.shoot,
            shoot_type: self.shoot_type,
            shoot_timer: self.shoot_timer,
        }
    }
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
            shoot_type: ShootType::Basic,
            shoot_timer: ShootTimer(Timer::from_seconds(0.3, TimerMode::Repeating))
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
        EnemyCoreBundle, 
        EnemyType,
        Transform,
        Wave,
    )
);

#[derive(Clone, Default, Debug, Resource)]
pub struct ShootTimer(pub Timer);

#[derive(Clone, Debug, Default)]
pub enum EnemyState {
    #[default]
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
    enemy_handles: ResMut<EnemyHandles>,
) {
    if !events.is_empty() {
        for event in events.read() {
            let (enemy_core_bundle, enemy_type, transform, wave) = &event.0;
            let texture = get_enemy_texture(&enemy_type, enemy_handles.as_ref());
            let enemy_core = enemy_core_bundle.enemy_core.clone();

            commands.spawn((
                EnemyCoreBundle {
                    enemy_core,
                },
                SpriteBundle {
                    texture,
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