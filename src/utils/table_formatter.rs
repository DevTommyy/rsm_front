// format the table to be shown in a pretty manner
use crate::api::api_list::{GetTaskResponse, TableCharacteristicsResponse};
use crate::api::ErrorResponse;
use std::fmt::Display;

// -- Custom trait impl
pub trait FormattedResponse {
    fn print(&self);
}
impl FormattedResponse for GetTaskResponse {
    fn print(&self) {
        println!("{}", self);
    }
}
impl FormattedResponse for TableCharacteristicsResponse {
    fn print(&self) {
        println!("{}", self);
    }
}
impl FormattedResponse for ErrorResponse {
    fn print(&self) {
        println!("{}", self);
    }
}

// -- Display impl
impl Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "+ - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - +"
        )?;
        writeln!(
            f,
            "| An error occurred in the request to the server: \x1b[31m{}\x1b[0m |",
            self.error.error_type
        )?;
        writeln!(
            f,
            "+ - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - +"
        )?;
        Ok(())
    }
}

impl std::fmt::Display for GetTaskResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "+ ------------------------------------- + ------------------- + ------------------- +"
        )?;
        writeln!(f, "| \x1b[34mTASK\x1b[0m                                  | \x1b[34mDUE\x1b[0m                 | \x1b[34mGROUP\x1b[0m               |")?;
        writeln!(
            f,
            "+ ------------------------------------- + ------------------- + ------------------- +"
        )?;
        for detail in &self.res {
            writeln!(
                f,
                "| {:<38}| {:<20}| {:<20}|",
                detail.description,
                detail.due.map_or_else(
                    || "None".to_string(),
                    |due| due.format("%Y-%m-%d %H:%M:%S").to_string()
                ),
                detail.group
            )?;
        }
        writeln!(
            f,
            "+ ------------------------------------- + ------------------- + ------------------- +"
        )?;
        Ok(())
    }
}

impl std::fmt::Display for TableCharacteristicsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "+ ----------------- + ------------- +")?;
        writeln!(
            f,
            "| \x1b[34mTABLE NAME\x1b[0m        | \x1b[34mSUPPORTS DUE\x1b[0m  |"
        )?;
        writeln!(f, "| ----------------- | ------------- |")?;
        for table in &self.res {
            writeln!(f, "| {:<18}| {:<14}|", table.name, table.has_due)?;
        }
        writeln!(f, "+ ----------------- + ------------- +")?;
        Ok(())
    }
}
