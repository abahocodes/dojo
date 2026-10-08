def cycle_length_queries(n: int, queries: list[list[int]]) -> list[int]:
    answer = []
    for a, b in queries:
        steps = 0
        while a != b:
            # the larger label is never shallower, so lift it toward the root
            if a > b:
                a >>= 1
            else:
                b >>= 1
            steps += 1
        answer.append(steps + 1)
    return answer
