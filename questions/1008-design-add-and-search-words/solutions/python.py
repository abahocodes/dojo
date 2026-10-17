class WordDictionary:
    def __init__(self):
        self.root = {}  # letter -> child dict; "$" marks a word end

    def addWord(self, word: str) -> None:
        node = self.root
        for ch in word:
            node = node.setdefault(ch, {})
        node["$"] = True

    def search(self, word: str) -> bool:
        def match(node, i):
            if i == len(word):
                return "$" in node
            ch = word[i]
            if ch != ".":
                child = node.get(ch)
                return child is not None and match(child, i + 1)
            # A dot can be any letter: try every child.
            return any(k != "$" and match(child, i + 1) for k, child in node.items())

        return match(self.root, 0)


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
