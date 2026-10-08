def num_jewels_in_stones(jewels: str, stones: str) -> int:
    kinds = set(jewels)
    return sum(1 for c in stones if c in kinds)
