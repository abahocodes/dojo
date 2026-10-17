// 1003. Time Based Key-Value Store (medium)
// /show to reread the problem · /test runs visible tests · /submit when done

class TimeMap {
  constructor() {

  }

  /**
   * @param {string} key
   * @param {string} value
   * @param {number} timestamp
   * @return {void}
   */
  set(key, value, timestamp) {

  }

  /**
   * @param {string} key
   * @param {number} timestamp
   * @return {string}
   */
  get(key, timestamp) {

  }
}

// dojo calls this to run the tests; you don't need to change it.
function timeMap(ops, keys, values, timestamps) {
  const store = new TimeMap();
  const out = [];
  for (let i = 0; i < ops.length; i++) {
    if (ops[i] === "set") store.set(keys[i], values[i], timestamps[i]);
    else out.push(store.get(keys[i], timestamps[i]));
  }
  return out;
}
