def array_rank_transform(arr: list[int]) -> list[int]:
    rank = {}
    for x in sorted(set(arr)):
        rank[x] = len(rank) + 1
    return [rank[x] for x in arr]
