extends Node3D
class_name BotController

@export var ServoJointPaths: Array[NodePath]
@export var BodyPath: NodePath

const TICK_DELAY_SECONDS = 0.1

var servoJoints: Array[HingeJoint3D] = []
var body: RigidBody3D = null


var start_centroid: Vector3 = Vector3(0,0,0)

var time_since_update: float = 0.0


signal stateVector(data: Array[float])


# Called when the node enters the scene tree for the first time.
func _ready():
	for joint_path in ServoJointPaths:
		var joint = get_node(joint_path)
		servoJoints.append(joint)
	
	body = get_node(BodyPath)
	
	set_target_positions([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])


func get_current_positions() -> Array[float]:
	var current_positions = []
	for servo in servoJoints:
		current_positions.append(servo.currentAngle)
	return current_positions


func set_target_positions(target_positions: Array):
	assert(len(target_positions) == len(servoJoints))
	for i in range(len(servoJoints)):
		var joint = servoJoints[i]
		joint.setTargetAngle(target_positions[i])


func get_body_vertical_axis() -> Vector3:
	return body.transform.basis.y
	
func get_body_forwards_axis() -> Vector3:
	return body.transform.basis.x
	
	
func get_state_vector() -> Array[float]:
	var positions := get_current_positions()
	var v_axis := get_body_vertical_axis()
	var f_axis := get_body_forwards_axis()
	
	var velocity := body.linear_velocity 

	var state_vector: Array[float] = [
		positions[0],
		positions[1],
		positions[2],
		positions[3],
		positions[4],
		positions[5],
		positions[6],
		positions[7],
		positions[8],
		positions[9],
		positions[10],
		positions[11],
		v_axis.x,
		v_axis.y,
		v_axis.z,
		f_axis.x,
		f_axis.y,
		f_axis.z,
		velocity.x,
		velocity.y,
		velocity.z
	]
	return state_vector


func _process(delta):
	# Feed data into the network
	time_since_update += delta
	
	if time_since_update > TICK_DELAY_SECONDS:
		time_since_update -= TICK_DELAY_SECONDS
		var state_vector = get_state_vector()
		emit_signal("stateVector", state_vector)


func _on_action_vector(actionVector: Array):
	assert(len(actionVector) == 12)
	set_target_positions(actionVector)


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
