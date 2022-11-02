extends HingeJoint3D


var currentAngle: float = 0.0
@export var targetAngle: float = 0.0

@export var MAX_VELOCITY = 1.5
@export var MAX_INPULSE = 400.0

# Called when the node enters the scene tree for the first time.
func _ready():
	set_param(PARAM_MOTOR_MAX_IMPULSE, MAX_INPULSE)
	set("motor/enable", true)
	
	# Hack to set physics bounds of useful objects
	for node in get_node(node_a).get_children():
		if node is CollisionShape3D:
			var node_shape: CollisionShape3D = node
			node_shape.shape.margin = 4.0


func _physics_process(delta):
	currentAngle = self.get_param(self.PARAM_CURRENT_ANGULAR_DISPLACEMENT)
	var error = currentAngle - targetAngle
	var targetVelocity = error * 10
	var velocity = -clamp(targetVelocity, -MAX_VELOCITY, MAX_VELOCITY)
	set_param(PARAM_MOTOR_TARGET_VELOCITY, velocity)


func setTargetAngle(angle: float):
	targetAngle = clamp(angle, get("angular_limit/lower"), get("angular_limit/upper"))
