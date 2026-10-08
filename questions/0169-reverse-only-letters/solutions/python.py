def reverse_only_letters(s: str) -> str:
    def is_letter(c):
        return "a" <= c <= "z" or "A" <= c <= "Z"

    chars = list(s)
    lo, hi = 0, len(chars) - 1
    while lo < hi:
        if not is_letter(chars[lo]):
            lo += 1
        elif not is_letter(chars[hi]):
            hi -= 1
        else:
            chars[lo], chars[hi] = chars[hi], chars[lo]
            lo += 1
            hi -= 1
    return "".join(chars)
