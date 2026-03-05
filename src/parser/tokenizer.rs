use crate::parser::state::InsertFormat;

pub fn split_tuple(data: &[u8], format: InsertFormat) -> Vec<Vec<u8>> {
    match format {
        InsertFormat::Copy => split_copy_tuple(data),
        InsertFormat::Values => split_insert_tuple(data),
    }
}

fn split_copy_tuple(data: &[u8]) -> Vec<Vec<u8>> {
    let clean_data = if data.ends_with(b"\r\n") {
        &data[..data.len() - 2]
    } else if data.ends_with(b"\n") {
        &data[..data.len() - 1]
    } else {
        data
    };

    clean_data
        .split(|&b| b == b'\t')
        .map(|col| col.to_vec())
        .collect()
}

fn split_insert_tuple(data: &[u8]) -> Vec<Vec<u8>> {
    let mut columns = Vec::new();
    let mut current_col = Vec::new();
    let mut inside_string = false;
    let mut escape_next = false;

    for &byte in data {
        if escape_next {
            current_col.push(byte);
            escape_next = false;
            continue;
        }

        match byte {
            b'\\' => {
                current_col.push(byte);
                escape_next = true;
            }
            b'\'' => {
                current_col.push(byte);
                inside_string = !inside_string;
            }
            b',' if !inside_string => {
                columns.push(std::mem::take(&mut current_col));
            }
            _ => {
                current_col.push(byte);
            }
        }
    }

    columns.push(current_col);

    columns
}

pub fn join_tuple(columns: &[Vec<u8>], format: InsertFormat) -> Vec<u8> {
    match format {
        InsertFormat::Copy => {
            let mut joined = columns.join(&b"\t"[..]);
            joined.push(b'\n');
            joined
        }
        InsertFormat::Values => columns.join(&b","[..]),
    }
}
