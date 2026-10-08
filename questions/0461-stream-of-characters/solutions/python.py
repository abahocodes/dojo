def stream_checker(words: list[str], stream: str) -> list[bool]:
    # Trie of reversed words: a suffix of the stream is a word exactly when
    # reading the stream backwards from the newest character reaches a word end.
    children = [{}]
    is_end = [False]
    longest = 0
    for word in words:
        longest = max(longest, len(word))
        node = 0
        for ch in reversed(word):
            nxt = children[node].get(ch)
            if nxt is None:
                nxt = len(children)
                children[node][ch] = nxt
                children.append({})
                is_end.append(False)
            node = nxt
        is_end[node] = True

    result = []
    for i in range(len(stream)):
        node = 0
        found = False
        stop = max(-1, i - longest)
        for j in range(i, stop, -1):
            node = children[node].get(stream[j])
            if node is None:
                break
            if is_end[node]:
                found = True
                break
        result.append(found)
    return result
