extends Node

enum Sanity {
	Normal,
	Tired,
	Hell
}

var credits: int = 0
var sanity: Sanity = Sanity.Normal
var company = -1

func get_credits() -> int:
	return credits

func get_sanity() -> Sanity:
	return sanity

func get_company() -> int:
	company += 1
	return company

func update_credits(amount: int) -> void:
	credits += amount
