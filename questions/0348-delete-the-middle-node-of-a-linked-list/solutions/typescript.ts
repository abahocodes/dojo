function deleteMiddle(head: ListNode | null): ListNode | null {
  if (head === null || head.next === null) return null;
  let slow: ListNode = head;
  let fast: ListNode | null = head.next.next;
  while (fast !== null && fast.next !== null) {
    slow = slow.next!;
    fast = fast.next.next;
  }
  slow.next = slow.next!.next;
  return head;
}
