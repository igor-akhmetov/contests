#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <iterator>
#include <algorithm>
#include <sstream>
#include <map>

using namespace std;

ifstream in("day7.in");

void part_one()
{
    string line;
    map<string, int> cnt;
    while (getline(in, line))
    {
        istringstream iss(line);
        string name, weight;
        iss >> name >> weight;
        cnt.insert({ name, 0 });
        string s;
        if (iss >> s)
        {
            while (iss >> s)
            {
                if (s[s.size() - 1] == ',')
                    s.pop_back();
                cnt[s]++;
            }                
        }
    }
    for (auto && v : cnt)
    {
        if (!v.second)
        {
            cout << v.first;
            return;
        }
    }
}

void calc_weights(string const & node, map<string, vector<string>> & children, map<string, int> & weights)
{
    for (auto && child : children[node])
    {
        calc_weights(child, children, weights);
        weights[node] += weights[child];
    }
}

int find_answer(string const & node, map<string, vector<string>> & children, map<string, int> & weights, int weight)
{
    vector<int> child_weights;
    for (auto && child : children[node])
        child_weights.push_back(weights[child]);
    sort(child_weights.begin(), child_weights.end());
    int sum = 0;
    for (auto && child : children[node])
    {
        if (weights[child] != child_weights[1])
            return find_answer(child, children, weights, child_weights[1]);
        sum += weights[child];
    }
    return weight - sum;
}

void part_two()
{
    string line;
    map<string, vector<string>> children;
    map<string, int> weights;
    map<string, int> cnt;
    while (getline(in, line))
    {
        istringstream iss(line);
        string name, weight;
        iss >> name >> weight;
        weights[name] = stoi(weight.substr(1, weight.length() - 2));
        cnt.insert({ name, 0 });
        string s;
        if (iss >> s)
        {
            while (iss >> s)
            {
                if (s[s.size() - 1] == ',')
                    s.pop_back();
                children[name].push_back(s);
                cnt[s]++;
            }                
        }
    }
    for (auto && v : cnt)
    {
        if (!v.second)
        {
            calc_weights(v.first, children, weights);
            cout << find_answer(v.first, children, weights, -1);
            return;
        }
    }
}

int main()
{
    part_two();
}
