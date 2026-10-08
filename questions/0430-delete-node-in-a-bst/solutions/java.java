class Solution {
    public TreeNode deleteNode(TreeNode root, int key) {
        TreeNode parent = null;
        TreeNode node = root;
        while (node != null && node.val != key) {
            parent = node;
            node = key < node.val ? node.left : node.right;
        }
        if (node == null) return root;

        if (node.left != null && node.right != null) {
            TreeNode succParent = node;
            TreeNode succ = node.right;
            while (succ.left != null) {
                succParent = succ;
                succ = succ.left;
            }
            node.val = succ.val;
            if (succParent == node) succParent.right = succ.right;
            else succParent.left = succ.right;
            return root;
        }

        TreeNode child = node.left != null ? node.left : node.right;
        if (parent == null) return child;
        if (parent.left == node) parent.left = child;
        else parent.right = child;
        return root;
    }
}
