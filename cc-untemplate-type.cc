template <class T>
struct Untemp
{
  using type = void;
};

template <template<class TemplateT, class ... TemplateTs> class Template, class T>
struct Untemp<Template<T>>
{
  using type = T;
};

template <class T>
using untemp = typename Untemp<T>::type;

#include <vector>
#include <type_traits>

class Class {};

signed main (void)
{
  std::vector<Class> v;

  static_assert(std::is_same<untemp<decltype(v)>, Class>::value);

  return 0;
}
