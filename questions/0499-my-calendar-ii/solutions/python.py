def book_calendar_double(bookings: list[list[int]]) -> list[bool]:
    booked: list[tuple[int, int]] = []    # every accepted booking
    overlaps: list[tuple[int, int]] = []  # stretches already covered twice
    result = []
    for start, end in bookings:
        if any(max(start, s) < min(end, e) for s, e in overlaps):
            result.append(False)
            continue
        for s, e in booked:
            lo, hi = max(start, s), min(end, e)
            if lo < hi:
                overlaps.append((lo, hi))
        booked.append((start, end))
        result.append(True)
    return result
