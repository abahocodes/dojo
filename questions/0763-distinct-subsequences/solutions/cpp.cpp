class Solution {
public:
    int numDistinct(string& s, string& t) {
        int m = t.size();
        // ways[j] = number of ways to form t[:j] from the part of s read so far.
        // Counts for short prefixes of t can overflow 64 bits. Unsigned
        // arithmetic wraps (signed overflow would be undefined behaviour), and
        // since the true answer fits in an int, ways[m] is exact.
        vector<unsigned long long> ways(m + 1, 0);
        ways[0] = 1;
        for (char ch : s) {
            // Go right to left so ways[j - 1] still holds the previous row's value.
            for (int j = m; j >= 1; j--) {
                if (t[j - 1] == ch) ways[j] += ways[j - 1];
            }
        }
        return (int)ways[m];
    }
};
