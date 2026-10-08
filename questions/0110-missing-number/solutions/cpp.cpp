class Solution {
public:
    int missingNumber(vector<int>& nums) {
        int result = (int)nums.size();
        for (int i = 0; i < (int)nums.size(); i++) {
            result ^= i ^ nums[i];
        }
        return result;
    }
};
