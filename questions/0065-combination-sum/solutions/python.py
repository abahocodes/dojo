def combination_sum(candidates: list[int], target: int) -> list[list[int]]:
    candidates = sorted(candidates)
    result, current = [], []

    def backtrack(start, remaining):
        if remaining == 0:
            result.append(current[:])
            return
        for i in range(start, len(candidates)):
            c = candidates[i]
            if c > remaining:
                break  # sorted, so every later candidate is too big as well
            current.append(c)
            backtrack(i, remaining - c)  # i, not i + 1: a candidate may be reused
            current.pop()

    backtrack(0, target)
    return result
