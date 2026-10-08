class Solution {
public:
    vector<int> searchRange(vector<int>& nums, int target) {
        int first = (int)(lower_bound(nums.begin(), nums.end(), target) - nums.begin());
        if (first == (int)nums.size() || nums[first] != target) {
            return {-1, -1};
        }
        int last = (int)(upper_bound(nums.begin(), nums.end(), target) - nums.begin()) - 1;
        return {first, last};
    }
};
