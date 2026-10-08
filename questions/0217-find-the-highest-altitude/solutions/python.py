def largest_altitude(gain: list[int]) -> int:
    altitude = 0
    best = 0
    for g in gain:
        altitude += g
        best = max(best, altitude)
    return best
