// dojo's C++ driver support, compiled with every C++ solution. Same protocol
// as harness.py: read the spec, run each case, rewrite the results file after
// every case and name the current step in the progress file. Comparison
// happens in dojo.
//
// Included before the solution, so it also provides what LeetCode does: the
// common standard headers, `using namespace std;`, ListNode and TreeNode.
#ifndef DOJO_SUPPORT_HPP
#define DOJO_SUPPORT_HPP

#include <algorithm>
#include <array>
#include <bitset>
#include <cassert>
#include <cctype>
#include <cerrno>
#include <climits>
#include <cmath>
#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <deque>
#include <exception>
#include <fstream>
#include <functional>
#include <iomanip>
#include <iostream>
#include <iterator>
#include <limits>
#include <list>
#include <map>
#include <memory>
#include <numeric>
#include <optional>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <stack>
#include <stdexcept>
#include <string>
#include <string_view>
#include <tuple>
#include <type_traits>
#include <typeinfo>
#include <unordered_map>
#include <unordered_set>
#include <utility>
#include <vector>

#include <chrono>
#include <cxxabi.h>
#include <pthread.h>
#include <signal.h>
#include <thread>
#include <unistd.h>

using namespace std;

// ListNode and TreeNode as on LeetCode.
struct ListNode {
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

namespace dojo {

const size_t STDOUT_CAP = 4000;
const size_t MAX_NODES = 100000;
// Cases run on a thread with this much stack, so deep recursion works.
const size_t STACK_SIZE = 512u * 1024 * 1024;

// ---- JSON -----------------------------------------------------------------

struct Json {
    enum Kind { Null, Bool, Int, Double, String, Array, Object } kind = Null;
    bool b = false;
    long long i = 0;
    double d = 0;
    std::string s;
    std::vector<Json> items;
    std::vector<std::pair<std::string, Json>> fields;

    const Json *get(const std::string &key) const {
        for (const auto &f : fields) {
            if (f.first == key) return &f.second;
        }
        return nullptr;
    }
};

class Parser {
  public:
    explicit Parser(const std::string &text) : t(text) {}

    Json parse() {
        Json v = value(0);
        ws();
        if (p != t.size()) fail("trailing data");
        return v;
    }

  private:
    const std::string &t;
    size_t p = 0;

