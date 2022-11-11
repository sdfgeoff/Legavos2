class_name NeuralNetwork


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


func set_neuron_value(layer_id: int, neuron_id: int, value: float):
	layers[layer_id][neuron_id].current_value = value
	
func get_neuron_value(layer_id: int, neuron_id: int):
	return layers[layer_id][neuron_id].current_value

func get_num_layers() -> int:
	return len(layers)
