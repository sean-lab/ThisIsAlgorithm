#ifndef KNUTHMORRISPRAT_H
#define KNUTHMORRISPRAT_H

#include <stdio.h>

int  KnuthMorrisPratt(char* Text, int TextSize, int Start, 
                      char* Pattern, int PatternSize );

void Preprocess( char* Pattern, int PatternSize, int* Border );

#endif
