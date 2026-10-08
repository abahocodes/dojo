class Solution {
    private final int[] need = new int[26];
    private final int[] have = new int[26];
    private int matches;

    private void change(int i, int delta) {
        if (have[i] == need[i]) matches--;
        have[i] += delta;
        if (have[i] == need[i]) matches++;
    }

    public boolean checkInclusion(String s1, String s2) {
        int m = s1.length();
        if (m > s2.length()) return false;
        Arrays.fill(need, 0);
        Arrays.fill(have, 0);
        for (int j = 0; j < m; j++) need[s1.charAt(j) - 'a']++;
        matches = 0;
        for (int x : need) if (x == 0) matches++;
        for (int j = 0; j < s2.length(); j++) {
            change(s2.charAt(j) - 'a', 1);
            if (j >= m) change(s2.charAt(j - m) - 'a', -1);
            if (j >= m - 1 && matches == 26) return true;
        }
        return false;
    }
}
