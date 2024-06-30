use bevy::{math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume}, prelude::*};

use crate::{player::{Player, USER_SPEED}, GameState};

pub struct CanonPlugin;

impl Plugin for CanonPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems((OnEnter((GameState::Playing))), setup)
        .add_systems(
            Update,
             (move_canon)
        )
        ;
    }
}

/**
 * Multiple canons could be present, and should follow the user, or its parent canon.
 */
#[derive(Component, Debug)]
struct Canon {
    level: u8, // The nth order of the canon.
    firing: bool
}
impl Canon {
    fn new(level: u8) -> Self {
        Canon {
            level: level,
            firing: false
        }
    }
}
const CANON_HEIGHT: f32 = 27.;
struct AnimationIndices {
    first: usize,
    last: usize,
}

fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    mut player_query: Query<(&mut Transform), With<Player>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,

) {
    let player_transform = player_query.single_mut();
    let texture = asset_server.load("cannon.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(21., CANON_HEIGHT), 4, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 0, last: 3 };

    commands.spawn((
        SpriteSheetBundle {
            texture: texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
            transform: Transform::from_xyz(player_transform.translation.x, player_transform.translation.y, player_transform.translation.z),
            ..default()
        },
        Canon::new(0),
    ));
}

fn move_canon(
    mut player_query: Query<(&Transform), With<Player>>,
    time:Res<Time>,
    mut canons: Query<(&mut Transform, &Canon), (With<Canon>, Without<Player>)>,
) {
    let player_transform = player_query.single_mut();
    for (mut canon_transform, canon) in canons.iter_mut() {
        let canon_circle = BoundingCircle::new(
            canon_transform.translation.truncate(),
            CANON_HEIGHT
        );
        let player_bb = Aabb2d::new(
            player_transform.translation.truncate(),
            player_transform.scale.truncate() / 2.
        );
        
        
        
        if !canon_circle.intersects(&player_bb) { 
            println!("{:?}", canon_circle);
            println!("{:?}", player_bb);
            println!("move");
            //move
            // let dir = Vec3::new(player_transform.translation.x - canon_transform.translation.x, player_transform.translation.y - canon_transform.translation.y, 0.0).normalize();
            let dir = (player_transform.translation - canon_transform.translation);
            canon_transform.translation += dir * time.delta_seconds() * 2.;
        }
    }
}

fn fire_canon() {

}