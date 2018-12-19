#include <iostream>
#include <fstream>
#include <string>

using namespace std;

ifstream in("day1.in");

void first_half()
{
    string s;
    in >> s;
    s.push_back(s[0]);
    int sum = 0;
    for (int i = 0; i < s.size() - 1; ++i)
    {
        if (s[i] != s[i + 1]) continue;
        sum += s[i] - '0';
    }
    cout << sum;
}

void second_half()
{
    string s;
    in >> s;
    int sum = 0;
    int off = s.size() / 2;
    for (int i = 0; i < s.size(); ++i)
    {
        if (s[i] != s[(i + off) % s.size()]) continue;
        sum += s[i] - '0';
    }
    cout << sum;
}

int main()
{
    second_half();
}