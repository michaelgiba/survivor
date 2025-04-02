import os
import random
import json
from typing import Dict, Any
from survivor.events import (
    SurvivorSimEventType,
    SurvivorSimEvent,
)
from survivor import events
from survivor._simulation import player_agent
from collections import Counter
import plomp


class NormalRoundCommunicationsState:

    def __init__(self, player_ids: list[int]):
        self.player_ids = list(player_ids)

    def execute(self) -> int:
        print("Executing Normal Round Communications Stage")

        plomp.record_event(
            SurvivorSimEvent(
                SurvivorSimEventType.ENTER_NORMAL_ROUND,
                events.EnterNormalRoundEventParams(self.player_ids),
            ).to_dict(),
            tags={
                "event_type": SurvivorSimEventType.ENTER_NORMAL_ROUND.name,
                "visibility": "public",
            },
        )

        def _parse_player_id_and_message(raw_answer: str) -> tuple[int, str]:
            parsed = json.loads(raw_answer)
            return parsed["dest_player_id"], parsed["message"]

        player_messages_allowed = {player_id: 1 for player_id in self.player_ids}

        while any(
            remaining_count > 0 for remaining_count in player_messages_allowed.values()
        ):
            for sending_player_id in self.player_ids:
                if player_messages_allowed[sending_player_id] > 0:
                    msgs_remaining = player_messages_allowed[sending_player_id]
                    if player_agent.ask_yes_or_no(
                        sending_player_id,
                        f"Do you want to send more messages to other players? ({msgs_remaining!r} remaining)",
                    ):
                        raw_answer = player_agent.ask_player(
                            sending_player_id,
                            "Who do you want to message and what do you want to say before eventually casting an elimination vote?",
                            response_json_schema={
                                "type": "object",
                                "required": ["dest_player_id", "message"],
                                "properties": {
                                    "dest_player_id": {
                                        "type": "integer",
                                        "description": "The ID of the player to send your message to",
                                        "enum": list(
                                            set(self.player_ids) - {sending_player_id}
                                        ),
                                    },
                                    "message": {
                                        "type": "string",
                                        "description": "The message you want to send",
                                    },
                                },
                            },
                        )

                        dest_player_id, message = _parse_player_id_and_message(
                            raw_answer
                        )

                        plomp.record_event(
                            SurvivorSimEvent(
                                SurvivorSimEventType.PRIVATE_MESSAGE,
                                events.PrivateMessageEventParams(
                                    sending_player_id, dest_player_id, message
                                ),
                            ).to_dict(),
                            tags={
                                "visibility": "private",
                                "event_type": SurvivorSimEventType.PRIVATE_MESSAGE.name,
                                f"p{sending_player_id}_visible": True,
                                f"p{dest_player_id}_visible": True,
                            },
                        )

                        player_messages_allowed[sending_player_id] -= 1
                    else:
                        player_messages_allowed[sending_player_id] = 0


class NormalRoundPublicStatementStates:

    def __init__(self, player_ids: list[int]):
        self.player_ids = list(player_ids)

    def execute(self):
        print("Executing Normal Round Public Statements Stage")
        for sending_player_id in self.player_ids:
            raw_answer = player_agent.ask_player(
                sending_player_id,
                (
                    f"It is time for your public statement before the eliminiation vote. "
                    f"P{sending_player_id}, what would you like to say publicly?"
                ),
            )

            plomp.record_event(
                SurvivorSimEvent(
                    SurvivorSimEventType.PUBLIC_STATEMENT,
                    events.PublicStatementEventParams(sending_player_id, raw_answer),
                ).to_dict(),
                tags={
                    "visibility": "public",
                    "event_type": SurvivorSimEventType.PUBLIC_STATEMENT.name,
                },
            )


