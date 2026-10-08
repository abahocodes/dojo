class Solution {
    public int[] exclusiveTime(int n, String[] logs) {
        int[] result = new int[n];
        int[] stack = new int[logs.length];
        int top = -1;
        int prev = 0;
        for (String entry : logs) {
            String[] parts = entry.split(":");
            int id = Integer.parseInt(parts[0]);
            int t = Integer.parseInt(parts[2]);
            if (parts[1].equals("start")) {
                if (top >= 0) result[stack[top]] += t - prev;
                stack[++top] = id;
                prev = t;
            } else {
                result[stack[top--]] += t - prev + 1;
                prev = t + 1;
            }
        }
        return result;
    }
}
