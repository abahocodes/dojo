def word_squares(words: list[str]) -> list[list[str]]:
    size = len(words[0])
    # Every prefix (including the empty one) -> the words starting with it.
    by_prefix: dict[str, list[str]] = {}
    for word in words:
        for i in range(size + 1):
            by_prefix.setdefault(word[:i], []).append(word)

    result = []
    square = []

    def backtrack() -> None:
        k = len(square)
        if k == size:
            result.append(square[:])
            return
        # Row k must start with column k of the rows placed so far.
        prefix = ''.join(row[k] for row in square)
        for word in by_prefix.get(prefix, []):
            square.append(word)
            backtrack()
            square.pop()

    backtrack()
    return result
