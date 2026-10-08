function gcd(a: number, b: number): number {
  while (b !== 0) {
    const t = a % b;
    a = b;
    b = t;
  }
  return a;
}

function insertGcds(head: ListNode | null): ListNode | null {
  let cur = head;
  while (cur !== null && cur.next !== null) {
    const nxt: ListNode = cur.next;
    cur.next = new ListNode(gcd(cur.val, nxt.val), nxt);
    cur = nxt;
  }
  return head;
}
