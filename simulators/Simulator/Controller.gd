extends Node3D

@export var ServoJointPaths: Array[NodePath]
@export var BodyPath: NodePath

var servoJoints: Array[HingeJoint3D] = []
var body: RigidBody3D = null


var score := 0

var brain = null
# Called when the node enters the scene tree for the first time.
func _ready():
	for joint_path in ServoJointPaths:
		var joint = get_node(joint_path)
		servoJoints.append(joint)
	
	body = get_node(BodyPath)
	
	set_target_positions([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])

	brain = Network.new()
	brain.add_layer(32) # 12 for servos, 6 for orientation information, 4 for time signals
	brain.add_layer(32)
	#brain.add_layer(32)
	#brain.add_layer(32)
	brain.add_layer(32)
	brain.connect_layer(len(brain.layers) - 1, 0)  # Feedback!
	
	
	score = 0


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

func _process(delta):
	# Feed data into the network
	var positions = get_current_positions()
	var i = 0
	
	for servo in servoJoints:
		brain.layers[0][i].current_value = positions[i]
		i += 1
	
	var v_axis = get_body_vertical_axis()
	brain.layers[0][i].current_value = v_axis.x
	i+=1
	brain.layers[0][i].current_value = v_axis.y
	i+=1
	brain.layers[0][i].current_value = v_axis.z
	i+=1
	
	var f_axis = get_body_forwards_axis()
	brain.layers[0][i].current_value = f_axis.x
	i+=1
	brain.layers[0][i].current_value = f_axis.y
	i+=1
	brain.layers[0][i].current_value = f_axis.z
	i+=1
	
	# Timing
	"""
	brain.layers[0][i].current_value = sin(Time.get_ticks_msec() / 1000)
	i+=1
	brain.layers[0][i].current_value = cos(Time.get_ticks_msec() / 1000 * 2.5)
	i+=1
	brain.layers[0][i].current_value = sin(Time.get_ticks_msec() / 1000 * 1.2) * 100.0
	i+=1
	brain.layers[0][i].current_value = cos(Time.get_ticks_msec() / 1000 * 5.6) * 100.0
	i+=1
	"""
	
	
	brain.step()
	
	# Grab data out of the network
	var target_positions: Array[float] = []
	var out_layer = brain.layers[len(brain.layers) - 1]
	for servo_id in range(len(servoJoints)):
		target_positions.append(out_layer[servo_id].current_value - 0.5)  # Neuron values don't go negative
	
	set_target_positions(target_positions)
	
	
	# Accumulate a score for evaluation purposes
	var body_is_upright = get_body_vertical_axis().dot(Vector3(0,1,0)) * 0.05
	score += body_is_upright * delta
	
	var body_is_facing_motion = get_body_forwards_axis().dot(Vector3(1,0,0)) * 0.05
	score += body_is_facing_motion * delta
	
	var body_off_ground = -abs(body.global_position.y - 5.0) * 0.005
	score += body_off_ground * delta
	
	var velocity = body.linear_velocity.dot(Vector3(1,0,0))
	score += velocity * delta
	
	


func get_all_children(start):
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
			
			

class Neuron:
	var current_value = 0.0
	var connections: Array[Neuron] = []
	var weights: Array[float] = []
	var feedback_weight: float = 0.0
	var offset: float = 0.0
	
	var rng = RandomNumberGenerator.new()
	
	func _init():
		rng.randomize()
		current_value = 0.0

	
	func connect_neuron(other: Neuron, weight: float):
		connections.append(other)
		weights.append(weight)

	func step():
		var new_val = current_value * feedback_weight
		for connection_id in range(len(connections)):
			new_val += connections[connection_id].current_value * weights[connection_id]
		current_value = clamp(new_val + offset, 0.0, 1000.0)
		
	func mutate(scale: float):
		feedback_weight += rng.randfn(0.0, scale)
		offset += rng.randfn(0.0, scale)
		for i in range(len(weights)):
			weights[i] += rng.randfn(0.0, scale)
	

class Network:
	var layers = []
	
	var rng = RandomNumberGenerator.new()
	
	func _init():
		rng.randomize()
	
	func add_layer(count: int):
		var new_layer = []
		for r in count:
			var neuron = Neuron.new()
			neuron.feedback_weight = rng.randfn(0.0, 1.0)
			neuron.offset = rng.randfn(0.0, 1.0)
			new_layer.append(neuron)
		
		layers.append(new_layer)	
		if len(layers) > 1:
			connect_layer(len(layers) - 2, len(layers) - 1)
		
	func connect_layer(prev_layer_id: int, new_layer_id: int):
		var prev_layer = layers[prev_layer_id]
		var new_layer = layers[new_layer_id]
		for prev_neuron in prev_layer:
			for new_neuron in new_layer:
				new_neuron.connect_neuron(prev_neuron, rng.randf_range(-1.0, 1.0))
		
		
	func step():
		for layer_id in range(len(layers)):

			if layer_id >= 1:
				for neuron in layers[layer_id]:
					neuron.step()
				
	func dump_weights() -> Array[float]:
		var weights = []
		for layer in layers:
			for neuron in layer:
				weights.append(neuron.feedback_weight)
				weights.append(neuron.offset)
				
				for weight in neuron.weights:
					weights.append(weight)
		
		return weights

	func load_weights(weights: Array[float]):
		var i:int  = 0
		for layer in layers:
			for neuron in layer:
				neuron.feedback_weight = weights[i]
				i += 1
				neuron.offset = weights[i]
				i += 1
				
				for weight_id in range(len(neuron.weights)):
					neuron.weights[weight_id] = weights[i]
					i += 1
		
	func mutate(mutation_scale: float):
		for layer in layers:
			for neuron in layer:
				neuron.mutate(mutation_scale)
