class Solution {
    public boolean validateStackSequences(int[] pushed, int[] popped) {
        int[] stack = new int[pushed.length];
        int top = 0, j = 0;
        for (int x : pushed) {
            stack[top++] = x;
            while (top > 0 && stack[top - 1] == popped[j]) {
                top--;
                j++;
            }
        }
        return top == 0;
    }
}
