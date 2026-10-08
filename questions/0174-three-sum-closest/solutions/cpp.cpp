class Solution {
public:
    int threeSumClosest(vector<int>& nums, int target) {
        vector<int> a(nums);
        sort(a.begin(), a.end());
        int n = a.size();
        int best = a[0] + a[1] + a[2];
        for (int i = 0; i < n - 2; i++) {
            int lo = i + 1, hi = n - 1;
            while (lo < hi) {
                int s = a[i] + a[lo] + a[hi];
                if (abs(s - target) < abs(best - target)) best = s;
                if (s < target) lo++;
                else if (s > target) hi--;
                else return s;
            }
        }
        return best;
    }
};
