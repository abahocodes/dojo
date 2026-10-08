class Solution {
public:
    bool isIdealPermutation(vector<int>& nums) {
        int best = -1; // max of nums[0..j-2]
        for (size_t j = 2; j < nums.size(); j++) {
            best = max(best, nums[j - 2]);
            if (best > nums[j]) return false;
        }
        return true;
    }
};
