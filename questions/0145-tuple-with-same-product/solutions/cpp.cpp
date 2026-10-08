class Solution {
public:
    int tupleSameProduct(vector<int>& nums) {
        unordered_map<int, int> seen;
        seen.reserve(nums.size() * nums.size());
        int total = 0;
        for (size_t i = 0; i < nums.size(); i++) {
            for (size_t j = i + 1; j < nums.size(); j++) {
                total += 8 * seen[nums[i] * nums[j]]++;
            }
        }
        return total;
    }
};
