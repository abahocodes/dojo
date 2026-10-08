class Solution {
public:
    vector<int> findDuplicates(vector<int>& nums) {
        vector<int> result;
        for (size_t i = 0; i < nums.size(); i++) {
            int v = abs(nums[i]);
            if (nums[v - 1] < 0) result.push_back(v);
            else nums[v - 1] = -nums[v - 1];
        }
        return result;
    }
};
