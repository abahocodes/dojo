function sortList(head: ListNode | null): ListNode | null {
  if (head === null || head.next === null) return head;
  let slow: ListNode = head;
  let fast: ListNode | null = head.next;
  while (fast !== null && fast.next !== null) {
    slow = slow.next!;
    fast = fast.next.next;
  }
  const mid: ListNode | null = slow.next;
  slow.next = null;
  let left = sortList(head);
  let right = sortList(mid);
  const dummy = new ListNode(0);
  let tail: ListNode = dummy;
  while (left !== null && right !== null) {
    if (left.val <= right.val) {
      tail.next = left;
      left = left.next;
    } else {
      tail.next = right;
      right = right.next;
    }
    tail = tail.next!;
  }
  tail.next = left !== null ? left : right;
  return dummy.next;
}
