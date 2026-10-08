def find_original_array(changed: list[int]) -> list[int]:
    if len(changed) % 2:
        return []
    top = max(changed)
    count = [0] * (top + 1)
    for v in changed:
        count[v] += 1
    if count[0] % 2:
        return []
    original = [0] * (count[0] // 2)
    for x in range(1, top + 1):
        c = count[x]
        if c == 0:
            continue
        if 2 * x > top or count[2 * x] < c:
            return []
        count[2 * x] -= c
        original.extend([x] * c)
    return original
