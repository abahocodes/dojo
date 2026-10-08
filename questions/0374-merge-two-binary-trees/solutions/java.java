class Solution {
    public TreeNode mergeTrees(TreeNode root1, TreeNode root2) {
        if (root1 == null) return root2;
        Deque<TreeNode[]> stack = new ArrayDeque<>();
        stack.push(new TreeNode[] {root1, root2});
        while (!stack.isEmpty()) {
            TreeNode[] pair = stack.pop();
            TreeNode a = pair[0], b = pair[1];
            if (b == null) continue;
            a.val += b.val;
            if (a.left == null) a.left = b.left;
            else stack.push(new TreeNode[] {a.left, b.left});
            if (a.right == null) a.right = b.right;
            else stack.push(new TreeNode[] {a.right, b.right});
        }
        return root1;
    }
}
