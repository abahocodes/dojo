class Solution {
    private ListNode cur;

    public TreeNode sortedListToBst(ListNode head) {
        int n = 0;
        for (ListNode node = head; node != null; node = node.next) n++;
        cur = head;
        return build(0, n - 1);
    }

    private TreeNode build(int lo, int hi) {
        if (lo > hi) return null;
        int mid = (lo + hi + 1) / 2;
        TreeNode left = build(lo, mid - 1);
        TreeNode root = new TreeNode(cur.val);
        root.left = left;
        cur = cur.next;
        root.right = build(mid + 1, hi);
        return root;
    }
}
