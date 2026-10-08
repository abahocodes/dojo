function oddEvenList(head: ListNode | null): ListNode | null {
  if (head === null) return null;
  let odd: ListNode = head;
  let even: ListNode | null = head.next;
  const evenHead = even;
  while (even !== null && even.next !== null) {
    odd.next = even.next;
    odd = even.next;
    even.next = odd.next;
    even = even.next;
  }
  odd.next = evenHead;
  return head;
}
