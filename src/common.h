#ifndef REPL_H
#define REPL_H

#include <stdio.h>

typedef struct {
  char *buffer;
  size_t buffer_length;
  ssize_t input_length;
} InputBuffer;

#endif

