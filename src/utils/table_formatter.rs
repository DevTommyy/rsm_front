// format the table to be shown in a pretty manner
use crate::api::api_list::{GetTaskResponse, TableCharacteristicsResponse};
use crate::api::{ErrorResponse, SuccessfulResponse};
use std::fmt::Display;

// -- Custom trait impl
pub trait FormattedResponse {
    fn print(&self);

    fn as_any(&self) -> &dyn std::any::Any;
}

impl FormattedResponse for GetTaskResponse {
    fn print(&self) {
        println!("{}", self);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl FormattedResponse for TableCharacteristicsResponse {
    fn print(&self) {
        println!("{}", self);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl FormattedResponse for ErrorResponse {
    fn print(&self) {
        println!("{}", self);
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl FormattedResponse for SuccessfulResponse {
    fn print(&self) {
        println!("{}", self);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
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

impl Display for SuccessfulResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let len = self.res.len();
        let line = "-".repeat(len + 2);

        let mut start_idx = 0;

        writeln!(f, "+ {} +", line)?;
        write!(f, "| ")?;

        while let Some(open_quote_idx) = self.res[start_idx..].find('\'') {
            let adjusted_idx = start_idx + open_quote_idx;
            let end_idx = match self.res[adjusted_idx + 1..].find('\'') {
                Some(idx) => adjusted_idx + idx + 1,
                None => return Err(std::fmt::Error),
            };

            write!(f, "{}", &self.res[start_idx..adjusted_idx])?;
            write!(f, "\x1b[32m")?;
            write!(f, "{}", &self.res[adjusted_idx..=end_idx])?;
            write!(f, "\x1b[0m")?;
            start_idx = end_idx + 1;
        }

        write!(f, "{}", &self.res[start_idx..])?;
        writeln!(f, "{:<3}|", " ")?;
        writeln!(f, "+ {} +", line)?;

        Ok(())
    }
}
impl std::fmt::Display for GetTaskResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "+ -------------------------------------------------------------------------- + ------------------- + ------------------- +"
        )?;
        writeln!(f, "| \x1b[34mTASK\x1b[0m {:<69} | \x1b[34mDUE\x1b[0m                 | \x1b[34mGROUP\x1b[0m               |", " ")?;
        writeln!(
            f,
            "+ -------------------------------------------------------------------------- + ------------------- + ------------------- +"
        )?;
        for detail in &self.res {
            writeln!(
                f,
                "| {:<75}| {:<20}| {:<20}|",
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
            "+ -------------------------------------------------------------------------- + ------------------- + ------------------- +"
        )?;
        Ok(())
    }
}

impl std::fmt::Display for TableCharacteristicsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "+ ------------------------------- + ------------- +")?;
        writeln!(
            f,
            "| \x1b[34mTABLE NAME\x1b[0m {:<20} | \x1b[34mSUPPORTS DUE\x1b[0m  |",
            " "
        )?;
        writeln!(f, "| ------------------------------- | ------------- |")?;
        for table in &self.res {
            writeln!(f, "| {:<32}| {:<14}|", table.name, table.has_due)?;
        }
        writeln!(f, "+ ------------------------------- + ------------- +")?;
        Ok(())
    }
}
