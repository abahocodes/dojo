class Solution {
public:
    int threeSumMulti(vector<int>& arr, int target) {
        const long long MOD = 1000000007LL;
        vector<long long> c(101, 0);
        for (int v : arr) c[v]++;
        long long total = 0;
        for (int x = 0; x <= 100; x++) {
            for (int y = x; y <= 100; y++) {
                int z = target - x - y;
                if (z < y || z > 100) continue;
                long long ways;
                if (x == y && y == z) ways = c[x] * (c[x] - 1) * (c[x] - 2) / 6;
                else if (x == y) ways = c[x] * (c[x] - 1) / 2 * c[z];
                else if (y == z) ways = c[x] * (c[y] * (c[y] - 1) / 2);
                else ways = c[x] * c[y] * c[z];
                total = (total + ways % MOD) % MOD;
            }
        }
        return (int)total;
    }
};
