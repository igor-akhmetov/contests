#include <fstream>
#include <string>
#include <iterator>
#include <iostream>
#include <numeric>
#include <vector>
#include <iomanip>

using namespace std;

ifstream in("day10.in");

void part_one()
{
    vector<int> v(256);
    iota(v.begin(), v.end(), 0);
    int pos = 0, skip_size = 0;
    for (;;)
    {
        int len; in >> len;
        if (!in) break;
        char ch; in >> ch;
        for (int i = 0; i < len / 2; ++i)
        {
            int i1 = (pos + i) % v.size();
            int i2 = (pos + len - i - 1) % v.size();
            swap(v[i1], v[i2]);
        }
        pos = (pos + len + skip_size) % v.size();
        ++skip_size;
    }
    cout << v[0] * v[1];
}

void part_two()
{
    vector<int> v(256);
    vector<int> lengths;
    iota(v.begin(), v.end(), 0);
    int pos = 0, skip_size = 0;
    for (;;)
    {
        char len; in >> len;
        if (!in) break;
        lengths.push_back(len);
    }
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
    for (auto d : dense)
        cout << std::hex << setw(2) << setfill('0') << d;
    cout << endl;
}

int main()
{
    part_two();
}
