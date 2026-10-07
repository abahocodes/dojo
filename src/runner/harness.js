// dojo javascript harness. Usage: node harness.js <solution.js> <spec.json> <results.json>
//
// Same protocol as harness.py. The solution runs in a vm context so a plain
// top-level `function twoSum(...)` works without exports, each case gets a
// real timeout (even for infinite loops), and console output is captured.
'use strict';

const fs = require('fs');
const path = require('path');
const util = require('util');
const vm = require('vm');

const STDOUT_CAP = 4000;
const MAX_NODES = 100000;

function ListNode(val, next) {
  this.val = val === undefined ? 0 : val;
  this.next = next === undefined ? null : next;
}

function TreeNode(val, left, right) {
  this.val = val === undefined ? 0 : val;
  this.left = left === undefined ? null : left;
  this.right = right === undefined ? null : right;
}

function toListNode(values) {
  const dummy = new ListNode();
  let tail = dummy;
  for (const v of values) {
    tail.next = new ListNode(v);
    tail = tail.next;
  }
  return dummy.next;
}

function fromListNode(node) {
  const out = [];
  while (node !== null && node !== undefined) {
    if (out.length >= MAX_NODES) throw new Error('returned linked list is too long (cycle?)');
    out.push(node.val);
    node = node.next;
  }
  return out;
}

function toTree(values) {
  if (!values.length || values[0] === null) return null;
  const root = new TreeNode(values[0]);
  const queue = [root];
  let i = 1;
  for (let q = 0; q < queue.length && i < values.length; q++) {
    const node = queue[q];
    if (values[i] !== null && values[i] !== undefined) {
      node.left = new TreeNode(values[i]);
      queue.push(node.left);
    }
    i++;
    if (i < values.length && values[i] !== null) {
      node.right = new TreeNode(values[i]);
      queue.push(node.right);
    }
    i++;
  }
  return root;
}

function fromTree(root) {
  const out = [];
  const queue = [root];
  for (let q = 0; q < queue.length; q++) {
    if (out.length >= MAX_NODES) throw new Error('returned tree is too large (cycle?)');
    const node = queue[q];
    if (node === null || node === undefined) {
      out.push(null);
      continue;
    }
    out.push(node.val);
    queue.push(node.left === undefined ? null : node.left);
    queue.push(node.right === undefined ? null : node.right);
  }
  while (out.length && out[out.length - 1] === null) out.pop();
  return out;
}

function decode(ty, v) {
  if (ty.endsWith('[]')) return v === null ? null : v.map((x) => decode(ty.slice(0, -2), x));
  if (ty === 'ListNode') return toListNode(v);
  if (ty === 'TreeNode') return toTree(v);
  return v;
}

function encode(ty, v) {
  if (v === undefined) return null;
  if (ty.endsWith('[]')) return v === null ? null : Array.from(v, (x) => encode(ty.slice(0, -2), x));
  if (ty === 'ListNode') return fromListNode(v);
  if (ty === 'TreeNode') return fromTree(v);
  return v;
}

// Keeps the message and the solution's own stack frames.
function formatError(err, file) {
  if (!err || !err.stack) return String(err);
  return String(err.stack)
    .split('\n')
    .filter((line) => !/^\s+at /.test(line) || line.includes(file))
    .map((line) => line.replace(/\(.*?([^/(]+\.js:\d+:\d+)\)/, '($1)'))
    .join('\n')
    .trimEnd();
}

function main() {
  const [solutionPath, specPath, resultsPath] = process.argv.slice(2);
  const spec = JSON.parse(fs.readFileSync(specPath, 'utf8'));
  const write = (obj) => fs.writeFileSync(resultsPath, JSON.stringify(obj));
  const file = path.basename(solutionPath);
  const timeout = Math.round(spec.timeout_secs * 1000);

  let printed = [];
  const capture = (...args) => printed.push(util.format(...args));
  const module = { exports: {} };
  const context = vm.createContext({
    console: { log: capture, info: capture, warn: capture, error: capture, debug: capture },
    ListNode,
    TreeNode,
    module,
    exports: module.exports,
  });

  let fn;
  try {
    const code = fs.readFileSync(solutionPath, 'utf8');
    new vm.Script(code, { filename: file }).runInContext(context, { timeout });
    fn = vm.runInContext(
      `typeof ${spec.function} === 'function' ? ${spec.function} : undefined`,
      context,
    );
    const exported = context.module.exports;
    if (!fn && typeof exported === 'function') fn = exported;
    if (!fn && exported && typeof exported[spec.function] === 'function') fn = exported[spec.function];
  } catch (e) {
    write({ fatal: formatError(e, file), load_stdout: printed.join('\n').slice(0, STDOUT_CAP) });
    return;
  }
  // Top-level console output is shown too ("printed while loading").
  const loadStdout = printed.join('\n').slice(0, STDOUT_CAP);
  if (!fn) {
    write({ fatal: `function \`${spec.function}\` not found in ${file}`, load_stdout: loadStdout });
    return;
  }

  const results = [];
  for (const c of spec.cases) {
    const r = { index: c.index };
    printed = [];
    const start = process.hrtime.bigint();
    try {
      context.__dojo_fn = fn;
      context.__dojo_args = spec.params.map((p) => decode(p.type, c.input[p.name]));
      const got = vm.runInContext('__dojo_fn(...__dojo_args)', context, { timeout });
      r.got = JSON.parse(JSON.stringify(encode(spec.returns, got)) ?? 'null');
      r.status = 'ok';
    } catch (e) {
      if (e && e.code === 'ERR_SCRIPT_EXECUTION_TIMEOUT') {
        r.status = 'timeout';
      } else {
        r.status = 'error';
        r.error = formatError(e, file);
      }
    }
    r.ms = Math.round(Number(process.hrtime.bigint() - start) / 1e4) / 100;
    const out = printed.join('\n');
    if (out) r.stdout = out.slice(0, STDOUT_CAP);
    results.push(r);
  }
  write({ results, load_stdout: loadStdout });
}

main();
