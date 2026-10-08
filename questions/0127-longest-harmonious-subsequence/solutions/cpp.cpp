class Solution {
public:
    int findLhs(vector<int>& nums) {
        unordered_map<long long, int> count;
        for (int x : nums) count[x]++;
        int best = 0;
        for (auto& [x, c] : count) {
            auto it = count.find(x + 1);
            if (it != count.end()) best = max(best, c + it->second);
        }
        return best;
    }
};
