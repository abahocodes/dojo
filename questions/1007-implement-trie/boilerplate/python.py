# 1007. Implement Trie (Prefix Tree) (medium)
# /show to reread the problem · /test runs visible tests · /submit when done

class Trie:
    def __init__(self):
        pass

    def insert(self, word: str) -> None:
        pass

    def search(self, word: str) -> bool:
        return False

    def startsWith(self, prefix: str) -> bool:
        return False


# dojo calls this to run the tests; you don't need to change it.
def run_trie(ops: list[str], words: list[str]) -> list[bool]:
    trie = Trie()
    out = []
    for op, word in zip(ops, words):
        if op == "insert":
            trie.insert(word)
        elif op == "search":
            out.append(trie.search(word))
        else:
            out.append(trie.startsWith(word))
    return out
