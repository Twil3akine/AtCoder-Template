#include <algorithm>
#include <climits>
#include <cstdio>
#include <deque>
#include <iostream>
#include <iterator>
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

int main(void) {

}
