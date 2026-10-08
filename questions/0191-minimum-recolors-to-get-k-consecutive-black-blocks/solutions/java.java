class Solution {
    public int minimumRecolors(String blocks, int k) {
        int whites = 0;
        for (int i = 0; i < k; i++) if (blocks.charAt(i) == 'W') whites++;
        int best = whites;
        for (int i = k; i < blocks.length(); i++) {
            if (blocks.charAt(i) == 'W') whites++;
            if (blocks.charAt(i - k) == 'W') whites--;
            best = Math.min(best, whites);
        }
        return best;
    }
}