class NormalRoundVoteState:

    def __init__(self, player_ids: list[int]):
        self.player_ids = list(player_ids)

    def execute(self):
        print("Executing Normal Round Voting Stage")

        def _parse_player_id(raw_answer) -> int:
            parsed = json.loads(raw_answer)
            return parsed["vote_eliminate_player_id"]

        def _options_string():
            random.shuffle(self.player_ids)
            return ", ".join(f"P{pid}" for pid in self.player_ids)

        vote_counts = Counter()

        for sending_player_id in self.player_ids:
            raw_answer = player_agent.ask_player(
                sending_player_id,
                (
                    f"It is time for your eliminiation vote. "
                    f"P{sending_player_id}, who would you like to vote to eliminate? Options: "
                    f"{_options_string()}"
                ),
                response_json_schema={
                    "type": "object",
                    "required": ["vote_eliminate_player_id"],
                    "properties": {
                        "vote_eliminate_player_id": {
                            "type": "integer",
                            "description": "The ID of the player to send your message to",
                            "enum": list(set(self.player_ids) - {sending_player_id}),
                        },
                    },
                },
            )
            voted_for_player_id = _parse_player_id(raw_answer)

            plomp.record_event(
                SurvivorSimEvent(
                    SurvivorSimEventType.PRIVATE_VOTE,
                    events.PrivateVoteEventParams(
                        sending_player_id, voted_for_player_id
                    ),
                ).to_dict(),
                tags={
                    "visibility": "private",
                    f"p{sending_player_id}_visible": True,
                    "event_type": SurvivorSimEventType.PRIVATE_VOTE.name,
                },
            )

            vote_counts[voted_for_player_id] += 1

        # Record the vote tally
        plomp.record_event(
            SurvivorSimEvent(
                SurvivorSimEventType.VOTE_TALLY,
                events.VoteTallyEventParams(dict(vote_counts)),
            ).to_dict(),
            tags={
                "visibility": "public",
                "event_type": SurvivorSimEventType.VOTE_TALLY.name,
            },
        )

        # Find the player(s) with the maximum votes
        max_votes = max(vote_counts.values())
        most_voted_players = [
            player_id for player_id, votes in vote_counts.items() if votes == max_votes
        ]

        # Choose one player to eliminate (randomly if there's a tie)
        eliminated_player_id = random.choice(most_voted_players)

        # Add the elimination event with appropriate message
        message = (
            "Max votes."
            if len(most_voted_players) == 1
            else "Max votes. Tie and random selection"
        )

        # Record the vote tally
        plomp.record_event(
            SurvivorSimEvent(
                SurvivorSimEventType.ELIMINATION,
                events.PlayerEliminatedEventParams(eliminated_player_id, message),
            ).to_dict(),
            tags={
                "visibility": "public",
                "event_type": SurvivorSimEventType.ELIMINATION.name,
            },
        )

        plomp.write_html(plomp.buffer(), "/home/michaelgiba/survivor-test.html")

        return eliminated_player_id


class NormalRoundState:

    def __init__(self, player_ids: list[int]):
        self.player_ids = list(player_ids)

    def execute(self):
        print("Executing Normal Round Stage")
        NormalRoundCommunicationsState(self.player_ids).execute()
        NormalRoundPublicStatementStates(self.player_ids).execute()
        return NormalRoundVoteState(self.player_ids).execute()


class FinalRoundPublicPleaState:

    def __init__(
        self,
        remaining_player_ids: tuple[int, int],
        eliminated_player_ids: tuple[int, int],
    ):
        self.remaining_player_ids = tuple(remaining_player_ids)
        self.eliminated_player_ids = tuple(eliminated_player_ids)

    def execute(self):
        print("Executing Final Round Public Plea Stage")
        plomp.record_event(
            SurvivorSimEvent(
                SurvivorSimEventType.ENTER_FINAL_ROUND,
                events.EnterFinalRoundEventParams(self.remaining_player_ids),
            ).to_dict(),
            tags={
                "visibility": "public",
                "event_type": SurvivorSimEventType.ENTER_FINAL_ROUND.name,
            },
        )

        def _options_string():
            return ", ".join(f"P{pid}" for pid in self.remaining_player_ids)

        f_pid_0 = self.remaining_player_ids[0]
        f_pid_1 = self.remaining_player_ids[1]

        def _format_prompt(you_id: int, enemy_id: int) -> str:
            return (
                f"P{you_id}, You are one of the finalists. "
                "It is time to make your verbal plea to the eliminated players why "
                f"they should vote for you instead of {enemy_id}"
                f"The remaining_players are: {_options_string()}. "
            )

        # finalist 1
        raw_answer = player_agent.ask_player(f_pid_0, _format_prompt(f_pid_0, f_pid_1))

        plomp.record_event(
            SurvivorSimEvent(
                SurvivorSimEventType.FINAL_PUBLIC_PLEA,
                events.FinalPublicPleaEventParams(f_pid_0, raw_answer),
            ).to_dict(),
            tags={
                "visibility": "public",
                "event_type": SurvivorSimEventType.FINAL_PUBLIC_PLEA.name,
            },
        )

        # finalist 2
        raw_answer = player_agent.ask_player(f_pid_1, _format_prompt(f_pid_1, f_pid_0))

        plomp.record_event(
            SurvivorSimEvent(
                SurvivorSimEventType.FINAL_PUBLIC_PLEA,
                events.FinalPublicPleaEventParams(f_pid_1, raw_answer),
            ).to_dict(),
            tags={
                "visibility": "public",
                "event_type": SurvivorSimEventType.FINAL_PUBLIC_PLEA.name,
            },
        )


