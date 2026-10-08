def max_score_split(s: str) -> int:
    score = s.count("1")
    best = 0
    for c in s[:-1]:
        score += 1 if c == "0" else -1
        best = max(best, score)
    return best
