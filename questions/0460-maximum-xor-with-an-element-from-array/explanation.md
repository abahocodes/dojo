# Approach: offline queries + binary trie

**Maximum XOR with a fixed set.** Store every candidate as a 30-bit path in a
binary trie (values are below `2^30`). To maximize `x ^ y`, walk from the top
bit down: if `x` has bit `b`, prefer the child for `1 - b`, because that sets
bit `b` of the result, which outweighs all lower bits together. If that child
doesn't exist, take the other one. After 30 steps you have the best XOR.

**Respecting the limit.** The allowed set for a query is "all numbers `<= m`".
If queries are processed in increasing `m`, that set only grows. So sort
`nums`, sort query indices by `m`, and before answering each query insert the
numbers that have become allowed. An empty trie means the answer is `-1`.
Write each answer back at the query's original index.

```python
def maximize_xor(nums, queries):
    nums = sorted(nums)
    order = sorted(range(len(queries)), key=lambda i: queries[i][1])
    trie = [[0, 0]]                    # trie[node][bit] = child (0 = none)
    answer = [-1] * len(queries)
    j = 0
    for qi in order:
        x, m = queries[qi]
        while j < len(nums) and nums[j] <= m:
            node = 0
            for b in range(29, -1, -1):
                bit = nums[j] >> b & 1
                if not trie[node][bit]:
                    trie[node][bit] = len(trie)
                    trie.append([0, 0])
                node = trie[node][bit]
            j += 1
        if j == 0:
            continue
        node, best = 0, 0
        for b in range(29, -1, -1):
            want = (x >> b & 1) ^ 1
            if trie[node][want]:
                best |= 1 << b
                node = trie[node][want]
            else:
                node = trie[node][want ^ 1]
        answer[qi] = best
    return answer
```

## Complexity

- Time: O(n log n + q log q + 30 · (n + q)) for sorting plus trie work.
- Space: O(30 · n) trie nodes, plus O(q) for the answer and order.

## Pitfalls

- Answers must come back in the original query order: sort indices, not the
  queries themselves.
- Use enough bits: `10^9` needs 30 bits. Starting at a lower bit silently
  drops high bits.
- Duplicates in `nums` are harmless; they just retrace an existing path.
- A per-query scan of all `nums` is O(n · q) = 2.5 · 10^9 at the limits.
- Answering online (without sorting) needs a trie that stores the minimum value
  in each subtree; the offline approach avoids that extra bookkeeping.
