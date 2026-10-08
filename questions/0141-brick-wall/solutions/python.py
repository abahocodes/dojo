def least_bricks(wall: list[list[int]]) -> int:
    seams = {}
    best = 0
    for row in wall:
        pos = 0
        for width in row[:-1]:
            pos += width
            seams[pos] = seams.get(pos, 0) + 1
            best = max(best, seams[pos])
    return len(wall) - best
