def valid_word_abbreviation(word: str, abbr: str) -> bool:
    i = j = 0
    n, m = len(word), len(abbr)
    while i < n and j < m:
        c = abbr[j]
        if c.isdigit():
            if c == "0":
                return False
            k = 0
            while j < m and abbr[j].isdigit():
                k = k * 10 + int(abbr[j])
                j += 1
            i += k
        else:
            if word[i] != c:
                return False
            i += 1
            j += 1
    return i == n and j == m
