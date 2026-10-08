class Solution {
public:
    vector<int> stringIndices(vector<string>& wordsContainer, vector<string>& wordsQuery) {
        size_t total = 0;
        for (const string& w : wordsContainer) total += w.size();
        vector<int> children((total + 1) * 26, 0);
        vector<int> best(total + 1, 0);
        int nodes = 1;
        for (int i = 0; i < (int)wordsContainer.size(); i++) {
            const string& w = wordsContainer[i];
            if (w.size() < wordsContainer[best[0]].size()) best[0] = i;
            int node = 0;
            for (int k = (int)w.size() - 1; k >= 0; k--) {
                size_t slot = (size_t)node * 26 + (w[k] - 'a');
                if (children[slot] == 0) {
                    children[slot] = nodes;
                    best[nodes] = i;
                    nodes++;
                } else if (w.size() < wordsContainer[best[children[slot]]].size()) {
                    best[children[slot]] = i;
                }
                node = children[slot];
            }
        }
        vector<int> result;
        result.reserve(wordsQuery.size());
        for (const string& q : wordsQuery) {
            int node = 0;
            for (int k = (int)q.size() - 1; k >= 0; k--) {
                int next = children[(size_t)node * 26 + (q[k] - 'a')];
                if (next == 0) break;
                node = next;
            }
            result.push_back(best[node]);
        }
        return result;
    }
};
