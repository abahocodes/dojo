function closestNodes(root, queries) {
  const values = [];
  const stack = [];
  let node = root;
  while (stack.length > 0 || node !== null) {
    while (node !== null) {
      stack.push(node);
      node = node.left;
    }
    node = stack.pop();
    values.push(node.val);
    node = node.right;
  }
  const answer = [];
  for (const q of queries) {
    let lo = 0;
    let hi = values.length;
    while (lo < hi) {
      const mid = (lo + hi) >> 1;
      if (values[mid] < q) lo = mid + 1;
      else hi = mid;
    }
    if (lo < values.length && values[lo] === q) {
      answer.push([q, q]);
    } else {
      answer.push([lo > 0 ? values[lo - 1] : -1, lo < values.length ? values[lo] : -1]);
    }
  }
  return answer;
}
