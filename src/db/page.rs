use super::data::Row;

const MAX_ROWS_PER_PAGE: usize = 128;

#[derive(Debug)]
pub struct Page {
    pub rows: [Option<Row>; MAX_ROWS_PER_PAGE],
    free_space: usize,
}

impl Page {
    pub fn new() -> Self {
        Self {
            rows: [const { None }; MAX_ROWS_PER_PAGE],
            free_space: MAX_ROWS_PER_PAGE,
        }
    }

    pub fn is_full(&self) -> bool {
        self.free_space == 0
    }

    pub fn num_rows(&self) -> usize {
        MAX_ROWS_PER_PAGE - self.free_space
    }

    pub fn insert_row(&mut self, row: Row) -> Result<(), String> {
        if self.is_full() {
            return Err("Page is full".to_string());
        }

        let index = self.rows.iter().position(|r| r.is_none()).unwrap();
        self.rows[index] = Some(row);
        self.free_space -= 1;

        Ok(())
    }

    /*
    pub fn get_row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index).unwrap().as_ref()
    }


    pub fn delete_row(&mut self, index: usize) -> Result<(), String> {
        if self.rows.get(index).is_none() {
            return Err("Row not found".to_string());
        }

        self.rows[index] = None;
        self.free_space += 1;

        Ok(())
    }

    pub fn update_row(&mut self, index: usize, row: Row) -> Result<(), String> {
        if self.rows.get(index).is_none() {
            return Err("Row not found".to_string());
        }

        self.rows[index] = Some(row);

        Ok(())
    }
    */
}
