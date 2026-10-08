def max_score_cards(card_points: list[int], k: int) -> int:
    n = len(card_points)
    current = sum(card_points[:k])  # take all k from the front
    best = current
    for i in range(1, k + 1):
        # Give back the i-th front card, take the i-th card from the back.
        current += card_points[n - i] - card_points[k - i]
        best = max(best, current)
    return best
