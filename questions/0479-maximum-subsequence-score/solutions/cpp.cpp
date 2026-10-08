class Solution {
public:
    long long maxScore(vector<int>& nums1, vector<int>& nums2, int k) {
        int n = nums1.size();
        vector<int> order(n);
        iota(order.begin(), order.end(), 0);
        sort(order.begin(), order.end(), [&](int a, int b) { return nums2[a] > nums2[b]; });
        priority_queue<int, vector<int>, greater<int>> chosen;
        long long total = 0, best = 0;
        for (int i : order) {
            chosen.push(nums1[i]);
            total += nums1[i];
            if ((int)chosen.size() > k) {
                total -= chosen.top();
                chosen.pop();
            }
            if ((int)chosen.size() == k) best = max(best, total * nums2[i]);
        }
        return best;
    }
};
