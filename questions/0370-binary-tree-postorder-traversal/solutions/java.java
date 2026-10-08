class Solution {
    public int[] postorderTraversal(TreeNode root) {
        if (root == null) return new int[0];
        List<Integer> result = new ArrayList<>();
        Deque<TreeNode> stack = new ArrayDeque<>();
        stack.push(root);
        while (!stack.isEmpty()) {
            TreeNode node = stack.pop();
            result.add(node.val);
            if (node.left != null) stack.push(node.left);
            if (node.right != null) stack.push(node.right);
        }
        int n = result.size();
        int[] out = new int[n];
        for (int i = 0; i < n; i++) out[i] = result.get(n - 1 - i);
        return out;
    }
}
