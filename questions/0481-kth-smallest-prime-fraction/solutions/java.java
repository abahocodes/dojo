class Solution {
    public int[] kthSmallestPrimeFraction(int[] arr, int k) {
        int n = arr.length;
        double lo = 0, hi = 1;
        while (true) {
            double mid = (lo + hi) / 2;
            int count = 0;
            int p = 0, q = 1;
            int i = 0;
            for (int j = 1; j < n; j++) {
                while (i < j && arr[i] < mid * arr[j]) i++;
                count += i;
                if (i > 0 && (long) arr[i - 1] * q > (long) p * arr[j]) {
                    p = arr[i - 1];
                    q = arr[j];
                }
            }
            if (count == k) return new int[] {p, q};
            if (count < k) lo = mid;
            else hi = mid;
        }
    }
}
