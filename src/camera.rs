use bevy::{
    app::{App, Plugin}, 
    prelude::*,
};

use crate::GameLevel;
pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<CameraShakeEvent>()
        .add_systems(OnEnter(GameLevel::SpaceOne), setup_camera)
        .add_systems(FixedUpdate, (shake_camera_event, shake_camera).chain())
        ;
    }
}
/// Used to help identify our main camera
#[derive(Component)]
pub struct MainCamera;

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

#[derive(Event)]
pub struct CameraShakeEvent;
#[derive(Component)]
pub struct CameraShake(Timer);
const CAMERA_SHAKE_TIME: f32 = 0.10;
const CAMERA_SHAKE_WIDTH: f32 = 100.;
pub fn shake_camera_event (
    mut commands: Commands,
    mut camera_query: Query<Entity, With<MainCamera>>,
    mut camera_shake_events: EventReader<CameraShakeEvent>
) {
    if !camera_shake_events.is_empty() {
        for _event in camera_shake_events.read() {
            let camera = camera_query.single_mut();
            commands.entity(camera).insert(CameraShake(Timer::from_seconds(CAMERA_SHAKE_TIME, TimerMode::Once)));
        }
    }
 }

 pub fn shake_camera (
    mut commands: Commands,
    mut camera_query: Query<(&mut Transform, &mut CameraShake, Entity), With<CameraShake>>,
    time: Res<Time>,
 ) {
    let camera_query = camera_query.get_single_mut();
    if let Ok(camera_query) = camera_query {
        let (mut transform, mut camera_shake,  entity) = camera_query;
        let t = camera_shake.0.tick(time.delta());
        let timer_elapsed_time = t.elapsed_secs();
        if t.finished() {
            commands.entity(entity).remove::<CameraShake>();
        } else {
            if transform.translation.x <= 0. {
                transform.translation.x += CAMERA_SHAKE_WIDTH * timer_elapsed_time;
            } else {
                transform.translation.x -= CAMERA_SHAKE_WIDTH * timer_elapsed_time;
            }
        }
    }
    
}