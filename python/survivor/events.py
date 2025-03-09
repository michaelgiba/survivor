import abc
from dataclasses import dataclass
from enum import Enum, auto


class SurvivorSimEventType(Enum):
    ENTER_NORMAL_ROUND = auto()
    PRIVATE_MESSAGE = auto()
    PUBLIC_STATEMENT = auto()
    PRIVATE_VOTE = auto()
    VOTE_TALLY = auto()
    ELIMINATION = auto()
    ENTER_FINAL_ROUND = auto()
    FINAL_PUBLIC_PLEA = auto()
    FINAL_VOTE = auto()
    WINNER = auto()


class EventParams(abc.ABC):
    def is_visible_to(self, player_id: int) -> bool:
        return True


@dataclass
class EnterNormalRoundEventParams(EventParams):
    player_ids_in_round: list[int]

    def description(self) -> str:
        sorted_player_ids = sorted(self.player_ids_in_round)
        player_names = [f"P{player_id}" for player_id in sorted_player_ids]
        return f"New round begins with players: {', '.join(player_names)}"


@dataclass
class PrivateMessageEventParams(EventParams):
    send_player_id: int
    recv_player_id: int
    message: str

    def is_visible_to(self, player_id: int) -> bool:
        return player_id in [self.send_player_id, self.recv_player_id]

    def description(self) -> str:
        return f"P{self.send_player_id} sent P{self.recv_player_id} a message: {self.message!r}"


@dataclass
class PublicStatementEventParams(EventParams):
    speaking_player_id: int
    statement: str

    def description(self) -> str:
        return f"P{self.speaking_player_id} made public statement: {self.statement!r}"


@dataclass
class PrivateVoteEventParams(EventParams):
    voting_player_id: int
    target_elimination_player_id: int

    def is_visible_to(self, player_id: int):
        return self.voting_player_id == player_id

    def description(self) -> str:
        return f"P{self.voting_player_id} voted to eliminate P{self.target_elimination_player_id}"


@dataclass
class VoteTallyEventParams(EventParams):
    player_id_to_vote_count: dict[int, int]

    def description(self) -> str:
        player_formatted_votes = [
            f"P{player_id}={vote_count}"
            for player_id, vote_count in self.player_id_to_vote_count.items()
        ]
        return f"Votes are tallied. Results show: {', '.join(player_formatted_votes)}"


@dataclass
class PlayerEliminatedEventParams(EventParams):
    eliminated_player_id: int
    reason: str

    def description(self) -> str:
        return f"P{self.eliminated_player_id} was eliminated. Reason: {self.reason!r}"


@dataclass
class EnterFinalRoundEventParams(EventParams):
    final_two_player_ids: tuple[int, int]

    def description(self) -> str:
        return f"Final round begins with P{self.final_two_player_ids[0]} and P{self.final_two_player_ids[1]}"


@dataclass
class FinalPublicPleaEventParams(EventParams):
    speaking_player_id: int
    speech_text: str

    def description(self) -> str:
        return f"P{self.speaking_player_id} made a final plea: {self.speech_text!r}"


@dataclass
class FinalVoteEventParams(EventParams):
    eliminated_player_id: int
    voted_to_win_player_id: int

    def description(self) -> str:
        return f"P{self.voted_to_win_player_id} voted for P{self.eliminated_player_id} to win"


@dataclass
class WinnerEventParams(EventParams):
    winner_player_id: int
    message: str

    def description(self) -> str:
        return f"P{self.winner_player_id} is the winner! ({message})"


EVENT_TYPE_TO_PARAMS = {
    SurvivorSimEventType.ENTER_NORMAL_ROUND: EnterNormalRoundEventParams,
    SurvivorSimEventType.PRIVATE_MESSAGE: PrivateMessageEventParams,
    SurvivorSimEventType.PUBLIC_STATEMENT: PublicStatementEventParams,
    SurvivorSimEventType.PRIVATE_VOTE: PrivateVoteEventParams,
    SurvivorSimEventType.VOTE_TALLY: VoteTallyEventParams,
    SurvivorSimEventType.ELIMINATION: PlayerEliminatedEventParams,
    SurvivorSimEventType.ENTER_FINAL_ROUND: EnterFinalRoundEventParams,
    SurvivorSimEventType.FINAL_PUBLIC_PLEA: FinalPublicPleaEventParams,
    SurvivorSimEventType.FINAL_VOTE: FinalVoteEventParams,
    SurvivorSimEventType.WINNER: WinnerEventParams,
}


@dataclass
class SurivivorSimConfig:
    num_survivors: int


@dataclass
class SurvivorSimEvent:
    seq_number: int
    event_type: SurvivorSimEventType
    event_params: EventParams

    def is_visible_to(self, player_id: int) -> bool:
        return self.event_params.is_visible_to(player_id)

    def as_text(self):
        return f"{self.event_type.name}: {self.event_params.description()} (event_sequence={self.seq_number})"


@dataclass
class EventBuffer:
    events: list[SurvivorSimEvent]

    def next_seq_number(self) -> int:
        if len(self.events) == 0:
            return 0
        else:
            return max(event.seq_number for event in self.events) + 1

    def visible_events_as_of(self, to_player_id: int, as_of_seq: int) -> "EventBuffer":
        visible_buffer = []
        for event in self.events:
            if event.seq_number > as_of_seq:
                break
            if event.is_visible_to(to_player_id):
                visible_buffer.append(event)

        return self.__class__(visible_buffer)

    def visible_events(self, to_player_id: int) -> "EventBuffer":
        return self.visible_events_as_of(to_player_id, self.next_seq_number())

    def add_event(self, event_type: SurvivorSimEventType, event_params: EventParams):
        assert type(event_params) == EVENT_TYPE_TO_PARAMS[event_type]

        seq_number = self.next_seq_number()
        self.events.append(SurvivorSimEvent(seq_number, event_type, event_params))

    def full_text(self):
        return "\n".join(event.as_text() for event in self.events)
