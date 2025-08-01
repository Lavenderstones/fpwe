extends Node2D

@onready var _animated_sprite = $GIF
@onready var _audio_main = $AudioMain
@onready var _employee_timer = $EmployeeNameReadoutTimer
@onready var _audio_auxillary = $AuxillaryAudio
@onready var _company_timer_1 = $CompanyTimer1
var mainAudioTrack = preload("res://assets/audio/miranda/intro/CompanyIntroTemplate.ogg")
var _mark_audio = preload("res://assets/audio/miranda/employee_names/Employee1.ogg")
var _judas_audio = preload("res://assets/audio/miranda/employee_names/Employee3.ogg")
var _insane_employee_audio = preload("res://assets/audio/miranda/employee_names/Employee5.ogg")

var _jobcorp_audio = preload("res://assets/audio/miranda/company_names/Company1.ogg")
var _shoit_audio = preload("res://assets/audio/miranda/company_names/Company3.ogg")
var _scream_audio = preload("res://assets/audio/miranda/company_names/Company5.ogg")

# offsets to start from for employee names in seconds
# [mark, judas, HELL] for now
var offsets = [0.8, 0.28, 0]
var current_employee = "Mark" # we all know i should make this an enum
var current_company = "JobCorp" # and this
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

func _on_employee_name_readout_timer_timeout() -> void:
	var offset_idx = -1
	if current_employee == "Mark":
		_audio_auxillary.stream = _mark_audio
		offset_idx = 0
	elif current_employee == "Judas":
		_audio_auxillary.stream = _judas_audio
		offset_idx = 1
	elif current_employee == "HELL":
		_audio_auxillary.stream = _insane_employee_audio
		offset_idx = 2

	_audio_auxillary.play(offsets[offset_idx])

func _on_company_timer_timeout() -> void:
	if current_company == "JobCorp":
		_audio_auxillary.stream = _jobcorp_audio
	elif current_company == "SHoIT":
		_audio_auxillary.strean = _shoit_audio
	elif current_company == "Scream":
		_audio_auxillary.stream = _scream_audio
		
	_audio_auxillary.play()
