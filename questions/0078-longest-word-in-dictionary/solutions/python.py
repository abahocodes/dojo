def longest_word(words: list[str]) -> str:
    trie = {}
    for word in words:
        node = trie
        for ch in word:
            node = node.setdefault(ch, {})
        node["$"] = word

    best = ""
    stack = [trie]
    while stack:
        node = stack.pop()
        for ch, child in node.items():
            if ch == "$" or "$" not in child:
                continue  # only walk through prefixes that are words themselves
            word = child["$"]
            if len(word) > len(best) or (len(word) == len(best) and word < best):
                best = word
            stack.append(child)
    return best
