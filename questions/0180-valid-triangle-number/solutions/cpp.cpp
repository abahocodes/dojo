class Solution {
public:
    int triangleNumber(vector<int>& nums) {
        vector<int> a(nums);
        sort(a.begin(), a.end());
        int count = 0;
        for (int k = (int)a.size() - 1; k >= 2; k--) {
            int i = 0, j = k - 1;
            while (i < j) {
                if (a[i] + a[j] > a[k]) {
                    count += j - i;
                    j--;
                } else {
                    i++;
                }
            }
        }
        return count;
    }
};
