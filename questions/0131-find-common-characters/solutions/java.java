class Solution {
    public String[] commonChars(String[] words) {
        int[] common = new int[26];
        Arrays.fill(common, Integer.MAX_VALUE);
        for (String w : words) {
            int[] freq = new int[26];
            for (char c : w.toCharArray()) freq[c - 'a']++;
            for (int i = 0; i < 26; i++) common[i] = Math.min(common[i], freq[i]);
        }
        List<String> result = new ArrayList<>();
        for (int i = 0; i < 26; i++) {
            for (int j = 0; j < common[i]; j++) result.add(String.valueOf((char) ('a' + i)));
        }
        return result.toArray(new String[0]);
    }
}
