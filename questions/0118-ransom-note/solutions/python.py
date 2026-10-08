def can_construct(ransom_note: str, magazine: str) -> bool:
    if len(ransom_note) > len(magazine):
        return False
    counts = [0] * 26
    for ch in magazine:
        counts[ord(ch) - 97] += 1
    for ch in ransom_note:
        i = ord(ch) - 97
        counts[i] -= 1
        if counts[i] < 0:
            return False
    return True
