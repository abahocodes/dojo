# 1008. Design Add and Search Words Data Structure (medium)
# /show to reread the problem · /test runs visible tests · /submit when done

class WordDictionary:
    def __init__(self):
        pass

    def addWord(self, word: str) -> None:
        pass

    def search(self, word: str) -> bool:
        return False


# dojo calls this to run the tests; you don't need to change it.
def run_word_dictionary(ops: list[str], words: list[str]) -> list[bool]:
    wd = WordDictionary()
    out = []
    for op, word in zip(ops, words):
        if op == "addWord":
            wd.addWord(word)
        else:
            out.append(wd.search(word))
    return out
