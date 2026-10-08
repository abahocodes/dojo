def num_rescue_boats(people: list[int], limit: int) -> int:
    p = sorted(people)
    i, j = 0, len(p) - 1
    boats = 0
    while i <= j:
        if p[i] + p[j] <= limit:
            i += 1
        j -= 1
        boats += 1
    return boats
