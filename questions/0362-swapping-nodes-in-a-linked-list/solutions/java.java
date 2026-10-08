class Solution {
    public ListNode swapNodes(ListNode head, int k) {
        ListNode first = head;
        for (int i = 1; i < k; i++) first = first.next;
        ListNode runner = first, second = head;
        while (runner.next != null) {
            runner = runner.next;
            second = second.next;
        }
        int t = first.val;
        first.val = second.val;
        second.val = t;
        return head;
    }
}
