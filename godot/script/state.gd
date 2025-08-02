extends Node

enum Sanity {
	Normal,
	Tired,
	Hell
}

var credits: int = 0
var sanity: Sanity = Sanity.Normal

func get_credits() -> int:
	return credits

func get_sanity() -> Sanity:
	return sanity
