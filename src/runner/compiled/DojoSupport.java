// dojo's Java driver support, compiled with every Java solution. Same
// protocol as harness.py: read the spec, run each case, rewrite the results
// file after every case and name the current step in the progress file.
// Comparison happens in dojo. No libraries beyond the JDK: JSON is read and
// written here.

import java.io.ByteArrayOutputStream;
import java.io.OutputStream;
import java.io.PrintStream;
import java.lang.reflect.Array;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.StandardCopyOption;
import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.function.Function;
import java.util.function.IntFunction;

// ListNode and TreeNode as on LeetCode.
class ListNode {
    int val;
    ListNode next;

    ListNode() {}

    ListNode(int val) {
        this.val = val;
    }

    ListNode(int val, ListNode next) {
        this.val = val;
        this.next = next;
    }
}

class TreeNode {
    int val;
    TreeNode left;
    TreeNode right;

    TreeNode() {}

    TreeNode(int val) {
        this.val = val;
    }

    TreeNode(int val, TreeNode left, TreeNode right) {
        this.val = val;
        this.left = left;
        this.right = right;
    }
}

/** One call of the solution, generated per question in DojoMain. */
interface DojoCall {
    Object call(Map<String, Object> input) throws Throwable;
}

/** A problem with dojo's input or the returned value, not the solution's code. */
class DojoError extends RuntimeException {
    DojoError(String message) {
        super(message);
    }
}

final class DojoSupport {
    static final int STDOUT_CAP = 4000;
    static final int MAX_NODES = 100_000;
    static final String SOLUTION_FILE = "solution.java";
    /** Most stack frames shown for an exception. */
    static final int MAX_FRAMES = 12;

    private DojoSupport() {}

    // ---- running cases ----

    static void run(String[] args, DojoCall call) throws Exception {
        Path specPath = Path.of(args[0]);
        Path resultsPath = Path.of(args[1]);
        Path progressPath = Path.of(args[1] + ".progress");
        progress(progressPath, "load");
        Object spec = new Json(Files.readString(specPath, StandardCharsets.UTF_8)).parseDocument();
        Map<String, Object> specMap = asMap(spec);
        String returns = asString(specMap.get("returns"));
        List<Object> cases = asList(specMap.get("cases"));

        PrintStream realOut = System.out;
        StringBuilder results = new StringBuilder();
        for (Object c : cases) {
            Map<String, Object> theCase = asMap(c);
            long index = asLong(theCase.get("index"));
            progress(progressPath, Long.toString(index));
            Map<String, Object> input = asMap(theCase.get("input"));

            CappedOutput printed = new CappedOutput();
            System.setOut(new PrintStream(printed, true, StandardCharsets.UTF_8));
            String status;
            String got = null;
            String error = null;
            long start = System.nanoTime();
            try {
                Object value = call.call(input);
                got = encode(returns, value);
                status = "ok";
            } catch (Throwable t) {
                status = "error";
                error = explain(t);
            } finally {
                System.out.flush();
                System.setOut(realOut);
            }
            double ms = (System.nanoTime() - start) / 1_000_000.0;

            if (results.length() > 0) {
                results.append(',');
            }
            results.append("{\"index\":").append(index);
            results.append(",\"status\":").append(quote(status));
            if (got != null) {
                results.append(",\"got\":").append(got);
            }
            if (error != null) {
                results.append(",\"error\":").append(quote(error));
            }
            String out = printed.text();
            if (!out.isEmpty()) {
                results.append(",\"stdout\":").append(quote(out));
            }
            results.append(",\"ms\":").append(ms).append('}');
            write(resultsPath, "{\"results\":[" + results + "]}");
        }
        progress(progressPath, "done");
        write(resultsPath, "{\"results\":[" + results + "]}");
    }

    static void progress(Path path, String step) {
        try {
            Files.writeString(path, step, StandardCharsets.UTF_8);
        } catch (Exception e) {
            // dojo only loses the step name; the run goes on.
        }
    }

    /** Writes whole: a run killed mid-write leaves the previous results. */
    static void write(Path path, String text) throws Exception {
        Path tmp = Path.of(path + ".tmp");
        Files.writeString(tmp, text, StandardCharsets.UTF_8);
        Files.move(tmp, path, StandardCopyOption.REPLACE_EXISTING, StandardCopyOption.ATOMIC_MOVE);
    }

