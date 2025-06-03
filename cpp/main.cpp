#include <algorithm>
#include <bitset>
#include <cassert>
#include <cctype>
#include <climits>
#include <cstddef>
#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <deque>
#include <exception>
#include <functional>
#include <future>
#include <iostream>
#include <istream>
#include <iterator>
#include <limits>
#include <locale>
#include <map>
#include <numeric>
#include <ostream>
#include <queue>
#include <set>
#include <stack>
#include <stdexcept>
#include <string>
#include <sys/types.h>
#include <system_error>
#include <tuple>
#include <type_traits>
#include <unordered_map>
#include <utility>
#include <valarray>
#include <vector>
#include <cmath>
#include <concepts>
#include <type_traits>

#define vec vector

// マクロオーバーロード用のヘルパー
#define GET_MACRO(_1, _2, _3, NAME, ...) NAME

#define REP1(n) for (ll i=0; i<(ll)(n); i++)
#define REP2(i, n) for (ll i=0; i<(ll)(n); i++)
#define REP3(i, begin, end) for (ll i=(ll)(begin); i<(ll)(end); i++)
#define rep(...) GET_MACRO(__VA_ARGS__, REP3, REP2, REP1)(__VA_ARGS__)

#define RREP1(n) for (ll i=(ll)(n)-1; i>=0; i--)
#define RREP2(i, n) for (ll i=(ll)(n)-1; i>=0; i--)
#define RREP3(i, begin, end) for (ll i=(ll)(end)-1; i>=(ll)(begin); i--)
#define rrep(...) GET_MACRO(__VA_ARGS__, RREP3, RREP2, RREP1)(__VA_ARGS__)

#define drep(i, j, ibegin, iend, jbegin, jend) \
	rep(i, ibegin, iend) rep(j, jbegin, jend)

#define trep(i, j, k, ibegin, iend, jbegin, jend, kbegin, kend) \
	rep(i, ibegin, iend) rep(j, jbegin, jend) rep(k, kbegin, kend)

#define arep(elm, iter) for (auto&& elm: iter)

// ==================================================

// 定数とかの宣言

using namespace std;

constexpr char el = '\n';

using ll = long long;
const ll INF = 5LL << 60;

template <typename T>
void chmin(T& x, const T& y) { x = min(x, y); }

template <typename T>
void chmax(T& x, const T& y) { x = max(x, y); }

// ==================================================

template <typename T>
using Graph = vector<vector<T>>;

template <typename T>
using CGraph = vector<vector<pair<ll, T>>>;

template <typename T>
using Grid = vector<vector<T>>;

// ==================================================

// 引数まとめて出力関数

template <typename... Args>
void out(const Args&... args) {
    ((cout << args << " "), ...);
    cout << '\n';
}

// ==================================================

// 解答出力関数
// yes(true) -> yes; exit
// no(false) -> yes; exit
// yes(bool) -> (bool ? yes : no); exit

void yes_print(bool cdt=true) {
    if (cdt) {
      cout << "Yes" << el;
      exit(0);
    }
  }

void no_print(bool cdt=false) {
  if (!cdt) {
    cout << "No" << el;
    exit(0);
  }
}

void yes_no_print(bool cdt, bool fin=true) {
  cout << (cdt ? "Yes" : "No") << el;

	if (fin) exit(0);
}

// ==================================================

// イテレータ出力関数( iter, stack, queue, priority_queue, 多次元配列 )

template <typename Container>
auto print_container(const Container& c) -> decltype(c.begin(), void()) {
	for (const auto& x: c) cout << x << " ";
	cout << el;
}

template <typename T1, typename T2>
void print_container(const pair<T1, T2>& p) {
	cout << "(" << p.first << ", " << p.second << ")" << el;
}

template <typename T>
void print_container(stack<T> s) {
	while (!s.empty()) {
		cout << s.top() << " "; s.pop();
	}
	cout << el;
}

template <typename T>
void print_container(queue<T> q) {
	while (!q.empty()) {
		cout << q.front() << " "; q.pop();
	}
	cout << el;
}

template <typename T>
void print_container(priority_queue<T> pq) {
	while (!pq.empty()) {
		cout << pq.top() << " "; pq.pop();
	}
	cout << el;
}

