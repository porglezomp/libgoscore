#include "goscore.h"

#include <stdio.h>

void print_board(char *data, size_t width, size_t height) {
  puts("-----");

  for (size_t i = 0; i < width; ++i) {
    for (size_t j = 0; j < height; ++j) {
      printf("%d ", data[i*width + j]);
    }
    puts("");
  }

  puts("-----");
}

int main() {
  char board[19*19] = {0};
  board[2] = STONE_PRESENCE | STONE_COLOR;
  print_board(board, 19, 19);
  guess_dead_tiles(board, 19, 19);
  print_board(board, 19, 19);
}
