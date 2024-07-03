use bevy::prelude::*;

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
       app
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

#[derive(Event, Default)]
pub struct WaveCompletedEvent;

fn wave_completed_event(
    mut commands: Commands,
    mut wave_completed_events: EventReader<WaveCompletedEvent>,
    asset_server: Res<AssetServer>,
) {
    if !wave_completed_events.is_empty() {
        println!("DEBUG");
            
    }
}
