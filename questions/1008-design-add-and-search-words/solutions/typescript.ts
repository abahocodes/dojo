class WordNode {
    children: Map<string, WordNode> = new Map();
    isEnd = false;
}

class WordDictionary {
    private root = new WordNode();

    addWord(word: string): void {
        let node = this.root;
        for (const ch of word) {
            let next = node.children.get(ch);
            if (!next) {
                next = new WordNode();
                node.children.set(ch, next);
            }
            node = next;
        }
        node.isEnd = true;
    }

    search(word: string): boolean {
        const match = (node: WordNode, i: number): boolean => {
            if (i === word.length) return node.isEnd;
            const ch = word[i];
            if (ch !== ".") {
                const child = node.children.get(ch);
                return child !== undefined && match(child, i + 1);
            }
            // A dot can be any letter: try every child.
            for (const child of node.children.values()) {
                if (match(child, i + 1)) return true;
            }
            return false;
        };
        return match(this.root, 0);
    }
}

// dojo calls this to run the tests; you don't need to change it.
function runWordDictionary(ops: string[], words: string[]): boolean[] {
    const wd = new WordDictionary();
    const out: boolean[] = [];
    for (let i = 0; i < ops.length; i++) {
        if (ops[i] === "addWord") wd.addWord(words[i]);
        else out.push(wd.search(words[i]));
    }
    return out;
}
