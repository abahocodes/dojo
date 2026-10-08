function reorderList(head) {
  if (head === null || head.next === null) return head;
  let slow = head;
  let fast = head;
  while (fast.next !== null && fast.next.next !== null) {
    slow = slow.next;
    fast = fast.next.next;
  }
  let second = slow.next;
  slow.next = null;
  let prev = null;
  while (second !== null) {
    const nxt = second.next;
    second.next = prev;
    prev = second;
    second = nxt;
  }
  let first = head;
  second = prev;
  while (second !== null) {
    const n1 = first.next;
    const n2 = second.next;
    first.next = second;
    second.next = n1;
    first = n1;
    second = n2;
  }
  return head;
}
