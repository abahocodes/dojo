class Solution {
    public boolean isSubtree(TreeNode root, TreeNode subRoot) {
        return encode(root).contains(encode(subRoot));
    }

    // pre-order with explicit null markers; the leading comma on every token
    // keeps "2" from matching inside "12"
    private static String encode(TreeNode root) {
        StringBuilder sb = new StringBuilder();
        Deque<TreeNode> stack = new ArrayDeque<>(); // ArrayDeque rejects null, so mark nulls with NIL
        stack.push(root == null ? NIL : root);
        while (!stack.isEmpty()) {
            TreeNode node = stack.pop();
            if (node == NIL) {
                sb.append(",#");
            } else {
                sb.append(',').append(node.val);
                stack.push(node.right == null ? NIL : node.right);
                stack.push(node.left == null ? NIL : node.left);
            }
        }
        return sb.toString();
    }

    private static final TreeNode NIL = new TreeNode();
}
