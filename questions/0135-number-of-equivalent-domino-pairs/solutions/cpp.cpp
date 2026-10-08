class Solution {
public:
    int numEquivDominoPairs(vector<vector<int>>& dominoes) {
        int seen[100] = {0};
        int pairs = 0;
        for (auto& d : dominoes) {
            int key = 10 * min(d[0], d[1]) + max(d[0], d[1]);
            pairs += seen[key];
            seen[key]++;
        }
        return pairs;
    }
};
