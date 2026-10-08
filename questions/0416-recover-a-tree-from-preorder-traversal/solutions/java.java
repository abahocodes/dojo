class Solution {
    public TreeNode recoverFromPreorder(String traversal) {
        List<TreeNode> stack = new ArrayList<>();
        int n = traversal.length();
        int i = 0;
        while (i < n) {
            int depth = 0;
            while (traversal.charAt(i) == '-') {
                depth++;
                i++;
            }
            int value = 0;
            while (i < n && traversal.charAt(i) != '-') {
                value = value * 10 + (traversal.charAt(i) - '0');
                i++;
            }
            TreeNode node = new TreeNode(value);
            while (stack.size() > depth) stack.remove(stack.size() - 1);
            if (!stack.isEmpty()) {
                TreeNode parent = stack.get(stack.size() - 1);
                if (parent.left == null) parent.left = node;
                else parent.right = node;
            }
            stack.add(node);
        }
        return stack.get(0);
    }
}
