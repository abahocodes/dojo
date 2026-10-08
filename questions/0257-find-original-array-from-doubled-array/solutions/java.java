class Solution {
    public int[] findOriginalArray(int[] changed) {
        int n = changed.length;
        if (n % 2 != 0) return new int[0];
        int top = 0;
        for (int v : changed) top = Math.max(top, v);
        int[] count = new int[top + 1];
        for (int v : changed) count[v]++;
        if (count[0] % 2 != 0) return new int[0];
        int[] original = new int[n / 2];
        int size = count[0] / 2; // zeros are already in place
        for (int x = 1; x <= top; x++) {
            int c = count[x];
            if (c == 0) continue;
            if (2 * x > top || count[2 * x] < c) return new int[0];
            count[2 * x] -= c;
            for (int k = 0; k < c; k++) original[size++] = x;
        }
        return original;
    }
}
