use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    MainMenu,
    Overworld,
    Fighting,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum RunningState {
    Running,
    Paused,
}
