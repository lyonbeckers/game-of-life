mod game_of_life;

use game_of_life::GameOfLife;
use gdnative::prelude::*;
use voxel_mesh::VoxelMesh;

fn init(handle: InitHandle) {
    handle.add_class::<VoxelMesh>();
    handle.add_class::<GameOfLife>();
}

godot_init!(init);
