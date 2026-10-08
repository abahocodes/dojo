def find_missing_ranges(nums: list[int], lower: int, upper: int) -> list[list[int]]:
    ranges = []
    prev = lower - 1  # last value known to be present (a virtual one before lower)
    for x in nums + [upper + 1]:
        if x - prev >= 2:
            ranges.append([prev + 1, x - 1])
        prev = x
    return ranges
