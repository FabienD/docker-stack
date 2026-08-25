#[cfg(test)]
mod tests {
    use crate::command::infos::render_items_table;
    use crate::parser::config::{ComposeItem, ComposeStatus};

    fn sample_items() -> Vec<ComposeItem> {
        vec![
            ComposeItem {
                alias: "webapp".to_string(),
                description: Some("Front office".to_string()),
                status: Some(ComposeStatus::Running),
                use_project_name: Some(true),
                enviroment_file: None,
                compose_files: vec!["/tmp/webapp/docker-compose.yml".to_string()],
            },
            ComposeItem {
                alias: "api".to_string(),
                description: None,
                status: Some(ComposeStatus::Stopped),
                use_project_name: Some(true),
                enviroment_file: None,
                compose_files: vec!["/tmp/api/docker-compose.yml".to_string()],
            },
        ]
    }

    #[test]
    fn it_renders_only_the_three_visible_columns() {
        let table = render_items_table(sample_items());

        assert!(table.contains("🐋 Alias"), "missing alias header:\n{table}");
        assert!(
            table.contains("📃 Description"),
            "missing description header:\n{table}"
        );
        assert!(table.contains("⚡Status"), "missing status header:\n{table}");

        // Fields marked #[tabled(skip)] must never leak into the output.
        assert!(
            !table.contains("docker-compose.yml"),
            "skipped compose_files leaked:\n{table}"
        );
        assert!(
            !table.contains("use_project_name"),
            "skipped use_project_name leaked:\n{table}"
        );
    }

    #[test]
    fn it_renders_one_row_per_item_through_the_display_helpers() {
        let table = render_items_table(sample_items());

        assert!(table.contains("webapp"), "missing first alias:\n{table}");
        assert!(
            table.contains("Front office"),
            "missing description:\n{table}"
        );
        assert!(
            table.contains("🟢 Running"),
            "display_status did not run:\n{table}"
        );
        assert!(table.contains("api"), "missing second alias:\n{table}");
        assert!(
            table.contains("🔴 Stopped"),
            "display_status did not run:\n{table}"
        );
    }

    #[test]
    fn it_keeps_modern_borders_and_vertical_margin() {
        let table = render_items_table(sample_items());

        // Style::modern() draws light box-drawing borders with row separators.
        for expected in ['┌', '┬', '┐', '├', '┼', '┤', '└', '┴', '┘', '│'] {
            assert!(
                table.contains(expected),
                "missing border char {expected:?}:\n{table}"
            );
        }

        // Margin::new(0, 0, 1, 1) adds one blank line above and below.
        let lines: Vec<&str> = table.lines().collect();
        assert_eq!(
            lines.first().map(|l| l.trim()),
            Some(""),
            "missing top margin:\n{table}"
        );
        assert_eq!(
            lines.last().map(|l| l.trim()),
            Some(""),
            "missing bottom margin:\n{table}"
        );
    }

    #[test]
    fn it_renders_headers_when_there_is_no_project() {
        let table = render_items_table(Vec::new());

        assert!(
            table.contains("🐋 Alias"),
            "empty table lost its headers:\n{table}"
        );
    }
}
