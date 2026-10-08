class Solution {
public:
    int subarraysDivByK(vector<int>& nums, int k) {
        vector<int> count(k, 0);
        count[0] = 1;
        int rem = 0, result = 0;
        for (int x : nums) {
            rem = ((rem + x) % k + k) % k;
            result += count[rem];
            count[rem]++;
        }
        return result;
    }
};
