from survivor import events
from survivor._simluation import game_states


class SurvivorSim:

    def __init__(self, num_players):
        if num_players <= 3:
            raise ValueError(f"Invalid {num_players=}")

        self.num_players = num_players
        self.players = list(range(num_players))
        self.event_buffer = events.EventBuffer([])

    def execute(self):

        current_players = set(self.players)
        eliminated_players = set()

        while len(current_players) != 2:
            eliminated_id = game_states.NormalRoundState(
                list(current_players), self.event_buffer
            )
            current_players.remove(eliminated_id)
            eliminated_players.add(eliminated_id)

        FinalRoundState(
            tuple(current_players),
            list(eliminated_players),
            self.event_buffer,
        ).execute()
