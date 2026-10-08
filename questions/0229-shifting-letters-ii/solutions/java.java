class Solution {
    public String shiftingLetters(String s, int[][] shifts) {
        int n = s.length();
        int[] diff = new int[n + 1];
        for (int[] op : shifts) {
            int delta = op[2] == 1 ? 1 : -1;
            diff[op[0]] += delta;
            diff[op[1] + 1] -= delta;
        }
        char[] out = new char[n];
        int net = 0;
        for (int i = 0; i < n; i++) {
            net += diff[i];
            int code = ((s.charAt(i) - 'a' + net) % 26 + 26) % 26;
            out[i] = (char) ('a' + code);
        }
        return new String(out);
    }
}
