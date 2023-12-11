use calamine::{open_workbook, Error, Xls, Reader, DataType};
use entity::dto::excel_dto::{ExcelInfo, ExcelColumnInfo};

const DATE_COLUMN_INDEX: usize= 0; 
const TIME_COLUMN_INDEX: usize= 1;
const START_ROW: usize = 5;
const IGNORE_CODE: &str ="-";

pub fn read_excel_info(file_path: Box<&str>, excel_obj: &mut ExcelInfo) -> Result<(),Error>{

    navigate_rows(file_path,true ,move |_ ,row| {

        navigate_columns(row, |index_column,column| {

            if index_column == DATE_COLUMN_INDEX ||  index_column == TIME_COLUMN_INDEX {
                return  Ok(());
            }

            excel_obj.append(column.to_string(), String::from(IGNORE_CODE));
            
            Ok(())
        })?;

        return Ok(())
    })?;

    Ok(())
}

pub fn read_from_excels(excel_path: Box<&str>, _: &mut ExcelInfo) -> Result<Vec<ExcelInfo>, Error>{
    let mut ret_result = Vec::new();

    navigate_rows(excel_path, false, |index_row ,row| {
        if index_row < START_ROW {
            return Ok(());
        }

        let filled_excel_info = ExcelInfo::new();
        navigate_columns(row, |index_column,column| {
            if column.is_empty() {
                return Ok(());
            }

            if index_column == DATE_COLUMN_INDEX || index_column == TIME_COLUMN_INDEX {
                                
            }
            else {

            }

            Ok(())
        })?;
        ret_result.push(filled_excel_info);
        Ok(())
    })?;

    Ok(ret_result)
}

fn navigate_columns<F>(row:&[DataType], mut natigator: F) -> Result<(), Error> 
    where F: FnMut(usize, &DataType) ->  Result<(), Error>
{
    for (index,column) in row.iter().enumerate() {
        natigator(index, column)?;
    }
    Ok(())
}

fn navigate_rows<F>(excel_path: Box<&str>,one_row: bool ,mut natigator: F) -> Result<(), Error>
    where F: FnMut(usize,&[DataType])->Result<(), Error>
{
    let mut workbook:   Xls<_> = open_workbook(*excel_path)?; 

    if let Some(Ok(range)) =  workbook.worksheet_range_at(0) {
        for (index, row) in range.rows().enumerate() {
            natigator(index,row)?;
            if one_row {
                break;
            }
        }
    }

    Ok(())
}

pub fn check_configurations(excel_info: &ExcelInfo) -> Result<String, String> {
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
