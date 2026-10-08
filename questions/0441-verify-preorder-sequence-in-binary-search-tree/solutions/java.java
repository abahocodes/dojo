class Solution {
    public boolean verifyPreorder(int[] preorder) {
        int low = Integer.MIN_VALUE;
        int[] stack = new int[preorder.length];
        int top = 0;
        for (int x : preorder) {
            if (x < low) return false;
            while (top > 0 && stack[top - 1] < x) {
                low = stack[--top];
            }
            stack[top++] = x;
        }
        return true;
    }
}
