def group_strings(strings: list[str]) -> list[list[str]]:
    groups = {}
    for s in strings:
        key = "".join(chr((ord(c) - ord(s[0])) % 26 + ord("a")) for c in s)
        groups.setdefault(key, []).append(s)
    return list(groups.values())
