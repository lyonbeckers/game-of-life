extends Node

export var tick_rate = 10.0;
var time_elapsed = 0.0;

var position = Vector3(0,0,0)
var move_left = false;
var move_right = false;
var move_forward = false;
var move_back = false;
var trigger_pressed = false;
var left_repeater = 0.0;
var right_repeater = 0.0;
var forward_repeater = 0.0;
var back_repeater = 0.0;

const SPEED = 0.20;

# Declare member variables here. Examples:
# var a = 2
# var b = "text"


# Called when the node enters the scene tree for the first time.
func _ready():
	$game_of_life.generate_life($voxel_mesh)


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	
	if move_back:
		back_repeater += delta;
	if move_forward:
		forward_repeater += delta;
	if move_left:
		left_repeater += delta;
	if move_right:
		right_repeater += delta;
		
	if back_repeater > SPEED:
		back_repeater = 0.0;
		position.z += 1;
	if forward_repeater > SPEED:
		forward_repeater = 0.0;
		position.z -= 1;
	if left_repeater > SPEED:
		left_repeater = 0.0;
		position.x -= 1;
	if right_repeater > SPEED:
		right_repeater = 0.0;
		position.x += 1;
		
	position.x = max(position.x, 0)
	position.z = max(position.z, 0)
	position.x = min(position.x, $game_of_life.get("map/width") - 1)
	position.z = min(position.z, $game_of_life.get("map/height") - 1)
	
	var focal_point = position + Vector3(0.5, 1.0, 0.5)
	$pickaxe.transform.origin = focal_point + Vector3(SPEED, 0.0, -SPEED)
	$camera_controller.heading = focal_point
	
	time_elapsed += delta
	if time_elapsed < tick_rate:
		return
	else:
		time_elapsed = 0.0
		
	$game_of_life.update_simulation($voxel_mesh)
	
func _input(event):
	if event.is_action_pressed("move_left"):
		move_left = true;
		position.x -= 1;
	if event.is_action_pressed("move_right"):
		move_right = true;
		position.x += 1;
	if event.is_action_pressed("move_forward"):
		move_forward = true;
		position.z -= 1;
	if event.is_action_pressed("move_back"):
		move_back = true;
		position.z += 1;
		
	if event.is_action_released("move_left"):
		move_left = false;
		left_repeater = 0.0;
	if event.is_action_released("move_right"):
		move_right = false;
		right_repeater = 0.0;
	if event.is_action_released("move_forward"):
		move_forward = false;
		forward_repeater = 0.0;
	if event.is_action_released("move_back"):
		move_back = false;
		back_repeater = 0.0;
	
	if event.is_action_pressed("action_trigger"):
		trigger_pressed = true;
		dig()
		
	if event.is_action_released("action_trigger"):
		trigger_pressed = false;
		
func dig():
	var neighbors = [
			position + Vector3(1,0,0),
			position + Vector3(-1,0,0),
			position + Vector3(0,0,1),
			position + Vector3(0,0,-1),
		]
	# var can_dig = $game_of_life_controller.tile_nutrients.has(position);
		
	# if !can_dig: return
		
	# can_dig = !$game_of_life_controller.tile_nutrients.has_all(neighbors)
					
	# if !can_dig: return
		
	$game_of_life.remove_point($voxel_mesh, position.x as int, position.y as int, position.z as int)

