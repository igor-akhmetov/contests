#include <fstream>
#include <string>
#include <iterator>
#include <iostream>
#include <vector>
#include <iomanip>
#include <algorithm>

using namespace std;

ifstream in("day11.in");

void part_one()
{
    int n = 0, ne = 0;
    for (;;)
    {
        std::string s;
        char ch;
        while (in >> ch && ch != ',')
            s.push_back(ch);
        if (s == "n") ++n;
        if (s == "ne") ++ne;
        if (s == "se") --n, ++ne;
        if (s == "s") --n;
        if (s == "sw") --ne;
        if (s == "nw") ++n, --ne;
        if (!in) break;
    }
    int res = 0;
    if (n < 0 && ne > 0 || n > 0 && ne < 0)
        res = abs(n) + abs(ne) - min(abs(n), abs(ne));
    else
        res = abs(n) + abs(ne);
    cout << res;
}

void part_two()
{
    int n = 0, ne = 0;
    int res = 0;
    for (;;)
    {
        std::string s;
        char ch;
        while (in >> ch && ch != ',')
            s.push_back(ch);
        if (s == "n") ++n;
        if (s == "ne") ++ne;
        if (s == "se") --n, ++ne;
        if (s == "s") --n;
        if (s == "sw") --ne;
        if (s == "nw") ++n, --ne;
        if (n < 0 && ne > 0 || n > 0 && ne < 0)
            res = max(res, abs(n) + abs(ne) - min(abs(n), abs(ne)));
        else
            res = max(res, abs(n) + abs(ne));
        if (!in) break;
    }
    cout << res;
}

int main()
{
    part_two();
}
