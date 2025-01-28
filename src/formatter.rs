use serde_json::Value;
use tabled::{settings::Style, Table, Tabled};

use crate::impl_table_parsing;

#[derive(Tabled)]
struct FullDataRow {
    description: String,
    due: String,
    group: String,
    id: i32,
}

#[derive(Tabled)]
struct NoDueDataRow {
    description: String,
    group: String,
    id: i32,
}

#[derive(Tabled)]
struct NoGroupDataRow {
    description: String,
    due: String,
    id: i32,
}

#[derive(Tabled)]
struct MinimalDataRow {
    description: String,
    id: i32,
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

pub fn format_list_res(res: &Value) -> String {
    if let Some(arr) = res.get("res").and_then(|v| v.as_array()) {
        let has_due = arr.iter().any(|item| item.get("due").is_some());
        let has_group = arr.iter().any(|item| item.get("group").is_some());

        if has_due && has_group {
            // Use FullDataRow
            let rows: Vec<FullDataRow> = arr.iter().map(FullDataRow::from_json).collect();
            Table::new(&rows).with(Style::rounded()).to_string()
        } else if has_due {
            // Use NoGroupDataRow
            let rows: Vec<NoGroupDataRow> = arr.iter().map(NoGroupDataRow::from_json).collect();
            Table::new(&rows).with(Style::rounded()).to_string()
        } else if has_group {
            // Use NoDueDataRow
            let rows: Vec<NoDueDataRow> = arr.iter().map(NoDueDataRow::from_json).collect();
            Table::new(&rows).with(Style::rounded()).to_string()
        } else {
            // Use MinimalDataRow
            let rows: Vec<MinimalDataRow> = arr.iter().map(MinimalDataRow::from_json).collect();
            Table::new(&rows).with(Style::rounded()).to_string()
        }
    } else {
        "No data to display.".to_string()
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

pub fn format_specs_res(res: &Value) -> String {
    if let Some(arr) = res.get("res").and_then(|v| v.as_array()) {
        let mut rows = vec![];

        for item in arr {
            rows.push(SupportRow {
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
            });
        }

        return Table::new(&rows).with(Style::rounded()).to_string();
    }

    // If no valid data is found (should never be the case)
    "No data to display.".to_string()
}
