use calamine::{open_workbook, DataType, Reader, Xls};
use chrono::{DateTime, NaiveDateTime, Utc};
use common::time_helper;
use entity::{
    dto::excel_dto::{ExcelColumnInfo, ExcelInfo, IGNORE_CODE},
    errors::ExcelError,
};

const DATE_COLUMN_INDEX: usize = 0;
const TIME_COLUMN_INDEX: usize = 1;
const START_ROW: usize = 3;

pub fn check_excel_info_with_others(
    excel_path: Box<&str>,
    excel_obj: &mut ExcelInfo,
) -> Result<(), ExcelError> {

    let excel_path_string = excel_path.to_owned().to_string();
    
    navigate_rows(excel_path, true, move |_, row, _| {
        navigate_columns(row,  |col_index: usize, column, row_total_size| {

            if col_index == DATE_COLUMN_INDEX || 
               col_index == TIME_COLUMN_INDEX 
            {
                return Ok(())
            }

            let column_value = match column.as_string() {
                Some(value) => Box::new(value.trim().to_string()), 
                None => Box::new("".to_string()),
            };

            let excel_obj_total_size = row_total_size - DATE_COLUMN_INDEX - TIME_COLUMN_INDEX - 1;
            let list_items_len = excel_obj.list_items.len();

            if  excel_obj_total_size != list_items_len {
                let error_msg = format!("expected '{list_items_len}' but got '{excel_obj_total_size}' in file '{excel_path_string}'");
                return Err(error_msg.into());
            }
            
            let column_index = col_index - DATE_COLUMN_INDEX - TIME_COLUMN_INDEX - 1;
            if !column_value.is_empty() {
                if let Some(excel_item) = excel_obj.list_items.get(column_index) {
                    let excel_item_column = excel_item.column.to_owned();
                    if  excel_item_column != *column_value {
                        let error_msg = format!("error at column '{column_index}', exptected '{excel_item_column}' but got '{column}', in file {excel_path_string}");    
                        return Err(error_msg.into());
                    }
                } else {
                    let error_msg = format!("error in get excel at item: {column_index}, in file '{excel_path_string}'");
                    return Err(error_msg.into());
                }
            }
            Ok(())
        })
        .map_err(|err| format!("erorr in check excel: {err}"))?;
        Ok(())
    })
    .map_err(|err| ExcelError::Message(format!("erorr in check excel: {err}")))?;
    Ok(())
}

pub fn read_excel_info(
    file_path: Box<&str>,
    excel_obj: &mut ExcelInfo,
) -> Result<(), Box<dyn std::error::Error>> {
    navigate_rows(file_path, true, move |_, row,_| {
        navigate_columns(row, |index_column, column, _| {
            if index_column == DATE_COLUMN_INDEX || index_column == TIME_COLUMN_INDEX {
                return Ok(());
            }

            excel_obj.append(column.to_string(), String::from(IGNORE_CODE));

            Ok(())
        })?;

        return Ok(());
    })?;

    Ok(())
}