    /** The error, the solution's own frames, and a hint for runaway cases. */
    static String explain(Throwable t) {
        if (t instanceof DojoError) {
            return t.getMessage();
        }
        StringBuilder sb = new StringBuilder(t.toString());
        String last = null;
        int repeats = 0;
        int shown = 0;
        for (StackTraceElement f : t.getStackTrace()) {
            if (!SOLUTION_FILE.equals(f.getFileName())) {
                continue;
            }
            String line = "  at " + f.getClassName() + "." + f.getMethodName()
                    + "(" + f.getFileName() + ":" + f.getLineNumber() + ")";
            if (line.equals(last)) {
                repeats++;
                continue;
            }
            if (repeats > 0) {
                sb.append("\n  ... (the same call, repeated)");
                repeats = 0;
            }
            if (shown == MAX_FRAMES) {
                sb.append("\n  ...");
                break;
            }
            sb.append('\n').append(line);
            last = line;
            shown++;
        }
        if (repeats > 0) {
            sb.append("\n  ... (the same call, repeated)");
        }
        if (t instanceof StackOverflowError) {
            sb.append("\nhint: the call stack overflowed: a missing base case, or recursion"
                    + " too deep for this input (try an explicit stack)");
        } else if (t instanceof OutOfMemoryError) {
            sb.append("\nhint: ran out of memory: a structure growing without bound?");
        }
        return sb.toString();
    }

    /** Collects printed bytes up to a cap; the rest is dropped as it's printed. */
    static final class CappedOutput extends OutputStream {
        // Enough bytes for STDOUT_CAP characters of any UTF-8 text.
        private final ByteArrayOutputStream kept = new ByteArrayOutputStream();
        private boolean dropped;

        @Override
        public void write(int b) {
            if (kept.size() < STDOUT_CAP * 4) {
                kept.write(b);
            } else {
                dropped = true;
            }
        }

        @Override
        public void write(byte[] b, int off, int len) {
            int room = STDOUT_CAP * 4 - kept.size();
            if (len > room) {
                dropped = true;
                len = Math.max(room, 0);
            }
            kept.write(b, off, len);
        }

        String text() {
            String out = kept.toString(StandardCharsets.UTF_8);
            boolean cut = dropped;
            if (out.codePointCount(0, out.length()) > STDOUT_CAP) {
                out = out.substring(0, out.offsetByCodePoints(0, STDOUT_CAP));
                cut = true;
            }
            if (cut) {
                out += "\n… (more output not shown)";
            }
            int end = out.length();
            while (end > 0 && (out.charAt(end - 1) == '\n' || out.charAt(end - 1) == '\r')) {
                end--;
            }
            return out.substring(0, end);
        }
    }

    // ---- decoding inputs (called by the generated DojoMain) ----

    static Object arg(Map<String, Object> input, String name) {
        if (!input.containsKey(name)) {
            throw new DojoError("dojo: input \"" + name + "\" missing");
        }
        return input.get(name);
    }

    static Map<String, Object> asMap(Object v) {
        if (v instanceof Map) {
            @SuppressWarnings("unchecked")
            Map<String, Object> m = (Map<String, Object>) v;
            return m;
        }
        throw new DojoError("dojo: expected an object, got " + Json.write(v));
    }

    static List<Object> asList(Object v) {
        if (v instanceof List) {
            @SuppressWarnings("unchecked")
            List<Object> l = (List<Object>) v;
            return l;
        }
        throw new DojoError("dojo: expected an array, got " + Json.write(v));
    }

    static int asInt(Object v) {
        if (v instanceof Long n && n >= Integer.MIN_VALUE && n <= Integer.MAX_VALUE) {
            return (int) (long) n;
        }
        throw new DojoError("dojo: expected an int, got " + Json.write(v));
    }

    static long asLong(Object v) {
        if (v instanceof Long n) {
            return n;
        }
        throw new DojoError("dojo: expected a long, got " + Json.write(v));
    }

    static double asDouble(Object v) {
        if (v instanceof Long n) {
            return n;
        }
        if (v instanceof Double d) {
            return d;
        }
        throw new DojoError("dojo: expected a number, got " + Json.write(v));
    }

