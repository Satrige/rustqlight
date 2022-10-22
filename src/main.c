#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdio.h>

typedef struct {
  char *buffer;
  size_t buffer_length;
  ssize_t input_length;
} InputBuffer;

InputBuffer *new_input_buffer () {
  InputBuffer *input_buffer = (InputBuffer *)malloc(sizeof(InputBuffer));

  input_buffer->buffer = NULL;
  input_buffer->buffer_length = 0;
  input_buffer->input_length = 0;

  return input_buffer;
}

void print_promt () {
  printf("db > ");
}

void read_input (InputBuffer *input_buffer) {
  ssize_t num_bytes_read = getline(&(input_buffer->buffer), &(input_buffer->buffer_length), stdin);

  if (num_bytes_read <= 0) {
    printf("Error reading input");

    exit(EXIT_FAILURE);
  }

  // Ignore trailing newline
  input_buffer->input_length = num_bytes_read - 1;
  input_buffer->buffer[num_bytes_read - 1] = 0;
}



