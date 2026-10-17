class Solution {
    public String multiply(String num1, String num2) {
        // Schoolbook multiplication: digit i of num1 times digit j of num2 lands
        // in positions i + j and i + j + 1 of the product (counting from the left).
        if (num1.equals("0") || num2.equals("0")) return "0";
        int m = num1.length(), n = num2.length();
        int[] out = new int[m + n];
        for (int i = m - 1; i >= 0; i--) {
            int a = num1.charAt(i) - '0';
            for (int j = n - 1; j >= 0; j--) {
                int total = a * (num2.charAt(j) - '0') + out[i + j + 1];
                out[i + j + 1] = total % 10;
                out[i + j] += total / 10;
            }
        }
        StringBuilder sb = new StringBuilder();
        int start = 0;
        while (start < out.length - 1 && out[start] == 0) start++;
        for (int i = start; i < out.length; i++) sb.append(out[i]);
        return sb.toString();
    }
}
