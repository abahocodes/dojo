class TrieNode:
    def __init__(self):
        self.children = {}
        self.is_end = False


class Trie:
    def __init__(self):
        self.root = TrieNode()

    def insert(self, word: str) -> None:
        node = self.root
        for ch in word:
            node = node.children.setdefault(ch, TrieNode())
        node.is_end = True

    def _walk(self, s):
        # The node spelled by s, or None if the path leaves the trie.
        node = self.root
        for ch in s:
            node = node.children.get(ch)
            if node is None:
                return None
        return node

    def search(self, word: str) -> bool:
        node = self._walk(word)
        return node is not None and node.is_end

    def startsWith(self, prefix: str) -> bool:
        return self._walk(prefix) is not None


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
