#include <algorithm>
#include <climits>
#include <cstdio>
#include <cstdlib>
#include <deque>
#include <functional>
#include <iostream>
#include <ostream>
#include <queue>
#include <set>
#include <string>
#include <utility>
#include <vector>

#define scanf1(x) ll x; scanf("%lld", &x);
#define scanf2(x, y) ll x, y; scanf("%lld%lld", &x, &y);
#define scanf3(x, y, z) ll x, y, z; scanf("%lld%lld%lld", &x, &y, &z);
#define scanf4(v, x, y, z) ll v, x, y, z; scanf("%lld%lld%lld%lld", &v &x, &y, &z);

#define printf1(x) printf("%lld\n", x);

constexpr char el = '\n';

using ll = long long;
const ll MOD = 1e9+7;
const ll INF = 1LL << 60;
using namespace std;

using Graph = vector<vector<ll>>;
using CGraph = vector<vector<pair<ll, ll>>>;

void yes(bool cdt) {
    if (cdt) {
      cout << "Yes" << el;
      exit(0);
    }
  }

void no(bool cdt) {
  if (!cdt) {
    cout << "No" << el;
    exit(0);
  }
}

void yes_no(bool cdt) {
  cout << (cdt ? "Yes" : "No") << el;
  exit(0);
}

template <typename T>
void printvec(const vector<T> array) {
	cout << "! ";
	for(T elm: array) cout << elm << " ";
	cout << el;
}

template <typename T>
T bound_search(const vector<T> &array, const function<bool(ll)> cdt) {
	T left = -1, right = array.size();

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

class SegmentTree {
	public:
		ll n = 1;
		vector<ll> node;
		ll identify;
		function<ll(ll, ll)> f;
		SegmentTree(vector<ll> array, ll identify, function<ll(ll, ll)> f) {
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
			ll vl = this->get(l, r, 2*current+1, ldx, mid);
			ll vr = this->get(l, r, 2*current+2, mid, rdx);
			return (this->f)(vl, vr);
		}
};

int main(void) {

	return 0;
}
