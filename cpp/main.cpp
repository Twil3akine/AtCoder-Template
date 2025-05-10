#include <algorithm>
#include <climits>
#include <cstddef>
#include <cstdio>
#include <cstdlib>
#include <deque>
#include <functional>
#include <iostream>
#include <iterator>
#include <limits>
#include <map>
#include <numeric>
#include <ostream>
#include <queue>
#include <set>
#include <stack>
#include <string>
#include <sys/types.h>
#include <tuple>
#include <type_traits>
#include <unordered_map>
#include <utility>
#include <valarray>
#include <vector>
#include <cmath>
#include <concepts>

// ==================================================

constexpr char el = '\n';

using ll = long long;
const ll INF = 1LL << 60;

using namespace std;

using Graph = vector<vector<ll>>;
using CGraph = vector<vector<pair<ll, ll>>>;

using Grid = vector<vector<char>>;

// ==================================================

template <typename... Args>
void out(const Args&... args) {
    ((cout << args << " "), ...);
    cout << '\n';
}

// ==================================================

void yes_print(bool cdt) {
    if (cdt) {
      cout << "Yes" << el;
      exit(0);
    }
  }

void no_print(bool cdt) {
  if (!cdt) {
    cout << "No" << el;
    exit(0);
  }
}

void yes_no_print(bool cdt) {
  cout << (cdt ? "Yes" : "No") << el;
  exit(0);
}

// ==================================================

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

template <typename T>
class RotatedGrid {
	private:
		vector<vector<T>> content;

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

			vector<vector<T>> rotated(size, vector<T>(size));

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

		auto begin() const {
			return content.begin();
		}

		auto end() const {
			return content.end();
		}
};

// ==================================================

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

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

		vector<vector<char>> s;
		s = {
			{'#', '.', '.'},
			{'.', '.', '.'},
			{'.', '.', '.'},
		};

		cout << "Base grid: " << el;
		print_ncontainer(s);

		RotatedGrid<char> rgs(s);

		cout << "Right Rotate: " << el;
		for (ll i=0; i<4; i++) {
			rgs >>= 1 ;
			print_ncontainer(rgs);
			cout << el;
		}

		cout << "Left Rotate: " << el;
		for (ll i=0; i<4; i++) {
			rgs <<= 1 ;
			print_ncontainer(rgs);
			cout << el;
		}

		cout << "Horizontal:" << el;
		for (ll i=0; i<2; i++) {
			rgs.horizontal();
			print_ncontainer(rgs);
			cout << el;
		}

		cout << "Vertical: " << el;
		for (ll i=0; i<2; i++) {
			rgs.vertical();
			print_ncontainer(rgs);
			cout << el;
		}

    return 0;
}
