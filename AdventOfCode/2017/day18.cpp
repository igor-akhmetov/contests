#include <fstream>
#include <string>
#include <iterator>
#include <iostream>
#include <vector>
#include <sstream>
#include <queue>
#include <tuple>
#include <map>

using namespace std;

ifstream in("day18.in");


int part_one()
{    
    map<string, long long> regs;

    auto eval = [&](const string& s) -> long long
    {
        if (s[0] >= 'a' && s[0] <= 'z')
            return regs[s];
        return stoi(s);
    };

    vector<tuple<string, string, string>> instructions;
    string str;
    while (getline(in, str  ))
    {
        istringstream iss(str);
        string s1, s2, s3;
        iss >> s1 >> s2 >> s3;
        instructions.emplace_back(s1, s2, s3);
    }
    int last_snd = 0;
    for (int instr = 0; instr >= 0 && instr < instructions.size(); ++instr)
    {
        auto const & [s1, s2, s3] = instructions[instr];
        if (s1 == "snd")
            last_snd = eval(s2);
        else if (s1 == "set")
            regs[s2] = eval(s3);
        else if (s1 == "add")
            regs[s2] += eval(s3);
        else if (s1 == "mul")
            regs[s2] *= eval(s3);
        else if (s1 == "mod")
            regs[s2] %= eval(s3);
        else if (s1 == "rcv")
        {
            if (eval(s2) != 0)
                return last_snd;
        } else if (s1 == "jgz")
        {
            if (eval(s2) > 0)
                instr += eval(s3) - 1;
        }
    }
}

int part_two()
{
    auto eval = [](map<string, long long> & regs, const string& s) -> long long
    {
        if (s[0] >= 'a' && s[0] <= 'z')
            return regs[s];
        return stoi(s);
    };
    vector<tuple<string, string, string>> instructions;
    string str;
    while (getline(in, str  ))
    {
        istringstream iss(str);
        string s1, s2, s3;
        iss >> s1 >> s2 >> s3;
        instructions.emplace_back(s1, s2, s3);
    }
    int instr[2] = {};
    map<string, long long> regs[2];
    regs[0]["p"] = 0;
    regs[1]["p"] = 1;
    queue<long long> q[2];
    int res = 0;
    for (;;)
    {
        int instr_prev[2] = {instr[0], instr[1]};
        for (int i = 0; i < 2; ++i)
        {
            if (instr[i] < 0 || instr[i] >= int(instructions.size()))
                continue;
            auto const & [s1, s2, s3] = instructions[instr[i]];            
            if (s1 == "set")
                regs[i][s2] = eval(regs[i], s3);
            else if (s1 == "add")
                regs[i][s2] += eval(regs[i], s3);
            else if (s1 == "mul")
                regs[i][s2] *= eval(regs[i], s3);
            else if (s1 == "mod")
                regs[i][s2] %= eval(regs[i], s3);
            else if (s1 == "rcv")
            {
                if (q[i].empty())
                    continue;
                regs[i][s2] = q[i].front();
                q[i].pop();                
            }
            else if (s1 == "snd")
            {
                res += i == 1;
                q[1-i].push(eval(regs[i], s2));
            }
            else if (s1 == "jgz")
            {
                if (eval(regs[i], s2) > 0)
                    instr[i] += eval(regs[i], s3) - 1;
            }
            ++instr[i];
        }
        if (instr[0] == instr_prev[0] && instr[0] == instr_prev[1])
            break;
    }
    return res;
}

int main()
{
    cout << part_two();
}
