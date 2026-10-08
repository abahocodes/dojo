def replace_words(roots: list[str], sentence: str) -> str:
    trie = {}
    for root in roots:
        node = trie
        for ch in root:
            node = node.setdefault(ch, {})
        node["$"] = True

    def shortest_root(word: str) -> str:
        node = trie
        for i, ch in enumerate(word):
            node = node.get(ch)
            if node is None:
                return word
            if "$" in node:
                return word[: i + 1]
        return word

    return " ".join(shortest_root(word) for word in sentence.split(" "))
