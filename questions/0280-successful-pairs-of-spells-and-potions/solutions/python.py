from bisect import bisect_left


def successful_pairs(spells: list[int], potions: list[int], success: int) -> list[int]:
    sorted_potions = sorted(potions)
    m = len(sorted_potions)
    result = []
    for s in spells:
        # Smallest potion strength p with s * p >= success.
        need = (success + s - 1) // s
        result.append(m - bisect_left(sorted_potions, need))
    return result
