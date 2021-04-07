use gdnative::prelude::*;

mod custom_mesh;
mod node;
mod voxel;
mod voxel_mesh;

use crate::{voxel::tile_data::TileData, voxel_mesh::VoxelMesh};

pub(crate) type Octree = octree::Octree<i32, TileData>;
pub(crate) type Point = nalgebra::Vector3<i32>;
pub(crate) type Vector3D = nalgebra::Vector3<f32>;
pub(crate) type Aabb = octree::geometry::aabb::Aabb<i32>;

fn init(handle: InitHandle) {
    handle.add_class::<VoxelMesh>();
}

godot_init!(init);
