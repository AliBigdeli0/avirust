use calamine::{open_workbook, DataType, Reader, Xls};
use chrono::{DateTime, Utc};
use entity::dto::excel_dto::{ExcelColumnInfo, ExcelInfo, IGNORE_CODE};

const DATE_COLUMN_INDEX: usize = 0;
const TIME_COLUMN_INDEX: usize = 1;
const START_ROW: usize = 5;

pub fn read_excel_info(
    file_path: Box<&str>,
    excel_obj: &mut ExcelInfo,
) -> Result<(), Box<dyn std::error::Error>> {
    navigate_rows(file_path, true, move |_, row| {
        navigate_columns(row, |index_column, column| {
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

    navigate_rows(excel_path, false, |index_row, row| {
        if index_row < START_ROW {
            return Ok(());
        }

        let mut filled_excel_info = ExcelInfo::new();
        navigate_columns(row, |index_column, column| {
            if column.is_empty() {
                return Err("the cell is empty".into());
            }

            if index_column == DATE_COLUMN_INDEX || index_column == TIME_COLUMN_INDEX {
                if index_column == DATE_COLUMN_INDEX {
                    
                    let date = match fetch_as_date(column.get_string()) {
                        Ok(value) => value,
                        Err(e) => {
                            logger::error(&format!("error in : {:?}", e));
                            common::time_helper::get_now_utc()
                        }
                    };
                    filled_excel_info.set_date(date);
                }
            } else {
                let value_result = fetch_as_value(column.as_string());

                if let Ok(value) = value_result {
                    if base_info.list_items[index_column].code != IGNORE_CODE {
                        filled_excel_info.set_value(index_column, value);
                    }
                } else {
                    logger::error(&format!(
                        "error in read data from excel with following information: {}, {}, {}",
                        base_info.get_well_id(),
                        base_info.get_well_name(),
                        index_row
                    ));
                }
            }

            Ok(())
        })?;
        ret_result.push(filled_excel_info);
        Ok(())
    })?;

    Ok(ret_result)
}

fn fetch_as_value(column_value: Option<String>) -> Result<String, Box<dyn std::error::Error>> {
    if let Some(value_str) = column_value {
        if value_str.trim().is_empty() {
            Ok(value_str)
        } else {
            Err("the value is empty".into())
        }
    } else {
        Err("error in convert column into value".into())
    }
}

fn fetch_as_date(date_str_opt: Option<&str>) -> Result<DateTime<Utc>, Box<dyn std::error::Error>> {
    if let Some(date_str) = date_str_opt {
        let date_result = common::time_helper::convert_string_to_local(date_str);
        if let Ok(date) = date_result {
            let utc_date = common::time_helper::convert_local_to_utc(date);
            Ok(utc_date)
        } else {
            Err(format!("error in convert string to local date: {}", date_str).into())
        }
    } else {
        Err("error in convert date to str".into())
    }
}

fn navigate_columns<F>(row: &[DataType], mut natigator: F) -> Result<(), Box<dyn std::error::Error>>
where
    F: FnMut(usize, &DataType) -> Result<(), Box<dyn std::error::Error>>,
{
    for (index, column) in row.iter().enumerate() {
        natigator(index, column)?;
    }
    Ok(())
}

fn navigate_rows<F>(
    excel_path: Box<&str>,
    one_row: bool,
    mut natigator: F,
) -> Result<(), Box<dyn std::error::Error>>
where
    F: FnMut(usize, &[DataType]) -> Result<(), Box<dyn std::error::Error>>,
{
    let mut workbook: Xls<_> = open_workbook(*excel_path)?;
    if let Some(Ok(range)) = workbook.worksheet_range_at(0) {
        for (index, row) in range.rows().enumerate() {
            natigator(index, row)?;
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
