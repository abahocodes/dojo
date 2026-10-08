def common_chars(words: list[str]) -> list[str]:
    common = [float("inf")] * 26
    for w in words:
        freq = [0] * 26
        for c in w:
            freq[ord(c) - 97] += 1
        common = [min(a, b) for a, b in zip(common, freq)]
    return [chr(97 + i) for i in range(26) for _ in range(common[i])]
