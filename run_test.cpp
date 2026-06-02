#include <string>
#include <iostream>
#include <fstream>
#include <chrono>

std::string maximum_odd_binary(const std::string& s) {
  int len = s.size();
  int ones = 0;

  for (char c : s)
    ones += (c == '1');

  if (ones == 0)
    return "";
  std::string result(len, '0');
  std::fill(result.begin(), result.begin() + (ones - 1), '1');
  result[len - 1] = '1';
  return result;
}

int main() {
  std::ifstream file("large_binary.txt");
  if (!file) {
    std::cerr << "Failed to open large_binary.txt\n";
    return 1;
  }
  std::string s((std::istreambuf_iterator<char>(file)),
                 std::istreambuf_iterator<char>());

  for (int i = 0; i < 5; ++i) {
    auto start = std::chrono::high_resolution_clock::now();
    std::string result = maximum_odd_binary(s);
    auto end = std::chrono::high_resolution_clock::now();
    auto us = std::chrono::duration_cast<std::chrono::microseconds>(end - start).count();
    std::cout << "Run " << (i + 1) << ": " << us << " µs\n";
  }
  return 0;
}
