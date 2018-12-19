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

ifstream in("day14.in");

vector<char> knot_hash(string const & str)
{
    vector<int> v(256);
    vector<int> lengths;
    iota(v.begin(), v.end(), 0);
    int pos = 0, skip_size = 0;
    for (char ch : str)
        lengths.push_back(ch);
    lengths.push_back(17);
    lengths.push_back(31);
    lengths.push_back(73);
    lengths.push_back(47);
    lengths.push_back(23);
    for (int j = 0; j < 64; ++j)
    {
        for (auto && len : lengths)
        {
            for (int i = 0; i < len / 2; ++i)
            {
                int i1 = (pos + i) % v.size();
                int i2 = (pos + len - i - 1) % v.size();
                swap(v[i1], v[i2]);
            }
            pos = (pos + len + skip_size) % v.size();
            ++skip_size;
        }
    }
    vector<int> dense(16);
    for (int i = 0; i < v.size(); ++i)
        dense[i / 16] ^= v[i];
    vector<char> res(128);
    for (int i = 0; i < dense.size(); ++i)
        for (int j = 0; j < 8; ++j) 
            res[i * 8 + j] = bool(dense[i] & (1 << j));
    for (int i = 0; i < res.size(); i += 8)
        reverse(res.begin() + i, res.begin() + i + 8);
    return res;
}

void part_one()
{
    string str; in >> str;
    int res = 0;
    for (int i = 0; i < 128; ++i)
    {
        auto row = knot_hash(str + "-" + to_string(i));
        res += count(row.begin(), row.end(), 1);
    }
    cout << res;
}

void part_two()
{
    string str; in >> str;
    vector<vector<char>> v;
    for (int i = 0; i < 128; ++i)
        v.push_back(knot_hash(str + "-" + to_string(i)));
    int res = 0;
    for (int i = 0; i < v.size(); ++i)
    {
        for (int j = 0; j < v[0].size(); ++j)
        {
            if (!v[i][j]) continue;
            ++res;
            queue<pair<int, int>> q;
            q.push(make_pair(i, j));
            v[i][j] = 0;
            while (!q.empty())
            {
                int dx[] = {0, -1, 0, 1};
                int dy[] = {1, 0, -1, 0};
                auto [x, y] = q.front();
                q.pop();
                for (int k = 0; k < 4; ++k)
                {
                    int xx = x + dx[k];
                    int yy = y + dy[k];
                    if (xx < 0 || yy < 0 || xx >= v.size() || yy >= v[0].size() || !v[xx][yy]) continue;
                    v[xx][yy] = 0;
                    q.push(make_pair(xx, yy));
                }
            }
        }
    }
    cout << res;
}

int main()
{
    part_two();
}
