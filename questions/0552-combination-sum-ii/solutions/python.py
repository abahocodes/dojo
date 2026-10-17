def combination_sum2(candidates: list[int], target: int) -> list[list[int]]:
    candidates = sorted(candidates)  # equal values become neighbours
    result, current = [], []

    def backtrack(start, remaining):
        if remaining == 0:
            result.append(current[:])
            return
        for i in range(start, len(candidates)):
            c = candidates[i]
            if c > remaining:
                break  # sorted, so every later candidate is too big as well
            if i > start and c == candidates[i - 1]:
                continue  # this value was already tried in this position
            current.append(c)
            backtrack(i + 1, remaining - c)  # i + 1: each card is used once
            current.pop()

    backtrack(0, target)
    return result
