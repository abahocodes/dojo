class Solution {
public:
    int arrayPairSum(vector<int>& nums) {
        vector<int> a(nums);
        sort(a.begin(), a.end());
        int total = 0;
        for (size_t i = 0; i < a.size(); i += 2) total += a[i];
        return total;
    }
};
