#include <fstream>
#include <string>
#include <iterator>
#include <iostream>
#include <vector>
#include <iomanip>
#include <algorithm>
#include <sstream>

using namespace std;

ifstream in("day13.in");

void part_one()
{
    vector<int> depth;
    for (;;)
    {
        char ch;
        int idx, d;
        in >> idx >> ch >> d;
        if (!in) break;
        depth.resize(idx + 1);
        depth[idx] = d;
    }
    vector<int> pos(depth.size());
    vector<int> dir(depth.size(), 1);
    int layer = -1, severity = 0;
    while (layer < int(depth.size()) - 1)
    {
        ++layer;
        if (!pos[layer])
            severity += depth[layer] * layer;
        for (int i = 0; i < pos.size(); ++i)
        {
            if (!depth[i]) continue;
            if (pos[i] + dir[i] < 0 || pos[i] + dir[i] >= depth[i])
                dir[i] = -dir[i];
            pos[i] += dir[i];
        }
    }
    cout << severity;
}

void part_two()
{
    vector<int> depth;
    for (;;)
    {
        char ch;
        int idx, d;
        in >> idx >> ch >> d;
        if (!in) break;
        depth.resize(idx + 1);
        depth[idx] = d;
    }
    vector<char> p(10000000, 1);
    for (int i = 0; i < depth.size(); ++i)
    {
        if (!depth[i]) continue;
        int idx = -i, step = 2 * depth[i] - 2;
        while (idx < int(p.size()))
        {
            if (idx >= 0) p[idx] = 0;
            idx += step;
        }
    }
    int res = find(p.begin(), p.end(), 1) - p.begin();
    cout << res;
    //vector<int> depth;
    //for (;;)
    //{
    //    char ch;
    //    int idx, d;
    //    in >> idx >> ch >> d;
    //    if (!in) break;
    //    depth.resize(idx + 1);
    //    depth[idx] = d;
    //}
    //vector<int> pos0(depth.size());
    //vector<int> dir0(depth.size(), 1);
    //for (int delay = 0;; delay++)
    //{
    //    vector<int> pos = pos0;
    //    vector<int> dir = dir0;
    //    int layer = -1, caught = 0;
    //    while (layer < int(depth.size()) - 1)
    //    {
    //        ++layer;
    //        if (layer >= 0 && depth[layer] && !pos[layer])
    //        {
    //            caught = 1;
    //            break;
    //        }
    //        for (int i = 0; i < pos.size(); ++i)
    //        {
    //            if (!depth[i]) continue;
    //            if (pos[i] + dir[i] < 0 || pos[i] + dir[i] >= depth[i])
    //                dir[i] = -dir[i];
    //            pos[i] += dir[i];
    //        }
    //    }
    //    if (!caught)
    //    {
    //        cout << delay;
    //        return;
    //    }
    //    for (int i = 0; i < pos.size(); ++i)
    //    {
    //        if (!depth[i]) continue;
    //        if (pos0[i] + dir0[i] < 0 || pos0[i] + dir0[i] >= depth[i])
    //            dir0[i] = -dir0[i];
    //        pos0[i] += dir0[i];
    //    }
    //}
}

int main()
{
    part_two();
}
