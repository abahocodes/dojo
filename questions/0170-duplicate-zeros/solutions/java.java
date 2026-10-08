class Solution {
    public int[] duplicateZeros(int[] arr) {
        int[] out = arr.clone();
        int n = out.length;
        // shift = number of zeros strictly before index i: out[i] lands at i + shift.
        int shift = 0;
        for (int v : out) if (v == 0) shift++;
        for (int i = n - 1; i >= 0; i--) {
            if (out[i] == 0) {
                shift--;
                if (i + shift + 1 < n) out[i + shift + 1] = 0;
            }
            if (i + shift < n) out[i + shift] = out[i];
        }
        return out;
    }
}
