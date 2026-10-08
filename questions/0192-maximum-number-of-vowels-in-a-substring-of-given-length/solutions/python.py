def max_vowels(s: str, k: int) -> int:
    vowels = set("aeiou")
    count = sum(1 for c in s[:k] if c in vowels)
    best = count
    for i in range(k, len(s)):
        count += (s[i] in vowels) - (s[i - k] in vowels)
        if count > best:
            best = count
    return best