    [[noreturn]] void fail(const char *why) {
        throw std::runtime_error(std::string("dojo: bad JSON (") + why + ") at byte " +
                                 std::to_string(p));
    }
    void ws() {
        while (p < t.size() && (t[p] == ' ' || t[p] == '\n' || t[p] == '\r' || t[p] == '\t')) p++;
    }
    bool lit(const char *word) {
        size_t n = std::strlen(word);
        if (t.compare(p, n, word) == 0) {
            p += n;
            return true;
        }
        return false;
    }
    Json value(int depth) {
        if (depth > 1000) fail("nested too deeply");
        ws();
        if (p >= t.size()) fail("unexpected end");
        Json v;
        char c = t[p];
        if (c == '{') {
            v.kind = Json::Object;
            p++;
            ws();
            if (p < t.size() && t[p] == '}') {
                p++;
                return v;
            }
            for (;;) {
                ws();
                if (p >= t.size() || t[p] != '"') fail("expected a key");
                std::string key = str();
                ws();
                if (p >= t.size() || t[p] != ':') fail("expected ':'");
                p++;
                v.fields.emplace_back(std::move(key), value(depth + 1));
                ws();
                if (p < t.size() && t[p] == ',') {
                    p++;
                } else if (p < t.size() && t[p] == '}') {
                    p++;
                    return v;
                } else {
                    fail("expected ',' or '}'");
                }
            }
        }
        if (c == '[') {
            v.kind = Json::Array;
            p++;
            ws();
            if (p < t.size() && t[p] == ']') {
                p++;
                return v;
            }
            for (;;) {
                v.items.push_back(value(depth + 1));
                ws();
                if (p < t.size() && t[p] == ',') {
                    p++;
                } else if (p < t.size() && t[p] == ']') {
                    p++;
                    return v;
                } else {
                    fail("expected ',' or ']'");
                }
            }
        }
        if (c == '"') {
            v.kind = Json::String;
            v.s = str();
            return v;
        }
        if (lit("true")) {
            v.kind = Json::Bool;
            v.b = true;
            return v;
        }
        if (lit("false")) {
            v.kind = Json::Bool;
            return v;
        }
        if (lit("null")) return v;
        if (c == '-' || (c >= '0' && c <= '9')) return number();
        fail("unexpected character");
    }
    Json number() {
        size_t start = p;
        bool integral = true;
        if (t[p] == '-') p++;
        if (p >= t.size() || !std::isdigit(static_cast<unsigned char>(t[p]))) fail("bad number");
        while (p < t.size() && std::isdigit(static_cast<unsigned char>(t[p]))) p++;
        if (p < t.size() && t[p] == '.') {
            integral = false;
            p++;
            if (p >= t.size() || !std::isdigit(static_cast<unsigned char>(t[p]))) fail("bad number");
            while (p < t.size() && std::isdigit(static_cast<unsigned char>(t[p]))) p++;
        }
        if (p < t.size() && (t[p] == 'e' || t[p] == 'E')) {
            integral = false;
            p++;
            if (p < t.size() && (t[p] == '+' || t[p] == '-')) p++;
            if (p >= t.size() || !std::isdigit(static_cast<unsigned char>(t[p]))) fail("bad number");
            while (p < t.size() && std::isdigit(static_cast<unsigned char>(t[p]))) p++;
        }
        std::string text = t.substr(start, p - start);
        Json v;
        if (integral) {
            errno = 0;
            long long n = std::strtoll(text.c_str(), nullptr, 10);
            if (errno != ERANGE) {
                v.kind = Json::Int;
                v.i = n;
                v.d = static_cast<double>(n);
                return v;
            }
        }
        v.kind = Json::Double;
        v.d = std::strtod(text.c_str(), nullptr);
        return v;
    }
    unsigned hex4() {
        if (p + 4 > t.size()) fail("bad \\u escape");
        unsigned u = 0;
        for (int k = 0; k < 4; k++) {
            char h = t[p++];
            u <<= 4;
            if (h >= '0' && h <= '9') u |= h - '0';
            else if (h >= 'a' && h <= 'f') u |= h - 'a' + 10;
            else if (h >= 'A' && h <= 'F') u |= h - 'A' + 10;
            else fail("bad \\u escape");
        }
        return u;
    }
    static void utf8(std::string &out, unsigned cp) {
        if (cp < 0x80) {
            out += static_cast<char>(cp);
        } else if (cp < 0x800) {
            out += static_cast<char>(0xC0 | (cp >> 6));
            out += static_cast<char>(0x80 | (cp & 0x3F));
        } else if (cp < 0x10000) {
            out += static_cast<char>(0xE0 | (cp >> 12));
            out += static_cast<char>(0x80 | ((cp >> 6) & 0x3F));
            out += static_cast<char>(0x80 | (cp & 0x3F));
        } else {
            out += static_cast<char>(0xF0 | (cp >> 18));
            out += static_cast<char>(0x80 | ((cp >> 12) & 0x3F));
            out += static_cast<char>(0x80 | ((cp >> 6) & 0x3F));
            out += static_cast<char>(0x80 | (cp & 0x3F));
        }
    }
    std::string str() {
        p++; // opening quote
        std::string out;
        for (;;) {
            if (p >= t.size()) fail("unterminated string");
            char c = t[p++];
            if (c == '"') return out;
            if (c != '\\') {
                out += c;
                continue;
            }
            if (p >= t.size()) fail("unterminated string");
            char e = t[p++];
            switch (e) {
            case '"': out += '"'; break;
            case '\\': out += '\\'; break;
            case '/': out += '/'; break;
            case 'b': out += '\b'; break;
            case 'f': out += '\f'; break;
            case 'n': out += '\n'; break;
            case 'r': out += '\r'; break;
            case 't': out += '\t'; break;
            case 'u': {
                unsigned cp = hex4();
                if (cp >= 0xD800 && cp < 0xDC00 && p + 6 <= t.size() && t[p] == '\\' &&
                    t[p + 1] == 'u') {
                    size_t save = p;
                    p += 2;
                    unsigned lo = hex4();
                    if (lo >= 0xDC00 && lo < 0xE000) {
                        cp = 0x10000 + ((cp - 0xD800) << 10) + (lo - 0xDC00);
                    } else {
                        p = save;
                    }
                }
                if (cp >= 0xD800 && cp < 0xE000) cp = 0xFFFD; // lone surrogate
                utf8(out, cp);
                break;
            }
            default: fail("bad escape");
            }
        }
    }
};

// A JSON string literal. Invalid UTF-8 becomes U+FFFD so the results file
// stays valid JSON whatever the solution returns or prints.
inline std::string quote(const std::string &s) {
    std::string out = "\"";
    size_t i = 0, n = s.size();
    while (i < n) {
        unsigned char c = static_cast<unsigned char>(s[i]);
        if (c < 0x80) {
            switch (c) {
            case '"': out += "\\\""; break;
            case '\\': out += "\\\\"; break;
            case '\n': out += "\\n"; break;
            case '\r': out += "\\r"; break;
            case '\t': out += "\\t"; break;
            case '\b': out += "\\b"; break;
            case '\f': out += "\\f"; break;
            default:
                if (c < 0x20 || c == 0x7F) {
                    char buf[8];
                    std::snprintf(buf, sizeof buf, "\\u%04x", c);
                    out += buf;
                } else {
                    out += static_cast<char>(c);
                }
            }
            i++;
            continue;
        }
        size_t len = c >= 0xC2 && c <= 0xDF ? 2 : c >= 0xE0 && c <= 0xEF ? 3 : c >= 0xF0 && c <= 0xF4 ? 4 : 0;
        bool ok = len > 0 && i + len <= n;
        for (size_t k = 1; ok && k < len; k++) {
            ok = (static_cast<unsigned char>(s[i + k]) & 0xC0) == 0x80;
        }
        if (ok && len == 3) {
            unsigned char c1 = static_cast<unsigned char>(s[i + 1]);
            ok = !(c == 0xE0 && c1 < 0xA0) && !(c == 0xED && c1 >= 0xA0); // overlong, surrogate
        }
        if (ok && len == 4) {
            unsigned char c1 = static_cast<unsigned char>(s[i + 1]);
            ok = !(c == 0xF0 && c1 < 0x90) && !(c == 0xF4 && c1 >= 0x90); // overlong, > U+10FFFF
        }
        if (ok) {
            out.append(s, i, len);
            i += len;
        } else {
            out += "\xEF\xBF\xBD";
            i++;
        }
    }
    out += '"';
    return out;
}

// ---- Decoding inputs -------------------------------------------------------

template <class T> struct is_vector : std::false_type {};
template <class T, class A> struct is_vector<std::vector<T, A>> : std::true_type {};
template <class T> struct always_false : std::false_type {};

// An input that doesn't decode into the parameter's type.
struct InputError : std::runtime_error {
    using std::runtime_error::runtime_error;
};

[[noreturn]] inline void mismatch(const char *expected) {
    throw InputError(std::string("is not ") + expected);
}

inline ListNode *make_list(const Json &j) {
    if (j.kind != Json::Array) mismatch("a ListNode array");
    ListNode dummy;
    ListNode *tail = &dummy;
    for (const Json &v : j.items) {
        if (v.kind != Json::Int) mismatch("a ListNode array");
        tail->next = new ListNode(static_cast<int>(v.i));
        tail = tail->next;
    }
    return dummy.next;
}

inline TreeNode *make_tree(const Json &j) {
    if (j.kind != Json::Array) mismatch("a TreeNode array");
    const auto &values = j.items;
    auto node = [&](size_t k) -> TreeNode * {
        if (values[k].kind == Json::Null) return nullptr;
        if (values[k].kind != Json::Int) mismatch("a TreeNode array");
        return new TreeNode(static_cast<int>(values[k].i));
    };
    if (values.empty() || values[0].kind == Json::Null) return nullptr;
    TreeNode *root = node(0);
    std::vector<TreeNode *> queue{root};
    size_t i = 1;
    for (size_t q = 0; q < queue.size() && i < values.size(); q++) {
        TreeNode *parent = queue[q];
        parent->left = node(i++);
        if (parent->left) queue.push_back(parent->left);
        if (i < values.size()) {
            parent->right = node(i++);
            if (parent->right) queue.push_back(parent->right);
        }
    }
    return root;
}

template <class T> T decode(const Json &j) {
    if constexpr (std::is_same_v<T, bool>) {
        if (j.kind != Json::Bool) mismatch("a bool");
        return j.b;
    } else if constexpr (std::is_same_v<T, int>) {
        if (j.kind != Json::Int || j.i < INT_MIN || j.i > INT_MAX) mismatch("an int");
        return static_cast<int>(j.i);
    } else if constexpr (std::is_same_v<T, long long>) {
        if (j.kind != Json::Int) mismatch("a long");
        return j.i;
    } else if constexpr (std::is_same_v<T, double>) {
        if (j.kind != Json::Int && j.kind != Json::Double) mismatch("a number");
        return j.d;
    } else if constexpr (std::is_same_v<T, std::string>) {
        if (j.kind != Json::String) mismatch("a string");
        return j.s;
    } else if constexpr (std::is_same_v<T, ListNode *>) {
        return make_list(j);
    } else if constexpr (std::is_same_v<T, TreeNode *>) {
        return make_tree(j);
    } else if constexpr (is_vector<T>::value) {
        if (j.kind != Json::Array) mismatch("an array");
        T out;
        out.reserve(j.items.size());
        for (const Json &item : j.items) out.push_back(decode<typename T::value_type>(item));
        return out;
    } else {
        static_assert(always_false<T>::value, "dojo can't decode this type");
    }
}

// One input of a case, by parameter name.
template <class T> T input(const Json &inputs, const char *name) {
    const Json *v = inputs.get(name);
    if (!v) throw InputError(std::string("dojo: input \"") + name + "\" missing");
    try {
        return decode<T>(*v);
    } catch (const InputError &e) {
        throw InputError(std::string("dojo: input \"") + name + "\" " + e.what());
    }
}

// ---- Encoding results --------------------------------------------------------

inline std::string number(double d) {
    if (!std::isfinite(d)) return "null";
    char buf[32];
    // The shortest form that reads back as the same double.
    for (int precision = 15; precision <= 17; precision++) {
        std::snprintf(buf, sizeof buf, "%.*g", precision, d);
        if (std::strtod(buf, nullptr) == d) break;
    }
    std::string s = buf;
    if (s.find_first_of(".eE") == std::string::npos) s += ".0";
    return s;
}

template <class T> std::string encode(const T &v) {
    using U = std::decay_t<T>;
    if constexpr (std::is_same_v<U, bool>) {
        return v ? "true" : "false";
    } else if constexpr (std::is_same_v<U, ListNode *> || std::is_same_v<U, const ListNode *>) {
        std::string out = "[";
        size_t n = 0;
        for (const ListNode *node = v; node; node = node->next) {
            if (++n > MAX_NODES) throw std::runtime_error("returned linked list is too long (cycle?)");
            if (n > 1) out += ',';
            out += std::to_string(node->val);
        }
        return out + "]";
    } else if constexpr (std::is_same_v<U, TreeNode *> || std::is_same_v<U, const TreeNode *>) {
        std::vector<const TreeNode *> queue{v};
        std::vector<std::string> out;
        for (size_t q = 0; q < queue.size(); q++) {
            if (out.size() >= MAX_NODES) throw std::runtime_error("returned tree is too large (cycle?)");
            const TreeNode *node = queue[q];
            if (!node) {
                out.push_back("null");
                continue;
            }
            out.push_back(std::to_string(node->val));
            queue.push_back(node->left);
            queue.push_back(node->right);
        }
        while (!out.empty() && out.back() == "null") out.pop_back();
        std::string s = "[";
        for (size_t k = 0; k < out.size(); k++) {
            if (k) s += ',';
            s += out[k];
        }
        return s + "]";
    } else if constexpr (std::is_integral_v<U>) {
        return std::to_string(v);
    } else if constexpr (std::is_floating_point_v<U>) {
        return number(static_cast<double>(v));
    } else if constexpr (std::is_convertible_v<const U &, std::string_view>) {
        return quote(std::string(std::string_view(v)));
    } else if constexpr (is_vector<U>::value) {
        using E = typename U::value_type;
        std::string out = "[";
        bool first = true;
        for (const E &item : v) {
            if (!first) out += ',';
            first = false;
            out += encode(item);
        }
        return out + "]";
    } else {
        static_assert(always_false<U>::value, "dojo can't return this type");
    }
}

// ---- Running cases -------------------------------------------------------

using Case = std::string (*)(const Json &input);

struct Result {
    long long index = 0;
    bool ok = false;
    std::string got, error, out;
    double ms = 0;
};

inline void write_file(const std::string &path, const std::string &data) {
    FILE *f = std::fopen(path.c_str(), "wb");
    if (!f) return;
    std::fwrite(data.data(), 1, data.size(), f);
    std::fclose(f);
}

inline void write_results(const std::string &path, const std::vector<Result> &results) {
    std::string s = "{\"results\":[";
    for (size_t k = 0; k < results.size(); k++) {
        const Result &r = results[k];
        if (k) s += ',';
        s += "{\"index\":" + std::to_string(r.index);
        s += r.ok ? ",\"status\":\"ok\",\"got\":" + r.got : ",\"status\":\"error\",\"error\":" + quote(r.error);
        if (!r.out.empty()) s += ",\"stdout\":" + quote(r.out);
        s += ",\"ms\":" + number(std::round(r.ms * 1000) / 1000) + "}";
    }
    write_file(path, s + "]}");
}

inline std::string demangle(const char *name) {
    int status = 0;
    char *real = abi::__cxa_demangle(name, nullptr, nullptr, &status);
    std::string out = status == 0 && real ? real : name;
    std::free(real);
    // libstdc++ and libc++ name their inline namespaces; users know std::.
    for (const char *ns : {"std::__1::", "std::__cxx11::"}) {
        for (size_t at; (at = out.find(ns)) != std::string::npos;) out.replace(at, std::strlen(ns), "std::");
    }
    return out;
}

// Sends stdout (printf and cout alike) into a pipe while a case runs; a
// thread keeps the first STDOUT_CAP bytes and drains the rest.
class Capture {
  public:
    Capture() {
        std::fflush(stdout);
        std::cout.flush();
        int fds[2];
        if (pipe(fds) != 0) return;
        saved = dup(1);
        if (saved < 0 || dup2(fds[1], 1) < 0) {
            close(fds[0]);
            close(fds[1]);
            return;
        }
        close(fds[1]);
        reader = std::thread([this, fd = fds[0]] {
            char buf[4096];
            for (;;) {
                ssize_t n = read(fd, buf, sizeof buf);
                if (n < 0 && errno == EINTR) continue;
                if (n <= 0) break;
                size_t room = STDOUT_CAP + 1 > kept.size() ? STDOUT_CAP + 1 - kept.size() : 0;
                kept.append(buf, std::min(room, static_cast<size_t>(n)));
            }
            close(fd);
        });
    }

