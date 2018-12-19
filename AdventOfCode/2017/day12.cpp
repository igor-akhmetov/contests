#include <fstream>
#include <string>
#include <iterator>
#include <iostream>
#include <vector>
#include <iomanip>
#include <algorithm>
#include <sstream>

using namespace std;

ifstream in("day12.in");

vector<vector<int>> edges;

void dfs(int v, vector<int>& mark)
{
    if (mark[v]) return;
    mark[v] = 1;
    for (auto w : edges[v])
        dfs(w, mark);
}

void part_one()
{
    string s, tmp;
    while (getline(in, s))
    {
        istringstream iss(s);
        int v, w;
        iss >> v >> tmp;
        while (iss)
        {
            char ch;
            iss >> w >> ch;
            if (edges.size() <= v)
                edges.resize(v + 1);
            edges[v].push_back(w);
        }
    }
    vector<int> mark(edges.size());
    dfs(0, mark);
    cout << count(mark.begin(), mark.end(), 1);
}

void part_two()
{
    string s, tmp;
    while (getline(in, s))
    {
        istringstream iss(s);
        int v, w;
        iss >> v >> tmp;
        while (iss)
        {
            char ch;
            iss >> w >> ch;
            if (edges.size() <= v)
                edges.resize(v + 1);
            edges[v].push_back(w);
        }
    }
    vector<int> mark(edges.size());
    int res = 0;
    for (int i = 0; i < edges.size(); ++i)
    {
        if (!mark[i])
            ++res, dfs(i, mark);
    }
    cout << res;
}

int main()
{
    part_two();
}
