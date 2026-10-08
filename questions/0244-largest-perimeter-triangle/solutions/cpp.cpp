class Solution {
public:
    int largestPerimeter(vector<int>& nums) {
        vector<int> a(nums);
        sort(a.begin(), a.end());
        for (int i = (int)a.size() - 1; i >= 2; i--) {
            if (a[i - 2] + a[i - 1] > a[i]) return a[i - 2] + a[i - 1] + a[i];
        }
        return 0;
    }
};
