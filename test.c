#include <stdio.h>
#include <time.h>
#include <unistd.h>
#include <crypt.h>

int main(void) {
  char salt[] = "$6$EWRtaRak22JtM7tk";
  char pass[] = "ausrobfin241814";
  char *password;

  password = crypt(pass, salt);
  puts(password);

  return 0;
}
