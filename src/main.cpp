#include <iostream>

#include <cub.h>

int main() {
  auto val{10};
  std::cout << "Cub of " << val << ": " << Cub(val) << "\n";

  return 0;
}