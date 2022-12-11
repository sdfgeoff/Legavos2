
extends Node
class_name NetworkAdapter


const docs = """
All packets are:

TYPE: u8
DATA_LEN: u8
RESERVED: u8
RESERVED: u8
DATA: Array[f32]

DATA_LEN is the number of f32's in the data array
"""

const MESSAGE_PING_CHAR: int = 0;
const MESSAGE_PONG_CHAR: int = 1;
const MESSAGE_STATE_CHAR: int = 2;
const MESSAGE_ACTION_CHAR: int = 3;

signal new_packet(data)


var server := UDPServer.new()
var peer: PacketPeerUDP = null

func _ready():
	server.listen(42424)
	
	print("Testing")
	var test_data: Array[float] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0]
	var testPacket = _encode_state_packet(test_data)
	var decoded = _decode_packet(testPacket)
	assert(decoded['type'] == MESSAGE_STATE_CHAR)
	print(testPacket)
	print(decoded['data'])
	assert(decoded['data'] == test_data)


func _process(delta):
	server.poll() # Important!
	if server.is_connection_available():
		peer = server.take_connection()
		print("Accepted peer: %s:%s" % [peer.get_packet_ip(), peer.get_packet_port()])


	if peer != null and peer.get_available_packet_count() > 0:
		emit_signal("new_packet", _decode_packet(peer.get_packet()))
	


func send_state_packet(data: Array[float]):
	if peer:
		peer.put_packet(_encode_state_packet(data))

func _encode_state_packet(state: Array[float]) -> PackedByteArray:
	var values := PackedFloat32Array()
	values.resize(len(state))
	for i in range(len(state)):
		values.set(i, state[i])
		
	var packed := PackedByteArray()
	packed.resize(4)
	packed.encode_u8(0, MESSAGE_STATE_CHAR)
	packed.encode_u8(1, len(state))
	packed.append_array(values.to_byte_array())
	return packed


func _decode_packet(data: PackedByteArray):
	var type = data.decode_u8(0)
	var dataLen = data.decode_u8(1)
	assert(len(data) == dataLen * 4 + 4)
	var d = data.slice(4, 4+dataLen*4)
	var floatData = d.to_float32_array()

	var unpacked = []
	for i in range(dataLen):
		unpacked.append(floatData[i])
	return {
		'type': type,
		'data': unpacked
	}
	
	
