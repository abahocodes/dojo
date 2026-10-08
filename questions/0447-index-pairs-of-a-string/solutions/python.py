def index_pairs(text: str, words: list[str]) -> list[list[int]]:
    # Trie as a list of child dicts; ends[node] marks the end of a word.
    children = [{}]
    ends = [False]
    for word in words:
        node = 0
        for ch in word:
            nxt = children[node].get(ch)
            if nxt is None:
                nxt = len(children)
                children[node][ch] = nxt
                children.append({})
                ends.append(False)
            node = nxt
        ends[node] = True

    result = []
    for i in range(len(text)):
        node = 0
        for j in range(i, len(text)):
            node = children[node].get(text[j])
            if node is None:
                break
            if ends[node]:
                result.append([i, j])
    return result
