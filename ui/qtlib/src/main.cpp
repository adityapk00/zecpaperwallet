#include <iostream>
#include <cstring>

#include "zecpaperrust.h"

using namespace std;

int main() {
  char * from_rust = rust_generate_wallet(true, 1, 1, "user-provided-entropy");
  auto stri = string(from_rust);
  cout << stri << endl;
  rust_free_string(from_rust);

  return 0;
}