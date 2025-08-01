extends Node

var path_to_name: String
var sanity_level: Sanity.SanityLevel
var shift_number: int # please never be negative

func _init(path: String, sanity: Sanity.SanityLevel):
	path_to_name = path
	sanity_level = Sanity.SanityLevel.NORMAL
	shift_number = 0
