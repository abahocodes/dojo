class Solution {
    public int sumSubarrayMins(int[] arr) {
        final long MOD = 1_000_000_007L;
        int n = arr.length;
        int[] stack = new int[n + 1];
        int top = 0;
        long total = 0;
        for (int j = 0; j <= n; j++) {
            int cur = j < n ? arr[j] : 0;
            while (top > 0 && arr[stack[top - 1]] >= cur) {
                int i = stack[--top];
                int left = top > 0 ? stack[top - 1] : -1;
                total = (total + (long) arr[i] * (i - left) % MOD * (j - i)) % MOD;
            }
            stack[top++] = j;
        }
        return (int) total;
    }
}
