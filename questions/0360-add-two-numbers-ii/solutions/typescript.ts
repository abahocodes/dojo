function addTwoNumbersIi(l1: ListNode | null, l2: ListNode | null): ListNode | null {
  const a: number[] = [];
  const b: number[] = [];
  for (let p = l1; p !== null; p = p.next) a.push(p.val);
  for (let p = l2; p !== null; p = p.next) b.push(p.val);
  let head: ListNode | null = null;
  let carry = 0;
  while (a.length > 0 || b.length > 0 || carry > 0) {
    let s = carry;
    if (a.length > 0) s += a.pop()!;
    if (b.length > 0) s += b.pop()!;
    head = new ListNode(s % 10, head);
    carry = Math.floor(s / 10);
  }
  return head;
}
