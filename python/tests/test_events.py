from survivor import events


def _create_mock_round():
    event_buffer = events.EventBuffer([])

    event_buffer.add_event(
        events.SurvivorSimEventType.ENTER_NORMAL_ROUND,
        events.EnterNormalRoundEventParams([1, 2, 3, 4]),
    )
    event_buffer.add_event(
        events.SurvivorSimEventType.PRIVATE_MESSAGE,
        events.PrivateMessageEventParams(
            1, 2, "hello P2. We should vote for P3 this round."
        ),
    )
    event_buffer.add_event(
        events.SurvivorSimEventType.PRIVATE_MESSAGE,
        events.PrivateMessageEventParams(2, 1, "I agree."),
    )
    event_buffer.add_event(
        events.SurvivorSimEventType.PRIVATE_MESSAGE,
        events.PrivateMessageEventParams(3, 4, "I think we should target P2 tonight."),
    )
    event_buffer.add_event(
        events.SurvivorSimEventType.PRIVATE_MESSAGE,
        events.PrivateMessageEventParams(
            4, 3, "Agreed, P2 seems like the biggest threat right now."
        ),
    )
    event_buffer.add_event(
        events.SurvivorSimEventType.PRIVATE_MESSAGE,
        events.PrivateMessageEventParams(
            4, 1, "Hey, just wanted to let you know I'm putting my vote on P2."
        ),
    )
    event_buffer.add_event(
        events.SurvivorSimEventType.PRIVATE_VOTE,
        events.PrivateVoteEventParams(1, 2),
    )
    event_buffer.add_event(
        events.SurvivorSimEventType.PRIVATE_VOTE,
        events.PrivateVoteEventParams(2, 3),
    )
    event_buffer.add_event(
        events.SurvivorSimEventType.PRIVATE_VOTE,
        events.PrivateVoteEventParams(3, 2),
    )
    event_buffer.add_event(
        events.SurvivorSimEventType.PRIVATE_VOTE,
        events.PrivateVoteEventParams(4, 2),
    )
    event_buffer.add_event(
        events.SurvivorSimEventType.VOTE_TALLY,
        events.VoteTallyEventParams({2: 3, 3: 1}),
    )
    event_buffer.add_event(
        events.SurvivorSimEventType.ELIMINATION,
        events.PlayerEliminatedEventParams(2, "Most votes."),
    )
    event_buffer.add_event(
        events.SurvivorSimEventType.ENTER_NORMAL_ROUND,
        events.EnterNormalRoundEventParams([1, 3, 4]),
    )
    event_buffer.add_event(
        events.SurvivorSimEventType.PRIVATE_MESSAGE,
        events.PrivateMessageEventParams(1, 3, "I think we should target P4 tonight."),
    )
    event_buffer.add_event(
        events.SurvivorSimEventType.PRIVATE_MESSAGE,
        events.PrivateMessageEventParams(3, 4, "I agree."),
    )
    event_buffer.add_event(
        events.SurvivorSimEventType.PRIVATE_MESSAGE,
        events.PrivateMessageEventParams(4, 1, "I'm voting for P3."),
    )
    event_buffer.add_event(
        events.SurvivorSimEventType.PRIVATE_VOTE,
        events.PrivateVoteEventParams(1, 3),
    )
    event_buffer.add_event(
        events.SurvivorSimEventType.PRIVATE_VOTE,
        events.PrivateVoteEventParams(3, 4),
    )
    event_buffer.add_event(
        events.SurvivorSimEventType.PRIVATE_VOTE,
        events.PrivateVoteEventParams(4, 3),
    )
    event_buffer.add_event(
        events.SurvivorSimEventType.VOTE_TALLY,
        events.VoteTallyEventParams({3: 2, 4: 1}),
    )
    event_buffer.add_event(
        events.SurvivorSimEventType.ELIMINATION,
        events.PlayerEliminatedEventParams(3, "Most votes."),
    )
    event_buffer.add_event(
        events.SurvivorSimEventType.ENTER_FINAL_ROUND,
        events.EnterFinalRoundEventParams([1, 4]),
    )
    event_buffer.add_event(
        events.SurvivorSimEventType.FINAL_PUBLIC_PLEA,
        events.FinalPublicPleaEventParams(
            1, "I've been loyal to the end. Please vote for me."
        ),
    )
    event_buffer.add_event(
        events.SurvivorSimEventType.FINAL_PUBLIC_PLEA,
        events.FinalPublicPleaEventParams(
            4, "I've played a strong game. Please vote for me."
        ),
    )
    event_buffer.add_event(
        events.SurvivorSimEventType.FINAL_VOTE,
        events.FinalVoteEventParams(2, 1),
    )
    event_buffer.add_event(
        events.SurvivorSimEventType.FINAL_VOTE,
        events.FinalVoteEventParams(3, 1),
    )
    event_buffer.add_event(
        events.SurvivorSimEventType.WINNER,
        events.WinnerEventParams(1),
    )
    return event_buffer


def _test_context_as_text():
    mock_round = _create_mock_round()
    assert len(mock_round.events) == 27, "Expected 27 events in the mock round."
    p1_visible_events = mock_round.visible_events(1)
    assert len(p1_visible_events.events) == (27 - 8)
    p2_visible_events = mock_round.visible_events(2)
    assert len(p2_visible_events.events) == (27 - 12), print(
        p2_visible_events.full_text()
    )


if __name__ == "__main__":
    _test_context_as_text()
