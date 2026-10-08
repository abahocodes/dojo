class Solution {
    public boolean wordBreak(String s, String[] words) {
        Set<String> vocab = new HashSet<>(Arrays.asList(words));
        TreeSet<Integer> lengths = new TreeSet<>();
        for (String w : words) lengths.add(w.length());
        boolean[] ok = new boolean[s.length() + 1];
        ok[0] = true;
        for (int i = 1; i <= s.length(); i++) {
            for (int length : lengths) {
                if (length > i) break;
                if (ok[i - length] && vocab.contains(s.substring(i - length, i))) {
                    ok[i] = true;
                    break;
                }
            }
        }
        return ok[s.length()];
    }
}
