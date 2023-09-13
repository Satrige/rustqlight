#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#include "meta_command.h"
#include "statement.h"

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
  input_buffer->buffer[num_bytes_read - 1] = '\0';
}

void close_input_buffer (InputBuffer *input_buffer) {
  free(input_buffer->buffer);
  free(input_buffer);
}


void run_repl() {
  InputBuffer *input_buffer = new_input_buffer();
  int need_exit = 0;

  while (1) {
    print_promt();
    read_input(input_buffer);

    if (input_buffer->buffer[0] == '.') {
      need_exit = handle_meta_command(input_buffer);

      if (need_exit) {
        close_input_buffer(input_buffer);
        exit(EXIT_SUCCESS);
      }
    } else {
      handle_statement(input_buffer);
    } 
  }
}

