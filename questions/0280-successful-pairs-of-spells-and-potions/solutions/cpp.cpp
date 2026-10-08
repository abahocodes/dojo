class Solution {
public:
    vector<int> successfulPairs(vector<int>& spells, vector<int>& potions, long long success) {
        vector<int> sorted(potions);
        sort(sorted.begin(), sorted.end());
        int m = sorted.size();
        vector<int> result;
        result.reserve(spells.size());
        for (int s : spells) {
            // Smallest potion strength p with s * p >= success.
            long long need = (success + s - 1) / s;
            int idx = lower_bound(sorted.begin(), sorted.end(), need,
                                  [](int p, long long v) { return p < v; }) - sorted.begin();
            result.push_back(m - idx);
        }
        return result;
    }
};
