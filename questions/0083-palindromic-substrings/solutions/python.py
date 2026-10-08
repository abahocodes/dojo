def count_substrings(s: str) -> int:
    n = len(s)
    count = 0
    # Every palindrome has a center: a character (odd length) or a gap (even length).
    for center in range(2 * n - 1):
        left = center // 2
        right = left + center % 2
        while left >= 0 and right < n and s[left] == s[right]:
            count += 1
            left -= 1
            right += 1
    return count
