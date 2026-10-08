def insert_interval(intervals: list[list[int]], new_interval: list[int]) -> list[list[int]]:
    result = []
    start, end = new_interval
    i, n = 0, len(intervals)
    # Intervals that end strictly before the new one starts.
    while i < n and intervals[i][1] < start:
        result.append(intervals[i])
        i += 1
    # Intervals that overlap or touch the new one: absorb them.
    while i < n and intervals[i][0] <= end:
        start = min(start, intervals[i][0])
        end = max(end, intervals[i][1])
        i += 1
    result.append([start, end])
    # Intervals that start strictly after the merged one ends.
    result.extend(intervals[i:])
    return result
