def merge_triplets(triplets: list[list[int]], target: list[int]) -> bool:
    # Merge every triplet that never exceeds the target; the rest are poison.
    best = [0, 0, 0]
    for t in triplets:
        if t[0] <= target[0] and t[1] <= target[1] and t[2] <= target[2]:
            best = [max(best[0], t[0]), max(best[1], t[1]), max(best[2], t[2])]
    return best == target
