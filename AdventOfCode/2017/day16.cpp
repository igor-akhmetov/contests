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

ifstream in("day16.in");

template<typename Out>
void split(const std::string &s, char delim, Out result) {
    std::stringstream ss(s);
    std::string item;
    while (std::getline(ss, item, delim)) {
        *(result++) = item;
    }
}

std::vector<std::string> split(const std::string &s, char delim) {
    std::vector<std::string> elems;
    split(s, delim, std::back_inserter(elems));
    return elems;
}

vector<int> part_one()
{
    string str;
    in >> str;
    int start = 0;
    static const int n = 16;
    vector<int> pos(n);
    char chars[n];
    iota(pos.begin(), pos.begin() + n, 0);
    iota(chars, chars + n, 0);
    for (auto && s : split(str, ','))
    {
        if (s[0] == 's')
        {
            start = (start + (n - stoi(s.substr(1)))) % n;
        }
        else if (s[0] == 'x')
        {
            istringstream iss(s.substr(1));
            int a, b;
            char ch;
            iss >> a >> ch >> b;
            a = (start + a) % n;
            b = (start + b) % n;
            swap(chars[a], chars[b]);
            swap(pos[chars[a]], pos[chars[b]]);
        }
        else
        {
            istringstream iss(s.substr(1));
            char a, b;
            char ch;
            iss >> a >> ch >> b;
            a -= 'a'; b -= 'a';
            swap(pos[a], pos[b]);
            swap(chars[pos[a]], chars[pos[b]]);
        }
    }
    rotate(chars, chars + start, chars + n);
    for (int & p : pos)
        p = (p + n - start) % n;
    for (char c : chars)
        cout << char(c + 'a');
    cout << endl;
    return pos;
}

void part_two()
{
    int pow = 1000000000;
    string str;
    in >> str;
    int start = 0;
    static const int n = 16;
    vector<int> pos(n);
    vector<char> chars(n), chars2;
    iota(pos.begin(), pos.begin() + n, 0);
    iota(chars.begin(), chars.begin() + n, 0);
    vector<tuple<int, int, int>> ops;
    for (auto && s : split(str, ','))
    {
        if (s[0] == 's')
            ops.emplace_back(0, stoi(s.substr(1)), 0);
        else if (s[0] == 'x')
        {
            istringstream iss(s.substr(1));
            int a, b;
            char ch;
            iss >> a >> ch >> b;
            ops.emplace_back(1, a, b);
        }
        else
        {
            istringstream iss(s.substr(1));
            char a, b;
            char ch;
            iss >> a >> ch >> b;
            a -= 'a'; b -= 'a';
            ops.emplace_back(2, a, b);
        }
    }
    map<vector<char>, int> permutations;
    int target = 1000000000;
    permutations[chars] = 0;
    for (int i = 1; i <= target; ++i)
    {
        for (auto [op, a, b] : ops)
        {
            if (op == 0)
                start = (start + (n - a)) % n;
            else if (op == 1)
            {
                a = (start + a) % n;
                b = (start + b) % n;
                swap(chars[a], chars[b]);
                swap(pos[chars[a]], pos[chars[b]]);
            }
            else
            {
                swap(pos[a], pos[b]);
                swap(chars[pos[a]], chars[pos[b]]);
            }
        }
        chars2 = chars;
        rotate(chars2.begin(), chars2.begin() + start, chars2.begin() + n);
        auto it = permutations.find(chars2);
        if (it == permutations.end())
            permutations.insert(it, make_pair(chars2, i));
        else
        {
            int last = it->second;
            int len = i - last;
            int pos = last + (target - last) % len;
            for (auto && [c, idx] : permutations)
                if (idx == pos)
                {
                    for (char ch : c)
                        cout << char(ch + 'a');
                    return;
                }
        }
    }
}

int main()
{
    part_two();
}
