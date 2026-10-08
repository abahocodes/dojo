class Solution {
    public int[] asteroidCollision(int[] asteroids) {
        int[] stack = new int[asteroids.length];
        int top = 0;
        for (int a : asteroids) {
            boolean alive = true;
            while (alive && a < 0 && top > 0 && stack[top - 1] > 0) {
                int last = stack[top - 1];
                if (last < -a) {
                    top--;
                } else if (last == -a) {
                    top--;
                    alive = false;
                } else {
                    alive = false;
                }
            }
            if (alive) stack[top++] = a;
        }
        return Arrays.copyOf(stack, top);
    }
}
