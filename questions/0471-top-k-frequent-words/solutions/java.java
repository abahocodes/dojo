class Solution {
    public String[] topKFrequentWords(String[] words, int k) {
        Map<String, Integer> counts = new HashMap<>();
        for (String w : words) counts.merge(w, 1, Integer::sum);
        List<String> ranked = new ArrayList<>(counts.keySet());
        ranked.sort((a, b) -> {
            int diff = counts.get(b) - counts.get(a);
            if (diff != 0) return diff;
            return a.compareTo(b);
        });
        return ranked.subList(0, k).toArray(new String[0]);
    }
}
