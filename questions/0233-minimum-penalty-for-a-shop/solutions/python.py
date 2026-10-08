def best_closing_time(customers: str) -> int:
    delta = 0
    best_delta = 0
    best_hour = 0
    for i, c in enumerate(customers):
        delta += -1 if c == "Y" else 1
        if delta < best_delta:
            best_delta = delta
            best_hour = i + 1
    return best_hour
