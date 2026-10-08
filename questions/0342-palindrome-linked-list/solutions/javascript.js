function isPalindromeList(head) {
  const reverse = (node) => {
    let prev = null;
    while (node !== null) {
      const nxt = node.next;
      node.next = prev;
      prev = node;
      node = nxt;
    }
    return prev;
  };
  let slow = head;
  let fast = head;
  while (fast !== null && fast.next !== null) {
    slow = slow.next;
    fast = fast.next.next;
  }
  const tail = reverse(slow);
  let ok = true;
  let a = head;
  let b = tail;
  while (b !== null) {
    if (a.val !== b.val) {
      ok = false;
      break;
    }
    a = a.next;
    b = b.next;
  }
  reverse(tail);
  return ok;
}
