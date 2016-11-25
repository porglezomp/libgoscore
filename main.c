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
  const int width = 5;
  const int height = 5;
  char board[width*height] = {0};
  board[2] = GO_STONE_PRESENCE | GO_STONE_COLOR;
  print_board(board, width, height);
  guess_dead_stones(board, width, height);
  print_board(board, width, height);
}
