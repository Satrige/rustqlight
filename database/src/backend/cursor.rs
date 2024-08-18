use std::io;
use super::pager::Pager;

pub struct Cursor<'pager_lifetime> {
    pager: &'pager_lifetime Pager,
    page_num: usize,
    cell_num: usize,
    end_of_table: bool,
}

pub enum CursorPosition {
    START,
    POS(usize),
    END,
}

impl Cursor{
    pub fn new(pager: &Pager, cursor_position: CursorPosition) -> io::Result<Self> {
        match cursor_position {
            CursorPosition::START => Ok(Cursor {
                pager,
                row_num: 0,
                end_of_table: false,
            }),
            CursorPosition::POS(row_num) => {
                let total_rows_number = pager.get_total_num_rows();

                if row_num >= total_rows_number {
                    Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "The row number is too big",
                    ))
                } else {
                    Ok(Cursor {
                        pager,
                        row_num,
                        end_of_table: row_num == total_rows_number - 1,
                    })
                }
            },
            CursorPosition::END => {
                let total_rows_number = pager.get_total_num_rows();

                Ok(Cursor {
                    pager,
                    row_num: total_rows_number - 1,
                    end_of_table: true,
                })
            },
        }
    }

    pub fn cursor_advance(&mut self) -> io::Result<usize> {
        let total_rows_number = self.pager.get_total_num_rows();

        // If we already at the end
        // Also handling the case when the total rows number was increased after we got to the end
        if (self.end_of_table && self.row_num >= total_rows_number - 1) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "The cursor is already at the end of table",
            ));
        }

        self.row_num += 1;
        self.end_of_table = self.row_num == total_rows_number - 1;

        Ok(self.row_num)
    }
}