pub fn read_from_excels(
    excel_path: Box<&str>,
    base_info: &mut ExcelInfo,
) -> Result<Vec<ExcelInfo>, Box<dyn std::error::Error>> {
    let mut ret_result = Vec::new();

    navigate_rows(excel_path, false, |index_row, row, _| {
        if index_row < START_ROW {
            return Ok(());
        }

        let mut filled_excel_info = base_info.clone();
        navigate_columns(row, |index_column, column, _| {
            if column.is_empty() {
                return Ok(());
            }

            
            if index_column == DATE_COLUMN_INDEX || index_column == TIME_COLUMN_INDEX{
                let date = match fetch_as_date(column.as_datetime()) {
                    Ok(value) => value,
                    Err(e) => {
                        logger::error(&format!("error in : {:?}", e));
                        common::time_helper::get_now_utc()
                    }
                };

                
                if index_column == DATE_COLUMN_INDEX { filled_excel_info.set_date(date) } 
                if index_column == TIME_COLUMN_INDEX  {filled_excel_info.set_time(date) };
            } else {
                let excel_info_column_index =
                    index_column - DATE_COLUMN_INDEX - TIME_COLUMN_INDEX - 1;
                if excel_info_column_index < base_info.list_items.len()
                    && base_info.list_items[excel_info_column_index].code != IGNORE_CODE
                {
                    let value_result = fetch_as_value(column);
                    if let Ok(value) = value_result {
                        filled_excel_info.set_value(excel_info_column_index, if value.is_empty() { "0".to_string() } else { value });
                    } else { 
                        logger::error(&format!(
                            "error in read data from excel with following information: {}, {} in [{},{}]",
                            base_info.get_well_id(),
                            base_info.get_well_name(),
                            index_row,
                            index_column,
                        ));
                    }
                }
            }

            Ok(())
        })?;
        ret_result.push(filled_excel_info);
        Ok(())
    })?;

    Ok(ret_result)
}

fn fetch_as_value(column_type: &DataType) -> Result<String, Box<dyn std::error::Error>> {
    let column_value = match column_type {
        DataType::Float(x) => format!("{:?}", x),
        DataType::Int(x) => format!("{:?}", x),
        DataType::String(x) => format!("{:?}", x),
        DataType::Error(x) => {
            logger::error(&format!("error in get data from cell : {}", x));
            String::from("")
        }
        _ => String::from(""),
    };

    if !column_value.trim().is_empty() {
        Ok(column_value)
    } else {
        Err("the value is empty".into())
    }
}

fn fetch_as_date(
    date_opt: Option<NaiveDateTime>,
) -> Result<DateTime<Utc>, Box<dyn std::error::Error>> {
    let local_date_time_result = time_helper::convert_naive_to_local(date_opt);

    if let Ok(local_date_time) = local_date_time_result {
        Ok(time_helper::convert_local_to_utc(local_date_time))
    } else {
        Err("error in convert naive date to local str".into())
    }
}

fn navigate_columns<F>(row: &[DataType], mut natigator: F) -> Result<(), Box<dyn std::error::Error>>
where
    F: FnMut(usize, &DataType, usize) -> Result<(), Box<dyn std::error::Error>>,
{   
    let column_total_size = row.iter().filter(|&x| {
        if let Some(value) =x.as_string() {
            !value.is_empty()
        }
        else {
            false   
        }
    }).count(); 

    for (index, column) in row.iter().enumerate() {
        natigator(index, column, column_total_size)?;
    }
    Ok(())
}

fn navigate_rows<F>(
    excel_path: Box<&str>,
    one_row: bool,
    mut natigator: F,
) -> Result<(), Box<dyn std::error::Error>>
where
    F: FnMut(usize, &[DataType], usize) -> Result<(), Box<dyn std::error::Error>>,
{
    let mut workbook: Xls<_> = open_workbook(*excel_path)?;
    if let Some(Ok(range)) = workbook.worksheet_range_at(0) {
        let total_column_size = range.rows().len();
        for (index, row) in range.rows().enumerate() {
            natigator(index, row, total_column_size)?;
            if one_row {
                break;
            }
        }
    }

    Ok(())
}

pub fn check_configurations(excel_info: &ExcelInfo) -> Result<String, String> {
    let none_empty_items: Vec<&ExcelColumnInfo> = excel_info
        .list_items
        .iter()
        .filter(|x| x.code != IGNORE_CODE)
        .clone()
        .collect();

    if none_empty_items.len() == 0 {
        return Err(String::from(
            "ERROR: there is 'NO' specified code in the file name",
        ));
    }

    Ok(none_empty_items
        .iter()
        .map(|s| format!("\n\t{}={}", s.column, s.code))
        .collect::<Vec<_>>()
        .join(", "))
}
