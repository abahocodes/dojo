from bisect import bisect_left


def find_radius(houses: list[int], heaters: list[int]) -> int:
    heaters = sorted(heaters)
    best = 0
    for x in houses:
        i = bisect_left(heaters, x)
        near = float("inf")
        if i < len(heaters):
            near = heaters[i] - x
        if i > 0:
            near = min(near, x - heaters[i - 1])
        best = max(best, near)
    return best
