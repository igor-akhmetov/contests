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
#include <tuple>
#include <set>
#include <map>

using namespace std;

ifstream in("day17.in");

void part_one(int step)
{
    vector<int> v(1);
    v.reserve(2018);
    int pos = 0;
    for (int i = 1; i <= 2017; ++i)
    {
        pos = (pos + step) % v.size() + 1;
        v.insert(v.begin() + pos, i);
    }
    for (int i = 0; i < v.size() - 1; ++i)
    {
        if (v[i] == 2017)
            cout << v[i+1];
    }
}

void part_two(int step)
{
    int len = 1;
    int zeropos = 0;
    int nextval = -1;
    int pos = 0;
    for (int i = 1; i <= 50000000; ++i)
    {
        pos = (pos + step) % len;
        if (pos == zeropos)
            nextval = i;
        ++pos;
        ++len;
    }
    cout << nextval;
}

int main()
{
    part_two(356);
}
