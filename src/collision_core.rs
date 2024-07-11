use bevy::{app::{App, FixedUpdate, Plugin, Update}, prelude::*, time::Timer};

pub struct CollisionCorePlugin;
impl Plugin for CollisionCorePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<CollisionEvent>()
        .add_systems(Update, collision)
        .add_systems(FixedUpdate, blink)
        ;
    }
}

#[derive(Event)]
pub struct CollisionEvent(pub Entity);

#[derive(Component, Resource)]
pub struct Blinking(pub Timer);
pub const BLINK_DURATION: f32 = 0.35;

pub fn collision (
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
) {
    if !collision_events.is_empty() {
        for event in collision_events.read() {
            match commands.get_entity(event.0) {
                Some(_) => {
                    commands.entity(event.0).insert(Blinking(Timer::from_seconds(BLINK_DURATION, TimerMode::Once)));
                },
                None => {}
            }
        }
    }
}

pub fn blink (
    mut commands: Commands,
    mut enemy_query: Query<(&mut Blinking, &mut Sprite, Entity)>,
    time: Res<Time>,
) {
    for (mut blinking_timer, mut sprite, entity) in enemy_query.iter_mut() {
        
        let blinking_timer = blinking_timer.0.tick(time.delta());
        sprite.color = Color::rgba(255., 255., 255., 1.);
        if blinking_timer.finished() {
            sprite.color = Color::rgba(1., 1., 1., 1.);
            commands.entity(entity).remove::<Blinking>();
        }
    }
}