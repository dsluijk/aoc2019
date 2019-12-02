#include <stdio.h>
#include <stdlib.h>

void run(int *opcodes) {
  int pos = 0;
  while(1) {
    switch(opcodes[pos]) {
      case 1: {
        int val = opcodes[opcodes[pos + 1]];
        val += opcodes[opcodes[pos + 2]];
        opcodes[opcodes[pos + 3]] = val;
        pos += 4;
        break;
      }
      case 2: {
        int val = opcodes[opcodes[pos + 1]];
        val *= opcodes[opcodes[pos + 2]];
        opcodes[opcodes[pos + 3]] = val;
        pos += 4;
        break;
      }
      case 99: {
        return;
      }
      default: {
        printf("Invalid opcode %i!", opcodes[pos]);
        exit(1);
      }
    }
  }
}

void reset(int *opcodes) {
  FILE *input = fopen("input.txt", "r");

  if (input == NULL) {
    perror("Error while opening the file");
    exit(1);
  }

  int charIn;
  int pos = 0;
  int current = 0;
  while((charIn = fgetc(input)) != EOF) {
    if(charIn == ',') {
      opcodes[pos] = current;
      current = 0;
      pos++;
      continue;
    }

    current *= 10;
    current += charIn - 48;
  }

  opcodes[pos] = current;
  fclose(input);
}

int main() {
  int *opcodes = malloc(sizeof(unsigned long) * 1000);

  for(int i = 0; i < 100; i++) {
    for(int j = 0; j < 100; j++) {
      reset(opcodes);
      opcodes[1] = i;
      opcodes[2] = j;

      run(opcodes);

      if(opcodes[0] == 19690720) {
        printf("Found the noun! %i\n", (opcodes[1] * 100) + opcodes[2]);
        free(opcodes);
        exit(0);
      }
    }
  }
}