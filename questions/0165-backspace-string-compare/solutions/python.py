def backspace_compare(s: str, t: str) -> bool:
    def prev_char(text, i):
        # Index of the next surviving character at or before i, or -1.
        skip = 0
        while i >= 0:
            if text[i] == "#":
                skip += 1
            elif skip > 0:
                skip -= 1
            else:
                return i
            i -= 1
        return -1

    i, j = len(s) - 1, len(t) - 1
    while True:
        i = prev_char(s, i)
        j = prev_char(t, j)
        if i < 0 or j < 0:
            return i < 0 and j < 0
        if s[i] != t[j]:
            return False
        i -= 1
        j -= 1
