def reorganize_string(s: str) -> str:
    counts = [0] * 26
    for ch in s:
        counts[ord(ch) - 97] += 1
    n = len(s)
    if max(counts) > (n + 1) // 2:
        return ""

    result = []
    prev = -1
    for pos in range(n):
        rest = n - pos - 1  # letters left after placing this one
        for c in range(26):
            if counts[c] == 0 or c == prev:
                continue
            counts[c] -= 1
            # The rest can follow c iff no letter needs more than half the
            # remaining slots, and c itself cannot take the very next slot.
            if counts[c] <= rest // 2 and max(counts) <= (rest + 1) // 2:
                result.append(chr(97 + c))
                prev = c
                break
            counts[c] += 1
    return "".join(result)
