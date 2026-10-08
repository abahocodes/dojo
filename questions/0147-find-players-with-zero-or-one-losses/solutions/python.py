def find_winners(matches: list[list[int]]) -> list[list[int]]:
    losses = {}
    for winner, loser in matches:
        losses.setdefault(winner, 0)
        losses[loser] = losses.get(loser, 0) + 1
    never = sorted(p for p, c in losses.items() if c == 0)
    once = sorted(p for p, c in losses.items() if c == 1)
    return [never, once]
