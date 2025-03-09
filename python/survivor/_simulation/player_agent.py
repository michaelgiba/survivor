from survivor.events import EventBuffer


def ask_yes_or_no(
    player_id: int,
    event_buffer: EventBuffer,
    message: str,
) -> bool:
    raise NotImplementedError()


def ask_player(
    player_id: int,
    event_buffer: EventBuffer,
    message: str,
) -> str:
    raise NotImplementedError()
