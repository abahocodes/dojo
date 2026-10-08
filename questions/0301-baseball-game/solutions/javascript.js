function calPoints(operations) {
  const record = [];
  for (const op of operations) {
    if (op === "+") record.push(record[record.length - 1] + record[record.length - 2]);
    else if (op === "D") record.push(2 * record[record.length - 1]);
    else if (op === "C") record.pop();
    else record.push(parseInt(op, 10));
  }
  return record.reduce((a, b) => a + b, 0);
}
