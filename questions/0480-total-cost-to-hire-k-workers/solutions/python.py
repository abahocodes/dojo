import heapq


def total_cost(costs: list[int], k: int, candidates: int) -> int:
    left, right = [], []  # (cost, index)
    i, j = 0, len(costs) - 1
    total = 0
    for _ in range(k):
        while len(left) < candidates and i <= j:
            heapq.heappush(left, (costs[i], i))
            i += 1
        while len(right) < candidates and i <= j:
            heapq.heappush(right, (costs[j], j))
            j -= 1
        if not right or (left and left[0] <= right[0]):
            total += heapq.heappop(left)[0]
        else:
            total += heapq.heappop(right)[0]
    return total
