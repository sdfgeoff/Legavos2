extends HingeJoint3D

var currentVelocity: float = 0.0
var currentAngle: float = 0.0
@export var targetAngle: float = 0.0


var MAX_ACCELERATION: float = 5.0
@export var MAX_VELOCITY = 1.5
@export var MAX_INPULSE = 400.0

func _init():
	currentVelocity = 0.0

# Called when the node enters the scene tree for the first time.
func _ready():
	set_param(PARAM_MOTOR_MAX_IMPULSE, MAX_INPULSE)
	set("motor/enable", true)
	
	
	# Hack to set physics bounds of useful objects
	for node in get_node(node_a).get_children():
		if node is CollisionShape3D:
			var node_shape: CollisionShape3D = node
			node_shape.shape.margin = 4.0


func compute_next_velocity(delta: float):
	var newAngle = self.get_param(self.PARAM_CURRENT_ANGULAR_DISPLACEMENT)
	if abs(newAngle) > 10:
		return 0.0 
		
	var newVelocity = (newAngle - currentAngle) / delta
	var newAcceleration = (newVelocity - currentVelocity) / delta
	
	var positionError = newAngle - targetAngle
	var targetVelocity = -positionError * 5.0
	targetVelocity = clamp(targetVelocity, -MAX_VELOCITY, MAX_VELOCITY)


	#var velocityError = newVelocity - targetVelocity
	#var targetAcceleration = -velocityError * 10.0
	#targetAcceleration = clamp(targetAcceleration, -MAX_ACCELERATION, MAX_ACCELERATION)

	#var outputVelocity = currentVelocity + targetAcceleration * delta
	
	
	#var currentAcceleration = newAcceleration
	#currentVelocity = outputVelocity  # Hack because Godot's joints at maintaining velocity
	currentAngle = newAngle
	
	
	return targetVelocity
	

func _process(delta):
	var outputVelocity = compute_next_velocity(delta)

	set_param(PARAM_MOTOR_TARGET_VELOCITY, outputVelocity)


func setTargetAngle(angle: float):
	targetAngle = clamp(angle, get("angular_limit/lower"), get("angular_limit/upper"))
