#include "kernel/types.h"
#include "user/user.h"

extern int calc_rs();

int
main(int argc, char **argv)
{
  exit(calc_rs());
}
