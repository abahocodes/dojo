// 1002. Min Stack (medium)
// /show to reread the problem · /test runs visible tests · /submit when done

class MinStack {
  constructor() {

  }

  push(val: number): void {

  }

  pop(): void {

  }

  top(): number {

  }

  getMin(): number {

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
