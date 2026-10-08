# Approach: BFS over words, generating neighbours letter by letter

Words are nodes; two words are adjacent when they differ in exactly one
letter. The shortest ladder is the shortest path in this unweighted graph, so
BFS finds it, with the depth counting words rather than steps.

Building the whole graph up front means comparing every pair of words, which
is O(N² · L). Instead, generate the candidates on the fly: for a word of
length L there are only 25 · L one-letter variants, and a hash set tells us in
O(L) which of them are real words. Deleting a word from the set when it is
enqueued doubles as the visited marker.

```python
from collections import deque

def ladder_length(begin_word, end_word, word_list):
    words = set(word_list)
    if end_word not in words:
        return 0
    letters = "abcdefghijklmnopqrstuvwxyz"
    words.discard(begin_word)
    queue = deque([(begin_word, 1)])
    while queue:
        word, steps = queue.popleft()
        for i in range(len(word)):
            prefix, suffix = word[:i], word[i + 1:]
            for ch in letters:
                candidate = prefix + ch + suffix
                if candidate in words:
                    if candidate == end_word:
                        return steps + 1
                    words.remove(candidate)
                    queue.append((candidate, steps + 1))
    return 0
```

**Faster in practice:** bidirectional BFS grows one frontier from each end and
always expands the smaller one, stopping when they meet. Same worst case,
but it explores far fewer words on large dictionaries.

## Complexity

- Time: O(N · L² · 26). Each of the N words is expanded once, producing
  26 · L candidates that each cost O(L) to build and hash.
- Space: O(N · L) for the word set and the queue.

## Pitfalls

- Returning the number of *changes* instead of the number of *words*: a ladder
  of one step has length `2`.
- Forgetting the early exit when `end_word` is not in the list.
- Marking words visited when they are dequeued instead of when they are
  enqueued. The same word can then sit in the queue many times and the search
  slows down dramatically.
- Using DFS. It finds *a* ladder, not the shortest one.
