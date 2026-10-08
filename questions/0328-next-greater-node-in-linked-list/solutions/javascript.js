function nextLargerNodes(head) {
  const vals = [];
  for (let node = head; node !== null; node = node.next) vals.push(node.val);
  const answer = new Array(vals.length).fill(0);
  const waiting = [];
  for (let i = 0; i < vals.length; i++) {
    while (waiting.length > 0 && vals[waiting[waiting.length - 1]] < vals[i]) {
      answer[waiting.pop()] = vals[i];
    }
    waiting.push(i);
  }
  return answer;
}
