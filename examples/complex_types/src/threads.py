from kybra import nat32, query, update, Vec
from src.candid_types import Thread
from src.state import state, StateThread, StateUser


@update
def create_thread(title: str, author_id: str, join_depth: nat32) -> Thread:
    id = str(len(state["threads"].keys()))

    state_thread: StateThread = {
        "id": id,
        "author_id": author_id,
        "post_ids": [],
        "title": title,
    }
    updated_state_author = get_updated_state_author(author_id, state_thread["id"])

    state["threads"][id] = state_thread
    state["users"][author_id] = updated_state_author

    thread = get_thread_from_state_thread(state_thread, join_depth)

    return thread


@query
def get_all_threads(join_depth: nat32) -> Vec[Thread]:
    return list(
        map(
            lambda state_thread: get_thread_from_state_thread(state_thread, join_depth),
            state["threads"].values(),
        )
    )


def get_thread_from_state_thread(
    state_thread: StateThread, join_depth: nat32
) -> Thread:
    # These are here because of problems with circular dependencies
    from src.posts import get_post_from_state_post
    from src.users import get_user_from_state_user

    state_author = state["users"][state_thread["author_id"]]
    author = get_user_from_state_user(state_author, join_depth)

    if join_depth == 0:
        return {
            "id": state_thread["id"],
            "author": author,
            "posts": [],
            "title": state_thread["title"],
        }
    else:
        state_posts = list(
            map(lambda post_id: state["posts"][post_id], state_thread["post_ids"])
        )
        posts = list(
            map(
                lambda state_post: get_post_from_state_post(state_post, join_depth - 1),
                state_posts,
            )
        )

        return {
            "id": state_thread["id"],
            "author": author,
            "posts": posts,
            "title": state_thread["title"],
        }


def get_updated_state_author(author_id: str, thread_id: str) -> StateUser:
    state_author = state["users"][author_id]
    updated_state_author: StateUser = {
        **state_author,
        "thread_ids": [*state_author["thread_ids"], thread_id],
    }

    return updated_state_author
