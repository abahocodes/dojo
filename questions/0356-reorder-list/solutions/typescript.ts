function reorderList(head: ListNode | null): ListNode | null {
  if (head === null || head.next === null) return head;
  let slow: ListNode = head;
  let fast: ListNode = head;
  while (fast.next !== null && fast.next.next !== null) {
    slow = slow.next!;
    fast = fast.next.next;
  }
  let second: ListNode | null = slow.next;
  slow.next = null;
  let prev: ListNode | null = null;
  while (second !== null) {
    const nxt: ListNode | null = second.next;
    second.next = prev;
    prev = second;
    second = nxt;
  }
  let first: ListNode | null = head;
  second = prev;
  while (second !== null && first !== null) {
    const n1: ListNode | null = first.next;
    const n2: ListNode | null = second.next;
    first.next = second;
    second.next = n1;
    first = n1;
    second = n2;
  }
  return head;
}
