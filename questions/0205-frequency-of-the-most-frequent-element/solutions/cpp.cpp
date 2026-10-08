class Solution {
public:
    int maxFrequency(vector<int>& nums, int k) {
        vector<int> a(nums);
        sort(a.begin(), a.end());
        int n = a.size();
        int left = 0;
        long long window = 0;
        int best = 0;
        for (int right = 0; right < n; right++) {
            window += a[right];
            while ((long long)a[right] * (right - left + 1) - window > k) window -= a[left++];
            best = max(best, right - left + 1);
        }
        return best;
    }
};
