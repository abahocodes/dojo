def corp_flight_bookings(bookings: list[list[int]], n: int) -> list[int]:
    diff = [0] * (n + 1)
    for first, last, seats in bookings:
        diff[first - 1] += seats
        diff[last] -= seats
    totals = []
    running = 0
    for i in range(n):
        running += diff[i]
        totals.append(running)
    return totals
