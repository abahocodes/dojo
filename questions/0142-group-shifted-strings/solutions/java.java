class Solution {
    public String[][] groupStrings(String[] strings) {
        Map<String, List<String>> groups = new LinkedHashMap<>();
        for (String s : strings) {
            char[] key = new char[s.length()];
            for (int i = 0; i < s.length(); i++) {
                key[i] = (char) ('a' + (s.charAt(i) - s.charAt(0) + 26) % 26);
            }
            groups.computeIfAbsent(new String(key), k -> new ArrayList<>()).add(s);
        }
        String[][] result = new String[groups.size()][];
        int i = 0;
        for (List<String> group : groups.values()) {
            result[i++] = group.toArray(new String[0]);
        }
        return result;
    }
}
