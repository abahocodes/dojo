function doubleIt(head: ListNode | null): ListNode | null {
  if (head === null) return null;
  if (head.val >= 5) head = new ListNode(0, head);
  for (let node: ListNode | null = head; node !== null; node = node.next) {
    node.val = (node.val * 2) % 10;
    if (node.next !== null && node.next.val >= 5) node.val += 1;
  }
  return head;
}
