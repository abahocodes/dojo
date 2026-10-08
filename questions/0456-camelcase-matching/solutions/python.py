def camel_match(queries: list[str], pattern: str) -> list[bool]:
    def matches(query: str) -> bool:
        j = 0
        for c in query:
            if j < len(pattern) and c == pattern[j]:
                j += 1
            elif "A" <= c <= "Z":
                return False
        return j == len(pattern)

    return [matches(q) for q in queries]
