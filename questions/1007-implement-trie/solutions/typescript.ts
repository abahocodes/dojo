class TrieNode {
    children: Map<string, TrieNode> = new Map();
    isEnd = false;
}

class Trie {
    private root = new TrieNode();

    insert(word: string): void {
        let node = this.root;
        for (const ch of word) {
            let next = node.children.get(ch);
            if (!next) {
                next = new TrieNode();
                node.children.set(ch, next);
            }
            node = next;
        }
        node.isEnd = true;
    }

    // The node spelled by s, or null if the path leaves the trie.
    private walk(s: string): TrieNode | null {
        let node: TrieNode | undefined = this.root;
        for (const ch of s) {
            node = node.children.get(ch);
            if (!node) return null;
        }
        return node;
    }

    search(word: string): boolean {
        const node = this.walk(word);
        return node !== null && node.isEnd;
    }

    startsWith(prefix: string): boolean {
        return this.walk(prefix) !== null;
    }
}

// dojo calls this to run the tests; you don't need to change it.
function runTrie(ops: string[], words: string[]): boolean[] {
    const trie = new Trie();
    const out: boolean[] = [];
    for (let i = 0; i < ops.length; i++) {
        if (ops[i] === "insert") trie.insert(words[i]);
        else if (ops[i] === "search") out.push(trie.search(words[i]));
        else out.push(trie.startsWith(words[i]));
    }
    return out;
}
