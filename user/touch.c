#include "kernel/types.h"
#include "user/user.h"

extern int touch_rs();

int main(int argc, char *argv[]) {
  exit(touch_rs());
}
