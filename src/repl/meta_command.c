#include <stdlib.h>
#include <string.h>

#include "../common.h"

typedef enum {
  EXIT_META_COMMAND,
  UNRECOGNIZED_META_COMMAND,
} MetaCommand;

MetaCommand recognize_meta_command (InputBuffer * input_buffer) {
  if (!input_buffer->input_length) {
    return UNRECOGNIZED_META_COMMAND;
  }

  if (strncmp(input_buffer->buffer, ".exit", input_buffer->input_length) == 0) {
    return EXIT_META_COMMAND;
  }

  return UNRECOGNIZED_META_COMMAND;
}

int execute_meta_command (MetaCommand meta_command) {
  printf("The command '%d' is not implemented yet.\n", meta_command);
  return 0;
}

int handle_meta_command (InputBuffer * input_buffer) {
  MetaCommand meta_command = recognize_meta_command(input_buffer);

  if (meta_command == UNRECOGNIZED_META_COMMAND) {
    printf("Unrecognized command: %s.\n", input_buffer->buffer);
    return 0;
  }

  if (meta_command == EXIT_META_COMMAND) {
    return 1;
  }

  return execute_meta_command(meta_command);
}

