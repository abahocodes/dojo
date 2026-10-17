// 1001. Encode and Decode Strings (medium)
// /show to reread the problem · /test runs visible tests · /submit when done

class Solution {
    public String encode(List<String> strs) {
        return "";
    }

    public List<String> decode(String s) {
        return new ArrayList<>();
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
