//#include <fstream>
//#include <string>
//#include <iterator>
//#include <iostream>
//#include <vector>
//#include <sstream>
//#include <queue>
//#include <tuple>
//#include <map>
//#include <cassert>
//#include <numeric>
//
//using namespace std;
//
//ifstream in("day20.in");
//
//int part_one()
//{    
//    string line;
//    vector<tuple<int, int, int>> acc;
//    int minacc = 1e9;
//    vector<pair<int, tuple<int, int, int>>> ans;
//    int idx = 0;
//    while (getline(in, line))
//    {
//        istringstream iss(line);
//        string s;
//        iss >> s >> s >> s;
//        s = s.substr(3, s.length() - 4);
//        iss = istringstream(s);
//        int x, y, z;
//        char ch;
//        iss >> x >> ch >> y >> ch >> z;
//        acc.emplace_back(x, y, z);
//        int curacc = abs(x) + abs(y) + abs(z);
//        if (minacc > curacc)
//        {
//            minacc = curacc;
//            ans.clear();
//            ans.emplace_back(idx, acc.back());
//        }
//        else if (minacc == curacc)
//        {
//            ans.emplace_back(idx, acc.back());
//        }
//        ++idx;
//    }
//    assert(ans.size() == 1);
//    return ans[0].first;
//}
//
//void part_two()
//{
//}
//

#include <memory>
#include <type_traits>

struct noncopyable {
    noncopyable();
    noncopyable(noncopyable const &) = delete;
};

template <typename T> struct C {
    T t_;
    C() = default;
    C(C const&) = default;
    C(C &&) = default;
    template <typename U> C(U&& value);
};
 
C<noncopyable> w;
C<noncopyable> mc(std::move(w)); // Forwarding ctor

static_assert(std::is_copy_constructible_v<noncopyable>);
static_assert(std::is_copy_constructible_v<C<noncopyable>>);

int main()
{
    return 0;    
}