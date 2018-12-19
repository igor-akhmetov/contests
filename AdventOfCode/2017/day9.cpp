#include <fstream>
#include <string>
#include <iterator>
#include <iostream>

using namespace std;

ifstream in("day9.in");

void part_one()
{
    int res = 0, level = 0;
    for (;;)
    {
        char ch;
        if (!(in >> ch))
            break;
        switch (ch)
        {
        case '{':
            ++level;
            res += level;
            break;
        case '}':
            --level;
            break;
        case '<':
            while (in >> ch)
            {
                if (ch == '>') break;
                if (ch == '!') in >> ch;
            }
            break;
        }
    }
    cout << res;
}

void part_two()
{
    int res = 0;
    for (;;)
    {
        char ch;
        if (!(in >> ch))
            break;
        switch (ch)
        {
        case '<':
            while (in >> ch)
            {
                if (ch == '>') break;
                if (ch == '!') in >> ch;
                else ++res;
            }
            break;
        }
    }
    cout << res;
}

int main()
{
    part_two();
}
