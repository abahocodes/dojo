class Solution {
    public ListNode addTwoNumbersIi(ListNode l1, ListNode l2) {
        Deque<Integer> a = new ArrayDeque<>();
        Deque<Integer> b = new ArrayDeque<>();
        for (ListNode p = l1; p != null; p = p.next) a.push(p.val);
        for (ListNode p = l2; p != null; p = p.next) b.push(p.val);
        ListNode head = null;
        int carry = 0;
        while (!a.isEmpty() || !b.isEmpty() || carry > 0) {
            int s = carry;
            if (!a.isEmpty()) s += a.pop();
            if (!b.isEmpty()) s += b.pop();
            head = new ListNode(s % 10, head);
            carry = s / 10;
        }
        return head;
    }
}
