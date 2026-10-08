def shortest_unique_prefixes(words: list[str]) -> list[str]:
    children = [{}]
    count = [0]
    for w in words:
        node = 0
        for c in w:
            nxt = children[node].get(c)
            if nxt is None:
                nxt = len(children)
                children[node][c] = nxt
                children.append({})
                count.append(0)
            node = nxt
            count[node] += 1
    result = []
    for w in words:
        node = 0
        length = len(w)
        for i, c in enumerate(w):
            node = children[node][c]
            if count[node] == 1:
                length = i + 1
                break
        result.append(w[:length])
    return result
