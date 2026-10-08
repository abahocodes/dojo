class Solution {
    private int gcd(int a, int b) {
        while (b != 0) {
            int t = a % b;
            a = b;
            b = t;
        }
        return a;
    }

    public ListNode insertGcds(ListNode head) {
        ListNode cur = head;
        while (cur.next != null) {
            ListNode nxt = cur.next;
            cur.next = new ListNode(gcd(cur.val, nxt.val), nxt);
            cur = nxt;
        }
        return head;
    }
}
