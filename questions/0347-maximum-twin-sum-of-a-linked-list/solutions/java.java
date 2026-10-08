class Solution {
    public int pairSum(ListNode head) {
        ListNode slow = head, fast = head;
        while (fast != null && fast.next != null) {
            slow = slow.next;
            fast = fast.next.next;
        }
        ListNode prev = null;
        while (slow != null) {
            ListNode nxt = slow.next;
            slow.next = prev;
            prev = slow;
            slow = nxt;
        }
        int best = 0;
        ListNode a = head, b = prev;
        while (b != null) {
            best = Math.max(best, a.val + b.val);
            a = a.next;
            b = b.next;
        }
        return best;
    }
}
