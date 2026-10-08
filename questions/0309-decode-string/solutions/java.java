class Solution {
    public String decodeString(String s) {
        Deque<StringBuilder> texts = new ArrayDeque<>();
        Deque<Integer> counts = new ArrayDeque<>();
        StringBuilder buf = new StringBuilder();
        int k = 0;
        for (int i = 0; i < s.length(); i++) {
            char ch = s.charAt(i);
            if (Character.isDigit(ch)) {
                k = k * 10 + (ch - '0');
            } else if (ch == '[') {
                texts.push(buf);
                counts.push(k);
                buf = new StringBuilder();
                k = 0;
            } else if (ch == ']') {
                StringBuilder prev = texts.pop();
                int times = counts.pop();
                String body = buf.toString();
                for (int t = 0; t < times; t++) prev.append(body);
                buf = prev;
            } else {
                buf.append(ch);
            }
        }
        return buf.toString();
    }
}
