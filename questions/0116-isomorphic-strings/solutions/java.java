class Solution {
    public boolean isIsomorphic(String s, String t) {
        // forward[a] / backward[b] hold the partner character + 1 (0 = unmapped).
        int[] forward = new int[128];
        int[] backward = new int[128];
        for (int i = 0; i < s.length(); i++) {
            int a = s.charAt(i);
            int b = t.charAt(i);
            if (forward[a] == 0 && backward[b] == 0) {
                forward[a] = b + 1;
                backward[b] = a + 1;
            } else if (forward[a] != b + 1) {
                return false;
            }
        }
        return true;
    }
}
