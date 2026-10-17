class TimeMap {
    // key -> its writes; set timestamps only increase, so each TreeMap is the full history.
    private final Map<String, TreeMap<Integer, String>> history = new HashMap<>();

    public TimeMap() {}

    public void set(String key, String value, int timestamp) {
        history.computeIfAbsent(key, k -> new TreeMap<>()).put(timestamp, value);
    }

    public String get(String key, int timestamp) {
        TreeMap<Integer, String> h = history.get(key);
        if (h == null) return "";
        Map.Entry<Integer, String> e = h.floorEntry(timestamp);  // last write at or before
        return e == null ? "" : e.getValue();
    }
}

class Solution {
    // dojo calls this to run the tests; you don't need to change it.
    public String[] timeMap(String[] ops, String[] keys, String[] values, int[] timestamps) {
        TimeMap store = new TimeMap();
        List<String> out = new ArrayList<>();
        for (int i = 0; i < ops.length; i++) {
            if (ops[i].equals("set")) store.set(keys[i], values[i], timestamps[i]);
            else out.add(store.get(keys[i], timestamps[i]));
        }
        return out.toArray(new String[0]);
    }
}
