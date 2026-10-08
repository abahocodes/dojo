class Solution {
public:
    int longestCommonPrefixNumbers(vector<int>& arr1, vector<int>& arr2) {
        unordered_set<int> prefixes;
        for (int x : arr1) {
            while (x > 0 && prefixes.insert(x).second) x /= 10;
        }
        int best = 0;
        for (int y : arr2) {
            while (y > 0 && !prefixes.count(y)) y /= 10;
            int digits = 0;
            for (int t = y; t > 0; t /= 10) digits++;
            best = max(best, digits);
        }
        return best;
    }
};
