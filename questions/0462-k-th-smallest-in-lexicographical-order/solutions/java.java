class Solution {
    public int findKthNumber(int n, int k) {
        long current = 1;
        long remaining = k - 1;
        while (remaining > 0) {
            long size = subtreeSize(n, current);
            if (size <= remaining) {
                remaining -= size;
                current += 1;
            } else {
                remaining -= 1;
                current *= 10;
            }
        }
        return (int) current;
    }

    // How many numbers in [1, n] start with the decimal digits of prefix.
    private long subtreeSize(long n, long prefix) {
        long count = 0, first = prefix, last = prefix;
        while (first <= n) {
            count += Math.min(n, last) - first + 1;
            first *= 10;
            last = last * 10 + 9;
        }
        return count;
    }
}
