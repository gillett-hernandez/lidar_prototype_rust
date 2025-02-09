use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};

const SHADER_ASSET_PATH: &'static str = "shaders/custom_material.wgsl";

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]

pub struct CustomMaterial {
    #[uniform(0)]
    color: LinearRgba,
    #[uniform(1)]
    player_position: Vec3,
}

impl Default for CustomMaterial {
    fn default() -> Self {
        Self {
            color: LinearRgba {
                red: 1.0,
                green: 1.0,
                blue: 1.0,
                alpha: 1.0,
            },
            player_position: Vec3::ZERO,
        }
    }
}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Add
    }
}
