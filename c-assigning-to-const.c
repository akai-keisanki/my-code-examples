void liar (const int* x)
{
  * (int*) x = 24;
  
  return;
}

#include <assert.h>
#include <stdio.h>

signed main (void)
{
  const int CONST = 1234;

  assert(CONST == 1234);

  liar(&CONST);

  assert(CONST == 24);

	return 0;
}
