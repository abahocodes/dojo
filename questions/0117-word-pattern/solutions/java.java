class Solution {
    public boolean wordPattern(String pattern, String s) {
        String[] words = s.split(" ");
        if (words.length != pattern.length()) return false;
        String[] letterToWord = new String[26];
        Map<String, Character> wordToLetter = new HashMap<>();
        for (int i = 0; i < words.length; i++) {
            char c = pattern.charAt(i);
            String w = words[i];
            if (letterToWord[c - 'a'] == null) {
                if (wordToLetter.containsKey(w)) return false;
                letterToWord[c - 'a'] = w;
                wordToLetter.put(w, c);
            } else if (!letterToWord[c - 'a'].equals(w)) {
                return false;
            }
        }
        return true;
    }
}
