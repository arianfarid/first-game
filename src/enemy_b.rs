// use bevy::prelude::*;

// pub struct EnemyBPlugin;
// impl Plugin for EnemyBPlugin {
//     fn build(&self, app: &mut App) {
//         app.
//         .add_event::<SpawnEnemyBEvent>()
//         .add_systems(Update, (spawn_enemy_b).chain())
//         ;
//     }
// }
// #[derive(Component)]
// pub struct EnemyB {
//     pub x_direction: f32,
//     pub y_direction: f32,
//     health: f32,
//     move_pattern: EnemyMovePattern,
//     state: EnemyState,
//     shoot: bool,
// }
// impl EnemyB {
//     pub fn new() -> Self {
//         EnemyB {
//         }
//     }
// }
// #[derive(Default, Debug, Event)]
// pub struct SpawnEnemyBEvent(pub Transform);
// fn spawn_enemy_b(
//     mut commands: Commands,
//     mut spawn_enemy_b_events: EventReader<SpawnEnemyBEvent>,
//     asset_server: Res<AssetServer>,
//     mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
// ) {
//     if !spawn_enemy_b_events.is_empty() {
//         for _event in spawn_enemy_b_events.read() {
//             let texture = asset_server.load("explosion_core.png");
//             let layout = TextureAtlasLayout::from_grid(Vec2::new(EXPLOSION_CORE_X, EXPLOSION_CORE_Y), EXPLOSION_CORE_COLS, EXPLOSION_CORE_ROWS, None, None);
//             let texture_atlas_layout = texture_atlas_layouts.add(layout);
//             let explosion_transform = event.0;
//             commands.spawn(
//                 (SpriteSheetBundle {
//                     texture,
//                     atlas: TextureAtlas {
//                         layout: texture_atlas_layout,
//                         index: explosion_bundle.animation_indices.first,
//                     },
//                     transform: explosion_transform,
//                     ..default()
//                 },
//                 explosion_bundle,)
//             );
//         }
//     }
// }