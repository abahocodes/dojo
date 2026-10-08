def word_pattern(pattern: str, s: str) -> bool:
    words = s.split(' ')
    if len(words) != len(pattern):
        return False
    letter_to_word = {}
    word_to_letter = {}
    for c, w in zip(pattern, words):
        if letter_to_word.setdefault(c, w) != w or word_to_letter.setdefault(w, c) != c:
            return False
    return True
