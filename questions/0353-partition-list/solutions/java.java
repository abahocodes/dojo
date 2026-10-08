class Solution {
    public ListNode partitionList(ListNode head, int x) {
        ListNode small = new ListNode(0);
        ListNode large = new ListNode(0);
        ListNode s = small;
        ListNode l = large;
        for (ListNode cur = head; cur != null; cur = cur.next) {
            if (cur.val < x) {
                s.next = cur;
                s = cur;
            } else {
                l.next = cur;
                l = cur;
            }
        }
        l.next = null;
        s.next = large.next;
        return small.next;
    }
}
