#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <vector>
#include <iterator>
#include <algorithm>

using namespace std;

ifstream in("day2.in");

void part_one(vector<vector<int>> const & table)
{
    int sum = 0;
    for (auto & v : table)
        sum += *max_element(v.begin(), v.end()) - *min_element(v.begin(), v.end());
    cout << sum;
}

void part_two(vector<vector<int>> const & table)
{
    int sum = 0;
    for (auto & v : table)
    {
        for (int i = 0; i < v.size(); ++i)
        {
            for (int j = 0; j < v.size(); ++j)
            {
                if (v[i] > v[j] && (v[i] % v[j]) == 0)
                    sum += v[i] / v[j];
            }
        }
    }
    cout << sum;
}

int main()
{
    string s;
    vector<vector<int>> table;
    for (;;)
    {
        if (!getline(in, s))
            break;
        istringstream iss(s);
        table.emplace_back(istream_iterator<int>(iss), istream_iterator<int>());
    }
    part_two(table);
}
