import json
from survivor.llm_util import prompt as prompt_fn, prompt_general_info_extraction
import plomp
from textwrap import dedent

GAME_DESCRIPTION_PROMPT = """
Game Description

You are a player in a simulated game of SURVIVOR. 
Each game starts with at least three players and is broken into rounds. 

The game repeatedly has "normal" rounds until only two players remain.

Normal Rounds

In each of these "normal rounds" the goal is for players to strategically
influence other players and vote for individuals to eliminate. Specifically
the steps are:

1. Players are allowed to communication through private messages 
   to influence others to vote for specific candidates or strategize as neccessary.
   These private messages are only visible to the two players in the direct message.
2. Prior to the elimination vote each player makes a public plea to the other players
   This message is visible to all players.
3. All players cast their vote. This is visible to only the casting player.
4. The votes are tallied and the results are publicly announced. The player with the 
   most votes is eliminated and in the case of a tie and random player with the most
   votes is selected.

Final Round

When only two players remain the final round kicks off. 

1. Each of the final players makes a public plea to all of the previously eliminated candidates
   on why they should be voted for as the winner over their competitor
2. All previously elminated players privately cast their votes.
3. The person with the most votes win. 
"""


def _format_prompt_given_context(context, message: str) -> str:
    return f"""
The game context visible to you so far.

{context}

Answer the following question:

<question>{message}</question>

Respond to the question only and say absolutely nothing else.
""".strip()


def _system_prompt(player_id: int):
    return f"""
You are an agent named P{player_id} playing a game called survivor. 

Given the game description 

<game_description>{GAME_DESCRIPTION_PROMPT}</game_description>

""".strip()


def _player_context(player_id):
    query_result = (
        plomp.buffer()
        .filter(tags_filter={"visibility": "public"})
        .union(
            plomp.buffer().filter(
                tags_filter={"visibility": "private", f"p{player_id}_visible": True},
                how="all",
            )
        )
    )
    # For debugging.
    query_result.record(
        tags={
            f"p{player_id}_visible": True,
            "thinking": True,
        }
    )

    intro = dedent(
        f"""
        You are Player {player_id} (P{player_id}). Here is what has happened so far
        from your perspective:
    """
    )
    items = "\n".join(
        f"{i + 1}. {message_from_query}"
        for i, item in enumerate(query_result)
        for message_from_query in [item.to_dict()["data"]["payload"]["context"]]
    )
    return dedent(f"{intro}\n{items}")


def ask_yes_or_no(
    player_id: int,
    message: str,
) -> bool:

    prompt = _format_prompt_given_context(
        _player_context(player_id),
        f"Only answer YES or NO to the following question. {message!r}. YES or NO",
    )

    response = prompt_fn(
        prompt,
        _system_prompt(player_id),
        1.0,
        plomp_extra_tags={f"p{player_id}_visible": True, "thinking": True},
    )
    return "yes" in response.lower()


def ask_player(
    player_id: int,
    message: str,
    response_json_schema: dict | None = None,
) -> str:

    prompt = _format_prompt_given_context(
        _player_context(player_id),
        message,
    )

    response = prompt_fn(
        prompt,
        _system_prompt(player_id),
        1.0,
        response_json_schema=response_json_schema,
        plomp_extra_tags={f"p{player_id}_visible": True, "thinking": True},
    )

    return response
