class Solution {
public:
    vector<int> moveZeroes(vector<int>& nums) {
        int w = 0;
        for (int read = 0; read < (int)nums.size(); read++) {
            if (nums[read] != 0) {
                swap(nums[w], nums[read]);
                w++;
            }
        }
        return nums;
    }
};
