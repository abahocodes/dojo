class Solution {
    public int inorderSuccessor(TreeNode root, int p) {
        int answer = -1;
        TreeNode node = root;
        while (node != null) {
            if (node.val > p) {
                answer = node.val;
                node = node.left;
            } else {
                node = node.right;
            }
        }
        return answer;
    }
}
