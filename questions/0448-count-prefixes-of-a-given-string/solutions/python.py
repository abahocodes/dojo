def count_prefixes(words: list[str], s: str) -> int:
    return sum(1 for word in words if s.startswith(word))