template <typename T>
void print_ncontainer(const T& x) {
	if constexpr (requires { x.begin(); }) {
		for (const auto& e: x) print_container(e);
	} else {
		cout << x << " ";
	}
}

// ==================================================

class tool {
	int8_t value;

	public:
		constexpr tool(ll val=0): value((val == 0 ? 0 : (val > 0 ? 1 : -1))) {}

		static const tool zero;
		static const tool plus;
		static const tool minus;

		constexpr operator int8_t() const { return value; }
		constexpr operator bool() const { return value != 0; }

		friend constexpr bool operator==(tool a, tool b) { return a.value == b.value; }
		friend constexpr bool operator!=(tool a, tool b) { return !(a == b); }

		friend ostream& operator<<(ostream& os, const tool& t) {
			switch (t.value) {
				case 0: return os << "zero";
				case 1: return os << "plus";
				case -1: return os << "minus";
				default: return os << "INVALID";
			}
		}

		constexpr int8_t raw() const { return value; }
};

const tool tool::zero{0};
const tool tool::plus{1};
const tool tool::minus{-1};

// ==================================================

// 上下左右回転反転可能二次元正方行列

template <typename T>
class RotatedGrid {
	private:
		Grid<T> content;

	public:
		T rotate_element(ll n, ll row, ll col) const {
			ll size = content.size();
			switch (n) {
				case 0: return content[row][col];
				case 1: return content[col][size-row-1];
				case 2: return content[size-col-1][size-row-1];
				case 3: return content[size-col-1][row];
				default: exit(1);
			}
		}

		void rotate(ll n) {
			n = ((n%4)+4)%4;
			ll size = content.size();

			Grid<T> rotated(size, vector<T>(size));

			for (ll r=0; r<size; r++) for (ll c=0; c<size; c++) {
				rotated[r][c] = rotate_element(n, r, c);
			}

			content = rotated;
		}

		T horizontal_element(ll row, ll col) const {
			ll size = content.size();
			return content[size-row-1][col];
		}

		void horizontal() {
			ll size = content.size();
			vector<vector<T>> horizontaled(size, vector<T>(size));
			
			for (ll r=0; r<size; r++) for (ll c=0; c<size; c++) {
				horizontaled[r][c] = horizontal_element(r, c);
			}

			content = horizontaled;
		}


		T vertical_element(ll row, ll col) const {
			ll size = content.size();
			return content[row][size-col-1];
		}

		void vertical() {
			ll size = content.size();
			vector<vector<T>> verticaled(size, vector<T>(size));
			
			for (ll r=0; r<size; r++) for (ll c=0; c<size; c++) {
				verticaled[r][c] = vertical_element(r, c);
			}

			content = verticaled;
		}

		RotatedGrid(vector<vector<T>> grid) : content(grid) {}

		auto begin() const {
			return content.begin();
		}

		auto end() const {
			return content.end();
		}

		const vector<T>& operator[](ll idx) const {
			return content[idx];
		}

		vector<T>& operator[](ll idx) {
			return content[idx];
		}

		T operator<(tuple<ll, ll, ll> t) const {
			ll n = get<0>(t), row = get<1>(t), col = get<2>(t);
			return this->rotate_element(((n%4)+4)%4, row, col);
		}

		T operator>(tuple<ll, ll, ll> t) const {
			ll n = get<0>(t), row = get<1>(t), col = get<2>(t);
			return this->rotate_element(((n%4)+6)%4, row, col);
		}

		RotatedGrid& operator<<=(ll n) {
			rotate(((n%4)+4)%4);
			return *this;
		}

		RotatedGrid& operator>>=(ll n) {
			rotate(((n%4)+6)%4);
			return *this;
		}

		T operator-(tuple<ll, ll>t) const {
			ll row = get<0>(t), col = get<1>(t);
			return this->horizontal_element(row, col);
		}

		RotatedGrid& operator-=(int) {
			horizontal();
			return *this;
		}

		T operator|(tuple<ll, ll>t) const {
			ll row = get<0>(t), col = get<1>(t);
			return this->vertical_element(row, col);
		}

