#include <iostream>

#include <vector>

//

namespace col_f
{

    enum ColumnFlags : int
    {
        NONE = 0b0000'0000'0000'0000,

        UNIQUE = 0b0000'0000'0000'0001,
        INDEX = 0b0000'0000'0000'0010,

        UNIQUE_N_INDEX = 0b0000'0000'0000'0011,

        PRIMARY_KEY = 0b1000'0000'0000'0011
    };

    bool check_flag(const ColumnFlags& flag, const ColumnFlags& flags)
    {
        if (flags & !flag) return false;

        return true;
    }

}

template <typename Derived>
class Table
{
public:
    static std::vector<Derived*> instances;

    Table (void)
    {
        instances.push_back(static_cast<Derived*>(this));
    }

    template <typename T>
    class Column
    {
        Derived* table;
        T value;
        col_f::ColumnFlags flags;

    public:

        Column (Derived* table, const col_f::ColumnFlags& flags, const T& dvalue)
        : table(table), flags(flags), value(dvalue)
        {
            if (col_f::check_flag(col_f::PRIMARY_KEY, flags)) value = instances.size();
        }

        Derived& operator()(const T& value)
        {
            this->value = value;
            return *table;
        }

        T& operator*(void)
        {
            return value;
        }
    };
};

template <typename Derived>
std::vector<Derived*> Table<Derived>::instances;

#define DEFINE_COLUMN(Type, name, flags, dvalue) Column<Type> (name) {this, static_cast<col_f::ColumnFlags>(flags), (dvalue)}

//

class Foo : public Table<Foo>
{
public:
    DEFINE_COLUMN(size_t, id, col_f::PRIMARY_KEY, 0);
    DEFINE_COLUMN(short, value, col_f::UNIQUE, 2);
};

class Bar : public Table<Bar>
{
public:
    DEFINE_COLUMN(short, valuez, col_f::UNIQUE | col_f::INDEX, 1);
};

//

signed main (void)
{
    Foo a = Foo().value(20);

    Foo b = Foo().value(4);

    Foo c = Foo().value(17);

    Foo d = Foo().value(8);

    Bar e = Bar().valuez(5);

    Bar f = Bar().valuez(7);

    for (auto inst : Foo::instances)
        std::cout << *inst->id << ' ' << *inst->value << ", ";

    std::cout << std::endl;

    for (auto inst : Bar::instances)
        std::cout << *inst->valuez << ", ";

    return 0;
}
