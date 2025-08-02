extends Node

var path_to_name: String
var sanity_level: Sanity.SanityLevel
var shift_number: int # please never be negative

static var mark = Employee.new("res://assets/audio/employee_names/Employee1.ogg", Sanity.SanityLevel.NORMAL)
static var john = Employee.new("res://assets/audio/employee_names/Employee2.ogg", Sanity.SanityLevel.TIRED)
static var judas = Employee.new("res://assets/audio/employee_names/Employee3.ogg", Sanity.SanityLevel.TIRED)
static var panopticon = Employee.new("res://assets/audio/employee_names/Employee4.ogg", Sanity.SanityLevel.INSANE)
static var hell = Employee.new("res://assets/audio/employee_names/Employee5.ogg", Sanity.SanityLevel.INSANE)

func _init(path: String, sanity: Sanity.SanityLevel):
	path_to_name = path
	sanity_level = sanity
	shift_number = 0

enum Employees {
	MARK,
	JOHN,
	JUDAS,
	PANOPTICON,
	HELL
}

# Because I wish I was writing Rust but the Rust API for Godot sucks so maybe
# I don't wish I was writing Rust but oh well
static func get_instance(employee: Employees) -> Employee:
	match employee:
		Employees.MARK:
			return mark
		Employees.JOHN:
			return john
		Employees.JUDAS:
			return judas
		Employees.PANOPTICON:
			return panopticon
		Employees.HELL:
			return hell
		_:
			print("Landlord Special")
			return null # sod em all 1989
