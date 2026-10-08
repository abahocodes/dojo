def number_of_substrings(s: str) -> int:
    # last[c] = latest index where letter c was seen (-1 if never).
    last = [-1, -1, -1]
    total = 0
    for i, ch in enumerate(s):
        last[ord(ch) - ord("a")] = i
        # Every start at or before min(last) gives a valid substring ending at i.
        total += min(last) + 1
    return total
