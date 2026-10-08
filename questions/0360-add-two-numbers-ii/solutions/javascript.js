function addTwoNumbersIi(l1, l2) {
  const a = [];
  const b = [];
  for (let p = l1; p !== null; p = p.next) a.push(p.val);
  for (let p = l2; p !== null; p = p.next) b.push(p.val);
  let head = null;
  let carry = 0;
  while (a.length > 0 || b.length > 0 || carry > 0) {
    let s = carry;
    if (a.length > 0) s += a.pop();
    if (b.length > 0) s += b.pop();
    head = new ListNode(s % 10, head);
    carry = Math.floor(s / 10);
  }
  return head;
}
