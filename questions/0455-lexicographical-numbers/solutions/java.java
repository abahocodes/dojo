class Solution {
    public int[] lexicalOrder(int n) {
        int[] result = new int[n];
        int cur = 1;
        for (int i = 0; i < n; i++) {
            result[i] = cur;
            if ((long) cur * 10 <= n) {
                cur *= 10;
            } else {
                while (cur % 10 == 9 || cur + 1 > n) cur /= 10;
                cur++;
            }
        }
        return result;
    }
}
