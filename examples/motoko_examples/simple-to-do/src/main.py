from kybra import nat, Record, query, update, void


class ToDo(Record):
    description: str
    completed: bool


todos: dict[nat, ToDo] = {}
next_id: nat = 0


@query
def get_todos() -> list[ToDo]:
    return list(todos.values())


@update
def add_todo(description: str) -> nat:
    global next_id

    id = next_id
    todos[id] = {'description': description, 'completed': False}
    next_id += 1
    return id


@update
def complete_todo(id: nat) -> void:
    todo = todos.get(id)

    if todo is not None:
        todos[id] = {'description': todo['description'], 'completed': True}


@query
def show_todos() -> str:
    output = '\n___TO-DOs___'
    for todo_entry in todos.values():
        output += f'\n{todo_entry["description"]}'
        if todo_entry['completed']:
            output += ' ✔'
    return output


def incomplete(todo: ToDo) -> bool:
    return not todo['completed']


@update
def clear_completed() -> void:
    global todos
    todos = {key: todo for key,
             todo in todos.items() if not todo['completed']}
