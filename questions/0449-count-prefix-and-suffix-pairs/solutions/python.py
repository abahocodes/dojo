def count_prefix_suffix_pairs(words: list[str]) -> int:
    count = 0
    for j in range(len(words)):
        for i in range(j):
            if words[j].startswith(words[i]) and words[j].endswith(words[i]):
                count += 1
    return count
