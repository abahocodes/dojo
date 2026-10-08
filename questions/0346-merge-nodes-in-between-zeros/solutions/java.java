class Solution {
    public ListNode mergeNodes(ListNode head) {
        ListNode dummy = new ListNode(0);
        ListNode tail = dummy;
        int total = 0;
        for (ListNode cur = head.next; cur != null; cur = cur.next) {
            if (cur.val == 0) {
                cur.val = total;
                tail.next = cur;
                tail = cur;
                total = 0;
            } else {
                total += cur.val;
            }
        }
        tail.next = null;
        return dummy.next;
    }
}
