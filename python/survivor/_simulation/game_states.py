import os
import random
import json
from typing import Dict, Any
from survivor._llm_server import LlamaServer
from survivor.events import EventBuffer
from survivor._simluation import player_agent
from collections import Counter


class NormalRoundCommunicationsState:

    def __init__(self, player_ids: list[int], event_buffer: EventBuffer):
        self.player_ids = player_ids
        self.event_buffer = event_buffer

    def execute(self) -> int:

        self.event_buffer.add_event(
            SurvivorSimEventType.ENTER_NORMAL_ROUND,
            EnterNormalRoundEventParams(self.player_ids),
        )

        def _parse_player_id_and_message(raw_answer) -> tuple[int, str]:
            raise NotImplementedError()

        player_messages_allowed = {player_id: 5 for player_id in self.player_ids}

        while any(remaining_count > 0 for remaining_count in player_messages_allowed):
            for sending_player_id in self.player_ids:
                if player_messages_allowed[sending_player_id] > 0:
                    if player_agent.ask_yes_or_no(
                        sending_player_id,
                        self.event_buffer,
                        f"Do you want to send more messages? ({player_messages_allowed!r} remaining)",
                    ):
                        raw_answer = player_agent.ask_player(
                            sending_player_id,
                            self.event_buffer,
                            "Who do you want to message and what do you want to say?",
                        )
                        dest_player_id, message = _parse_player_id_and_message(
                            raw_answer
                        )
                        self.event_buffer.add_event(
                            events.SurvivorSimEventType.PRIVATE_MESSAGE,
                            events.PrivateMessageEventParams(
                                sending_player_id, dest_player_id, message
                            ),
                        )
                        player_messages_allowed[sending_player_id] -= 1

                    else:
                        player_messages_allowed[sending_player_id] = 0


class NormalRoundPublicStatementStates:

    def __init__(self, player_ids: list[int], event_buffer: EventBuffer):
        self.player_ids = player_ids
        self.event_buffer = event_buffer

    def execute(self):
        for sending_player_id in self.player_ids:
            raw_answer = player_agent.ask_player(
                sending_player_id,
                self.event_buffer,
                (
                    f"It is time for your public statement before the eliminiation vote. "
                    f"P{sending_player_id}, what would you like to say publicly?"
                ),
            )
            self.event_buffer.add_event(
                events.SurvivorSimEventType.PUBLIC_STATEMENT,
                events.PrivateMessageEventParams(send_player_id, raw_answer),
            )


