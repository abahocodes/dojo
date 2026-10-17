// 1007. Implement Trie (Prefix Tree) (medium)
// /show to reread the problem · /test runs visible tests · /submit when done

class Trie {
    constructor() {

    }

    insert(word: string): void {

    }

    search(word: string): boolean {
        return false;
    }

    startsWith(prefix: string): boolean {
        return false;
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
