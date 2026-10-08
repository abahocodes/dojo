function getDecimalValue(head: ListNode | null): number {
  let value = 0;
  while (head !== null) {
    value = value * 2 + head.val;
    head = head.next;
  }
  return value;
}
