from collections import deque


def shortest_subarray(nums: list[int], k: int) -> int:
    prefix = [0]
    for x in nums:
        prefix.append(prefix[-1] + x)
    best = len(nums) + 1
    starts = deque()  # indices into prefix, prefix values strictly increasing
    for j, p in enumerate(prefix):
        while starts and p - prefix[starts[0]] >= k:
            best = min(best, j - starts.popleft())
        while starts and prefix[starts[-1]] >= p:
            starts.pop()
        starts.append(j)
    return best if best <= len(nums) else -1
