extends Node3D


func _on_robot_built_state_vector(data: Array[float]):
	$Network.send_state_packet(data)


func _on_network_new_packet(data):

	if data['type'] == NetworkAdapter.MESSAGE_ACTION_CHAR:
		$RobotBuilt._on_action_vector(data['data'])
