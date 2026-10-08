class Solution {
public:
    bool wordBreak(string& s, vector<string>& words) {
        unordered_set<string> vocab(words.begin(), words.end());
        set<int> lengths;
        for (auto& w : words) lengths.insert(w.size());
        int n = s.size();
        vector<bool> ok(n + 1, false);
        ok[0] = true;
        for (int i = 1; i <= n; i++) {
            for (int length : lengths) {
                if (length > i) break;
                if (ok[i - length] && vocab.count(s.substr(i - length, length))) {
                    ok[i] = true;
                    break;
                }
            }
        }
        return ok[n];
    }
};
