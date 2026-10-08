from bisect import bisect_left


def book_calendar(bookings: list[list[int]]) -> list[bool]:
    # Accepted bookings are disjoint, so sorting by start also sorts the ends.
    starts: list[int] = []
    ends: list[int] = []
    result = []
    for start, end in bookings:
        # Accepted bookings with index < i begin before `end`; the last of them
        # ends latest, so it is the only one that can reach past `start`.
        i = bisect_left(starts, end)
        if i > 0 and ends[i - 1] > start:
            result.append(False)
            continue
        starts.insert(i, start)
        ends.insert(i, end)
        result.append(True)
    return result
