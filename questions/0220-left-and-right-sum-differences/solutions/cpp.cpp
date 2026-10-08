class Solution {
public:
    vector<int> leftRightDifference(vector<int>& nums) {
        int total = 0;
        for (int x : nums) total += x;
        int left = 0;
        vector<int> result;
        result.reserve(nums.size());
        for (int x : nums) {
            int right = total - left - x;
            result.push_back(abs(left - right));
            left += x;
        }
        return result;
    }
};
