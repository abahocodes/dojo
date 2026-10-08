class Solution {
    public int maxNumberOfBalloons(String text) {
        int[] have = new int[26];
        for (int i = 0; i < text.length(); i++) have[text.charAt(i) - 'a']++;
        int[] need = new int[26];
        for (char c : "balloon".toCharArray()) need[c - 'a']++;
        int best = Integer.MAX_VALUE;
        for (int i = 0; i < 26; i++) {
            if (need[i] > 0) best = Math.min(best, have[i] / need[i]);
        }
        return best;
    }
}
