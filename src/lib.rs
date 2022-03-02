#[derive(Clone, Copy)]
enum CellValue {
    Empty,
    Mine,
    Number(u16)
}

impl CellValue {
    fn to_char(&self) -> char {
        match self {
            CellValue::Empty => ' ',
            CellValue::Mine => '*',
            CellValue::Number(num) => {
                match std::char::from_u32(*num as u32) {
                    Some(c) => c,
                    None => ' ',
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
struct Cell {
    row: usize,
    col: usize,
    val: CellValue
}

impl Cell {
    fn new(row: usize, col: usize) -> Self {
        Cell {
            row: row,
            col: col,
            val: CellValue::Empty
        }
    }

    fn set_val(&mut self, new_val: CellValue) {
        self.val = new_val;
    } 

    fn increment_number(&mut self) {
        match self.val {
            CellValue::Number(num) => self.val = CellValue::Number(num + 1),
            _ => ()
        }        
    }
}

type CellRow = Vec<Vec<Cell>>;

struct Minefield {
    rows: CellRow,
}

impl Minefield {
    fn new(nr_rows: usize, nr_cols: usize) -> Self {
        let mut rows = Vec::new();
        for row in 0..nr_rows {
            let mut cols = Vec::new();
            for col in 0..nr_cols {
                cols.insert(col, Cell::new(row, col));
            }
            rows.insert(row, cols);
        }

        Minefield { rows }
    }

    fn update_cell_at(&mut self, row_to_find: usize, col_to_find: usize, new_val: CellValue) {
        for row in &mut self.rows {
            for cell in row {
                if cell.row == row_to_find && cell.col == col_to_find {
                    cell.set_val(new_val);
                    return;
                }
            }
        }
    }

    fn update(&mut self, input: &[&str]) {
        for (row_i, row) in input.iter().enumerate() {
            for (col_i, col) in row.bytes().enumerate() {
                if col == b'*' {
                    self.update_cell_at(row_i, col_i, CellValue::Mine);
                }
                // TODO: if around mine, update numbers
            }
        }
    }

    fn render(self) -> Vec<String> {
        let mut output = Vec::new();
        for row in self.rows {
            let mut row_text = String::new();
            for cell in row {
                row_text.push(cell.val.to_char());
            }
            output.push(row_text);
        }
        output
    }
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let nr_rows= minefield.len();
    let mut nr_cols = 0;
    if nr_rows > 0 {
        nr_cols = minefield[0].len();
    }

    let mut field = Minefield::new(nr_rows, nr_cols);
    field.update(minefield);
    field.render()
}
