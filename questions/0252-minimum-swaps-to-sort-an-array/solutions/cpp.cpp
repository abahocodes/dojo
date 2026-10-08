class Solution {
public:
    int minSwapsToSort(vector<int>& nums) {
        int n = nums.size();
        // order[k] = index of the k-th smallest value
        vector<int> order(n);
        iota(order.begin(), order.end(), 0);
        sort(order.begin(), order.end(), [&](int a, int b) { return nums[a] < nums[b]; });
        vector<bool> seen(n, false);
        int swaps = 0;
        for (int i = 0; i < n; i++) {
            int length = 0;
            for (int j = i; !seen[j]; j = order[j]) {
                seen[j] = true;
                length++;
            }
            if (length > 0) swaps += length - 1;
        }
        return swaps;
    }
};
