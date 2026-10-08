class Solution {
public:
    long long minMovesToEqual(vector<int>& nums) {
        vector<int> a = nums;
        size_t k = a.size() / 2;
        // nth_element is the standard library's quickselect (introselect).
        nth_element(a.begin(), a.begin() + k, a.end());
        long long median = a[k];
        long long moves = 0;
        for (int v : a) moves += llabs((long long)v - median);
        return moves;
    }
};
