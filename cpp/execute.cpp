#include <algorithm>
#include <climits>
#include <cstdio>
#include <deque>
#include <functional>
#include <ios>
#include <iostream>
#include <iterator>
#include <ostream>
#include <stack>
#include <tuple>
#include <unordered_set>
#include <utility>
#include <vector>

using ll = long long;
using namespace std;
using Graph = vector<vector<ll>>;
using CGraph = vector<vector<pair<ll, ll>>>;

void yes(bool cdt) {
    if (cdt) {
      cout << "Yes" << endl;
      exit(0);
    }
  }

void no(bool cdt) {
  if (!cdt) {
    cout << "No" << endl;
    exit(0);
  }
}

void yes_no(bool cdt) {
  cout << (cdt ? "Yes" : "No") << endl;
  exit(0);
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

int main(void) {

}
