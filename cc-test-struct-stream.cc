#include <cstdlib>
#include <ctime>
#include <functional>
#include <algorithm>
#include <vector>
#include <iostream>
#include <optional>

//

template<class Structure>
class StructureStream
{
    const Structure structure;

public:

    StructureStream(const Structure structure)
    : structure(structure)
    {}

    template<typename F>
    StructureStream operator<<(F f) const
    {
        Structure that = this->structure;
        f(that.begin(), that.end());
        return StructureStream(that);
    }

    Structure operator*(void) const
    {
        return this->structure;
    }
};

template<class Structure>
StructureStream<Structure> sstr(const Structure structure)
{
    return StructureStream<Structure>(structure);
}

//


template<typename PredicateT>
struct SortT
{
private:
    std::optional<PredicateT> predicate;

public:
    SortT(PredicateT predicate)
    : predicate(predicate)
    {}

    SortT()
    : predicate({})
    {}

    void operator()(auto begin, auto end)
    {
        if (auto p = this->predicate)
            std::sort(begin, end, *p);
        else
            std::sort(begin, end);
    }
};

template<typename PredicateT>
SortT<PredicateT> sort(PredicateT predicate)
{
    return SortT<PredicateT>(predicate);
}

SortT<bool> sort()
{
    return SortT<bool>();
}


template<typename PredicateT>
struct FilterT
{
private:
    PredicateT predicate;

public:
    FilterT(PredicateT predicate)
    : predicate(predicate)
    {}

    void operator()(auto begin, auto end)
    {
        std::remove_if(begin, end, [this](auto x){ return !this->predicate(x); });
    }
};

template<typename PredicateT>
FilterT<PredicateT> filter(PredicateT predicate)
{
    return FilterT<PredicateT>(predicate);
}

//

using std::vector, std::cout, std::endl;

constexpr size_t N = 20;

struct
{
    void operator()(auto begin, auto end)
    {
        for (; begin != end; ++ begin)
            if (*begin % 2 == 0) *begin = 0;
    }
} zero_pairs;

signed main(void)
{
    srand(time(NULL));

    vector<int> v(N);

    for (auto& vi : v) vi = rand() % 5;

    for (const auto& vi : v) cout << vi << ' ';
    cout << endl;

    vector<int> w = *(sstr(v) << sort([](int x, int y){ return x < y; }) << zero_pairs << filter([](int x){ return x != 0; }));

    for (const auto& wi : w) cout << wi << ' ';
    cout << endl;

    return 0;
}
