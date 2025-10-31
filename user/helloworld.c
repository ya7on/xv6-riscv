#include "kernel/types.h"
#include "user/user.h"

extern int hello_world_rs();

int
main(int argc, char **argv)
{
  exit(hello_world_rs());
}
