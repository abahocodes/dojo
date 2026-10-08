function insertGcds(head) {
  const gcd = (a, b) => {
    while (b !== 0) {
      [a, b] = [b, a % b];
    }
    return a;
  };
  let cur = head;
  while (cur.next !== null) {
    const nxt = cur.next;
    cur.next = new ListNode(gcd(cur.val, nxt.val), nxt);
    cur = nxt;
  }
  return head;
}
