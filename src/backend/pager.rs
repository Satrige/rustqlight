pub mod page;

use std::io;
use std::process;
use std::fs::{File, metadata, remove_file, rename};
use std::io::{Read, Write};
use page::row_data::RowData;
use page::{Page, PAGE_SIZE};

pub struct Pager {
    file_name: String,
    file: Option<File>,
    pages: Vec<Page>,
}

impl Pager {
    pub fn new(file_name: &str) -> Self {
        if !metadata(file_name).is_ok() {
            println!("The db dump file {} doesn't exist", file_name);
            return Pager {
                file: None,
                file_name: file_name.to_string(),
                pages: Vec::new(),
            };
        }

        match File::open(file_name) {
            Ok(mut file) => {
                let saved_data = Self::read_file(&mut file);

                if let Ok(pages) = saved_data {
                    Pager {
                        file: Some(file),
                        file_name: file_name.to_string(),
                        pages,
                    }
                } else {
                    println!("Unable to read file metadata {}", file_name);
                    process::exit(1);
                }
            },
            Err(err) => {
                println!("Unable to open file {}: {}", file_name, err);
                process::exit(1);
            },
        }
    }

    fn read_file(file: &mut File) -> io::Result<Vec<Page>> {
        let file_metadata = file.metadata()?;
        let file_size = file_metadata.len();

        if file_size == 0 {
            return Ok(Vec::new());
        }

        let mut pages= Vec::new();
        // Read the last id (4 bytes)

        let mut buffer = vec![0u8; PAGE_SIZE];

        loop {
            let bytes_read = file.read(&mut buffer)?;

            if bytes_read == 0 {
                break;
            }

            pages.push(Page::new(Some(&buffer)));
        }

        Ok(pages)
    }

    pub fn get_last_saved_index(&self) -> Option<u32> {
        if let Some(last_page) = self.pages.last() {
            Some(last_page.get_last_idx())
        } else {
            None
        }
    }

    /// The method inserts new row to the last page
    /// If the page isn't inside the `self.pages` it tries to get it from file
    /// Otherwise it creates a new page.
    pub fn insert_new_row(&mut self, new_row_data: &RowData) -> io::Result<usize> {
        let cur_page = match self.pages.last_mut() {
            Some(last_page) => if last_page.get_num_rows() % PAGE_SIZE == 0 { // The last page is full
                self.pages.push(Page::new(None));

                self.pages.last_mut().unwrap()
            } else { // There is still place in the current page
                last_page
            },
            // No pages
            None => {
                self.pages.push(Page::new(None));

                self.pages.last_mut().unwrap()
            }
        };

        cur_page.insert_row(new_row_data)
    }

    pub fn select_all(&self) {
        for page in self.pages.iter() {
            page.print();
        }
    }

    pub fn save(&mut self) -> io::Result<()> {
        let tmp_file_name = format!("{}.tmp", self.file_name);

        let mut tmp_file = File::create(&tmp_file_name)?;

        for page in self.pages.iter() {
            tmp_file.write_all(
                page.get_destination()
            )?;
        }

        if metadata(&self.file_name).is_ok() {
            drop(self.file.take());
            remove_file(&self.file_name)?;
        }

        rename(&tmp_file_name, &self.file_name)?;

        Ok(())
    }
}
