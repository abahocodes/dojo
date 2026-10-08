class Solution {
    public String[] binaryTreePaths(TreeNode root) {
        List<String> paths = new ArrayList<>();
        Deque<TreeNode> nodes = new ArrayDeque<>();
        Deque<String> texts = new ArrayDeque<>();
        nodes.push(root);
        texts.push(String.valueOf(root.val));
        while (!nodes.isEmpty()) {
            TreeNode node = nodes.pop();
            String path = texts.pop();
            if (node.left == null && node.right == null) {
                paths.add(path);
                continue;
            }
            if (node.right != null) {
                nodes.push(node.right);
                texts.push(path + "->" + node.right.val);
            }
            if (node.left != null) {
                nodes.push(node.left);
                texts.push(path + "->" + node.left.val);
            }
        }
        return paths.toArray(new String[0]);
    }
}
