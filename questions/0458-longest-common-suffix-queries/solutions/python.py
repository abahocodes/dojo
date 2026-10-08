def string_indices(words_container: list[str], words_query: list[str]) -> list[int]:
    # Trie over reversed container words; best[node] is the preferred word
    # among those whose suffix passes through node.
    children = [{}]
    best = [0]
    for i, w in enumerate(words_container):
        if len(w) < len(words_container[best[0]]):
            best[0] = i
        node = 0
        for c in reversed(w):
            nxt = children[node].get(c)
            if nxt is None:
                nxt = len(children)
                children[node][c] = nxt
                children.append({})
                best.append(i)
            elif len(w) < len(words_container[best[nxt]]):
                best[nxt] = i
            node = nxt
    result = []
    for q in words_query:
        node = 0
        for c in reversed(q):
            nxt = children[node].get(c)
            if nxt is None:
                break
            node = nxt
        result.append(best[node])
    return result
