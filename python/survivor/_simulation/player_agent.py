from survivor.events import EventBuffer
import json
from survivor.llm_util import prompt as prompt_fn, prompt_general_info_extraction

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


def _format_prompt_given_context(
    player_id: int,
    event_buffer: EventBuffer,
    message: str,
) -> str:

    return f"""
    The game context visible to you so far

    <visible_context>{event_buffer.full_text()}</visible_context>

    Answer the following question:

    <question>{message}</question>

    Respond to the question only and say absolutely nothing else.
    """


def _system_prompt(player_id: int):
    return f"""
    You are an agent named P{player_id} playing a game called survivor. 
    
    Given the game description 
    
    <game_description>{GAME_DESCRIPTION_PROMPT}</game_description>

    """


def ask_yes_or_no(
    player_id: int,
    event_buffer: EventBuffer,
    message: str,
) -> bool:

    prompt = _format_prompt_given_context(
        player_id,
        event_buffer.visible_events(player_id),
        f"Only answer YES or NO to the following question. {message!r}. YES or NO",
    )

    response = prompt_fn(prompt, _system_prompt(player_id), 1.0)
    return "yes" in response.lower()


def ask_player(
    player_id: int,
    event_buffer: EventBuffer,
    message: str,
    response_json_schema: dict | None = None,
) -> str:

    prompt = _format_prompt_given_context(
        player_id,
        event_buffer.visible_events(player_id),
        message,
    )

    response = prompt_fn(
        prompt,
        _system_prompt(player_id),
        1.0,
        response_json_schema=response_json_schema,
    )

    # if response_json_schema is None:
    #     return response
    # else:
    #     extracted = prompt_general_info_extraction(
    #         f"""You have the following response from an LLM and you need to repackage this
    #         into an object which conforms to the 'json-schema' format: {response_json_schema}.
    #         Ok here is the content, respond with the JSON extraction and nothig else:

    #         {response}

    #         """
    #     )
    #     return extracted

    return response
