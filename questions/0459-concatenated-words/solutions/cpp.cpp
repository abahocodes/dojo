class Solution {
public:
    vector<string> findAllConcatenatedWords(vector<string>& words) {
        vector<int> order(words.size());
        iota(order.begin(), order.end(), 0);
        stable_sort(order.begin(), order.end(),
                    [&](int a, int b) { return words[a].size() < words[b].size(); });
        unordered_set<string> known;
        vector<bool> found(words.size(), false);
        for (int i : order) {
            const string& w = words[i];
            if (!known.empty()) {
                int n = (int)w.size();
                vector<bool> can(n + 1, false);
                can[0] = true;
                for (int end = 1; end <= n; end++) {
                    for (int start = 0; start < end; start++) {
                        if (can[start] && known.count(w.substr(start, end - start))) {
                            can[end] = true;
                            break;
                        }
                    }
                }
                found[i] = can[n];
            }
            known.insert(w);
        }
        vector<string> result;
        for (size_t i = 0; i < words.size(); i++) if (found[i]) result.push_back(words[i]);
        return result;
    }
};
