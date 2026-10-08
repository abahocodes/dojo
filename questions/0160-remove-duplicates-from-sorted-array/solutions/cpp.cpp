class Solution {
public:
    vector<int> removeDuplicates(vector<int>& nums) {
        int write = 1;
        for (int read = 1; read < (int) nums.size(); read++) {
            if (nums[read] != nums[write - 1]) {
                nums[write++] = nums[read];
            }
        }
        nums.resize(write);
        return nums;
    }
};