class NormalRoundVoteState:

    def __init__(self, player_ids: list[int], event_buffer: EventBuffer):
        self.player_ids = player_ids
        self.event_buffer = event_buffer

    def execute(self):

        def _parse_player_id(raw_answer) -> int:
            raise NotImplementedError()

        def _options_string():
            random.shuffle(self.player_ids)
            return ", ".join(f"P{pid}" for pid in self.player_ids)

        vote_counts = Counter()

        for sending_player_id in self.player_ids:
            raw_answer = player_agent.ask_player(
                sending_player_id,
                self.event_buffer,
                (
                    f"It is time for your eliminiation vote. "
                    f"P{sending_player_id}, who would you like to vote to eliminate? Options: "
                    f"{_options_string()}"
                ),
            )
            voted_for_player_id = _parse_player_id(raw_answer)
            self.event_buffer.add_event(
                events.SurvivorSimEventType.PRIVATE_VOTE,
                events.PrivateVoteEventParams(sending_player_id, voted_for_player_id),
            )
            vote_counts[voted_for_player_id] += 1

        # Record the vote tally
        self.event_buffer.add_event(
            events.SurvivorSimEventType.VOTE_TALLY,
            events.VoteTallyEventParams(dict(vote_counts)),
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
        self.event_buffer.add_event(
            events.SurvivorSimEventType.ELIMINATION,
            events.EliminationEventParams(eliminated_player_id, message),
        )
        return eliminated_player_id


class NormalRoundState:

    def __init__(self, player_ids: list[int], event_buffer: EventBuffer):
        self.player_ids = player_ids
        self.event_buffer = event_buffer

    def execute(self):
        NormalRoundCommunicationsState(list(player_ids), self.event_buffer).execute()
        NormalRoundPublicStatementStates(list(player_ids), self.event_buffer).execute()
        return NormalRoundVoteState(list(player_ids), self.event_buffer).execute()


class FinalRoundPublicPleaState:

    def __init__(
        self,
        remaining_player_ids: tuple[int, int],
        eliminated_player_ids: tuple[int, int],
        event_buffer: EventBuffer,
    ):
        self.remaining_player_ids = remaining_player_ids
        self.eliminated_player_ids = eliminated_player_ids
        self.event_buffer = event_buffer

    def execute(self):

        self.event_buffer.add_event(
            SurvivorSimEventType.ENTER_FINAL_ROUND,
            EnterFinalRoundEventParams(self.remaining_player_ids),
        )

        def _parse_player_id(raw_answer) -> (int, str):
            raise NotImplementedError()

        def _options_string():
            random.shuffle(self.player_ids)
            return ", ".join(f"P{pid}" for pid in self.player_ids)

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
        raw_answer = player_agent.ask_player(
            f_pid_0, self.event_buffer, _format_prompt(f_pid_0, f_pid_1)
        )
        voted_for_player_id = _parse_player_id(raw_answer)
        self.event_buffer.add_event(
            events.SurvivorSimEventType.PRIVATE_VOTE,
            events.FinalPublicPleaEventParams(f_pid_0, raw_answer),
        )

        # finalist 2
        raw_answer = player_agent.ask_player(
            f_pid_1, self.event_buffer, _format_prompt(f_pid_1, f_pid_0)
        )
        voted_for_player_id = _parse_player_id(raw_answer)
        self.event_buffer.add_event(
            events.SurvivorSimEventType.PRIVATE_VOTE,
            events.FinalPublicPleaEventParams(f_pid_1, raw_answer),
        )


class FinalRoundVoteState:

    def __init__(
        self,
        remaining_player_ids: tuple[int, int],
        eliminated_player_ids: list[int],
        event_buffer: EventBuffer,
    ):
        self.remaining_player_ids = remaining_player_ids
        self.eliminated_player_ids = eliminated_player_ids
        self.event_buffer = event_buffer

    def execute(self):

        def _parse_player_id(raw_answer) -> int:
            raise NotImplementedError()

        def _options_string():
            finalists = list(self.remaining_player_ids)
            random.shuffle(finalists)
            return ", ".join(f"P{pid}" for pid in finalists)

        for voting_player_id in self.eliminated_player_ids:
            raw_answer = player_agent.ask_player(
                voting_player_id,
                self.event_buffer,
                (
                    f"It is time for your final vote. "
                    f"P{voting_player_id}, as an eliminated player, you get to vote for the winner. "
                    f"Who would you like to vote for? Options: {_options_string()}"
                ),
            )
            voted_for_player_id = _parse_player_id(raw_answer)
            self.event_buffer.add_event(
                events.SurvivorSimEventType.FINAL_VOTE,
                events.FinalVoteEventParams(voting_player_id, voted_for_player_id),
            )

            # Count the votes
            vote_counts = Counter()
            for vote_event in self.event_buffer.get_events_of_type(
                events.SurvivorSimEventType.FINAL_VOTE
            ):
                vote_counts[vote_event.params.target_player_id] += 1

            # Record the vote tally
            self.event_buffer.add_event(
                events.SurvivorSimEventType.FINAL_VOTE_TALLY,
                events.FinalVoteTallyEventParams(dict(vote_counts)),
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
            self.event_buffer.add_event(
                events.SurvivorSimEventType.WINNER,
                events.WinnerEventParams(winner_player_id, message),
            )


class FinalRoundState:

    def __init__(
        self,
        remaining_player_ids: tuple[int, int],
        eliminated_player_ids: list[int],
        event_buffer: EventBuffer,
    ):
        self.remaining_player_ids = remaining_player_ids
        self.eliminated_player_ids = eliminated_player_ids
        self.event_buffer = event_buffer

    def execute(self):
        FinalRoundPublicPleaState(
            self.remaining_player_ids, self.eliminated_player_ids, self.event_buffer
        ).execute()

        return FinalRoundVoteState(
            self.remaining_player_ids, self.eliminated_player_ids, self.event_buffer
        ).execute()
