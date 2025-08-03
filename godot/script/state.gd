extends Node

enum Sanity {
	Normal,
	Tired,
	Hell
}

var credits: int = 0
var sanity: Sanity = Sanity.Normal
var company = -1
var fired_seen = []

func get_credits() -> int:
	return credits

func get_sanity() -> Sanity:
	return sanity

func next_sanity():
	if sanity == Sanity.Normal:
		sanity = Sanity.Tired
	elif sanity == Sanity.Tired:
		sanity = Sanity.Hell

func get_company() -> int:
	company += 1
	return company

func update_credits(amount: int) -> void:
	credits += amount

func update_fired_seen(fired: int) -> void:
	if not fired in fired_seen:
		fired_seen.append(fired)