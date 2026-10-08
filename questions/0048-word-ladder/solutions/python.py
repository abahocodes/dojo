from collections import deque


def ladder_length(begin_word: str, end_word: str, word_list: list[str]) -> int:
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
                    words.remove(candidate)  # visited: never enqueue twice
                    queue.append((candidate, steps + 1))
    return 0
