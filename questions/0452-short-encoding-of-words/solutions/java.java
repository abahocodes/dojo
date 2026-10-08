class Solution {
    public int minimumLengthEncoding(String[] words) {
        Set<String> keep = new HashSet<>(Arrays.asList(words));
        for (String w : words) {
            for (int k = 1; k < w.length(); k++) keep.remove(w.substring(k));
        }
        int total = 0;
        for (String w : keep) total += w.length() + 1;
        return total;
    }
}
