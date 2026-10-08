def orderly_queue(s: str, k: int) -> str:
    if k == 1:
        doubled = s + s
        return min(doubled[i:i + len(s)] for i in range(len(s)))
    return "".join(sorted(s))
