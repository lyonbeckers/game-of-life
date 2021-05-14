extends Node

export var tick_rate = 10.0;
var time_elapsed = 0.0;

# Declare member variables here. Examples:
# var a = 2
# var b = "text"


# Called when the node enters the scene tree for the first time.
func _ready():
	$game_of_life.generate_life($voxel_mesh)


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	time_elapsed += delta
	if time_elapsed < tick_rate:
		return
	else:
		time_elapsed = 0.0
		
	$game_of_life.update_simulation($voxel_mesh)
