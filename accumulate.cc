#include <cstdint>
#include <numeric>
#include <vector>

int32_t accumulate() {
    std::vector<int32_t> v{1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
    return std::accumulate(v.begin(), v.end(), 0);
}
