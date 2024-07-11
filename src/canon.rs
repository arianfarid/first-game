use bevy::{math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume}, prelude::*};

use crate::{beam::{Beam, BeamType}, player::{Player, WeaponType}, GameState};

pub struct CanonPlugin;

impl Plugin for CanonPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameState::Playing), setup)
        .add_systems(
            Update,
             (move_canon, fire_canon).chain().run_if(in_state(GameState::Playing))
        )
        .add_systems(FixedUpdate, (animate_canon).after(fire_canon))
        ;
    }
}

/**
 * Multiple canons could be present, and should follow the user, or its parent canon.
 */
#[derive(Component, Debug)]
struct Canon {
    level: u8, // The nth order of the canon.
    lockout_time: f32,
    needs_cooldown: bool,
    position: Position,
}
#[derive(Component, Debug)]
pub enum Position {
    Left,
    Right
}
impl Canon {
    fn new(level: u8, position: Position,) -> Self {
        Canon {
            level: level,
            lockout_time: 0.35,
            needs_cooldown: false,
            position: position,
        }
    }
}
#[derive(Component, Resource)]
pub struct ShootTimer(Timer);

const CANON_ANIMATION_SPEED: f32 = 0.03;
const CANON_DISTANCE: f32 = 20.;
const CANON_HEIGHT: f32 = 27.;

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    mut player_query: Query<(&mut Transform, &Player), With<Player>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,

) {
    if let Ok(player_query) = player_query.get_single_mut() {
        let (player_transform, player) = player_query;
        if player.left_weapon == WeaponType::PlasmaCanon {
            let texture = asset_server.load("cannon.png");
            let layout = TextureAtlasLayout::from_grid(Vec2::new(21., CANON_HEIGHT), 4, 1, None, None);
            let texture_atlas_layout = texture_atlas_layouts.add(layout);
            let animation_indices = AnimationIndices { first: 0, last: 3 };
            let canon = Canon::new(0, Position::Left); 
            let canon_lockout = canon.lockout_time;
            let mut animation_timer = AnimationTimer(Timer::from_seconds(CANON_ANIMATION_SPEED, TimerMode::Repeating));
            animation_timer.0.pause();
            commands.spawn((
                SpriteSheetBundle {
                    texture: texture,
                    atlas: TextureAtlas {
                        layout: texture_atlas_layout,
                        index: animation_indices.first,
                    },
                    transform: Transform::from_xyz(player_transform.translation.x - CANON_DISTANCE, player_transform.translation.y, player_transform.translation.z),
                    ..default()
                },
                canon,
                animation_indices,
                ShootTimer(Timer::from_seconds(canon_lockout, TimerMode::Once)),
                animation_timer,
            ));
        }
        if player.right_weapon == WeaponType::PlasmaCanon {
            let texture = asset_server.load("cannon.png");
            let layout = TextureAtlasLayout::from_grid(Vec2::new(21., CANON_HEIGHT), 4, 1, None, None);
            let texture_atlas_layout = texture_atlas_layouts.add(layout);
            let animation_indices = AnimationIndices { first: 0, last: 3 };
            let canon = Canon::new(0, Position::Right); 
            let canon_lockout = canon.lockout_time;
            let mut animation_timer = AnimationTimer(Timer::from_seconds(CANON_ANIMATION_SPEED, TimerMode::Repeating));
            animation_timer.0.pause();
            commands.spawn((
                SpriteSheetBundle {
                    texture: texture,
                    atlas: TextureAtlas {
                        layout: texture_atlas_layout,
                        index: animation_indices.first,
                    },
                    transform: Transform::from_xyz(player_transform.translation.x + CANON_DISTANCE, player_transform.translation.y, player_transform.translation.z),
                    ..default()
                },
                canon,
                animation_indices,
                ShootTimer(Timer::from_seconds(canon_lockout, TimerMode::Once)),
                animation_timer,
            ));
        }
    }
}

fn move_canon(
    mut player_query: Query<&Transform, With<Player>>,
    time:Res<Time>,
    mut canons: Query<(&mut Transform, &Canon), (With<Canon>, Without<Player>)>,
) {
    let player_transform = player_query.single_mut();
    for (mut canon_transform, canon) in canons.iter_mut() {
        let transform_x_dim = match canon.position {
            Position::Left =>  -CANON_DISTANCE,
            Position::Right => CANON_DISTANCE,
        };
        let canon_circle = BoundingCircle::new(
            canon_transform.translation.truncate(),
            CANON_HEIGHT
        );
        let mut move_target = player_transform.translation;
        move_target.x += transform_x_dim;
        let target_bb = Aabb2d::new(
            move_target.truncate(),
            player_transform.scale.truncate() / 2.
        );
        if !canon_circle.intersects(&target_bb) { 
            //move
            // let dir = Vec3::new(player_transform.translation.x - canon_transform.translation.x, player_transform.translation.y - canon_transform.translation.y, 0.0).normalize();
            let dir = move_target - canon_transform.translation;
            canon_transform.translation += dir * time.delta_seconds() * 3.;
        }
    }
}

fn fire_canon(
    mut canons: Query<(&mut Transform, &mut Canon, &mut ShootTimer, &mut AnimationTimer), (With<Canon>, Without<Player>)>,
    mouse_buttons:Res<ButtonInput<MouseButton>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut commands: Commands,
) {
    if keyboard_input.pressed(KeyCode::Space) || mouse_buttons.pressed(MouseButton::Left) {
        for (canon_transform, mut canon, mut shoot_timer, mut animation_timer) in canons.iter_mut() {
            if shoot_timer.0.tick(time.delta()).finished() {
                canon.needs_cooldown = false;
            }
            match canon.needs_cooldown {
                true => {
                }
                false => {
                    let canon_location = canon_transform.translation;
                    // let canon_angle: Quat = canon_transform.rotation;
                   
                    let mut spawn_transform = Transform::from_scale(Vec3::splat(1.0));
                    spawn_transform.translation = canon_location;
                    // spawn_transform.rotation = canon_angle;
                    let plasma_orb = Beam::new(&BeamType::PlasmaOrb, Vec2::new(0., 1.));
                    canon.needs_cooldown = true;
                    shoot_timer.0.reset();
                    animation_timer.0.unpause();
                    commands.spawn((
                        SpriteBundle {
                            transform: spawn_transform,
                            texture: asset_server.load("plasma_orb.png"),
                            ..default()
                        },
                        //todo: 2 weapons, should be enum w/ params
                        plasma_orb,
                    ));
                }
            }
        } 
    }
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_canon(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas), With<Canon>>,
) {
    for (indices, 
        mut animation_timer, 
        mut atlas
    ) in query.iter_mut() {
        animation_timer.tick(time.delta());
        if animation_timer.just_finished() {
            if atlas.index == indices.last {
                atlas.index = indices.first;
                animation_timer.0.pause();
            }
            else {
                atlas.index += 1;
            };
        }
    }
}