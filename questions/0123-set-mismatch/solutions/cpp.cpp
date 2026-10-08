class Solution {
public:
    vector<int> findErrorNums(vector<int>& nums) {
        int n = nums.size();
        vector<bool> seen(n + 1, false);
        int dup = 0;
        long long total = 0;
        for (int x : nums) {
            if (seen[x]) dup = x;
            seen[x] = true;
            total += x;
        }
        long long expected = (long long)n * (n + 1) / 2;
        int missing = (int)(expected - (total - dup));
        return {dup, missing};
    }
};
