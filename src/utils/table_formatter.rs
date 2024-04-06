// format the table to be shown in a pretty manner
use crate::api::api::{GetTaskResponse, TableCharacteristicsResponse};
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
        write!(
            f,
            "An error occurred in the request to the server: {:?}",
            self.error.error_type
        )
    }
}
impl Display for GetTaskResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "| task           | DUE                 | GROUP       |")?;
        writeln!(f, "| -------------- | ------------------- | ----------- |")?;
        for detail in &self.res {
            writeln!(
                f,
                "| {:<15}| {:<20}| {:<12}|",
                detail.description,
                detail
                    .due
                    .map_or_else(|| "None".to_string(), |due| due.to_string()),
                detail.group
            )?;
        }
        Ok(())
    }
}

impl Display for TableCharacteristicsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "| table name        | supports due  |")?;
        writeln!(f, "| ----------------- | ------------- |")?;
        for table in &self.res {
            writeln!(f, "| {:<18}| {:<14}|", table.name, table.has_due)?;
        }
        Ok(())
    }
}
