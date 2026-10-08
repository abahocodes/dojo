def is_prefix_of_word(sentence: str, search_word: str) -> int:
    for position, word in enumerate(sentence.split(" "), start=1):
        if word.startswith(search_word):
            return position
    return -1
