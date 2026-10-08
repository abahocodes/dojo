class Solution {
    private final int[] need = new int[26];
    private final int[] have = new int[26];
    private int matches;

    private void change(int i, int delta) {
        if (have[i] == need[i]) matches--;
        have[i] += delta;
        if (have[i] == need[i]) matches++;
    }

    public int[] findAnagrams(String s, String p) {
        int m = p.length();
        if (m > s.length()) return new int[0];
        Arrays.fill(need, 0);
        Arrays.fill(have, 0);
        for (int j = 0; j < m; j++) need[p.charAt(j) - 'a']++;
        matches = 0;
        for (int x : need) if (x == 0) matches++;
        List<Integer> result = new ArrayList<>();
        for (int j = 0; j < s.length(); j++) {
            change(s.charAt(j) - 'a', 1);
            if (j >= m) change(s.charAt(j - m) - 'a', -1);
            if (j >= m - 1 && matches == 26) result.add(j - m + 1);
        }
        int[] out = new int[result.size()];
        for (int i = 0; i < out.length; i++) out[i] = result.get(i);
        return out;
    }
}
