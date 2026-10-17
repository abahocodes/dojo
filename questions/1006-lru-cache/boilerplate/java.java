// 1006. LRU Cache (medium)
// /show to reread the problem · /test runs visible tests · /submit when done

class LRUCache {
    public LRUCache(int capacity) {

    }

    public int get(int key) {
        return 0;
    }

    public void put(int key, int value) {

    }
}

class Solution {
    // dojo calls this to run the tests; you don't need to change it.
    public int[] lruCache(int capacity, String[] ops, int[][] args) {
        LRUCache cache = new LRUCache(capacity);
        List<Integer> out = new ArrayList<>();
        for (int i = 0; i < ops.length; i++) {
            if (ops[i].equals("put")) cache.put(args[i][0], args[i][1]);
            else out.add(cache.get(args[i][0]));
        }
        int[] result = new int[out.size()];
        for (int i = 0; i < result.length; i++) result[i] = out.get(i);
        return result;
    }
}
