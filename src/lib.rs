
mod wb;
mod ws;
mod utils;

pub use wb::Workbook;
pub use ws::Worksheet;
pub use utils::{col2num, excel_number_to_date, num2col};

pub struct Config {
    /// Which xlsx file should we print?
    pub workbook_path: String,
    /// Which tab should we print?
    pub tab: String,
    /// How many rows should we print?
    pub nrows: Option<u32>,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 2 {
            return Err(format!("Usage: {} <path-to-xlsx> <tab> [-n num-rows]", args[0]))
        } else if args.len() < 3 {
            return Err("must also provide which tab you want to view in workbook".to_owned())
        }
        let workbook_path = args[1].clone();
        let tab = args[2].clone();
        let mut config = Config { workbook_path, tab, nrows: None };
        let mut iter = args[3..].iter();
        while let Some(flag) = iter.next() {
            let flag = &flag[..];
            match flag {
                "-n" => {
                    if let Some(nrows) = iter.next() {
                        if let Ok(nrows) = nrows.parse::<u32>() {
                            config.nrows = Some(nrows)
                        } else {
                            return Err("number of rows must be an integer value".to_owned())
                        }
                    } else {
                        return Err("must provide number of rows when using -n".to_owned())
                    }
                },
                _ => return Err(format!("unknown flag: {}", flag)),
            }
        }
        Ok(config)
    }
}

pub fn run(config: Config) -> Result<(), String> {
    match crate::Workbook::new(&config.workbook_path) {
        Ok(mut wb) => {
            let sheets = wb.sheets();
            if let Some(ws) = sheets.get(&*config.tab) {
                let nrows = if let Some(nrows) = config.nrows {
                    nrows as usize
                } else {
                    1048576 // max number of rows in an Excel worksheet
                };
                for row in ws.rows(&mut wb).take(nrows) {
                    println!("{}", row);
                }
            } else {
                return Err("that sheet does not exist".to_owned())
            }
            Ok(())
        },
        Err(e) => Err(e)
    }
}