#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <vector>
#include <iterator>
#include <unordered_set>
#include <algorithm>

using namespace std;

ifstream in("day4.in");

void part_one()
{
    string line;
    int res = 0;
    while (getline(in, line))
    {
        istringstream iss(line);
        string word;
        unordered_set<string> words;
        bool valid = true;
        while (iss >> word)
        {
            if (!words.insert(word).second)
            {
                valid = false;
                break;
            }
        }
        res += valid;
    }
    cout << res;
}

void part_two()
{
    string line;
    int res = 0;
    while (getline(in, line))
    {
        istringstream iss(line);
        string word;
        unordered_set<string> words;
        bool valid = true;
        while (iss >> word)
        {
            sort(word.begin(), word.end());
            if (!words.insert(word).second)
            {
                valid = false;
                break;
            }
        }
        res += valid;
    }
    cout << res;
}

int main()
{
    part_two();
}
