function pairSum(head) {
  let slow = head;
  let fast = head;
  while (fast !== null && fast.next !== null) {
    slow = slow.next;
    fast = fast.next.next;
  }
  let prev = null;
  while (slow !== null) {
    const nxt = slow.next;
    slow.next = prev;
    prev = slow;
    slow = nxt;
  }
  let best = 0;
  let a = head;
  let b = prev;
  while (b !== null) {
    best = Math.max(best, a.val + b.val);
    a = a.next;
    b = b.next;
  }
  return best;
}
