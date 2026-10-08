class Solution {
public:
    vector<vector<string>> wordSquares(vector<string>& words) {
        size = words[0].size();
        // Every prefix (including the empty one) -> the words starting with it.
        for (const string& word : words) {
            for (size_t i = 0; i <= size; i++) byPrefix[word.substr(0, i)].push_back(word);
        }
        backtrack();
        return result;
    }

private:
    size_t size = 0;
    unordered_map<string, vector<string>> byPrefix;
    vector<vector<string>> result;
    vector<string> square;

    void backtrack() {
        size_t k = square.size();
        if (k == size) {
            result.push_back(square);
            return;
        }
        // Row k must start with column k of the rows placed so far.
        string prefix;
        for (const string& row : square) prefix += row[k];
        auto it = byPrefix.find(prefix);
        if (it == byPrefix.end()) return;
        for (const string& word : it->second) {
            square.push_back(word);
            backtrack();
            square.pop_back();
        }
    }
};
