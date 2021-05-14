mod game_of_life;

use game_of_life::GameOfLife;
use gdnative::prelude::*;
use voxel_mesh::VoxelMesh;

fn init(handle: InitHandle) {
    handle.add_class::<VoxelMesh>();
    handle.add_class::<GameOfLife>();

    tracing_subscriber::fmt::init();
}

godot_init!(init);
