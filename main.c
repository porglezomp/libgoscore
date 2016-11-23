#include <stdio.h>
#include <stdint.h>

const char STONE_PRESENCE = 0x1;
const char STONE_COLOR = 0x2;
const char STONE_DEAD = 0x4;
const char STONE_SCORE = 0x8;
const char STONE_SCORE_COLOR = 0x10;

extern void guess_dead_tiles(char *data, size_t width, size_t height);
extern void score_tiles(char *data, size_t width, size_t height);

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
