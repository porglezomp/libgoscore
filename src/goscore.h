#ifndef GOSCORE_H_INCLUDED
#define GOSCORE_H_INCLUDED

#include <stddef.h>

#define GO_STONE_PRESENCE 0x1
#define GO_STONE_COLOR 0x2
#define GO_STONE_DEAD 0x4
#define GO_STONE_SCORE 0x8
#define GO_STONE_SCORE_COLOR 0x10

extern void guess_dead_stones(char *data, size_t width, size_t height);
extern void score_stones(char *data, size_t width, size_t height);
extern int score_sums(char *data, size_t width, size_t height,
                      unsigned komi, unsigned *black, unsigned *white);

#endif
