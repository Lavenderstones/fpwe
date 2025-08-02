extends Node

@export var path_to_name: String
@export var shift_ending: ShiftEnding

# these are fun to write :3
static var _fire_messages = [
	"You were fired for bad odour.",
	"You were fired for sitting in a funny position.",
	"You're fired. We know you did it.\nWhat's 'it'? No thanks, I've just had lunch.",
	"You were fired for your Louis Armstrong impression. It's getting old.",
	"You were struck by the banhammer! I, your manager, am become meme.",
	"Fired.\nI asked for a sandwich\nYou said 'You want beef?'\nGet out my office.\nWasteman.",
	"We've decided to move on with another applicant",
	"You are done. Fired. Do not show your face at the office again.",
	"This is your shot at freedom, start running and dont turn ba-\nSorry, we had to fire who wrote these messages for treason.",
	"You can run but you cant hide. y̵̨̩̱̓ö̷́̈͠͝u̷͚̓̊͘ ̶̯̒̔͜c̵̦͕̘̄a̵̟̯̅̓n̵̫͂̓̔ ̸̆͒͊͝n̶̆́̑͝e̴̛͌̅ver h̴̑̽̂͋ỉ̴̒̚͝ḑ̶̃ę̸͉͒͛",
	"Fired for rebellious home lifestyle.",
	"Fired for contraband. Didn't we say tell you to keep away from books?",
	"Your execution has been scheduled for 3 hours. Thank you for working at our company, valued employee!"
]

enum ShiftEnding {
	FIRE_MESSAGE,
	BREAK_THE_CYCLE
}

enum Companies {
	JOBCORP_FAMILY,
	WINSTON_SMITH_AND_CO,
	STATE_HEADQUARTERS_OF_INTERNET_TRUTH,
	LEGION,
	SCREAM
}

static func	get_random_fire_message():
	# rand range is inclusive of the end for some reason. such an unserious language
	return _fire_messages[randi_range(0, len(_fire_messages) - 1)]
