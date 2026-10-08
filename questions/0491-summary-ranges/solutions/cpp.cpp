class Solution {
public:
    vector<string> summaryRanges(vector<int>& nums) {
        vector<string> result;
        size_t i = 0;
        while (i < nums.size()) {
            size_t j = i;
            // nums[j] < nums[j + 1] <= 2^31 - 1, so nums[j] + 1 cannot overflow.
            while (j + 1 < nums.size() && nums[j + 1] == nums[j] + 1) j++;
            if (i == j) {
                result.push_back(to_string(nums[i]));
            } else {
                result.push_back(to_string(nums[i]) + "->" + to_string(nums[j]));
            }
            i = j + 1;
        }
        return result;
    }
};
