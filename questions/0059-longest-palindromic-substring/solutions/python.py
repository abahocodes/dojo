def longest_palindrome(s: str) -> str:
    best_lo, best_len = 0, 1
    for center in range(len(s)):
        for lo, hi in ((center, center), (center, center + 1)):
            while lo >= 0 and hi < len(s) and s[lo] == s[hi]:
                lo -= 1
                hi += 1
            length = hi - lo - 1
            if length > best_len:
                best_lo, best_len = lo + 1, length
    return s[best_lo:best_lo + best_len]
