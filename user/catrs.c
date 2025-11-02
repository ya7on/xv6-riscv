#include "kernel/types.h"
#include "user/user.h"

extern int cat_rs();

int main(int argc, char *argv[]) {
  exit(cat_rs());
}
