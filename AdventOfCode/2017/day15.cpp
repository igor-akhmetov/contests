#include <fstream>
#include <string>
#include <iterator>
#include <iostream>
#include <vector>
#include <iomanip>
#include <algorithm>
#include <sstream>
#include <numeric>
#include <queue>

using namespace std;

void part_one(long long l, long long r)
{
    int res = 0;
    for (int i = 0; i < 40000000; ++i)
    {
        l = l * 16807 % 2147483647;
        r = r * 48271 % 2147483647;
        res += (l & 0xFFFF) == (r & 0xFFFF);
    }
    cout << res;
}

void part_two(long long l, long long r)
{
    int res = 0;
    for (int i = 0; i < 5000000; ++i)
    {
        do {
            l = l * 16807 % 2147483647;
        } while (l % 4 != 0);
        do {
            r = r * 48271 % 2147483647;
        } while (r % 8 != 0);
        res += (l & 0xFFFF) == (r & 0xFFFF);
    }
    cout << res;
}

int main()
{
    //part_one(722, 354);
    //part_two(65, 8921);
    part_two(722, 354);
}
