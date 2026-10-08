class Solution {
    public TreeNode flatten(TreeNode root) {
        TreeNode node = root;
        while (node != null) {
            if (node.left != null) {
                // Splice the left subtree between node and its right subtree.
                TreeNode tail = node.left;
                while (tail.right != null) tail = tail.right;
                tail.right = node.right;
                node.right = node.left;
                node.left = null;
            }
            node = node.right;
        }
        return root;
    }
}
