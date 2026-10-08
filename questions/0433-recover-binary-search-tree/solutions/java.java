class Solution {
    public TreeNode recoverTree(TreeNode root) {
        TreeNode first = null, second = null, prev = null;
        Deque<TreeNode> stack = new ArrayDeque<>();
        TreeNode node = root;
        while (!stack.isEmpty() || node != null) {
            while (node != null) {
                stack.push(node);
                node = node.left;
            }
            node = stack.pop();
            if (prev != null && prev.val > node.val) {
                if (first == null) first = prev;
                second = node;
            }
            prev = node;
            node = node.right;
        }
        int tmp = first.val;
        first.val = second.val;
        second.val = tmp;
        return root;
    }
}
