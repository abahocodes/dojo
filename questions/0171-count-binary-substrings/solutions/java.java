class Solution {
    public int countBinarySubstrings(String s) {
        int total = 0, prevRun = 0, curRun = 1;
        for (int i = 1; i < s.length(); i++) {
            if (s.charAt(i) == s.charAt(i - 1)) curRun++;
            else {
                total += Math.min(prevRun, curRun);
                prevRun = curRun;
                curRun = 1;
            }
        }
        return total + Math.min(prevRun, curRun);
    }
}
