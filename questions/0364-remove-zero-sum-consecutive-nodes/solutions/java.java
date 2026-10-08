class Solution {
    public ListNode removeZeroSumSublists(ListNode head) {
        ListNode dummy = new ListNode(0, head);
        Map<Integer, ListNode> last = new HashMap<>();
        int total = 0;
        for (ListNode node = dummy; node != null; node = node.next) {
            total += node.val;
            last.put(total, node);
        }
        total = 0;
        for (ListNode node = dummy; node != null; node = node.next) {
            total += node.val;
            node.next = last.get(total).next;
        }
        return dummy.next;
    }
}
