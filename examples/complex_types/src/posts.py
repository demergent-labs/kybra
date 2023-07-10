from kybra import nat32, query, update, Vec
from src.candid_types import Post
from src.state import state, StatePost, StateThread, StateUser


@update
def create_post(author_id: str, text: str, thread_id: str, join_depth: nat32) -> Post:
    id = str(len(state["posts"].keys()))

    state_post: StatePost = {
        "id": id,
        "author_id": author_id,
        "reaction_ids": [],
        "text": text,
        "thread_id": thread_id,
    }
    updated_state_author = get_updated_state_author(author_id, state_post["id"])
    updated_state_thread = get_updated_state_thread(thread_id, state_post["id"])

    state["posts"][id] = state_post
    state["users"][author_id] = updated_state_author
    state["threads"][thread_id] = updated_state_thread

    post = get_post_from_state_post(state_post, join_depth)

    return post


@query
def get_all_posts(join_depth: nat32) -> Vec[Post]:
    return list(
        map(
            lambda state_post: get_post_from_state_post(state_post, join_depth),
            state["posts"].values(),
        )
    )


def get_post_from_state_post(state_post: StatePost, join_depth: nat32) -> Post:
    # These are here because of problems with circular dependencies
    from src.reactions import get_reaction_from_state_reaction
    from src.threads import get_thread_from_state_thread
    from src.users import get_user_from_state_user

    state_author = state["users"][state_post["author_id"]]
    author = get_user_from_state_user(state_author, join_depth)

    state_thread = state["threads"][state_post["thread_id"]]
    thread = get_thread_from_state_thread(state_thread, join_depth)

    if join_depth == 0:
        return {
            "id": state_post["id"],
            "author": author,
            "reactions": [],
            "text": state_post["text"],
            "thread": thread,
        }
    else:
        state_reactions = list(
            map(
                lambda reaction_id: state["reactions"][reaction_id],
                state_post["reaction_ids"],
            )
        )
        reactions = list(
            map(
                lambda state_reaction: get_reaction_from_state_reaction(
                    state_reaction, join_depth
                ),
                state_reactions,
            )
        )

    return {
        "id": state_post["id"],
        "author": author,
        "reactions": reactions,
        "text": state_post["text"],
        "thread": thread,
    }


def get_updated_state_author(author_id: str, post_id: str) -> StateUser:
    state_author = state["users"][author_id]
    updated_state_author: StateUser = {
        **state_author,
        "post_ids": [*state_author["post_ids"], post_id],
    }

    return updated_state_author


def get_updated_state_thread(thread_id: str, post_id: str) -> StateThread:
    state_thread = state["threads"][thread_id]
    updated_state_thread: StateThread = {
        **state_thread,
        "post_ids": [*state_thread["post_ids"], post_id],
    }

    return updated_state_thread
