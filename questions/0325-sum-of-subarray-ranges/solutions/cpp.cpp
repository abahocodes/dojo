class Solution {
public:
    long long subArrayRanges(vector<int>& nums) {
        return total(nums, 1) - total(nums, -1);
    }

private:
    // sign = 1 sums subarray maxima, sign = -1 sums subarray minima.
    long long total(const vector<int>& nums, int sign) {
        int n = nums.size();
        vector<int> stack;
        long long result = 0;
        for (int i = 0; i <= n; i++) {
            while (!stack.empty() &&
                   (i == n || (long long) sign * nums[stack.back()] <= (long long) sign * nums[i])) {
                int j = stack.back();
                stack.pop_back();
                int left = stack.empty() ? -1 : stack.back();
                result += (long long) nums[j] * (j - left) * (i - j);
            }
            stack.push_back(i);
        }
        return result;
    }
};
