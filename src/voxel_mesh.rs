use crate::{custom_mesh, voxel, Aabb, Octree, Point};
use gdnative::prelude::*;
use legion::*;
use voxel::tile_data::TileData;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct VoxelMesh {
    world: World,
    resources: Resources,
    schedule: Schedule,
}

#[methods]
impl VoxelMesh {
    fn new(owner: &Node) -> Self {
        let owner = unsafe { owner.assume_shared() };

        Self {
            world: World::default(),
            resources: Resources::default(),
            schedule: Schedule::builder()
                .add_thread_local(custom_mesh::create_tag_system(owner))
                .flush()
                .add_system(voxel::mesh::create_default_material_components_system(
                    "res://materials/ground.material",
                ))
                .flush()
                .add_thread_local_fn(voxel::mesh::create_drawing_system())
                .add_thread_local(custom_mesh::create_draw_system())
                .build(),
        }
    }

    #[export]
    fn _ready(&mut self, _owner: &Node) {
        self.resources
            .insert(voxel::Map::new(Point::new(10, 2, 10)));
    }

    #[export]
    fn _process(&mut self, _owner: &Node, _delta: f64) {
        self.schedule.execute(&mut self.world, &mut self.resources);
    }

    #[export]
    fn insert_point(&mut self, _owner: &Node, tile: u32, x: i32, y: i32, z: i32) {
        godot_print!("inserting {}, {}, {}", x, y, z);
        if let Some(map) = self.resources.get::<voxel::Map>() {
            let point = Point::new(x, y, z);
            let mut octree =
                Octree::new(Aabb::new(point, Point::new(1, 1, 1)), octree::DEFAULT_MAX);
            octree.insert(TileData::new(point, tile)).ok();
            map.change(&mut self.world, octree);
        }
    }
}
