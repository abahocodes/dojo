class Solution {
    public ListNode removeNodes(ListNode head) {
        Deque<ListNode> stack = new ArrayDeque<>();
        for (ListNode node = head; node != null; node = node.next) {
            while (!stack.isEmpty() && stack.peekLast().val < node.val) stack.pollLast();
            stack.addLast(node);
        }
        ListNode result = stack.peekFirst();
        ListNode prev = null;
        for (ListNode node : stack) {
            if (prev != null) prev.next = node;
            prev = node;
        }
        prev.next = null;
        return result;
    }
}
