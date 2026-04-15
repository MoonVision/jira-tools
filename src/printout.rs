use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, CellAlignment, Table};
use owo_colors::OwoColorize;
use std::collections::HashMap;

pub fn print_issue_line(
    key: &str,
    issue_type_name: Option<&str>,
    status_name: Option<&str>,
    story_points: Option<f64>,
) {
    print!("{:>8} - ", key.blue().bold());

    if let Some(type_name) = issue_type_name {
        print!("{:<5} - ", type_name.yellow());
    }

    if let Some(status) = status_name {
        print!("{:<11} - ", status.green());
    }

    if let Some(sp) = story_points {
        print!("{:>3} SP", sp.cyan());
    } else {
        print!("  ? SP");
    }
    println!();
}

pub fn print_stats_for_humans(
    issue_count_by_status: &HashMap<String, u32>,
    issue_count_by_issue_type: &HashMap<String, u32>,
    story_points_by_status: &HashMap<String, f64>,
    story_points_by_issue_type: &HashMap<String, f64>,
    story_points_unset_by_issue_type: &HashMap<String, u32>,
    total_story_points: f64,
    total_issue_count: u32,
) {
    println!("\n{}", "Issues by type:".bold());
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_header(vec!["Issue Type", "Count", "Story Points"]);
    for (issue_type, issue_count) in issue_count_by_issue_type.iter() {
        let sp = story_points_by_issue_type.get(issue_type).unwrap_or(&0.0);
        table.add_row(vec![
            Cell::new(issue_type),
            Cell::new(issue_count).set_alignment(CellAlignment::Right),
            Cell::new(sp).set_alignment(CellAlignment::Right),
        ]);
    }
    println!("{table}");

    println!("\n{}", "Unset SPs by issue type:".bold());
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_header(vec!["Issue Type", "Unset Count"]);
    for (issue_type, unset_count) in story_points_unset_by_issue_type.iter() {
        table.add_row(vec![
            Cell::new(issue_type),
            Cell::new(unset_count).set_alignment(CellAlignment::Right),
        ]);
    }
    if story_points_unset_by_issue_type.is_empty() {
        println!("No issues with unset story points.");
    } else {
        println!("{table}");
    }

    println!("\n{}", "Issues by status:".bold());
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_header(vec!["Status", "Count", "Story Points"]);
    for (status, issue_count) in issue_count_by_status.iter() {
        let sp = story_points_by_status.get(status).unwrap_or(&0.0);
        table.add_row(vec![
            Cell::new(status),
            Cell::new(issue_count).set_alignment(CellAlignment::Right),
            Cell::new(sp).set_alignment(CellAlignment::Right),
        ]);
    }
    println!("{table}");

    println!("\n{}", "Totals:".bold());
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_header(vec!["Metric", "Total"]);
    table.add_row(vec![
        Cell::new("Issue Count"),
        Cell::new(total_issue_count).set_alignment(CellAlignment::Right),
    ]);
    table.add_row(vec![
        Cell::new("Story Points"),
        Cell::new(total_story_points).set_alignment(CellAlignment::Right),
    ]);
    println!("{table}");
}
