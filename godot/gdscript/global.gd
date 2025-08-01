extends Node

@onready var Employee = preload("res://gdscript/employee.gd")

class Employees:
	var mark = Employee.new("res://assets/audio/employee_names/Employee1.ogg", Sanity.SanityLevel.NORMAL)
	var john = Employee.new("res://assets/audio/employee_names/Employee2.ogg", Sanity.SanityLevel.TIRED)
	var judas = Employee.new("res://assets/audio/employee_names/Employee3.ogg", Sanity.SanityLevel.TIRED)
	var panopticon = Employee.new("res://assets/audio/employee_names/Employee4.ogg", Sanity.SanityLevel.INSANE)
	var hell = Employee.new("res://assets/audio/employee_names/Employee5.ogg", Sanity.SanityLevel.INSANE)
