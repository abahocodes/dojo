# Approach: build letter rules, then smallest-first topological sort

**Step 1: extract rules.** A sorted list is sorted exactly when every pair of
adjacent words is in order, so only adjacent pairs matter. For a pair
`first, second`, find the first position where they differ. If the letters
there are `a` and `b`, the alien alphabet has `a` before `b`; the positions
after it say nothing. If there's no differing position, the shorter word must
come first, so `first` longer than `second` makes the input impossible.

**Step 2: order the letters.** The rules form a directed graph on letters,
and the valid orderings are exactly its topological orders. To get the one
that comes first in normal `a`–`z` order, run Kahn's algorithm with a min-heap:
at each step output the smallest letter whose rules are all satisfied. Picking
the smallest available letter at every position gives the alphabetically first
result, since any other choice would put a larger letter at that position.

**Step 3: detect cycles.** If the heap empties before every letter is
output, the remaining letters wait on each other in a cycle. Return `""`.

```python
import heapq

def alien_order(words):
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
        else:                       # no differing position
            if len(first) > len(second):
                return ""
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
```

## Complexity

- Time: O(C + U log U), where C is the total number of characters in `words`
  and U ≤ 26 is the number of distinct letters. Effectively O(C).
- Space: O(U²) for the rules, at most 26 × 25.

## Pitfalls

- Include **every** letter that appears, even ones no rule mentions (like
  `f` in Example 1). They still need a place in the answer.
- Only the **first** differing position of a pair gives a rule. Comparing
  later letters invents rules that don't exist.
- `["abcd", "ab"]` is invalid even though it produces no rule. Check the
  prefix case.
- Don't count a repeated rule twice in the in-degrees, or the letter's
  in-degree never reaches zero.
- A plain queue or DFS gives *a* valid order, but not necessarily the
  alphabetically first one this problem asks for.
