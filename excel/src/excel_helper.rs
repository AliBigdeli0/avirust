
use calamine::{open_workbook, Error, Xlsx, Reader};
use entity::dto::excel_dto::ExcelInfo;

const DATE_COLUMN_INDEX: usize= 0; 
const TIME_COLUMN_INDEX: usize= 1;


pub fn read_excel_info(file_path: Box<&str>, excel_obj: &mut ExcelInfo) -> Result<(),Error>{
    let mut workbook:   Xlsx<_> = open_workbook(*file_path)?; 
    
    if let Some(Ok(range)) =  workbook.worksheet_range_at(0) {
        if let Some(row) = range.rows().next() {
            for (index, column) in row.iter().enumerate() {
                if index == DATE_COLUMN_INDEX ||  index == TIME_COLUMN_INDEX { 
                    continue;
                }
                excel_obj.append(column.to_string(), String::from(""));
            }
        }
    }

    Ok(())
}
