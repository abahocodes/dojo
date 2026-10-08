def merge_alternately(word1: str, word2: str) -> str:
    out = []
    i = 0
    while i < len(word1) or i < len(word2):
        if i < len(word1):
            out.append(word1[i])
        if i < len(word2):
            out.append(word2[i])
        i += 1
    return "".join(out)
