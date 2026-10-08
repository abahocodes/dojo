class Solution {
    public ListNode insertionSortList(ListNode head) {
        ListNode dummy = new ListNode(0);
        ListNode tail = null;
        ListNode cur = head;
        while (cur != null) {
            ListNode nxt = cur.next;
            if (tail != null && tail.val <= cur.val) {
                tail.next = cur;
                cur.next = null;
                tail = cur;
            } else {
                ListNode p = dummy;
                while (p.next != null && p.next.val <= cur.val) p = p.next;
                cur.next = p.next;
                p.next = cur;
                if (cur.next == null) tail = cur;
            }
            cur = nxt;
        }
        return dummy.next;
    }
}
