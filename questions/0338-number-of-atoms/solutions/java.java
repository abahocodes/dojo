class Solution {
    private String formula;
    private int i;

    public String countOfAtoms(String formula) {
        this.formula = formula;
        this.i = 0;
        int n = formula.length();
        Deque<Map<String, Long>> stack = new ArrayDeque<>(); // one map per open group
        stack.push(new HashMap<>());
        while (i < n) {
            char ch = formula.charAt(i);
            if (ch == '(') {
                stack.push(new HashMap<>());
                i++;
            } else if (ch == ')') {
                i++;
                long mult = readNumber();
                Map<String, Long> group = stack.pop();
                Map<String, Long> top = stack.peek();
                for (Map.Entry<String, Long> e : group.entrySet()) {
                    top.merge(e.getKey(), e.getValue() * mult, Long::sum);
                }
            } else {
                int start = i++;
                while (i < n && Character.isLowerCase(formula.charAt(i))) i++;
                String name = formula.substring(start, i);
                stack.peek().merge(name, readNumber(), Long::sum);
            }
        }
        TreeMap<String, Long> sorted = new TreeMap<>(stack.peek());
        StringBuilder sb = new StringBuilder();
        for (Map.Entry<String, Long> e : sorted.entrySet()) {
            sb.append(e.getKey());
            if (e.getValue() > 1) sb.append(e.getValue());
        }
        return sb.toString();
    }

    private long readNumber() {
        int start = i;
        while (i < formula.length() && Character.isDigit(formula.charAt(i))) i++;
        return i > start ? Long.parseLong(formula.substring(start, i)) : 1L;
    }
}
