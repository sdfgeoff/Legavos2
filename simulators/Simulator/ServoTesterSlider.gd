extends HSlider

@export var targetServo : NodePath = ""

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	get_node(targetServo).setTargetAngle(value * 3.14159)
