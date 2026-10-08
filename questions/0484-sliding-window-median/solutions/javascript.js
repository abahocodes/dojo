class Heap {
  constructor(less) {
    this.less = less;
    this.items = [];
  }

  size() {
    return this.items.length;
  }

  peek() {
    return this.items[0];
  }

  push(x) {
    const a = this.items;
    a.push(x);
    let i = a.length - 1;
    while (i > 0) {
      const p = (i - 1) >> 1;
      if (!this.less(a[i], a[p])) break;
      [a[i], a[p]] = [a[p], a[i]];
      i = p;
    }
  }

  pop() {
    const a = this.items;
    const top = a[0];
    const last = a.pop();
    if (a.length > 0) {
      a[0] = last;
      let i = 0;
      for (;;) {
        const l = 2 * i + 1;
        const r = l + 1;
        let m = i;
        if (l < a.length && this.less(a[l], a[m])) m = l;
        if (r < a.length && this.less(a[r], a[m])) m = r;
        if (m === i) break;
        [a[i], a[m]] = [a[m], a[i]];
        i = m;
      }
    }
    return top;
  }
}

function medianSlidingWindow(nums, k) {
  const low = new Heap((a, b) => a > b);  // max-heap: smaller half
  const high = new Heap((a, b) => a < b); // min-heap: larger half
  const delayed = new Map(); // value -> copies removed but still inside a heap
  let lowSize = 0;
  let highSize = 0;

  const prune = (heap) => {
    while (heap.size() > 0 && (delayed.get(heap.peek()) || 0) > 0) {
      delayed.set(heap.peek(), delayed.get(heap.peek()) - 1);
      heap.pop();
    }
  };

  const rebalance = () => {
    if (lowSize > highSize + 1) {
      high.push(low.pop());
      lowSize--;
      highSize++;
      prune(low);
    } else if (lowSize < highSize) {
      low.push(high.pop());
      lowSize++;
      highSize--;
      prune(high);
    }
  };

  const add = (x) => {
    if (low.size() === 0 || x <= low.peek()) {
      low.push(x);
      lowSize++;
    } else {
      high.push(x);
      highSize++;
    }
    rebalance();
  };

  const remove = (x) => {
    delayed.set(x, (delayed.get(x) || 0) + 1);
    if (x <= low.peek()) {
      lowSize--;
      if (x === low.peek()) prune(low);
    } else {
      highSize--;
      if (high.size() > 0 && x === high.peek()) prune(high);
    }
    rebalance();
  };

  const result = [];
  for (let i = 0; i < nums.length; i++) {
    add(nums[i]);
    if (i >= k) remove(nums[i - k]);
    if (i >= k - 1) {
      result.push(k % 2 === 1 ? low.peek() : (low.peek() + high.peek()) / 2);
    }
  }
  return result;
}
