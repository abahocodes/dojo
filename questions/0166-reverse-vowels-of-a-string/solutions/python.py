def reverse_vowels(s: str) -> str:
    vowels = set("aeiouAEIOU")
    chars = list(s)
    lo, hi = 0, len(chars) - 1
    while lo < hi:
        if chars[lo] not in vowels:
            lo += 1
        elif chars[hi] not in vowels:
            hi -= 1
        else:
            chars[lo], chars[hi] = chars[hi], chars[lo]
            lo += 1
            hi -= 1
    return "".join(chars)
