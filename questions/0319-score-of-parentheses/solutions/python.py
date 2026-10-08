def score_of_parentheses(s: str) -> int:
    score = depth = 0
    for i, c in enumerate(s):
        if c == "(":
            depth += 1
        else:
            depth -= 1
            if s[i - 1] == "(":
                score += 1 << depth
    return score
