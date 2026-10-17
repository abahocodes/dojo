class MinStack {
  constructor() {
    // Each entry is [value, minimum of this entry and everything below it].
    this.items = [];
  }

  push(val) {
    const n = this.items.length;
    const low = n === 0 ? val : Math.min(val, this.items[n - 1][1]);
    this.items.push([val, low]);
  }

  pop() {
    this.items.pop();
  }

  top() {
    return this.items[this.items.length - 1][0];
  }

  getMin() {
    return this.items[this.items.length - 1][1];
  }
}

// dojo calls this to run the tests; you don't need to change it.
function minStack(ops, vals) {
  const stack = new MinStack();
  const out = [];
  for (let i = 0; i < ops.length; i++) {
    if (ops[i] === "push") stack.push(vals[i]);
    else if (ops[i] === "pop") stack.pop();
    else if (ops[i] === "top") out.push(stack.top());
    else out.push(stack.getMin());
  }
  return out;
}
