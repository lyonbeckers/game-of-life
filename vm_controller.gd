extends Node

export var width = 10
export var height = 10

var tile_nutrients = {}
var time_elapsed = 0.0
export var tick_rate = 10.0

func _ready():
	var tile_datas = []
	for x in range(0, width):
		for z in range(0, height):
			var pt = Vector3(x, 0, z)
			var nutrients = rand_range(0, 10) as int
			tile_nutrients[pt] = nutrients
			tile_datas.push_back({
				point = pt,
				tile = nutrients
			})
			tile_datas.push_back({
				point = Vector3(x, -1, z),
				tile = 0
			})
	$voxel_mesh.insert_points(tile_datas)

# Called when the node enters the scene tree for the first time.
func _process(delta):

	time_elapsed += delta
	if time_elapsed < tick_rate:
		return
	else:
		time_elapsed = 0.0
	
	var tile_datas = []
	
	for point in tile_nutrients.keys():
		var neighbors = [
			point + Vector3(1,0,0),
			point + Vector3(1,0,1),
			point + Vector3(0,0,1),
			point + Vector3(-1,0,1),
			point + Vector3(-1,0,0),
			point + Vector3(-1,0,-1),
			point + Vector3(1,0,-1)
		]
		
		var num_neighbors = 0
		var num_nutritious_neighbors = 0
		var old_nutrients = tile_nutrients[point]
		var new_nutrients = old_nutrients
		
		for neighbor in neighbors:
			var nutrients = tile_nutrients.get(neighbor)
			if nutrients != null && nutrients > 4:
				if nutrients > 8:
					num_nutritious_neighbors += 1
				num_neighbors += 1
					
		if new_nutrients < 3 && num_nutritious_neighbors > 0:
			new_nutrients += 1
		else:
			if num_neighbors <= 1 || num_neighbors >= 4:
				new_nutrients -= 1
			elif num_neighbors == 3:
				new_nutrients += 1
			
		if new_nutrients < 0:
			new_nutrients = 0
			
		if new_nutrients > 10:
			new_nutrients = 10
		
		tile_nutrients[point] = new_nutrients
		tile_datas.push_back({
			point = point,
			tile = new_nutrients
		})
		
	$voxel_mesh.insert_points(tile_datas)
	
func remove_point(point):
	tile_nutrients.erase(point)
	$voxel_mesh.remove_point(point.x as int, point.y as int, point.z as int)
