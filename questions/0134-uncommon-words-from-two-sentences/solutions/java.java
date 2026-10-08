class Solution {
    public String[] uncommonFromSentences(String s1, String s2) {
        String[] words = (s1 + " " + s2).split(" ");
        Map<String, Integer> counts = new HashMap<>();
        for (String w : words) counts.merge(w, 1, Integer::sum);
        List<String> result = new ArrayList<>();
        for (String w : words) {
            if (counts.get(w) == 1) result.add(w);
        }
        return result.toArray(new String[0]);
    }
}
