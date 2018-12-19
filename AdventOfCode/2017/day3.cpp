#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <vector>
#include <iterator>
#include <algorithm>

using namespace std;

ifstream in("day3.in");

void part_one(int n)
{
    vector<pair<int, int>> v;
    v.emplace_back( 0, 0 );
    v.emplace_back( 0, 0 );
    int len = 1;
    for (;;)
    {
        len += 2;
        auto last = v.back();
        v.emplace_back( last.first + 1, last.second );
        for (int i = 0; i < len - 2; ++i)
        {
            last = v.back();
            v.emplace_back( last.first, last.second + 1 );
        }
        for (int i = 0; i < len - 1; ++i)
        {
            last = v.back();
            v.emplace_back(last.first - 1, last.second);
        }
        for (int i = 0; i < len - 1; ++i)
        {
            last = v.back();
            v.emplace_back(last.first, last.second - 1);
        }
        for (int i = 0; i < len - 1; ++i)
        {
            last = v.back();
            v.emplace_back(last.first + 1, last.second);
        }
        if (n < v.size()) break;
    }
    cout << abs(v[n].first) + abs(v[n].second);
}

const int off = 250;
void calc_sum(vector<vector<int>>& sums, pair<int, int> pair, int maxsum, int & res)
{
    int dx[] = { -1, -1, -1, 0, 0, 1, 1, 1 };
    int dy[] = { -1, 0, 1, -1, 1, -1, 0, 1 };
    int sum = 0;
    pair.first += off; pair.second += off;
    for (int i = 0; i < 8; ++i)
        sum += sums[pair.first + dx[i]][pair.second + dy[i]];
    sums[pair.first][pair.second] = sum;
    if (sum >= maxsum && res == 0)
        res = sum;
}

void part_two(int n)
{
    vector<pair<int, int>> v;
    vector<vector<int>> sums(500, vector<int>(500));
    v.emplace_back(0, 0);
    v.emplace_back(0, 0);    
    sums[off][off] = 1;
    int len = 1;
    int res = 0;
    for (;;)
    {
        len += 2;
        auto last = v.back();
        v.emplace_back(last.first + 1, last.second);
        calc_sum(sums, v.back(), n, res);
        for (int i = 0; i < len - 2; ++i)
        {
            last = v.back();
            v.emplace_back(last.first, last.second + 1);
            calc_sum(sums, v.back(), n, res);
        }
        for (int i = 0; i < len - 1; ++i)
        {
            last = v.back();
            v.emplace_back(last.first - 1, last.second);
            calc_sum(sums, v.back(), n, res);
        }
        for (int i = 0; i < len - 1; ++i)
        {
            last = v.back();
            v.emplace_back(last.first, last.second - 1);
            calc_sum(sums, v.back(), n, res);
        }
        for (int i = 0; i < len - 1; ++i)
        {
            last = v.back();
            v.emplace_back(last.first + 1, last.second);
            calc_sum(sums, v.back(), n, res);
        }
        if (res != 0) break;
    }
    cout << res;
}

int main()
{
    int n;
    in >> n;
    //part_one(n);
    part_two(n);
}
