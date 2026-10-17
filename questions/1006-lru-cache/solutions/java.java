class LRUCache {
    private static class Node {
        int key, value;
        Node prev, next;

        Node(int key, int value) {
            this.key = key;
            this.value = value;
        }
    }

    private final int capacity;
    private final Map<Integer, Node> nodes = new HashMap<>();
    // Dummy ends: head.next is the most recently used, tail.prev the least.
    private final Node head = new Node(0, 0);
    private final Node tail = new Node(0, 0);

    public LRUCache(int capacity) {
        this.capacity = capacity;
        head.next = tail;
        tail.prev = head;
    }

    private void unlink(Node node) {
        node.prev.next = node.next;
        node.next.prev = node.prev;
    }

    private void pushFront(Node node) {
        node.prev = head;
        node.next = head.next;
        head.next.prev = node;
        head.next = node;
    }

    public int get(int key) {
        Node node = nodes.get(key);
        if (node == null) return -1;
        unlink(node);
        pushFront(node);
        return node.value;
    }

    public void put(int key, int value) {
        Node node = nodes.get(key);
        if (node != null) {
            node.value = value;
            unlink(node);
        } else {
            node = new Node(key, value);
            nodes.put(key, node);
            if (nodes.size() > capacity) {
                Node lru = tail.prev;
                unlink(lru);
                nodes.remove(lru.key);
            }
        }
        pushFront(node);
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