		RotatedGrid& operator|=(int) {
			vertical();
			return *this;
		}
};

// ==================================================

// 剰余整数型(long long)
template <typename T>
using integer = enable_if_t<is_integral_v<T>>;

template <ll MOD>
class ModInt {
	private:
		ll val;

		static constexpr ll normalize(ll x) {
			x %= MOD;
			if (x < 0) x += MOD;
			return x;
		}

	public:
		template <typename T, typename = integer<T>>
		ModInt(T v=0) : val(normalize(static_cast<ll>(v))) {}
		ModInt() : val(normalize(static_cast<ll>(0))) {}

		operator ll() const { return val; }

		template <typename T> ModInt operator+(T rhs) { return ModInt(val + ModInt(rhs)); }
		template <typename T> ModInt operator-(T rhs) { return ModInt(val - ModInt(rhs)); }
		template <typename T> ModInt operator*(T rhs) { return ModInt(val * ModInt(rhs)); }
		template <typename T> ModInt operator/(T rhs) { return *this * ModInt(rhs).inv(); }

		template <typename T> ModInt& operator+=(T rhs) { return *this = *this + rhs; }
		template <typename T> ModInt& operator-=(T rhs) { return *this = *this - rhs; }
		template <typename T> ModInt& operator*=(T rhs) { return *this = *this * rhs; }
		template <typename T> ModInt& operator/=(T rhs) { return *this = *this / rhs; }

		ModInt pow(ll exp) const {
			ModInt res(1), base(val);
			while (exp) {
				if (exp & 1) res *= base;
				base *= base;
				exp >>= 1;
			}
			return res;
		}

		ModInt inv() const {
			return pow(MOD - 2);
		}
};

template <ll MOD>
ostream& operator<<(ostream& os, const ModInt<MOD>& m) {
	return os << static_cast<ll>(m);
}

template <ll MOD>
istream& operator>>(istream& is, ModInt<MOD>& m) {
	ll x; is >> x;
	m = ModInt<MOD>(x);
	return is;
}
		
// ==================================================

// 繰り返し二乗法

ll pow(ll x, ll p) {
	ll rlt = 1;
	while (p) {
		if (p&1) rlt *= x;
		x *= x;
		p >>= 1;
	}
	return rlt;
}

ll pow(ll x, ll p, ll mod) {
	ll rlt = 1;
	while (p) {
		if (p&1) rlt = rlt * x % mod;
		x = x * x % mod;
		p >>= 1;
	}
	return rlt;
}

// binary Greater Common Divisor

ll binGCD(ll x, ll y, ll k=0) {
	if (x == 0) return y << k;
	if (y == 0) return x << k;

	if (x < y) swap(x, y);

	if (x&1 && y&1) return binGCD((x-y)>>1, y, k);
	else if (!(x&1 || y&1)) return binGCD(x>>1, y>>1, k+1);
	else return binGCD((x&1 ? x : x>>1), (y&1 ? y : y>>1), k);
}

// Least Common Multiple

ll lcm(ll x, ll y) {
	return (x*y == 0) ? 0 : (x*y)/binGCD(x, y);
}

// LCS (longest Common Subsequence)
string lcs(string& s, string& t) {
	ll h = s.size(), w = t.size();
	
	Grid<ll> dp(h+1, vec<ll>(w+1, 0));
	
	rep(i, 1, h+1) {
		rep(j, 1, w+1) {
			if (s[i-1] == t[j-1]) {
				dp[i][j] = dp[i-1][j-1]+1;
			}
			else {
				if (dp[i-1][j] >= dp[i][j-1]) {
					dp[i][j] = dp[i-1][j];
				} else {
					dp[i][j] = dp[i][j-1];
				}
			}
		}
	}

	string lcs; lcs.reserve(dp[h][w]);
	ll i = h, j = w;
	while (i > 0 && j > 0) {
		if (s[i-1] == t[j-1]) {
			lcs += s[i-1];
			i--; j--;
		} else if (dp[i-1][j] >= dp[i][j-1]) {
			i--;
		} else {
			j--;
		}
	}
	reverse(lcs.begin(), lcs.end());

	return lcs;
}

