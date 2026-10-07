def top_k_frequent(nums: list[int], k: int) -> list[int]:
    counts = {}
    for x in nums:
        counts[x] = counts.get(x, 0) + 1

    # buckets[f] holds every value that occurs exactly f times
    buckets = [[] for _ in range(len(nums) + 1)]
    for value, freq in counts.items():
        buckets[freq].append(value)

    result = []
    for freq in range(len(nums), 0, -1):
        for value in buckets[freq]:
            result.append(value)
            if len(result) == k:
                return result
    return result
