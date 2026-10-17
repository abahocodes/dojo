class WordDictionary {
    struct Node {
        Node* next[26] = {};
        bool isEnd = false;
    };
    // Nodes live here so they are freed with the dictionary.
    deque<Node> pool;
    Node* root;

    bool match(Node* node, const string& word, size_t i) {
        if (i == word.size()) return node->isEnd;
        char ch = word[i];
        if (ch != '.') {
            Node* child = node->next[ch - 'a'];
            return child && match(child, word, i + 1);
        }
        // A dot can be any letter: try every child.
        for (Node* child : node->next) {
            if (child && match(child, word, i + 1)) return true;
        }
        return false;
    }

public:
    WordDictionary() {
        pool.emplace_back();
        root = &pool.back();
    }

    void addWord(string word) {
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
        return match(root, word, 0);
    }
};

// dojo calls this to run the tests; you don't need to change it.
class Solution {
public:
    vector<bool> runWordDictionary(vector<string>& ops, vector<string>& words) {
        WordDictionary wd;
        vector<bool> out;
        for (size_t i = 0; i < ops.size(); i++) {
            if (ops[i] == "addWord") wd.addWord(words[i]);
            else out.push_back(wd.search(words[i]));
        }
        return out;
    }
};
