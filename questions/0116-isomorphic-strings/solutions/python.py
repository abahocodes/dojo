def is_isomorphic(s: str, t: str) -> bool:
    forward = {}
    backward = {}
    for a, b in zip(s, t):
        if forward.setdefault(a, b) != b or backward.setdefault(b, a) != a:
            return False
    return True
