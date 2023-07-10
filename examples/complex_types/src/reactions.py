from kybra import nat32, query, update, Vec
from src.candid_types import Reaction, ReactionType
from src.state import state, StatePost, StateReaction, StateUser


@update
def create_reaction(
    author_id: str, post_id: str, reaction_type: ReactionType, join_depth: nat32
) -> Reaction:
    id = str(len(state["reactions"].keys()))

    state_reaction: StateReaction = {
        "id": id,
        "author_id": author_id,
        "post_id": post_id,
        "reaction_type": reaction_type,
    }
    updated_state_author = get_updated_state_author(author_id, state_reaction["id"])
    updated_state_post = get_updated_state_post(post_id, state_reaction["id"])

    state["reactions"][id] = state_reaction
    state["users"][author_id] = updated_state_author
    state["posts"][post_id] = updated_state_post

    reaction = get_reaction_from_state_reaction(state_reaction, join_depth)

    return reaction


@query
def get_all_reactions(join_depth: nat32) -> Vec[Reaction]:
    return list(
        map(
            lambda state_reaction: get_reaction_from_state_reaction(
                state_reaction, join_depth
            ),
            state["reactions"].values(),
        )
    )


def get_reaction_from_state_reaction(
    state_reaction: StateReaction, join_depth: nat32
) -> Reaction:
    # These are here because of problems with circular dependencies
    from src.posts import get_post_from_state_post
    from src.users import get_user_from_state_user

    state_author = state["users"][state_reaction["author_id"]]
    author = get_user_from_state_user(state_author, join_depth)

    state_post = state["posts"][state_reaction["post_id"]]
    post = get_post_from_state_post(state_post, join_depth)

    return {
        "id": state_reaction["id"],
        "author": author,
        "post": post,
        "reaction_type": state_reaction["reaction_type"],
    }


def get_updated_state_author(author_id: str, reaction_id: str) -> StateUser:
    state_author = state["users"][author_id]
    updated_state_author: StateUser = {
        **state_author,
        "reaction_ids": [*state_author["reaction_ids"], reaction_id],
    }

    return updated_state_author


def get_updated_state_post(post_id: str, reaction_id: str) -> StatePost:
    state_post = state["posts"][post_id]
    updated_state_post: StatePost = {
        **state_post,
        "reaction_ids": [*state_post["reaction_ids"], reaction_id],
    }

    return updated_state_post
