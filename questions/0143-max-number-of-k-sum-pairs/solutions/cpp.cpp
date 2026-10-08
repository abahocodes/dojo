class Solution {
public:
    int maxOperations(vector<int>& nums, int k) {
        unordered_map<int, int> waiting;
        int ops = 0;
        for (int x : nums) {
            auto it = waiting.find(k - x);
            if (it != waiting.end() && it->second > 0) {
                it->second--;
                ops++;
            } else {
                waiting[x]++;
            }
        }
        return ops;
    }
};
