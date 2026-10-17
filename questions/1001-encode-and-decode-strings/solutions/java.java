class Solution {
    public String encode(List<String> strs) {
        // Each string becomes "<length>#<string>"; the length says where it ends.
        StringBuilder sb = new StringBuilder();
        for (String s : strs) sb.append(s.length()).append('#').append(s);
        return sb.toString();
    }

    public List<String> decode(String s) {
        List<String> result = new ArrayList<>();
        int i = 0;
        while (i < s.length()) {
            int j = s.indexOf('#', i);
            int length = Integer.parseInt(s.substring(i, j));
            result.add(s.substring(j + 1, j + 1 + length));
            i = j + 1 + length;
        }
        return result;
    }

    // dojo calls this to run the tests; you don't need to change it.
    public String[] encodeAndDecode(String[] strs) {
        String encoded = encode(new ArrayList<>(Arrays.asList(strs)));
        List<String> other = new ArrayList<>(Arrays.asList(strs));
        Collections.reverse(other);
        other.add("dojo");
        String otherEncoded = encode(other);
        if (encoded == null || otherEncoded == null) {
            return new String[] {"<encode must return one string>"};
        }
        List<String> decoded = decode(encoded);
        return decoded == null ? null : decoded.toArray(new String[0]);
    }
}
