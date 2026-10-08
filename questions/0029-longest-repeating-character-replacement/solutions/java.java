class Solution {
    public int characterReplacement(String s, int k) {
        int[] counts = new int[26];
        int maxFreq = 0;
        int left = 0;
        for (int right = 0; right < s.length(); right++) {
            int idx = s.charAt(right) - 'A';
            counts[idx]++;
            if (counts[idx] > maxFreq) maxFreq = counts[idx];
            if (right - left + 1 - maxFreq > k) {
                counts[s.charAt(left) - 'A']--;
                left++;
            }
        }
        return s.length() - left;
    }
}
