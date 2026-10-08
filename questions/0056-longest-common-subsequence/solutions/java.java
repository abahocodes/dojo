class Solution {
    public int longestCommonSubsequence(String a, String b) {
        int m = b.length();
        int[] prev = new int[m + 1];
        for (int i = 0; i < a.length(); i++) {
            char ca = a.charAt(i);
            int[] curr = new int[m + 1];
            for (int j = 1; j <= m; j++) {
                if (ca == b.charAt(j - 1)) curr[j] = prev[j - 1] + 1;
                else curr[j] = Math.max(prev[j], curr[j - 1]);
            }
            prev = curr;
        }
        return prev[m];
    }
}
