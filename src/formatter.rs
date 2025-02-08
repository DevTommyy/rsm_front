use serde_json::Value;
use tabled::{
    settings::{
        object::{Columns, Object, Rows},
        themes::Colorization,
        Alignment, Color, Format, Style, Width,
    },
    Table, Tabled,
};

use crate::impl_table_parsing;

#[derive(Tabled)]
struct FullDataRow {
    id: i32,
    description: String,
    due: String,
    group: String,
}

#[derive(Tabled)]
struct NoDueDataRow {
    id: i32,
    description: String,
    group: String,
}

#[derive(Tabled)]
struct NoGroupDataRow {
    id: i32,
    description: String,
    due: String,
}

#[derive(Tabled)]
struct MinimalDataRow {
    id: i32,
    description: String,
}

impl_table_parsing!(FullDataRow {
    description => "description",
    due => "due",
    group => "group",
});

impl_table_parsing!(NoDueDataRow {
    description => "description",
    group => "group",
});

impl_table_parsing!(NoGroupDataRow {
    description => "description",
    due => "due",
});

impl_table_parsing!(MinimalDataRow {
    description => "description",
});

pub fn format_list_res(res: &Value) -> Option<Table> {
    if let Some(arr) = res.get("res").and_then(|v| v.as_array()) {
        let has_due = arr.iter().any(|item| item.get("due").is_some());
        let has_group = arr.iter().any(|item| item.get("group").is_some());

        if has_due && has_group {
            Some(to_table(arr, FullDataRow::from_json))
        } else if has_due {
            Some(to_table(arr, NoGroupDataRow::from_json))
        } else if has_group {
            Some(to_table(arr, NoDueDataRow::from_json))
        } else {
            Some(to_table(arr, MinimalDataRow::from_json))
        }
    } else {
        None
    }
}

// since this is only this struct, it's manually implemented
#[derive(Tabled)]
struct SupportRow {
    name: String,
    #[tabled(rename = "group support")]
    group_support: String,
    #[tabled(rename = "due support")]
    due_support: String,
}

pub fn format_specs_res(res: &Value) -> Option<Table> {
    if let Some(arr) = res.get("res").and_then(|v| v.as_array()) {
        return Some(to_table(arr, |item| SupportRow {
            name: item
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
            group_support: item
                .get("has_group")
                .and_then(|v| v.as_bool())
                .map(|b| if b { "Yes" } else { "No" })
                .unwrap_or("No")
                .to_string(),
            due_support: item
                .get("has_due")
                .and_then(|v| v.as_bool())
                .map(|b| if b { "Yes" } else { "No" })
                .unwrap_or("No")
                .to_string(),
        }));
    }

    // If no valid data is found (should never be the case)
    None
}

fn to_table<T: Tabled>(arr: &[Value], from_json_fn: impl Fn(&Value) -> T) -> Table {
    let rows: Vec<T> = arr.iter().map(from_json_fn).collect();

    Table::new(&rows)
        .with(Style::modern_rounded())
        .with(Colorization::exact(
            [Color::BOLD | Color::FG_GREEN],
            Rows::first(),
        ))
        .modify(Rows::new(1..), Width::wrap(110).keep_words(true))
        .modify(Rows::first(), Format::content(|text| text.to_uppercase()))
        .modify(Columns::first().not(Rows::first()), Color::BOLD)
        .modify(Columns::first().not(Rows::first()), Alignment::right())
        .to_owned()
}
