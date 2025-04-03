from survivor._simulation import game_states
from survivor._simulation.game_states import FinalRoundState
from typing import Callable


class SurvivorSim:

    def __init__(self, config):
        num_players = int(config["num_players"])
        if num_players < 3:
            raise ValueError(f"Invalid {num_players=}")

        self.num_players = num_players
        self.players = list(range(num_players))

    def execute(self, *, write_progress: Callable[[], None]):

        current_players = set(self.players)
        eliminated_players = set()
        write_progress()

        while len(current_players) != 2:
            eliminated_id = game_states.NormalRoundState(current_players).execute(
                write_progress=write_progress
            )
            current_players.remove(eliminated_id)
            eliminated_players.add(eliminated_id)

        FinalRoundState(
            tuple(current_players),
            list(eliminated_players),
        ).execute(write_progress=write_progress)


__all__ = ["SurvivorSimulation"]
