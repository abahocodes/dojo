def check_valid_string(s: str) -> bool:
    # lo..hi is the range of possible counts of unmatched '(' so far.
    lo = hi = 0
    for ch in s:
        if ch == "(":
            lo += 1
            hi += 1
        elif ch == ")":
            lo -= 1
            hi -= 1
        else:  # '*' can be ')', nothing, or '('
            lo -= 1
            hi += 1
        if hi < 0:
            return False
        lo = max(lo, 0)
    return lo == 0
