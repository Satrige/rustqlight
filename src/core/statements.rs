use super::input_buffer::InputBuffer;

pub enum PrepareResult {
    PrepareSuccess,
    PrepareUnrecognizedStatement,
}

pub fn prepare_statement(input_buffer: &InputBuffer) -> PrepareResult {
    PrepareResult::PrepareSuccess
}
