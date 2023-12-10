use calamine::{open_workbook, Error, Xls, Reader};
use entity::dto::excel_dto::{ExcelInfo, ExcelColumnInfo};

const DATE_COLUMN_INDEX: usize= 0; 
const TIME_COLUMN_INDEX: usize= 1;

const IGNORE_CODE: &str ="-";

pub fn read_excel_info(file_path: Box<&str>, excel_obj: &mut ExcelInfo) -> Result<(),Error>{
    let mut workbook:   Xls<_> = open_workbook(*file_path)?; 
    
    if let Some(Ok(range)) =  workbook.worksheet_range_at(0) {
        if let Some(row) = range.rows().next() {
            for (index, column) in row.iter().enumerate() {
                if index == DATE_COLUMN_INDEX ||  index == TIME_COLUMN_INDEX { 
                    continue;
                }
            excel_obj.append(column.to_string(), String::from(IGNORE_CODE));
            }
        }
    }

    Ok(())
}

pub fn check_configurations(excel_info: ExcelInfo) -> Result<String, String> {
    let none_empty_items: Vec<&ExcelColumnInfo>=   excel_info
        .list_items
        .iter()
        .filter(|x| x.code != IGNORE_CODE)
        .clone()
        .collect();
    
    if none_empty_items.len() ==  0 {
        return Err(String::from("ERROR: there is 'NO' specified code in the file name"));
    }

    Ok(
        none_empty_items
        .iter()
        .map(|s| format!("\n\t{}={}", s.column, s.code))
        .collect::<Vec<_>>().join(", ")
    )
}

// fn read_excel_to_end(range: &Range<DataType>) -> u32 {
//     let mut counter:u32 = 3;
//     loop {
//         let value = range.get_value((0, counter));
//         match value {
//             Some(_) => {
//                 counter += 1;
//             } ,
//             None => {
//                 break;
//             }
//         }
//     }
//     counter
// }
