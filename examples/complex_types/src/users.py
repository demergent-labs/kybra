from kybra import nat32, query, update, Vec
from src.candid_types import User
from src.state import state, StateUser


@update
def create_user(username: str, join_depth: nat32) -> User:
    id = str(len(state["users"].keys()))

    state_user: StateUser = {
        "id": id,
        "post_ids": [],
        "reaction_ids": [],
        "thread_ids": [],
        "username": username,
    }

    state["users"][id] = state_user

    user = get_user_from_state_user(state_user, join_depth)

    return user


@query
def get_all_users(join_depth: nat32) -> Vec[User]:
    return list(
        map(
            lambda state_user: get_user_from_state_user(state_user, join_depth),
            state["users"].values(),
        )
    )


def get_user_from_state_user(state_user: StateUser, join_depth: nat32) -> User:
    # These are here because of problems with circular dependencies
    from src.posts import get_post_from_state_post
    from src.reactions import get_reaction_from_state_reaction
    from src.threads import get_thread_from_state_thread

    if join_depth == 0:
        return {
            "id": state_user["id"],
            "posts": [],
            "reactions": [],
            "threads": [],
            "username": state_user["username"],
        }
    else:
        state_posts = list(
            map(lambda post_id: state["posts"][post_id], state_user["post_ids"])
        )
        posts = list(
            map(
                lambda state_post: get_post_from_state_post(state_post, join_depth - 1),
                state_posts,
            )
        )

        state_reactions = list(
            map(
                lambda reaction_id: state["reactions"][reaction_id],
                state_user["reaction_ids"],
            )
        )
        reactions = list(
            map(
                lambda state_reaction: get_reaction_from_state_reaction(
                    state_reaction, join_depth - 1
                ),
                state_reactions,
            )
        )

        state_threads = list(
            map(lambda thread_id: state["threads"][thread_id], state_user["thread_ids"])
        )
        threads = list(
            map(
                lambda state_thread: get_thread_from_state_thread(
                    state_thread, join_depth - 1
                ),
                state_threads,
            )
        )

        return {
            "id": state_user["id"],
            "posts": posts,
            "reactions": reactions,
            "threads": threads,
            "username": state_user["username"],
        }
