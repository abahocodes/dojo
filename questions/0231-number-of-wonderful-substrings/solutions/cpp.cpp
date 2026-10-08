class Solution {
public:
    long long wonderfulSubstrings(string& word) {
        vector<long long> seen(1024, 0);
        seen[0] = 1;
        int mask = 0;
        long long total = 0;
        for (char ch : word) {
            mask ^= 1 << (ch - 'a');
            total += seen[mask];
            for (int k = 0; k < 10; k++) total += seen[mask ^ (1 << k)];
            seen[mask]++;
        }
        return total;
    }
};
