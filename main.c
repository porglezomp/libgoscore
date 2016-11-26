#include "goscore.h"

#include <stdio.h>

void print_board(char *data, size_t width, size_t height) {
  putchar('+');
  for (size_t i = 0; i < width*2 + 1; ++i) {
    putchar('-');
  }
  putchar('+');
  putchar('\n');

  for (size_t i = 0; i < width; ++i) {
    putchar('|');
    putchar(' ');
    for (size_t j = 0; j < height; ++j) {
      int val = data[i*width + j];
      if (val == 0) {
        putchar(' ');
      } else if (val < 27) {
        putchar('a' + val - 1);
      } else {
        putchar('A' + val - 27);
      }
      putchar(' ');
    }
    putchar('|');
    putchar('\n');
  }

  putchar('+');
  for (size_t i = 0; i < width*2 + 1; ++i) {
    putchar('-');
  }
  putchar('+');
  putchar('\n');
}

int main() {
  const int width = 5;
  const int height = 5;
  unsigned black = 0, white = 0;
  unsigned komi = 6;
  char board[width*height] = {0};
  board[2] = GO_STONE_PRESENCE | GO_STONE_COLOR;
  board[7] = GO_STONE_PRESENCE | GO_STONE_COLOR;
  board[10] = GO_STONE_PRESENCE | GO_STONE_COLOR;
  board[11] = GO_STONE_PRESENCE | GO_STONE_COLOR;

  board[12] = GO_STONE_PRESENCE;
  board[13] = GO_STONE_PRESENCE;
  board[14] = GO_STONE_PRESENCE;
  board[17] = GO_STONE_PRESENCE;
  board[22] = GO_STONE_PRESENCE;

  board[19] = GO_STONE_PRESENCE | GO_STONE_COLOR | GO_STONE_DEAD;
  board[23] = GO_STONE_PRESENCE | GO_STONE_COLOR | GO_STONE_DEAD;

  print_board(board, width, height);
  guess_dead_stones(board, width, height);
  print_board(board, width, height);
  score_stones(board, width, height);
  print_board(board, width, height);
  int winner = score_sums(board, width, height, komi, &black, &white);
  print_board(board, width, height);
  if (winner) {
    printf("White wins\n");
  } else {
    printf("Black wins\n");
  }
  printf("%d %d.5\n", black, white);
}
