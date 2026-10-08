function isPalindromeList(head: ListNode | null): boolean {
  const reverse = (node: ListNode | null): ListNode | null => {
    let prev: ListNode | null = null;
    while (node !== null) {
      const nxt: ListNode | null = node.next;
      node.next = prev;
      prev = node;
      node = nxt;
    }
    return prev;
  };
  let slow = head;
  let fast = head;
  while (fast !== null && fast.next !== null) {
    slow = slow!.next;
    fast = fast.next.next;
  }
  const tail = reverse(slow);
  let ok = true;
  let a = head;
  let b = tail;
  while (b !== null) {
    if (a!.val !== b.val) {
      ok = false;
      break;
    }
    a = a!.next;
    b = b.next;
  }
  reverse(tail);
  return ok;
}
