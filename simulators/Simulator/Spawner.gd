extends Node3D

@export var Robot: PackedScene = null
@export var bot_count: int = 9

var current_generation = 0

var bots = []
var timer := 0.0


var simulation_lifespan = 30.0
@export var mutation_scale = 1.0


func grid(i: int, row_count: float):
	return Vector2(i % int(row_count), floor(i / row_count))


# Called when the node enters the scene tree for the first time.
func respawn():
	var rows: float = ceil(pow(bot_count, 0.5))
	for i in range(bot_count):
		var bot: Node3D = Robot.instantiate()
		var offset = (grid(i, rows) - Vector2(rows/2, rows/2)) * 300 
		#var offset = Vector2(0,0)
		
		add_child(bot)
		bot.global_position = Vector3(offset.x,100, offset.y)
		
		bots.append(bot)


func _ready():
	respawn()


func _process(delta):
	timer += delta
		
	if timer > simulation_lifespan or Input.is_action_just_pressed("ui_accept"):
		
		var best_brain = null
		
		var highest_score = null
		for bot in bots:
			# TODO: evaluate

			var body_off_ground = bot.body.global_position.y
			var distance_travelled = bot.body.global_position.x #Vector2(bot.body.global_position.x, bot.body.global_position.z).length()
			if distance_travelled == NAN:
				distance_travelled == 0
			var score = bot.score # + distance_travelled

			if highest_score == null or score > highest_score:
				highest_score = score
				best_brain = bot.brain.dump_weights()
				
			
			bot.queue_free()
			
		bots = []
			
		respawn()

		for bot_id in range(len(bots)):
			var bot = bots[bot_id]
			bot.brain.load_weights(best_brain)
			if bot_id == 0:
				bot.mark_best()

			else:
				bot.brain.mutate(mutation_scale)
			
		timer = 0
		current_generation += 1
		
		mutation_scale = 0.2 * pow(2.16, -0.01 * current_generation)  # Simulated annealing
		
		print("Mutation Scale", mutation_scale)
