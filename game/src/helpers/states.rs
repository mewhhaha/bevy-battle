use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    MainMenu,
    Overworld,
    LoadBattle,
    Battle,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum RunningState {
    Running,
    Paused,
}
