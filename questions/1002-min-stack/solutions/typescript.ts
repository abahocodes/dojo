class MinStack {
  // Each entry is [value, minimum of this entry and everything below it].
  private items: [number, number][] = [];

  push(val: number): void {
    const n = this.items.length;
    const low = n === 0 ? val : Math.min(val, this.items[n - 1][1]);
    this.items.push([val, low]);
  }

  pop(): void {
    this.items.pop();
  }

  top(): number {
    return this.items[this.items.length - 1][0];
  }

  getMin(): number {
    return this.items[this.items.length - 1][1];
  }
}

// dojo calls this to run the tests; you don't need to change it.
function minStack(ops: string[], vals: number[]): number[] {
  const stack = new MinStack();
  const out: number[] = [];
  for (let i = 0; i < ops.length; i++) {
    if (ops[i] === "push") stack.push(vals[i]);
    else if (ops[i] === "pop") stack.pop();
    else if (ops[i] === "top") out.push(stack.top());
    else out.push(stack.getMin());
  }
  return out;
}
