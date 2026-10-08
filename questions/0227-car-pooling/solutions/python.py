def car_pooling(trips: list[list[int]], capacity: int) -> bool:
    # change[x] = passengers boarding at km x minus passengers leaving at km x
    change = [0] * 1001
    for passengers, start, end in trips:
        change[start] += passengers
        change[end] -= passengers
    load = 0
    for delta in change:
        load += delta
        if load > capacity:
            return False
    return True