    static boolean asBool(Object v) {
        if (v instanceof Boolean b) {
            return b;
        }
        throw new DojoError("dojo: expected true or false, got " + Json.write(v));
    }

    static String asString(Object v) {
        if (v instanceof String s) {
            return s;
        }
        throw new DojoError("dojo: expected a string, got " + Json.write(v));
    }

    static int[] ints(Object v) {
        List<Object> xs = asList(v);
        int[] out = new int[xs.size()];
        for (int i = 0; i < out.length; i++) {
            out[i] = asInt(xs.get(i));
        }
        return out;
    }

    static long[] longs(Object v) {
        List<Object> xs = asList(v);
        long[] out = new long[xs.size()];
        for (int i = 0; i < out.length; i++) {
            out[i] = asLong(xs.get(i));
        }
        return out;
    }

    static double[] doubles(Object v) {
        List<Object> xs = asList(v);
        double[] out = new double[xs.size()];
        for (int i = 0; i < out.length; i++) {
            out[i] = asDouble(xs.get(i));
        }
        return out;
    }

    static boolean[] bools(Object v) {
        List<Object> xs = asList(v);
        boolean[] out = new boolean[xs.size()];
        for (int i = 0; i < out.length; i++) {
            out[i] = asBool(xs.get(i));
        }
        return out;
    }

    /** An array of non-primitive elements, each decoded by `each`. */
    static <T> T[] arr(Object v, IntFunction<T[]> make, Function<Object, T> each) {
        List<Object> xs = asList(v);
        T[] out = make.apply(xs.size());
        for (int i = 0; i < out.length; i++) {
            out[i] = each.apply(xs.get(i));
        }
        return out;
    }

    static ListNode toList(Object v) {
        ListNode dummy = new ListNode();
        ListNode tail = dummy;
        for (Object x : asList(v)) {
            tail.next = new ListNode(asInt(x));
            tail = tail.next;
        }
        return dummy.next;
    }

    static TreeNode toTree(Object v) {
        List<Object> values = asList(v);
        if (values.isEmpty() || values.get(0) == null) {
            return null;
        }
        TreeNode root = new TreeNode(asInt(values.get(0)));
        List<TreeNode> queue = new ArrayList<>();
        queue.add(root);
        int i = 1;
        for (int q = 0; q < queue.size() && i < values.size(); q++) {
            TreeNode node = queue.get(q);
            if (values.get(i) != null) {
                node.left = new TreeNode(asInt(values.get(i)));
                queue.add(node.left);
            }
            i++;
            if (i < values.size() && values.get(i) != null) {
                node.right = new TreeNode(asInt(values.get(i)));
                queue.add(node.right);
            }
            i++;
        }
        return root;
    }

    // ---- encoding the result ----

    /** The returned value as JSON, guided by the declared type (`int[][]`, `ListNode`). */
    static String encode(String type, Object v) {
        StringBuilder sb = new StringBuilder();
        encode(type, v, sb);
        return sb.toString();
    }

    private static void encode(String type, Object v, StringBuilder sb) {
        if (type.endsWith("[]")) {
            if (v == null) {
                sb.append("null");
                return;
            }
            String inner = type.substring(0, type.length() - 2);
            sb.append('[');
            int n = Array.getLength(v);
            for (int i = 0; i < n; i++) {
                if (i > 0) {
                    sb.append(',');
                }
                encode(inner, Array.get(v, i), sb);
            }
            sb.append(']');
            return;
        }
        switch (type) {
            case "ListNode" -> {
                sb.append('[');
                ListNode node = (ListNode) v;
                int n = 0;
                while (node != null) {
                    if (n >= MAX_NODES) {
                        throw new DojoError("returned linked list is too long (cycle?)");
                    }
                    if (n++ > 0) {
                        sb.append(',');
                    }
                    sb.append(node.val);
                    node = node.next;
                }
                sb.append(']');
            }
            case "TreeNode" -> {
                List<Object> out = new ArrayList<>();
                List<TreeNode> order = new ArrayList<>();
                order.add((TreeNode) v);
                for (int q = 0; q < order.size(); q++) {
                    if (out.size() >= MAX_NODES) {
                        throw new DojoError("returned tree is too large (cycle?)");
                    }
                    TreeNode node = order.get(q);
                    if (node == null) {
                        out.add(null);
                        continue;
                    }
                    out.add((long) node.val);
                    order.add(node.left);
                    order.add(node.right);
                }
                int end = out.size();
                while (end > 0 && out.get(end - 1) == null) {
                    end--;
                }
                sb.append(Json.write(out.subList(0, end)));
            }
            // Numbers, strings and booleans; NaN and infinities are refused there.
            default -> sb.append(Json.write(v));
        }
    }

