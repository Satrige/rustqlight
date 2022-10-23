#include <string.h>
#include "../common.h"

typedef enum {
  PREPARE_SUCCESS,
  PREPARE_UNRECOGNIZED_STATEMENT,
} PrepareResult;

typedef enum {
  STATEMENT_INSERT,
  STATEMENT_SELECT
} StatementType;

typedef struct {
  StatementType type;
} Statement;

PrepareResult prepare_statement(InputBuffer *input_buffer, Statement *statement) {
  if (strncmp(input_buffer->buffer, "insert", input_buffer->buffer_length) == 0) {
    statement->type = STATEMENT_INSERT;
    return PREPARE_SUCCESS;
  } else if (strncmp(input_buffer->buffer, "select", input_buffer->buffer_length) == 0) {
    statement->type = STATEMENT_SELECT;
    return PREPARE_SUCCESS;
  }

  return PREPARE_UNRECOGNIZED_STATEMENT;
}

void execute_statement(Statement *statement) {
  printf("Here will be command execution.\n");
}

void handle_statement(InputBuffer *input_buffer) {
  Statement statement;

  switch (prepare_statement(input_buffer, &statement)) {
    case PREPARE_SUCCESS:
      execute_statement(&statement);
      printf("Here will be command execution");
      break;
    case PREPARE_UNRECOGNIZED_STATEMENT:
      printf("Can't recognize statement: %s.\n", input_buffer->buffer);
  }
}

