class Solution {
    public int calPoints(String[] operations) {
        int[] record = new int[operations.length];
        int size = 0;
        for (String op : operations) {
            switch (op) {
                case "+": record[size] = record[size - 1] + record[size - 2]; size++; break;
                case "D": record[size] = 2 * record[size - 1]; size++; break;
                case "C": size--; break;
                default: record[size++] = Integer.parseInt(op);
            }
        }
        int total = 0;
        for (int i = 0; i < size; i++) total += record[i];
        return total;
    }
}
