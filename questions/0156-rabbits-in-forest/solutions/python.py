def num_rabbits(answers: list[int]) -> int:
    counts = {}
    for x in answers:
        counts[x] = counts.get(x, 0) + 1
    total = 0
    for x, c in counts.items():
        size = x + 1
        groups = (c + size - 1) // size
        total += groups * size
    return total
