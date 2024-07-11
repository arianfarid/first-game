use bevy::{app::{App, FixedUpdate, Plugin, Update}, asset::{AssetServer, Assets}, math::Vec2, prelude::{default, Bundle, Commands, Component, Deref, DerefMut, Entity, Event, EventReader, IntoSystemConfigs, Query, Res, ResMut, With}, sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasLayout}, time::{Time, Timer, TimerMode}, transform::components::Transform};

pub struct ExplosionCorePlugin;
impl Plugin for ExplosionCorePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<ExplosionEvent>()
        .add_systems(Update, spawn_explosion)
        .add_systems(FixedUpdate, (animate_explosion).chain())
        ;
    }
}

#[derive(Default, Debug, Event)]
pub struct ExplosionEvent(pub Transform);
#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);
#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}
#[derive(Component)]
pub struct Explosion;

#[derive(Bundle)]
pub struct ExplosionBundle {
    explosion: Explosion,
    animation_indices: AnimationIndices,
    animation_timer: AnimationTimer
}
impl Default for ExplosionBundle {
    fn default() -> Self {
        let animation_indices = AnimationIndices { first: 0, last: 4 };
        Self {
            animation_indices: animation_indices,
            animation_timer: AnimationTimer(Timer::from_seconds(0.12, TimerMode::Repeating)),
            explosion: Explosion,
        }
    }
}

fn spawn_explosion(
    mut commands: Commands,
    mut explosion_events: EventReader<ExplosionEvent>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,

) {
    if !explosion_events.is_empty() {
        for event in explosion_events.read() {
            let texture = asset_server.load("test_explosion.png");
            let layout = TextureAtlasLayout::from_grid(Vec2::new(24.0, 24.0), 5, 1, None, None);
            let texture_atlas_layout = texture_atlas_layouts.add(layout);
            let explosion_bundle = ExplosionBundle::default();
            let explosion_transform = event.0;
            commands.spawn(
                (SpriteSheetBundle {
                    texture,
                    atlas: TextureAtlas {
                        layout: texture_atlas_layout,
                        index: explosion_bundle.animation_indices.first,
                    },
                    transform: explosion_transform,
                    ..default()
                },
                explosion_bundle,)
            );
        }
    }
}

fn animate_explosion(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas, Entity), With<Explosion>>,
) {
    for (indices, 
        mut timer, 
        mut atlas, 
        entity) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            if atlas.index == indices.last {
                commands.entity(entity).despawn();
            }
            else {
                atlas.index += 1;
            };
        }
    }
}