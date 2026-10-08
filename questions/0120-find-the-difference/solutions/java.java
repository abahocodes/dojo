class Solution {
    public String findTheDifference(String s, String t) {
        int x = 0;
        for (int i = 0; i < s.length(); i++) x ^= s.charAt(i);
        for (int i = 0; i < t.length(); i++) x ^= t.charAt(i);
        return String.valueOf((char) x);
    }
}
