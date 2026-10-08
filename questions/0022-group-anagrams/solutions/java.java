class Solution {
    public String[][] groupAnagrams(String[] words) {
        Map<String, List<String>> groups = new LinkedHashMap<>();
        for (String w : words) {
            int[] counts = new int[26];
            for (int i = 0; i < w.length(); i++) counts[w.charAt(i) - 'a']++;
            String key = Arrays.toString(counts);
            groups.computeIfAbsent(key, k -> new ArrayList<>()).add(w);
        }
        String[][] result = new String[groups.size()][];
        int i = 0;
        for (List<String> group : groups.values()) {
            result[i++] = group.toArray(new String[0]);
        }
        return result;
    }
}
