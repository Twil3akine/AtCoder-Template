#include <algorithm>
#include <cassert>
#include <cctype>
#include <climits>
#include <cstddef>
#include <cstdio>
#include <cstdlib>
#include <deque>
#include <functional>
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
#define rep(i, begin, end) for (ll i=(ll)(begin); i<(ll)(end); i++)
#define rrep(i, begin, end) for (ll i=(ll)(begin)-1; (ll)(end)<=i; --i)

// ==================================================

// 定数とかの宣言

using namespace std;

constexpr char el = '\n';

using ll = long long;
const ll INF = 5LL << 60;

// ==================================================

// 前方宣言とか、型判定？の定義

template <ll MOD>
class ModInt;

template <typename T>
struct is_integrals : is_integral<T> {};

template <ll MOD>
struct is_integrals<ModInt<MOD>> : true_type {};

template <typename T>
inline constexpr bool is_integrals_v = is_integrals<T>::value;

template <typename T>
using integer = enable_if_t<is_integrals_v<T>>;

// ==================================================

// データ型の定義
using Visited = vector<vector<bool>>;

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

// セグメント木

class SegmentTree {
	private:
		ll n = 1;
		vector<ll> node;
		ll identify;
		function<ll(ll, ll)> f;

	public:
		SegmentTree(vector<ll>& array, ll identify, function<ll(ll, ll)> f) {
			ll size = array.size();
			while (n < size) n <<= 1;

			node.assign(2*n-1, identify);
			this -> identify = identify;
			this -> f = f;

			for(ll i=0; i<size; i++) this->node[i+n-1] = array[i];
			for(ll i=n-2; 0<=i; --i) this->node[i] = (this->f)(this->node[2*i+1], this->node[2*i+2]);
		}

		void update(ll idx, ll v) {
			idx += this->n-1;
			this->node[idx] = v;
			while(idx > 0) {
				idx = (idx-1)/2;
				this->node[idx] = (this->f)(this->node[2*idx+1], this->node[2*idx+2]);
			}
		}

		ll get(ll l, ll r, ll current=0, ll ldx=0, ll rdx=-1) {
			if (rdx == -1) rdx = this->n;
			if (rdx <= l || r <= ldx) return this->identify;
			if (l <= ldx && rdx <= r) return this->node[current];
			ll mid = (ldx+rdx)/2;
			ll vl = this->get(l, r, 2*current+ 1, ldx, mid);
			ll vr = this->get(l, r, 2*current+2, mid, rdx);
			return (this->f)(vl, vr);
		}
};

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

const ll dx[] = { 1, 0, -1, 0 };
const ll dy[] = { 0, -1, 0, 1 };

bool is_in_range(ll value, ll bottom, ll top) {
	return (bottom <= value) && (value < top);
}

// ==================================================

int main() {
  ios::sync_with_stdio(false);
  cin.tie(nullptr);



  return 0;
}
