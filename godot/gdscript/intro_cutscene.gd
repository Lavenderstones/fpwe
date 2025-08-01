extends Node2D

@onready var _animated_sprite = $GIF
@onready var _audio_main = $AudioMain
var mainAudioTrack = preload("res://assets/audio/miranda/intro/CompanyIntroTemplate.ogg")
var _audio_played = false

# TODO: figure out playing the correct employee name and company name in the
# right places at the start of each new character after a reset. Needs a global
# var that stores the audio for the names maybe?

func _process(delta: float) -> void:
	_animated_sprite.play()
	if !_audio_played:
		_audio_played = true
		_audio_main.stream = mainAudioTrack
		_audio_main.play()

func _on_audio_main_finished() -> void:
	print("audio finish")
	get_tree().change_scene_to_file("res://scenes/shift.tscn")
