import heapq


def alien_order(words: list[str]) -> str:
    letters = sorted({ch for w in words for ch in w})
    after = {ch: set() for ch in letters}
    indegree = {ch: 0 for ch in letters}
    for first, second in zip(words, words[1:]):
        for a, b in zip(first, second):
            if a != b:
                if b not in after[a]:
                    after[a].add(b)
                    indegree[b] += 1
                break
        else:
            if len(first) > len(second):
                return ""  # a longer word sits before its own prefix
    heap = [ch for ch in letters if indegree[ch] == 0]
    heapq.heapify(heap)
    order = []
    while heap:
        ch = heapq.heappop(heap)
        order.append(ch)
        for nxt in after[ch]:
            indegree[nxt] -= 1
            if indegree[nxt] == 0:
                heapq.heappush(heap, nxt)
    return "".join(order) if len(order) == len(letters) else ""
