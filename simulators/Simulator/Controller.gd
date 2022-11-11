extends Node3D
class_name BotController

@export var ServoJointPaths: Array[NodePath]
@export var BodyPath: NodePath

const TICK_DELAY_SECONDS = 0.1

var servoJoints: Array[HingeJoint3D] = []
var body: RigidBody3D = null


var score: float = 0.0

var servo_previous_positions: Array[float] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]
var servo_previous_velocities: Array[float] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]

var brain: NeuralNetwork = null
var start_centroid: Vector3 = Vector3(0,0,0)

var time_since_update: float = 0.0

# Called when the node enters the scene tree for the first time.
func _ready():
	for joint_path in ServoJointPaths:
		var joint = get_node(joint_path)
		servoJoints.append(joint)
	
	body = get_node(BodyPath)
	
	set_target_positions([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])

	brain = NeuralNetwork.new()
	brain.add_layer(32) # 12 for servos, 6 for orientation information, 4 for time signals
	brain.add_layer(32)
	brain.add_layer(32)
	brain.add_layer(32)
	brain.add_layer(32)
	brain.connect_layer(len(brain.layers) - 1, 0)  # Feedback!
	score = 0
	
	await get_tree().process_frame
	start_centroid = get_feet_tip_centeroid()
	
	var tmp_rand := RandomNumberGenerator.new()
	tmp_rand.randomize()


func get_current_positions() -> Array[float]:
	var current_positions = []
	for servo in servoJoints:
		current_positions.append(servo.currentAngle)
	return current_positions


func set_target_positions(target_positions: Array[float]):
	assert(len(target_positions) == len(servoJoints))
	for i in range(len(servoJoints)):
		var joint = servoJoints[i]
		joint.setTargetAngle(target_positions[i])


func get_body_vertical_axis() -> Vector3:
	return body.transform.basis.y
	
func get_body_forwards_axis() -> Vector3:
	return body.transform.basis.x
	
	
func interate_network():
	var positions = get_current_positions()
	var i = 0
	
	for servo in servoJoints:
		brain.set_neuron_value(0, i, positions[i])
		i += 1
	
	var v_axis = get_body_vertical_axis()
	brain.set_neuron_value(0, i, v_axis.x)
	i+=1
	brain.set_neuron_value(0, i, v_axis.y)
	i+=1
	brain.set_neuron_value(0, i, v_axis.z)
	i+=1
	
	var f_axis = get_body_forwards_axis()
	brain.set_neuron_value(0, i, f_axis.x)
	i+=1
	brain.set_neuron_value(0, i, f_axis.y)
	i+=1
	brain.set_neuron_value(0, i, f_axis.z)
	i+=1
	
	# Timing

	brain.set_neuron_value(0, i, sin(Time.get_ticks_msec() / 1000.0))
	i+=1
	
	brain.set_neuron_value(0, i, cos(Time.get_ticks_msec() / 1000.0 * 0.5))
	i+=1
	brain.set_neuron_value(0, i, sin(Time.get_ticks_msec() / 1000.0 * 2.0) * 100.0)
	i+=1
	brain.set_neuron_value(0, i, cos(Time.get_ticks_msec() / 1000.0 * 4.0) * 100.0)
	i+=1

	brain.step()
	
	# Grab data out of the network
	var target_positions: Array[float] = []
	var out_layer_id = brain.get_num_layers() - 1
	for servo_id in range(len(servoJoints)):
		target_positions.append(brain.get_neuron_value(out_layer_id, servo_id) - 0.5)  # Neuron values don't go negative
	
	set_target_positions(target_positions)

func _process(delta):
	# Feed data into the network
	time_since_update += delta
	
	if time_since_update > TICK_DELAY_SECONDS:
		time_since_update -= TICK_DELAY_SECONDS
		interate_network()
	
	# Accumulate a score for evaluation purposes
	var body_is_upright = get_body_vertical_axis().dot(Vector3(0,1,0)) * 0.5
	score += body_is_upright * delta
	
	var body_is_facing_motion = get_body_forwards_axis().dot(Vector3(1,0,0)) * 0.5
	score += body_is_facing_motion * delta
	
	# This seems to promote knuckle walking
	# var body_off_ground = -abs(body.global_position.y - 5.0) * 0.05
	# score += body_off_ground * delta
	
	var velocity = body.linear_velocity.dot(Vector3(1,0,0))
	score += velocity * delta
	
	
	var servo_velocities: Array[float] = []
	var accelerations: Array[float] = []
	
	var servo_score := 0.0
	var positions := get_current_positions()
	
	for si in range(len(positions)):
		var servo_velocity = servo_previous_positions[si] - positions[si]
		var prev_servo_velocity: float = servo_previous_velocities[si]
		var acceleration: float = servo_velocity - prev_servo_velocity
		accelerations.append(acceleration)
		servo_velocities.append(servo_velocity)
		servo_velocities.append(acceleration)
		
		# punish for servo acceleration
		servo_score = servo_score - abs(acceleration)
		
		# Promote servo motion
		servo_score = servo_score + abs(servo_velocity)
	score += servo_score * delta * 10.0

	servo_previous_positions = positions
	servo_previous_velocities = servo_velocities


func get_feet_tip_centeroid() -> Vector3:
	var tip_positions = Vector3(0,0,0)
	var tip_count = 0
	for child in get_all_children(self):
		if child.is_in_group("Tips"):
			tip_positions += child.global_position
			tip_count += 1
	
	return tip_positions / tip_count


func get_all_children(start: Node) -> Array[Node]:
	var all_children: Array = []
	for child in start.get_children():
		if child.get_child_count() > 0:
			all_children.append(child)
			all_children.append_array(get_all_children(child))
		else:
			all_children.append(child)
			
	return all_children

func mark_best():
	for child in get_all_children(self):
		if child is MeshInstance3D:
			var child_mesh: MeshInstance3D = child
			child_mesh.material_override = preload("res://RobotModel/PhysicsMaterial-Blue.tres")
			
			
