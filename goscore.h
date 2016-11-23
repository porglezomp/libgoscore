#ifndef GOSCORE_H_INCLUDED
#define GOSCORE_H_INCLUDED

#include <stddef.h>

const char STONE_PRESENCE = 0x1;
const char STONE_COLOR = 0x2;
const char STONE_DEAD = 0x4;
const char STONE_SCORE = 0x8;
const char STONE_SCORE_COLOR = 0x10;

extern void guess_dead_tiles(char *data, size_t width, size_t height);
extern void score_tiles(char *data, size_t width, size_t height);
extern void score_sums(char *data, size_t width, size_t height,
                       unsigned komi, unsigned *white, unsigned *black);

#endif
