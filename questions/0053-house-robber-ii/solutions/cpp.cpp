class Solution {
public:
    int robCircular(vector<int>& nums) {
        int n = nums.size();
        if (n == 1) return nums[0];
        return max(line(nums, 0, n - 1), line(nums, 1, n));
    }

private:
    int line(const vector<int>& nums, int lo, int hi) {
        int prev = 0, curr = 0;
        for (int i = lo; i < hi; i++) {
            int next = max(curr, prev + nums[i]);
            prev = curr;
            curr = next;
        }
        return curr;
    }
};
