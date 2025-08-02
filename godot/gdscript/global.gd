extends Node

@onready var Employee = preload("res://gdscript/employee.gd")

# because sod using actual pointers i want to be silly
@onready var current_employee = Employee.get_instance(Employee.Employees.MARK)
