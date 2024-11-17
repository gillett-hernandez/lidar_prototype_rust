use bevy::prelude::*;

pub struct OctTree {}


#[derive(Resource)]
pub struct Space {
    pub accelerator: OctTree,
}


