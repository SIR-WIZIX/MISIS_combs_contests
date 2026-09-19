#include <iostream>

using namespace std;

int32_t main() {
  int n;
  cin >> n;
  if (n == 0) {
    cout << 0 << '\n';
    return 0;
  }
  int base[2][2] = {{1, 1}, {1, 0}};
  int ans[2][2] = {{1, 1}, {1, 1}};
  for (int i = 0; i < n - 2; i++) {
    int mid[2][2] = {
        {
            (ans[0][0] * base[0][0] + ans[0][1] * base[1][0]) % 10,
            (ans[0][0] * base[0][1] + ans[0][1] * base[1][1]) % 10,
        },
        {
            (ans[1][0] * base[0][0] + ans[1][1] * base[1][0]) % 10,
            (ans[1][0] * base[0][1] + ans[1][1] * base[1][1]) % 10,
        },
    };
    swap(ans, mid);
  }
  cout << ans[0][0] << '\n';
}
