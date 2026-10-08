class Solution {
    public String reorganizeString(String s) {
        int[] counts = new int[26];
        for (char ch : s.toCharArray()) counts[ch - 'a']++;
        int n = s.length();
        if (maxOf(counts) > (n + 1) / 2) return "";

        StringBuilder result = new StringBuilder();
        int prev = -1;
        for (int pos = 0; pos < n; pos++) {
            int rest = n - pos - 1; // letters left after placing this one
            for (int c = 0; c < 26; c++) {
                if (counts[c] == 0 || c == prev) continue;
                counts[c]--;
                // The rest can follow c iff no letter needs more than half the
                // remaining slots, and c itself cannot take the very next slot.
                if (counts[c] <= rest / 2 && maxOf(counts) <= (rest + 1) / 2) {
                    result.append((char) ('a' + c));
                    prev = c;
                    break;
                }
                counts[c]++;
            }
        }
        return result.toString();
    }

    private int maxOf(int[] counts) {
        int best = 0;
        for (int x : counts) best = Math.max(best, x);
        return best;
    }
}
