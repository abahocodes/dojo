def sum_prefix_scores(words: list[str]) -> list[int]:
    # Trie where every node counts how many words pass through it.
    children = [{}]
    count = [0]
    for word in words:
        node = 0
        for ch in word:
            nxt = children[node].get(ch)
            if nxt is None:
                nxt = len(children)
                children[node][ch] = nxt
                children.append({})
                count.append(0)
            node = nxt
            count[node] += 1

    result = []
    for word in words:
        node = 0
        total = 0
        for ch in word:
            node = children[node][ch]
            total += count[node]
        result.append(total)
    return result
