class Solution {
    public TreeNode constructMaximumBinaryTree(int[] nums) {
        Deque<TreeNode> stack = new ArrayDeque<>();
        for (int x : nums) {
            TreeNode node = new TreeNode(x);
            TreeNode last = null;
            while (!stack.isEmpty() && stack.peek().val < x) {
                last = stack.pop();
            }
            node.left = last;
            if (!stack.isEmpty()) stack.peek().right = node;
            stack.push(node);
        }
        return stack.peekLast();
    }
}
