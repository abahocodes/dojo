class Solution {
public:
    int countCombinations(vector<int>& coins, int amount) {
        // Intermediate counts can outgrow 64 bits; unsigned sums wrap modulo 2^64
        // without undefined behaviour, so the final count (which fits in an int) is exact.
        vector<unsigned long long> ways(amount + 1, 0);
        ways[0] = 1;
        for (int c : coins) {
            for (int x = c; x <= amount; x++) {
                ways[x] += ways[x - c];
            }
        }
        return (int)ways[amount];
    }
};
