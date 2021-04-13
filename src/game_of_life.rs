use crate::{voxel::tile_data::TileData, voxel_mesh::VoxelMesh, Point};
use gdnative::prelude::*;
use octree::PointData;
use rand::Rng;
use rayon::prelude::*;
use std::collections::HashMap;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct GameOfLife {
    #[property(path = "map/width", default = 50)]
    width: i32,
    #[property(path = "map/height", default = 50)]
    height: i32,
    tile_nutrients: HashMap<Point, i32>,
}

#[methods]
impl GameOfLife {
    fn new(_owner: &Node) -> Self {
        Self {
            width: 50,
            height: 50,
            tile_nutrients: Default::default(),
        }
    }

    #[export]
    fn generate_life(&mut self, _owner: &Node, voxel_mesh: Variant) {
        let (nutrient_tx, nutrient_rx) = crossbeam_channel::unbounded::<(Point, i32)>();
        let (tile_tx, tile_rx) = crossbeam_channel::unbounded::<TileData>();

        (0..self.height * self.width).into_par_iter().for_each_with(
            (tile_tx, nutrient_tx),
            |(tile_tx, nutrient_tx), i| {
                let x = i % self.width;
                let z = i / self.width;

                let point = Point::new(x, 0, z);
                let nutrients = rand::thread_rng().gen_range(0..10);
                nutrient_tx.send((point, nutrients)).ok();
                tile_tx.send(TileData::new(point, nutrients as u32)).ok();
            },
        );

        self.tile_nutrients = nutrient_rx.into_iter().collect::<HashMap<Point, i32>>();
        let mut tiles = tile_rx.into_iter().collect::<Vec<TileData>>();

        let ground = tiles
            .iter()
            .map(|tile| {
                let point = tile.get_point();
                TileData::new(Point::new(point.x, -1, point.z), 0)
            })
            .collect::<Vec<TileData>>();

        tiles.extend(&ground);

        voxel_mesh
            .try_to_object::<Node>()
            .and_then(|node| unsafe { node.assume_safe() }.cast_instance::<VoxelMesh>())
            .and_then(|voxel_mesh| {
                voxel_mesh
                    .map_mut(|voxel_mesh, _| voxel_mesh.insert_points_internal(tiles))
                    .ok()
            });
    }

    #[export]
    fn remove_point(&mut self, _owner: &Node, voxel_mesh: Variant, x: i32, y: i32, z: i32) {
        let point = Point::new(x, y, z);

        self.tile_nutrients.remove(&point);
        voxel_mesh
            .try_to_object::<Node>()
            .and_then(|node| unsafe { node.assume_safe() }.cast_instance::<VoxelMesh>())
            .and_then(|voxel_mesh| {
                voxel_mesh
                    .map_mut(|voxel_mesh, _| voxel_mesh.remove_point_internal(point))
                    .ok()
            });
    }

    #[export]
    fn update_simulation(&mut self, _owner: &Node, voxel_mesh: Variant) {
        let tile_nutrients = self.tile_nutrients.clone();

        let (td_tx, td_rx) = crossbeam_channel::unbounded::<TileData>();

        self.tile_nutrients
            .par_iter_mut()
            .for_each_with(td_tx, |td_tx, (point, tile)| {
                let neighbors = [
                    point + Point::x(),
                    point + Point::z(),
                    point - Point::x(),
                    point - Point::z(),
                    point + Point::new(1, 0, 1),
                    point + Point::new(-1, 0, 1),
                    point + Point::new(-1, 0, -1),
                    point + Point::new(1, 0, -1),
                ];

                let mut num_neighbors = 0;
                let mut num_nutritious_neighbors = 0;

                neighbors.iter().for_each(|neighbor| {
                    if let Some(nutrients) = tile_nutrients.get(neighbor) {
                        if *nutrients > 4 {
                            if *nutrients > 8 {
                                num_nutritious_neighbors += 1;
                            }
                            num_neighbors += 1;
                        }
                    }
                });

                let mut new_nutrients = *tile;
                if *tile < 3 && num_nutritious_neighbors > 0 {
                    new_nutrients += 1;
                } else {
                    if num_neighbors <= 1 || num_neighbors >= 4 {
                        new_nutrients -= 1;
                    } else if num_neighbors == 3 {
                        new_nutrients += 1;
                    }
                }

                new_nutrients = new_nutrients.max(0);
                new_nutrients = new_nutrients.min(10);

                *tile = new_nutrients;
                td_tx.send(TileData::new(*point, *tile as u32)).ok();
            });

        let tiles = td_rx.into_iter().collect::<Vec<TileData>>();

        voxel_mesh
            .try_to_object::<Node>()
            .and_then(|node| unsafe { node.assume_safe() }.cast_instance::<VoxelMesh>())
            .and_then(|voxel_mesh| {
                voxel_mesh
                    .map_mut(|voxel_mesh, _| voxel_mesh.insert_points_internal(tiles))
                    .ok()
            });
    }
}
