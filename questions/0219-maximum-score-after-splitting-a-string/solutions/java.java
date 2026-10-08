class Solution {
    public int maxScoreSplit(String s) {
        int score = 0;
        for (int i = 0; i < s.length(); i++) if (s.charAt(i) == '1') score++;
        int best = 0;
        for (int i = 0; i < s.length() - 1; i++) {
            score += s.charAt(i) == '0' ? 1 : -1;
            best = Math.max(best, score);
        }
        return best;
    }
}
