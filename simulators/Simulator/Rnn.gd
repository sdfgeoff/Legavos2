class_name Rnn

extends Node


var states: PackedFloat32Array = []

var weights: Array[PackedFloat32Array] = []
var offsets: PackedFloat32Array = []

var num_input_neurons: int = 0


# Called when the node enters the scene tree for the first time.
func _init(size: int, input_neuron_count: int):
	offsets.resize(size)
	offsets.fill(0.0)
	
	states.resize(size)
	states.fill(0.0)
	
	weights = []
	for _i in range(size):
		var new_arr = PackedFloat32Array()
		new_arr.resize(size)
		new_arr.fill(0.0)
		weights.append(new_arr)
		
	num_input_neurons = input_neuron_count


func set_neuron_value(layer: int, neuron_id: int, value: float):
	states.set(layer + neuron_id, value)
	
func get_neuron_value(layer: int, neuron_id: int) -> float:
	return states[layer + neuron_id]

func step():
	var prev_state = states.duplicate();

	var weight_array = null
	var input_gain = 0
	var sum = 0
	
	for i in range(len(weights)):
		weight_array = weights[i]
		input_gain = -offsets[i];
		input_gain += dot_arrays(prev_state, weight_array)
		input_gain = relu_activation_function(input_gain)
		states[i] = input_gain

func dump_weights():
	return [weights, offsets]
	
func load_weights(new_weights):
	weights = new_weights[0]
	offsets = new_weights[1]
	

func get_num_layers():
	return num_input_neurons + 1

func dot_arrays(a: PackedFloat32Array, b: PackedFloat32Array) -> float:
	var sum = 0
	for i in range(a.size()):
		sum += a[i] * b[i]
	return sum

func relu_activation_function(val: float) -> float:
	return clamp(val, 0.0, 1000.0);


func mutate(mutate_scale: float):
	var rng = RandomNumberGenerator.new()
	rng.randomize()
	
	for weight_array_id in range(len(weights)):
		for weight_id in range(len(weights[weight_array_id])):
			weights[weight_array_id].set(
				weight_id, 
				weights[weight_array_id][weight_id] + rng.randfn(0.0, mutate_scale)
			)
	print(weights[0])
	
	for offset_id in range(len(offsets)):
		offsets.set(offset_id, offsets[offset_id] + rng.randfn(0.0, mutate_scale))

