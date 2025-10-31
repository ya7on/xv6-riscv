#include "kernel/types.h"
#include "kernel/stat.h"
#include "user/user.h"

int getcmd(char *prompt, char *buf, int n) {
  write(2, prompt, strlen(prompt));
  memset(buf, 0, n);
  gets(buf, n);
  if(buf[0] == 0) // EOF
    return -1;
  return 0;
}

char *clear_cmd(char *prompt, char *buf, int n) {
    if (getcmd(prompt, buf, n) >= 0) {
      char *cmd = buf;
      while (*cmd == ' ' || *cmd == '\t') {
        cmd++;
      }
      cmd[strlen(cmd)-1] = 0;
      return cmd;
    };
    return 0;
}

int main(void) {
  static char buf[100];
  clear_cmd("A=", buf, sizeof(buf));
  int a = atoi(buf);
  clear_cmd("B=", buf, sizeof(buf));
  int b = atoi(buf);
  char *op = clear_cmd("op=", buf, sizeof(buf));

  if (strcmp(op, "+") == 0) {
    printf("%d + %d = %d\n", a, b, a + b);
  } else if (strcmp(op, "-") == 0) {
    printf("%d - %d = %d\n", a, b, a - b);
  } else if (strcmp(op, "*") == 0) {
    printf("%d * %d = %d\n", a, b, a * b);
  } else if (strcmp(op, "/") == 0) {
    printf("%d / %d = %d\n", a, b, a / b);
  } else {
    printf("Invalid operation\n");
    exit(1);
  }

  exit(0);
}
