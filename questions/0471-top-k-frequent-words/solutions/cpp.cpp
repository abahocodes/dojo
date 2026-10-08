class Solution {
public:
    vector<string> topKFrequentWords(vector<string>& words, int k) {
        unordered_map<string, int> counts;
        for (const string& w : words) counts[w]++;
        vector<string> ranked;
        for (const auto& [w, c] : counts) ranked.push_back(w);
        sort(ranked.begin(), ranked.end(), [&](const string& a, const string& b) {
            int ca = counts[a], cb = counts[b];
            if (ca != cb) return ca > cb;
            return a < b;
        });
        ranked.resize(k);
        return ranked;
    }
};
