#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <iterator>
#include <algorithm>
#include <sstream>
#include <map>
#include <cassert>

using namespace std;

ifstream in("day8.in");

bool check_cond(int reg, int cond_val, const string& cond)
{
    if (cond == "<") return reg < cond_val;
    if (cond == "<=") return reg <= cond_val;
    if (cond == "==") return reg == cond_val;
    if (cond == ">") return reg > cond_val;
    if (cond == ">=") return reg >= cond_val;
    if (cond == "!=") return reg != cond_val;
    assert(false);
}

void part_one()
{
    map<string, int> registers;
    for (;;)
    {
        string reg, op, cond_reg, cond;
        int op_val, cond_val;
        if (!(in >> reg >> op >> op_val >> cond_reg >> cond_reg >> cond >> cond_val))
            break;
        if (check_cond(registers[cond_reg], cond_val, cond))
        {
            if (op == "inc")
                registers[reg] += op_val;
            else
                registers[reg] -= op_val;
        }
    }
    int res = 0;
    for (auto && item : registers)
        res = max(res, item.second);
    cout << res;
}

void part_two()
{
    map<string, int> registers;
    int res = 0;
    for (;;)
    {
        string reg, op, cond_reg, cond;
        int op_val, cond_val;
        if (!(in >> reg >> op >> op_val >> cond_reg >> cond_reg >> cond >> cond_val))
            break;
        if (check_cond(registers[cond_reg], cond_val, cond))
        {
            if (op == "inc")
                registers[reg] += op_val;
            else
                registers[reg] -= op_val;
            res = max(res, registers[reg]);
        }
    }
    cout << res;
}

int main()
{
    part_two();
}
