class Solution {
    private ListNode reverse(ListNode node) {
        ListNode prev = null;
        while (node != null) {
            ListNode nxt = node.next;
            node.next = prev;
            prev = node;
            node = nxt;
        }
        return prev;
    }

    public boolean isPalindromeList(ListNode head) {
        ListNode slow = head, fast = head;
        while (fast != null && fast.next != null) {
            slow = slow.next;
            fast = fast.next.next;
        }
        ListNode tail = reverse(slow);
        boolean ok = true;
        ListNode a = head, b = tail;
        while (b != null) {
            if (a.val != b.val) {
                ok = false;
                break;
            }
            a = a.next;
            b = b.next;
        }
        reverse(tail);
        return ok;
    }
}
