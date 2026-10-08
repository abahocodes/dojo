class Solution {
    public boolean isLongPressedName(String name, String typed) {
        int i = 0;
        for (int j = 0; j < typed.length(); j++) {
            char c = typed.charAt(j);
            if (i < name.length() && name.charAt(i) == c) i++;
            else if (j == 0 || typed.charAt(j - 1) != c) return false;
        }
        return i == name.length();
    }
}
