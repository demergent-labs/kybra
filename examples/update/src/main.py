from kybra import query, update, text, void

current_message: text = ''


@query
def get_current_message() -> text:
    return current_message


@update
def simple_update(message: text) -> void:
    global current_message

    current_message = message