    static String quote(String s) {
        return Json.write(s);
    }
}

/** A small, strict JSON reader and writer. */
final class Json {
    private final String s;
    private int i;

    Json(String s) {
        this.s = s;
    }

    /** The one value in the text, with nothing after it. */
    Object parseDocument() {
        Object v = value();
        space();
        if (i != s.length()) {
            throw fail("unexpected text after the value");
        }
        return v;
    }

    private DojoError fail(String why) {
        return new DojoError("dojo: invalid JSON at " + i + ": " + why);
    }

    private void space() {
        while (i < s.length()) {
            char c = s.charAt(i);
            if (c == ' ' || c == '\t' || c == '\n' || c == '\r') {
                i++;
            } else {
                break;
            }
        }
    }

    private Object value() {
        space();
        if (i >= s.length()) {
            throw fail("unexpected end");
        }
        char c = s.charAt(i);
        switch (c) {
            case '{':
                return object();
            case '[':
                return array();
            case '"':
                return string();
            case 't':
                return word("true", Boolean.TRUE);
            case 'f':
                return word("false", Boolean.FALSE);
            case 'n':
                return word("null", null);
            default:
                if (c == '-' || (c >= '0' && c <= '9')) {
                    return number();
                }
                throw fail("unexpected '" + c + "'");
        }
    }

    private Object word(String w, Object v) {
        if (!s.startsWith(w, i)) {
            throw fail("expected " + w);
        }
        i += w.length();
        return v;
    }

    private Map<String, Object> object() {
        Map<String, Object> out = new LinkedHashMap<>();
        i++; // {
        space();
        if (i < s.length() && s.charAt(i) == '}') {
            i++;
            return out;
        }
        while (true) {
            space();
            if (i >= s.length() || s.charAt(i) != '"') {
                throw fail("expected a key");
            }
            String key = string();
            space();
            if (i >= s.length() || s.charAt(i) != ':') {
                throw fail("expected ':'");
            }
            i++;
            out.put(key, value());
            space();
            if (i < s.length() && s.charAt(i) == ',') {
                i++;
            } else if (i < s.length() && s.charAt(i) == '}') {
                i++;
                return out;
            } else {
                throw fail("expected ',' or '}'");
            }
        }
    }

    private List<Object> array() {
        List<Object> out = new ArrayList<>();
        i++; // [
        space();
        if (i < s.length() && s.charAt(i) == ']') {
            i++;
            return out;
        }
        while (true) {
            out.add(value());
            space();
            if (i < s.length() && s.charAt(i) == ',') {
                i++;
            } else if (i < s.length() && s.charAt(i) == ']') {
                i++;
                return out;
            } else {
                throw fail("expected ',' or ']'");
            }
        }
    }

    private String string() {
        i++; // opening quote
        StringBuilder sb = new StringBuilder();
        while (true) {
            if (i >= s.length()) {
                throw fail("unterminated string");
            }
            char c = s.charAt(i++);
            if (c == '"') {
                return sb.toString();
            }
            if (c < 0x20) {
                throw fail("control character in string");
            }
            if (c != '\\') {
                sb.append(c);
                continue;
            }
            if (i >= s.length()) {
                throw fail("unterminated escape");
            }
            char e = s.charAt(i++);
            switch (e) {
                case '"' -> sb.append('"');
                case '\\' -> sb.append('\\');
                case '/' -> sb.append('/');
                case 'b' -> sb.append('\b');
                case 'f' -> sb.append('\f');
                case 'n' -> sb.append('\n');
                case 'r' -> sb.append('\r');
                case 't' -> sb.append('\t');
                // Surrogate pairs arrive as two escapes and join up in the
                // UTF-16 string by themselves.
                case 'u' -> sb.append(hex4());
                default -> throw fail("bad escape \\" + e);
            }
        }
    }

