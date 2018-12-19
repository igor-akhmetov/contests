#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <iterator>
#include <algorithm>
#include <set>

using namespace std;

ifstream in("day6.in");

void part_one()
{
    vector<int> v{ istream_iterator<int>(in), istream_iterator<int>() };
    set<vector<int>> s;
    int res = 0;
    for (;;)
    {
        if (!s.insert(v).second) {
            break;
        }
        ++res;
        int idx = distance(v.begin(), max_element(v.begin(), v.end()));
        int val = v[idx];
        v[idx] = 0;
        while (val)
        {
            idx = (idx + 1) % v.size();
            v[idx]++;
            --val;
        }
    }
    cout << res;
}

void part_two()
{
    vector<int> v{ istream_iterator<int>(in), istream_iterator<int>() };
    set<vector<int>> s;
    int res = 0;
    vector<int> loop_start;
    int loop_start_idx = 0;
    for (;;)
    {
        if (loop_start == v)
            break;
        if (!s.insert(v).second && loop_start.size() == 0) {
            loop_start = v;
            loop_start_idx = res;
        }
        ++res;
        int idx = distance(v.begin(), max_element(v.begin(), v.end()));
        int val = v[idx];
        v[idx] = 0;
        while (val)
        {
            idx = (idx + 1) % v.size();
            v[idx]++;
            --val;
        }
    }
    cout << res - loop_start_idx;
}

int main()
{
    part_two();
}
