class Solution {
public:
    vector<int> removeDuplicatesKeepTwo(vector<int>& nums) {
        vector<int> a(nums);
        size_t k = 0;
        for (size_t i = 0; i < a.size(); i++) {
            int x = a[i];
            if (k < 2 || a[k - 2] != x) {
                a[k++] = x;
            }
        }
        a.resize(k);
        return a;
    }
};
