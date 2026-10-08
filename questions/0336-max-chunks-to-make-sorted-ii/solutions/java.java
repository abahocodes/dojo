class Solution {
    public int maxChunksToSorted(int[] arr) {
        int[] stack = new int[arr.length]; // maximum of each chunk, non-decreasing
        int top = 0;
        for (int x : arr) {
            if (top == 0 || x >= stack[top - 1]) {
                stack[top++] = x;
            } else {
                int biggest = stack[top - 1];
                while (top > 0 && stack[top - 1] > x) top--;
                stack[top++] = biggest;
            }
        }
        return top;
    }
}
