class_name Neuron

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