// ==================================================

// 二分探索

template <typename T>
T binary_search(T left, T right, const function<bool(T)> cdt) {
	while (right - left > 1) {
		T middle = left + (right - left) / 2;
		if (cdt(middle)) {
			left = middle;
		} else {
			right = middle;
		}
	}

	return right;
}

// ==================================================

class UnionFind {
	private:
		vector<ll> content;
		function<bool(ll, ll)> f;

	public:
		UnionFind(ll n) : content(n+1, -1), f([](ll, ll) { return true; }) {}
		UnionFind(ll n, function<bool(ll, ll)> f) : content(n+1, -1), f(f) {}

		ll search(ll target) {
			if (content[target] < 0) return target;

			return content[target] = search(content[target]);
		}

		void connect(ll from, ll to) {
			if (!f(from, to)) swap(from, to);

			ll target = search(to);
			if (!check(search(from), search(to))) content[from] = target;
			content[target]--;
		}

		bool check(ll x, ll y) {
			return (x == y) || (search(x) == search(y));
		}

		ll size(ll target) {
			return -content[search(target)]-1;
		}

		void print() {
			print_container(content);
		}
};

// ==================================================

template <typename T>
class BinaryTree {
	private:
		ll _size = 0;

		struct Node {
			T value;
			Node* left;
			Node* right;
			ll height;

			T subtree_min;
			T subtree_max;
			T subtree_sum;

			Node(T value): value(value), left(nullptr), right(nullptr), height(0), subtree_min(value), subtree_max(value), subtree_sum(value) {}
		};

		Node* root;
		function<bool(const T&, const T&)> f;
		
		void update(Node* node) {
			if (!node) return;

			node->subtree_min = node->value;
			node->subtree_max = node->value;
			node->subtree_sum = node->value;

			if (node->left) {
				node->subtree_min = std::min(node->subtree_min, node->left->subtree_min);
				node->subtree_max = std::max(node->subtree_max, node->left->subtree_max);
				node->subtree_sum += node->left->subtree_sum;
			}

			if (node->right) {
				node->subtree_min = std::min(node->subtree_min, node->right->subtree_min);
				node->subtree_max = std::max(node->subtree_max, node->right->subtree_max);
				node->subtree_sum += node->right->subtree_sum;
			}
		}

		Node* insert(Node*& node, const T value) {
			if (!node) {
				_size++;
				return new Node(value);
			}

			if (node->value == value) return node;

			if (f(node->value, value)) {
				node->left = insert(node->left, value);
			} else {
				node->right = insert(node->right, value);
			}
			update(node);
			return node;
		}

		bool search(Node*& node, const T value) const {
			if (!node) return false;
			if (node->value == value) return true;

			if (f(node->value, value)) return search(node->right, value);
			else return search(node->left, value);
		}

		Node* findMin(Node* node) const {
			while (node && node->left) node = node->left;
			return node;
		}

		Node* remove(Node*& node, const T value) {
			if (!node) return nullptr;

			if (node->value == value) {
				if (!node->left) {
					Node* r = node->right;
					delete node;
					_size--;
					return r;
				} else if (!node->right) {
					Node* l = node->left;
					delete node;
					_size--;
					return l;
				} else {
					Node* succ = findMin(node->right);
					node->value = succ->value;
					node->right = remove(node->right, succ->value);
				}
			} else if (f(node->value, value)) {
				node->left = remove(node->left, value);
			} else {
				node->right = remove(node->right, value);
			}

			update(node);
			return node;
		}

	public:
		BinaryTree(): root(nullptr), f([](const T& x, const T& y) { return x < y; }) {}
		BinaryTree(function<bool(const T&, const T&)> f): root(nullptr), f(f) {}

		void insert(const T value) { root = insert(root, value); }
		bool search(const T value) const { return search(root, value); }
		bool remove(const T value) { return remove(root, value); }

		T size() const { return _size; }
		T min() const { return root ? root->subtree_min : LLONG_MAX; }
		T max() const { return root ? root->subtree_max : LLONG_MIN; }
		T sum() const { return root ? root->subtree_sum : 0; }
};

