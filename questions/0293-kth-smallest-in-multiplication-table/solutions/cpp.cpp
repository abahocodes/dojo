class Solution {
public:
    int findKthNumber(int m, int n, int k) {
        if (m > n) swap(m, n);
        int lo = 1, hi = m * n;
        while (lo < hi) {
            int mid = lo + (hi - lo) / 2;
            if (countAtMost(m, n, mid) >= k) hi = mid;
            else lo = mid + 1;
        }
        return lo;
    }

private:
    long long countAtMost(int m, int n, int x) {
        long long total = 0;
        for (int i = 1; i <= m; i++) {
            int inRow = x / i;
            if (inRow == 0) break;
            total += min(inRow, n);
        }
        return total;
    }
};
