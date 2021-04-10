use crate::{custom_mesh, voxel, Aabb, Octree, Point};
use gdnative::prelude::*;
use legion::*;
use octree::PointData;
use voxel::tile_data::TileData;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct VoxelMesh {
    world: World,
    resources: Resources,
    schedule: Schedule,
    #[property(path = "chunk_dimensions/width", default = 10)]
    width: i32,
    #[property(path = "chunk_dimensions/height", default = 10)]
    height: i32,
    #[property(path = "chunk_dimensions/depth", default = 10)]
    depth: i32,
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
                .add_thread_local(voxel::mesh::create_visibility_notifier_system(owner))
                .flush()
                .add_system(voxel::mesh::create_default_material_components_system(
                    "res://materials/ground.material",
                ))
                .flush()
                .add_thread_local_fn(voxel::mesh::create_drawing_system())
                .add_thread_local(custom_mesh::create_draw_system())
                .build(),
            width: 10,
            height: 10,
            depth: 10,
        }
    }

    #[export]
    fn _ready(&mut self, _owner: &Node) {
        self.resources.insert(voxel::Map::new(Point::new(
            self.width,
            self.height,
            self.depth,
        )));
    }

    #[export]
    fn _process(&mut self, _owner: &Node, _delta: f64) {
        self.schedule.execute(&mut self.world, &mut self.resources);
    }

    #[export]
    fn insert_point(&mut self, _owner: &Node, tile: u32, x: i32, y: i32, z: i32) {
        if let Some(map) = self.resources.get::<voxel::Map>() {
            let point = Point::new(x, y, z);
            let mut octree =
                Octree::new(Aabb::new(point, Point::new(1, 1, 1)), octree::DEFAULT_MAX);
            octree.insert(TileData::new(point, tile)).ok();
            map.change(&mut self.world, octree);
        }
    }

    #[export]
    fn remove_point(&mut self, _owner: &Node, x: i32, y: i32, z: i32) {
        if let Some(map) = self.resources.get::<voxel::Map>() {
            let point = Point::new(x, y, z);
            let octree = Octree::new(Aabb::new(point, Point::new(1, 1, 1)), octree::DEFAULT_MAX);
            map.change(&mut self.world, octree);
        }
    }

    #[export]
    fn insert_points(&mut self, _owner: &Node, points: VariantArray) {
        if let Some(map) = self.resources.get::<voxel::Map>() {
            let tiles = points
                .into_iter()
                .filter_map(|v| {
                    Dictionary::from_variant(&v)
                        .and_then(|v| {
                            let pt = v.get("point");
                            let tile = v.get("tile");
                            Vector3::from_variant(&pt).and_then(|pt| {
                                u32::from_variant(&tile).and_then(|tile| {
                                    Ok(TileData::new(
                                        Point::new(pt.x as i32, pt.y as i32, pt.z as i32),
                                        tile,
                                    ))
                                })
                            })
                        })
                        .ok()
                })
                .collect::<Vec<TileData>>();

            let mut min = Point::new(i32::MAX, i32::MAX, i32::MAX);
            let mut max = Point::new(i32::MIN, i32::MIN, i32::MIN);

            tiles.iter().for_each(|td| {
                let pt = td.get_point();
                min.x = min.x.min(pt.x);
                min.y = min.y.min(pt.y);
                min.z = min.z.min(pt.z);

                max.x = max.x.max(pt.x);
                max.y = max.y.max(pt.y);
                max.z = max.z.max(pt.z);
            });

            let mut octree = Octree::new(Aabb::from_extents(min, max), octree::DEFAULT_MAX);
            octree.insert_elements(tiles).ok();

            map.change(&mut self.world, octree);
        }
    }
}
