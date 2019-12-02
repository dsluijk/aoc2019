#include <stdio.h>

int main() {
  FILE *input = fopen("input.txt", "r");

  if (input == NULL) {
    perror("Error while opening the file");
    return 1;
  }

  unsigned long opcodes[1000];

  int charIn;
  unsigned long pos = 0;
  unsigned long current = 0;
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
  pos = 0;

  opcodes[1] = 12;
  opcodes[2] = 2;

  while(1) {
    switch(opcodes[pos]) {
      case 1: {
        unsigned long val = opcodes[opcodes[pos + 1]];
        val += opcodes[opcodes[pos + 2]];
        opcodes[opcodes[pos + 3]] = val;
        pos += 4;
        break;
      }
      case 2: {
        unsigned long val = opcodes[opcodes[pos + 1]];
        val *= opcodes[opcodes[pos + 2]];
        opcodes[opcodes[pos + 3]] = val;
        pos += 4;
        break;
      }
      case 99: {
        printf("program done, result: %lu", opcodes[0]);
        fclose(input);
        return 0;
      }
      default: {
        printf("Invalid opcode %lu!", opcodes[pos]);
        fclose(input);
        return 1;
      }
    }
  }
}