    private char hex4() {
        if (i + 4 > s.length()) {
            throw fail("short \\u escape");
        }
        int v = 0;
        for (int k = 0; k < 4; k++) {
            int d = Character.digit(s.charAt(i++), 16);
            if (d < 0) {
                throw fail("bad \\u escape");
            }
            v = v * 16 + d;
        }
        return (char) v;
    }

    /** Integers become Long, anything with a fraction or exponent Double. */
    private Object number() {
        int start = i;
        if (s.charAt(i) == '-') {
            i++;
        }
        if (i < s.length() && s.charAt(i) == '0') {
            i++;
        } else if (!digits()) {
            throw fail("bad number");
        }
        boolean integral = true;
        if (i < s.length() && s.charAt(i) == '.') {
            integral = false;
            i++;
            if (!digits()) {
                throw fail("bad number");
            }
        }
        if (i < s.length() && (s.charAt(i) == 'e' || s.charAt(i) == 'E')) {
            integral = false;
            i++;
            if (i < s.length() && (s.charAt(i) == '+' || s.charAt(i) == '-')) {
                i++;
            }
            if (!digits()) {
                throw fail("bad number");
            }
        }
        String text = s.substring(start, i);
        if (integral) {
            try {
                return Long.parseLong(text);
            } catch (NumberFormatException tooBig) {
                // Out of long range: kept as a double.
            }
        }
        return Double.parseDouble(text);
    }

    private boolean digits() {
        int start = i;
        while (i < s.length() && s.charAt(i) >= '0' && s.charAt(i) <= '9') {
            i++;
        }
        return i > start;
    }

    // ---- writing ----

    static String write(Object v) {
        StringBuilder sb = new StringBuilder();
        write(v, sb);
        return sb.toString();
    }

    private static void write(Object v, StringBuilder sb) {
        if (v == null) {
            sb.append("null");
        } else if (v instanceof String str) {
            writeString(str, sb);
        } else if (v instanceof Character ch) {
            writeString(ch.toString(), sb);
        } else if (v instanceof Boolean || v instanceof Integer || v instanceof Long
                || v instanceof Short || v instanceof Byte) {
            sb.append(v);
        } else if (v instanceof Double || v instanceof Float) {
            double d = ((Number) v).doubleValue();
            if (Double.isNaN(d) || Double.isInfinite(d)) {
                throw new DojoError("returned " + d + ", which isn't a number JSON can hold");
            }
            sb.append(d);
        } else if (v instanceof Map<?, ?> m) {
            sb.append('{');
            boolean first = true;
            for (Map.Entry<?, ?> e : m.entrySet()) {
                if (!first) {
                    sb.append(',');
                }
                first = false;
                writeString(String.valueOf(e.getKey()), sb);
                sb.append(':');
                write(e.getValue(), sb);
            }
            sb.append('}');
        } else if (v instanceof Iterable<?> it) {
            sb.append('[');
            boolean first = true;
            for (Object x : it) {
                if (!first) {
                    sb.append(',');
                }
                first = false;
                write(x, sb);
            }
            sb.append(']');
        } else if (v.getClass().isArray()) {
            sb.append('[');
            int n = Array.getLength(v);
            for (int k = 0; k < n; k++) {
                if (k > 0) {
                    sb.append(',');
                }
                write(Array.get(v, k), sb);
            }
            sb.append(']');
        } else {
            writeString(v.toString(), sb);
        }
    }

    private static void writeString(String str, StringBuilder sb) {
        sb.append('"');
        for (int k = 0; k < str.length(); k++) {
            char c = str.charAt(k);
            switch (c) {
                case '"' -> sb.append("\\\"");
                case '\\' -> sb.append("\\\\");
                case '\n' -> sb.append("\\n");
                case '\r' -> sb.append("\\r");
                case '\t' -> sb.append("\\t");
                case '\b' -> sb.append("\\b");
                case '\f' -> sb.append("\\f");
                default -> {
                    if (c < 0x20) {
                        sb.append(String.format("\\u%04x", (int) c));
                    } else if (Character.isHighSurrogate(c) && k + 1 < str.length()
                            && Character.isLowSurrogate(str.charAt(k + 1))) {
                        sb.append(c).append(str.charAt(++k));
                    } else if (Character.isSurrogate(c)) {
                        sb.append('�'); // a lone half of a pair isn't valid text
                    } else {
                        sb.append(c);
                    }
                }
            }
        }
        sb.append('"');
    }
}
