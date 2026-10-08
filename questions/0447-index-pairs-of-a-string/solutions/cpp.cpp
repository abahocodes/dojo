class Solution {
public:
    vector<vector<int>> indexPairs(string& text, vector<string>& words) {
        // Trie with 26-way child arrays; end[node] marks the end of a word.
        vector<array<int, 26>> next(1);
        next[0].fill(0);
        vector<bool> end(1, false);
        for (const string& word : words) {
            int node = 0;
            for (char ch : word) {
                int c = ch - 'a';
                if (next[node][c] == 0) {
                    next[node][c] = next.size();
                    next.emplace_back();
                    next.back().fill(0);
                    end.push_back(false);
                }
                node = next[node][c];
            }
            end[node] = true;
        }

        vector<vector<int>> result;
        int n = text.size();
        for (int i = 0; i < n; i++) {
            int node = 0;
            for (int j = i; j < n; j++) {
                node = next[node][text[j] - 'a'];
                if (node == 0) break;
                if (end[node]) result.push_back({i, j});
            }
        }
        return result;
    }
};
