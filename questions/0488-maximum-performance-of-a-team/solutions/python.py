import heapq

MOD = 10**9 + 7


def max_performance(n: int, speed: list[int], efficiency: list[int], k: int) -> int:
    engineers = sorted(zip(efficiency, speed), reverse=True)
    heap = []  # min-heap of the speeds in the current team
    total_speed = 0
    best = 0
    for eff, spd in engineers:
        # eff is the smallest efficiency seen so far: it is the team minimum.
        heapq.heappush(heap, spd)
        total_speed += spd
        if len(heap) > k:
            total_speed -= heapq.heappop(heap)
        best = max(best, total_speed * eff)
    return best % MOD
