class Trie {
    struct Node {
        Node* next[26] = {};
        bool isEnd = false;
    };
    // Nodes live here so they are freed with the trie.
    deque<Node> pool;
    Node* root;

    // The node spelled by s, or nullptr if the path leaves the trie.
    Node* walk(const string& s) {
        Node* node = root;
        for (char ch : s) {
            node = node->next[ch - 'a'];
            if (!node) return nullptr;
        }
        return node;
    }

public:
    Trie() {
        pool.emplace_back();
        root = &pool.back();
    }

    void insert(string word) {
        Node* node = root;
        for (char ch : word) {
            int c = ch - 'a';
            if (!node->next[c]) {
                pool.emplace_back();
                node->next[c] = &pool.back();
            }
            node = node->next[c];
        }
        node->isEnd = true;
    }

    bool search(string word) {
        Node* node = walk(word);
        return node && node->isEnd;
    }

    bool startsWith(string prefix) {
        return walk(prefix) != nullptr;
    }
};

// dojo calls this to run the tests; you don't need to change it.
class Solution {
public:
    vector<bool> runTrie(vector<string>& ops, vector<string>& words) {
        Trie trie;
        vector<bool> out;
        for (size_t i = 0; i < ops.size(); i++) {
            if (ops[i] == "insert") trie.insert(words[i]);
            else if (ops[i] == "search") out.push_back(trie.search(words[i]));
            else out.push_back(trie.startsWith(words[i]));
        }
        return out;
    }
};
