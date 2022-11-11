extends Node3D

@export var Robot: PackedScene = null
@export var bot_count: int = 16
@export var output_text: NodePath = ""
@export var performance_graph: NodePath = ""

var current_generation = 0

var bots = []
var timer := 0.0

var highest_score = null
var best_brain = null


var simulation_lifespan = 40.0
@export var mutation_scale = 1.0


var run_name = Time.get_date_string_from_system()+'-'+Time.get_time_string_from_system()


var performance_graph_data: Array[float] = []

func grid(i: int, row_count: float):
	return Vector2(i % int(row_count), floor(i / row_count))


# Called when the node enters the scene tree for the first time.
func respawn():
	var rows: float = ceil(pow(bot_count, 0.5))
	for i in range(bot_count):
		var bot: BotController = Robot.instantiate()
		var offset = (grid(i, rows) - Vector2(rows/2, rows/2)) * 300 
		
		add_child(bot)
		bot.global_position = Vector3(offset.x,100, offset.y)
		bot.time_since_update = bot.TICK_DELAY_SECONDS * i / bot_count 
		bots.append(bot)
		


func _ready():
	respawn()
	
	#var brain = load_brain("user://checkpoint-<null>-750.data")
	#var brain = load_brain("user://checkpoint-50-slow-shuffle.data")
	print("Starting Run: ", run_name)
	#for bot_id in range(len(bots)):
	#	var bot = bots[bot_id]
	#	bot.brain.load_weights(brain)




func calculate_score(bot: Node3D):
	var distance_travelled = (bot.start_centroid - bot.get_feet_tip_centeroid()).x
	if distance_travelled == NAN:
		distance_travelled = 0

	return bot.score + distance_travelled * 0.1


func check_active() -> bool:
	""" Checks to see if any bots are still alive/moving or if they 
	are all stationary """
	var active = false
	for bot in bots:
		if bot.body.linear_velocity.length() > 20:  # Units = mm/s
			active = true
	return active



func new_generation():
	""" Grabs the brain of the best robot, and propogates/mutates it """
	for bot in bots:
		var score = calculate_score(bot)
		if highest_score == null or score > highest_score:
			highest_score = score
			best_brain = bot.brain.dump_weights()
			
		
		bot.queue_free()
		
	bots = []
		
	respawn()

	for bot_id in range(len(bots)):
		var bot = bots[bot_id]
		bot.brain.load_weights(best_brain)
		if bot_id == len(bots) - 2:
			print(bot_id)
			bot.mark_best()
		else:
			bot.brain.mutate(mutation_scale)
		
	timer = 0
	current_generation += 1
	
	performance_graph_data.append(highest_score)
	

	mutation_scale = 0.5#  * pow(2.16, -0.01 * current_generation)  # Simulated annealing


func _process(delta):
	timer += delta

	var life = simulation_lifespan
	var active = check_active()
	if !active:
		life = 7

	if timer > life or Input.is_action_just_pressed("ui_accept"):
		new_generation()
		
		if current_generation % 10 == 0:
			var filename = "user://checkpoint-%s-%d.data" % [run_name, current_generation]
			save_brain(filename, best_brain)
	
		render_performance_graph()

	var text_output: RichTextLabel = get_node(output_text)
	text_output.text = "Generation %d\n Active: %s \n Best Score: %.1f\n Mutation Rate: %.3f\n FPS: %d" % [current_generation, active, highest_score, mutation_scale, Engine.get_frames_per_second()]
	

func render_performance_graph():
	const WIDTH = 300
	const HEIGHT = 200
	var graph: Line2D = get_node(performance_graph)
	var line = PackedVector2Array()
	line.resize(len(performance_graph_data))
	
	for i in range(len(performance_graph_data)):
		line.set(i, Vector2(
			float(i) / current_generation * WIDTH,
			HEIGHT - performance_graph_data[i] / highest_score * HEIGHT
		))

	graph.points = line


func save_brain(path: String, brain: Array[float]):
	print("Saving ", path)
	var file = FileAccess.open(path, FileAccess.WRITE)
	file.store_var(brain)
	file.flush()

func load_brain(path: String) -> Array[float]:
	print("Loading ", path)
	var file = FileAccess.open(path, FileAccess.READ)
	var content = file.get_var()
	return content
