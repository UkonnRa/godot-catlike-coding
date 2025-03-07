extends Node

# MathSurfaceController - Controls the MathSurface example scene
# Lets the user switch between mathematical functions with keyboard inputs

# Reference to the MathSurface node
@onready var math_surface = $MathSurface
@onready var title_label = $Info/Title

# Function names
var function_names = [
	"Wave",
	"Multi Wave",
	"Ripple",
	"Sphere",
	"Torus"
]

func _ready():
	# Update the title initially
	update_title()

func _input(event):
	if event is InputEventKey and event.pressed:
		var function_index = -1
		
		# Check for number keys 1-5
		if event.keycode == KEY_1:
			function_index = 0  # Wave
		elif event.keycode == KEY_2:
			function_index = 1  # Multi Wave
		elif event.keycode == KEY_3:
			function_index = 2  # Ripple
		elif event.keycode == KEY_4:
			function_index = 3  # Sphere
		elif event.keycode == KEY_5:
			function_index = 4  # Torus
		
		# If we pressed a valid key, change the function
		if function_index >= 0:
			change_function(function_index)

func change_function(index):
	# Only update if it's a different function
	if math_surface.function_index != index:
		math_surface.function_index = index
		update_title()
		print("Changed function to: ", function_names[index])

func update_title():
	# Update the UI label to show the current function
	if math_surface.function_index >= 0 and math_surface.function_index < function_names.size():
		title_label.text = "Mathematical Surface - " + function_names[math_surface.function_index] + " Function" 
