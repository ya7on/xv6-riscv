#include "kernel/types.h"
#include "user/user.h"

extern int echo_rs();

int main(int argc, char *argv[]) {
  exit(echo_rs());
}
