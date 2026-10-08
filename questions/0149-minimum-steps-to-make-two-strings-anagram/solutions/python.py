def min_steps(s: str, t: str) -> int:
    diff = [0] * 26
    for ch in s:
        diff[ord(ch) - 97] += 1
    for ch in t:
        diff[ord(ch) - 97] -= 1
    return sum(d for d in diff if d > 0)
