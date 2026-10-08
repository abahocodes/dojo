class Solution {
public:
    vector<bool> streamChecker(vector<string>& words, string& stream) {
        // Trie of reversed words: child[node][c], 0 means "no child".
        vector<array<int, 26>> child(1);
        child[0].fill(0);
        vector<bool> isEnd(1, false);
        int longest = 0;
        for (const string& word : words) {
            longest = max(longest, (int)word.size());
            int node = 0;
            for (int i = (int)word.size() - 1; i >= 0; i--) {
                int c = word[i] - 'a';
                if (child[node][c] == 0) {
                    child[node][c] = (int)child.size();
                    child.emplace_back();
                    child.back().fill(0);
                    isEnd.push_back(false);
                }
                node = child[node][c];
            }
            isEnd[node] = true;
        }

        int n = (int)stream.size();
        vector<bool> result(n, false);
        for (int i = 0; i < n; i++) {
            int node = 0;
            for (int j = i; j >= 0 && j > i - longest; j--) {
                node = child[node][stream[j] - 'a'];
                if (node == 0) break;
                if (isEnd[node]) {
                    result[i] = true;
                    break;
                }
            }
        }
        return result;
    }
};
