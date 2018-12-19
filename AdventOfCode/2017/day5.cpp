#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <vector>
#include <iterator>
#include <unordered_set>
#include <algorithm>

using namespace std;

ifstream in("day5.in");

void part_one()
{
    vector<int> v{istream_iterator<int>(in), istream_iterator<int>()};
    int pos = 0;
    int steps = 0;
    for (;;)
    {
        ++steps;
        int jump = v[pos];
        ++v[pos];
        pos += jump;
        if (pos < 0 || pos >= v.size())
            break;
    }
    cout << steps;
}

void part_two()
{
    vector<int> v{ istream_iterator<int>(in), istream_iterator<int>() };
    int pos = 0;
    int steps = 0;
    for (;;)
    {
        ++steps;
        int jump = v[pos];
        if (v[pos] >= 3)
            --v[pos];
        else
            ++v[pos];
        pos += jump;
        if (pos < 0 || pos >= v.size())
            break;
    }
    cout << steps;
}

int main()
{
    part_two();
}