class FinalRoundVoteState:

    def __init__(
        self,
        remaining_player_ids: tuple[int, int],
        eliminated_player_ids: list[int],
    ):
        self.remaining_player_ids = tuple(remaining_player_ids)
        self.eliminated_player_ids = list(eliminated_player_ids)

    def execute(self):
        print("Executing Final Round Voting Stage")

        def _parse_player_id(raw_answer) -> int:
            parsed = json.loads(raw_answer)
            return parsed["vote_winner_player_id"]

        def _options_string():
            finalists = list(self.remaining_player_ids)
            random.shuffle(finalists)
            return ", ".join(f"P{pid}" for pid in finalists)

        vote_counts = Counter()

        for voting_player_id in self.eliminated_player_ids:
            raw_answer = player_agent.ask_player(
                voting_player_id,
                (
                    f"It is time for your final vote. "
                    f"P{voting_player_id}, as an eliminated player, you get to vote for the winner. "
                    f"Who would you like to vote for? Options: {_options_string()}"
                ),
                response_json_schema={
                    "type": "object",
                    "required": ["vote_winner_player_id"],
                    "properties": {
                        "vote_winner_player_id": {
                            "type": "integer",
                            "enum": list(self.remaining_player_ids),
                        },
                    },
                },
            )
            voted_for_player_id = _parse_player_id(raw_answer)

            plomp.record_event(
                SurvivorSimEvent(
                    SurvivorSimEventType.FINAL_VOTE,
                    events.FinalVoteEventParams(voting_player_id, voted_for_player_id),
                ).to_dict(),
                tags={
                    "visibility": "public",
                    "event_type": SurvivorSimEventType.FINAL_VOTE.name,
                },
            )

            vote_counts[voted_for_player_id] += 1

            # Record the vote tally
            plomp.record_event(
                SurvivorSimEvent(
                    SurvivorSimEventType.VOTE_TALLY,
                    events.VoteTallyEventParams(dict(vote_counts)),
                ).to_dict(),
                tags={
                    "visibility": "public",
                    "event_type": SurvivorSimEventType.VOTE_TALLY.name,
                },
            )

            # Find the player(s) with the maximum votes
            max_votes = max(vote_counts.values()) if vote_counts else 0
            most_voted_players = [
                player_id
                for player_id, votes in vote_counts.items()
                if votes == max_votes
            ]

            # Choose one player as the winner (randomly if there's a tie)
            winner_player_id = random.choice(most_voted_players)

            # Determine if it was a tie
            message = (
                "Won by majority vote."
                if len(most_voted_players) == 1
                else "Won by tiebreaker."
            )

            # Add the winner event
            plomp.record_event(
                SurvivorSimEvent(
                    SurvivorSimEventType.WINNER,
                    events.WinnerEventParams(winner_player_id, message),
                ).to_dict(),
                tags={
                    "visibility": "public",
                    "event_type": SurvivorSimEventType.WINNER.name,
                },
            )


class FinalRoundState:

    def __init__(
        self,
        remaining_player_ids: tuple[int, int],
        eliminated_player_ids: list[int],
    ):
        self.remaining_player_ids = remaining_player_ids
        self.eliminated_player_ids = eliminated_player_ids

    def execute(self):
        print("Executing Final Round Stage")
        FinalRoundPublicPleaState(
            self.remaining_player_ids, self.eliminated_player_ids
        ).execute()

        return FinalRoundVoteState(
            self.remaining_player_ids, self.eliminated_player_ids
        ).execute()
