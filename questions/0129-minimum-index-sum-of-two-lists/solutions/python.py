def find_restaurant(list1: list[str], list2: list[str]) -> list[str]:
    index = {s: i for i, s in enumerate(list1)}
    best = None
    result = []
    for j, s in enumerate(list2):
        if s not in index:
            continue
        total = index[s] + j
        if best is None or total < best:
            best = total
            result = [s]
        elif total == best:
            result.append(s)
    return result
