import heapq


def get_skyline(buildings: list[list[int]]) -> list[list[int]]:
    # Events sorted by x; at equal x, starts come first, tallest start first.
    events = []
    for left, right, height in buildings:
        events.append((left, -height, right))  # a building starts
        events.append((right, 0, 0))           # a building may end here
    events.sort()

    result = []
    live = []  # max-heap of (-height, right) for buildings that may still stand
    for x, neg_height, right in events:
        # Lazily drop buildings that ended at or before x.
        while live and live[0][1] <= x:
            heapq.heappop(live)
        if neg_height:
            heapq.heappush(live, (neg_height, right))
        # The first event at x already settles the height at x, so a later
        # event at the same x never adds a second point.
        current = -live[0][0] if live else 0
        if not result or result[-1][1] != current:
            result.append([x, current])
    return result