// ==================================================

template <typename T>
class SegmentTree {
	protected:
		using F = function<T(T, T)>;

		ll n = 1;
		vector<T> node;
		T identify;
		F f;

	public:
		SegmentTree(
			vector<T>& array, 
			T identify,
			F f
		) : identify(identify), f(f) {
			ll size = array.size();
			while (n < size) n <<= 1;

			node.assign(2*n-1, identify);

			for(ll i=0; i<size; i++) node[i+n-1] = array[i];
			for(ll i=n-2; 0<=i; --i) node[i] = f(node[2*i+1], node[2*i+2]);
		}

		void update(ll idx, ll v) {
			idx += n-1;
			node[idx] = v;
			while(idx > 0) {
				idx = (idx-1)/2;
				node[idx] = f(node[2*idx+1], node[2*idx+2]);
			}
		}

		ll get(ll l, ll r, ll current=0, ll ldx=0, ll rdx=-1) {
			if (rdx == -1) rdx = n;
			if (rdx <= l || r <= ldx) return identify;
			if (l <= ldx && rdx <= r) return node[current];
			ll mid = (ldx+rdx)/2;
			ll vl = get(l, r, 2*current+1, ldx, mid);
			ll vr = get(l, r, 2*current+2, mid, rdx);
			return f(vl, vr);
		}

		void display_node(void) {
			print_container(node);
		}
};

// ==================================================

template <typename T, typename U>
class LazySegmentTree: public SegmentTree<T> {
	private:
		using G = function<T(T, U)>;
		using H = function<U(U, U)>;

		vec<U> lazy;
		G g;
		H h;
		U lazy_identity;

		void push(ll current, ll ldx, ll rdx) {
			if (lazy[current] == lazy_identity) return;

			this->node[current] = g(this->node[current], lazy[current]);

			if (rdx-ldx > 1) {
				lazy[2*current+1] = h(lazy[2*current+1], lazy[current]);
				lazy[2*current+2] = h(lazy[2*current+2], lazy[current]);
			}

			lazy[current] = lazy_identity;
		}

	public:
		LazySegmentTree(
				vector<T>& array,
				T identify,
				U lazy_identity,
				typename SegmentTree<T>::F f,
				G g,
				H h
		) : SegmentTree<T>(array, identify, f), g(g), h(h), lazy_identity(lazy_identity) {
			lazy.assign(2*this->n-1, lazy_identity);
		}

		void update(ll l, ll r, U v, ll current=0, ll ldx=0, ll rdx=-1) {
			if (rdx == -1) rdx = this->n;
			push(current, ldx, rdx);

			if (rdx <= l || r <= ldx) return;
			if (l <= ldx && rdx <= r) {
				lazy[current] = h(lazy[current], v);
				push(current, ldx, rdx);
				return;
			}
			ll mid = (ldx+rdx)/2;
			update(l, r, v, 2*current+1, ldx, mid);
			update(l, r, v, 2*current+2, mid, rdx);
			this->node[current] = this->f(this->node[2*current+1], this->node[2*current+2]);
		}

		T get(ll l, ll r, ll current=0, ll ldx=0, ll rdx=-1) {
			if (rdx == -1) rdx = this->n;
			push(current, ldx, rdx);

			if (rdx <= l || r <= ldx) return this->identify;
			if (l <= ldx && rdx <= r) return this->node[current];
			ll mid = (ldx+rdx)/2;
			T vl = get(l, r, 2*current+1, ldx, mid);
			T vr = get(l, r, 2*current+2, mid, rdx);
			return this->f(vl, vr);
		}
};

// ==================================================

const ll dx[] = { 1, 0, -1, 0 };
const ll dy[] = { 0, -1, 0, 1 };

bool is_in_range(ll value, ll bottom, ll top) {
	return (bottom <= value) && (value < top);
}

const auto MAX = [](const ll x, const ll y) { return max(x, y); };
const auto MIN = [](const ll x, const ll y) { return min(x, y); };
const auto SUM = [](const ll x, const ll y) { return x+y; };

// ==================================================

int main() {
  ios::sync_with_stdio(false);
  cin.tie(nullptr);



  return 0;
}
