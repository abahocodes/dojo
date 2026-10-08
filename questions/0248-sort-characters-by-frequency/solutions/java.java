class Solution {
    public String frequencySort(String s) {
        int[] counts = new int[128];
        for (int i = 0; i < s.length(); i++) counts[s.charAt(i)]++;
        List<Integer> chars = new ArrayList<>();
        for (int c = 0; c < 128; c++) if (counts[c] > 0) chars.add(c);
        chars.sort((a, b) -> counts[a] != counts[b] ? counts[b] - counts[a] : a - b);
        StringBuilder sb = new StringBuilder(s.length());
        for (int c : chars) {
            for (int k = 0; k < counts[c]; k++) sb.append((char) c);
        }
        return sb.toString();
    }
}
