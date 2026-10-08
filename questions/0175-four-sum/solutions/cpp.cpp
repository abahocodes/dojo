class Solution {
public:
    vector<vector<int>> fourSum(vector<int>& nums, int target) {
        vector<int> a(nums);
        sort(a.begin(), a.end());
        int n = a.size();
        vector<vector<int>> res;
        for (int i = 0; i < n - 3; i++) {
            if (i > 0 && a[i] == a[i - 1]) continue;
            for (int j = i + 1; j < n - 2; j++) {
                if (j > i + 1 && a[j] == a[j - 1]) continue;
                int lo = j + 1, hi = n - 1;
                while (lo < hi) {
                    long long s = (long long)a[i] + a[j] + a[lo] + a[hi];
                    if (s < target) lo++;
                    else if (s > target) hi--;
                    else {
                        res.push_back({a[i], a[j], a[lo], a[hi]});
                        lo++;
                        while (lo < hi && a[lo] == a[lo - 1]) lo++;
                        hi--;
                        while (lo < hi && a[hi] == a[hi + 1]) hi--;
                    }
                }
            }
        }
        return res;
    }
};
