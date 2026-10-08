function deleteMiddle(head) {
  if (head.next === null) return null;
  let slow = head;
  let fast = head.next.next;
  while (fast !== null && fast.next !== null) {
    slow = slow.next;
    fast = fast.next.next;
  }
  slow.next = slow.next.next;
  return head;
}