    // What was printed, capped.
    std::string finish() {
        std::cout.flush();
        std::fflush(stdout);
        if (saved >= 0) {
            dup2(saved, 1); // closes the pipe's last write end: the reader sees EOF
            close(saved);
            saved = -1;
        }
        if (reader.joinable()) reader.join();
        std::string out = kept.substr(0, STDOUT_CAP);
        if (kept.size() > STDOUT_CAP) out += "\n… (more output not shown)";
        while (!out.empty() && out.back() == '\n') out.pop_back();
        return out;
    }

  private:
    int saved = -1;
    std::thread reader;
    std::string kept;
};

inline double now_ms() {
    using namespace std::chrono;
    return duration<double, std::milli>(steady_clock::now().time_since_epoch()).count();
}

// A crash can't be recovered: say why on stderr and exit. dojo reports it on
// the case named in the progress file.
inline char *stack_low = nullptr;

inline void on_fatal(int sig, siginfo_t *info, void *) {
    const char *msg = "fatal error: segmentation fault (a null or invalid pointer, or out-of-bounds access?)\n";
    char *addr = static_cast<char *>(info ? info->si_addr : nullptr);
    if (sig == SIGFPE) {
        msg = "fatal error: arithmetic exception (integer division by zero?)\n";
    } else if (stack_low && addr && addr < stack_low + 65536 && addr + (64u << 20) > stack_low) {
        msg = "fatal error: stack overflow (recursion too deep?)\n";
    } else if (sig == SIGBUS) {
        msg = "fatal error: bus error (invalid memory access?)\n";
    }
    ssize_t ignored = write(2, msg, std::strlen(msg));
    (void)ignored;
    _exit(2);
}

// Handles crashes on an alternate stack (the thread's own may be full).
inline void catch_crashes() {
    static thread_local std::vector<char> alt(1 << 16);
    stack_t ss{};
    ss.ss_sp = alt.data();
    ss.ss_size = alt.size();
    sigaltstack(&ss, nullptr);
    struct sigaction sa {};
    sa.sa_sigaction = on_fatal;
    sa.sa_flags = SA_SIGINFO | SA_ONSTACK;
    sigemptyset(&sa.sa_mask);
    sigaction(SIGSEGV, &sa, nullptr);
    sigaction(SIGBUS, &sa, nullptr);
    sigaction(SIGFPE, &sa, nullptr);
}

struct Job {
    Case call;
    std::string spec_path, results_path;
    bool big_stack;
};

inline void progress(const std::string &path, const std::string &step) { write_file(path, step); }

inline void run_cases(const Job &job) {
    char top;
    if (job.big_stack) stack_low = &top - STACK_SIZE;
    catch_crashes();
    std::string progress_path = job.results_path + ".progress";
    progress(progress_path, "load");
    std::ifstream in(job.spec_path, std::ios::binary);
    std::stringstream text;
    text << in.rdbuf();
    std::string data = text.str();
    Json spec = Parser(data).parse();
    const Json *cases = spec.get("cases");
    std::vector<Result> results;
    if (cases) {
        for (const Json &c : cases->items) {
            Result r;
            const Json *index = c.get("index");
            r.index = index ? index->i : 0;
            progress(progress_path, std::to_string(r.index));
            const Json *input = c.get("input");
            Json none;
            Capture capture;
            double start = now_ms();
            try {
                r.got = job.call(input ? *input : none);
                r.ok = true;
            } catch (const InputError &e) {
                r.error = e.what();
            } catch (const std::exception &e) {
                r.error = "uncaught exception " + demangle(typeid(e).name()) + ": " + e.what();
            } catch (...) {
                std::type_info *t = abi::__cxa_current_exception_type();
                r.error = std::string("uncaught exception of type ") + (t ? demangle(t->name()) : "unknown");
            }
            r.ms = now_ms() - start;
            r.out = capture.finish();
            results.push_back(std::move(r));
            write_results(job.results_path, results);
        }
    }
    progress(progress_path, "done");
    write_results(job.results_path, results);
}

inline void *thread_main(void *job) {
    run_cases(*static_cast<Job *>(job));
    return nullptr;
}

// Runs every case through `call` (generated per question), on a thread with
// a large stack.
inline int run(int argc, char **argv, Case call) {
    if (argc < 3) {
        std::fprintf(stderr, "usage: %s <spec.json> <results.json>\n", argv[0]);
        return 2;
    }
    Job job{call, argv[1], argv[2], true};
    pthread_attr_t attr;
    pthread_t thread;
    if (pthread_attr_init(&attr) == 0 && pthread_attr_setstacksize(&attr, STACK_SIZE) == 0 &&
        pthread_create(&thread, &attr, thread_main, &job) == 0) {
        pthread_join(thread, nullptr);
    } else {
        job.big_stack = false; // none available: run on this thread's stack
        run_cases(job);
    }
    return 0;
}

} // namespace dojo

#endif
