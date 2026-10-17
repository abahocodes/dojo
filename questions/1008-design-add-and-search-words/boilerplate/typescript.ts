// 1008. Design Add and Search Words Data Structure (medium)
// /show to reread the problem · /test runs visible tests · /submit when done

class WordDictionary {
    constructor() {

    }

    addWord(word: string): void {

    }

    search(word: string): boolean {
        return false;
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
