class Solution {
    public String pushDominoes(String dominoes) {
        int n = dominoes.length();
        char[] res = dominoes.toCharArray();
        // Virtual 'L' before the row and 'R' after it never push anything inward.
        int prevIdx = -1;
        char prev = 'L';
        for (int j = 0; j <= n; j++) {
            char cur = j == n ? 'R' : dominoes.charAt(j);
            if (cur == '.') continue;
            if (prev == cur) {
                for (int k = prevIdx + 1; k < j; k++) res[k] = cur;
            } else if (prev == 'R' && cur == 'L') {
                int lo = prevIdx + 1, hi = j - 1;
                while (lo < hi) {
                    res[lo++] = 'R';
                    res[hi--] = 'L';
                }
            }
            prevIdx = j;
            prev = cur;
        }
        return new String(res);
    }
}
