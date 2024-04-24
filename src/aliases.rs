use crate::utils::{THeadCellRenderer, TimediffRenderer, TailwindClassesPreset};

use chrono::{DateTime, Utc};
use leptos::ev::MouseEvent;
use leptos::*;
use leptos_struct_table::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::{QueryBuilder, Row};
use std::collections::VecDeque;
use std::ops::Range;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, TableRow)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[table(sortable, classes_provider = TailwindClassesPreset, thead_cell_renderer = THeadCellRenderer)]
pub struct Alias {
    #[table(class = "w-40")]
    pub address: String,
    #[table(class = "w-40")]
    pub target: String,
    pub comment: String,
    #[table(class = "w-1", title = "Received")]
    pub n_recv: i64,
    #[table(class = "w-1", title = "Sent")]
    pub n_sent: i64,
    #[table(class = "w-1", renderer = "TimediffRenderer")]
    pub created_at: DateTime<Utc>,
    #[table(class = "w-1")]
    pub active: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AliasQuery {
    #[serde(default)]
    sort: VecDeque<(usize, ColumnSort)>,
    range: Range<usize>,
    search: String,
}

#[server]
pub async fn list_aliases(query: AliasQuery) -> Result<Vec<Alias>, ServerFnError> {
    let AliasQuery { sort, range, search } = query;

    let mut query = QueryBuilder::new("SELECT * FROM aliases");
    if !search.is_empty() {
        query.push(" WHERE address LIKE concat('%', ");
        query.push_bind(&search);
        query.push(", '%') OR comment LIKE concat('%', ");
        query.push_bind(&search);
        query.push(", '%')");
    }

    if let Some(order) = Alias::sorting_to_sql(&sort) {
        query.push(" ");
        query.push(order);
    }

    query.push(" LIMIT ");
    query.push_bind(range.len() as i64);
    query.push(" OFFSET ");
    query.push_bind(range.start as i64);

    let pool = crate::database::ssr::pool()?;
    Ok(query.build_query_as::<Alias>().fetch_all(&pool).await?)
}

#[server]
pub async fn alias_count() -> Result<usize, ServerFnError> {
    let pool = crate::database::ssr::pool()?;
    let count: i64 = sqlx::query("SELECT COUNT(*) FROM aliases")
        .fetch_one(&pool)
        .await?
        .get(0);

    Ok(count as usize)
}

#[derive(Default)]
pub struct AliasTableDataProvider {
    sort: VecDeque<(usize, ColumnSort)>,
    pub search: RwSignal<String>,
}

impl TableDataProvider<Alias> for AliasTableDataProvider {
    async fn get_rows(&self, range: Range<usize>) -> Result<(Vec<Alias>, Range<usize>), String> {
        list_aliases(AliasQuery {
            search: self.search.get_untracked().trim().to_string(),
            sort: self.sort.clone(),
            range: range.clone(),
        })
        .await
        .map(|rows| {
            let len = rows.len();
            (rows, range.start..range.start + len)
        })
        .map_err(|e| format!("{e:?}"))
    }

    async fn row_count(&self) -> Option<usize> {
        alias_count().await.ok()
    }

    fn set_sorting(&mut self, sorting: &VecDeque<(usize, ColumnSort)>) {
        self.sort = sorting.clone();
    }

    fn track(&self) {
        self.search.track();
    }
}

/// Custom row renderer that adds a link to the end of the row
#[allow(unused_variables, non_snake_case)]
pub fn AliasRowRenderer(
    // The class attribute for the row element. Generated by the classes provider.
    class: Signal<String>,
    // The row to render.
    row: Alias,
    // The index of the row. Starts at 0 for the first body row.
    index: usize,
    // The selected state of the row. True, when the row is selected.
    selected: Signal<bool>,
    // Event handler callback when this row is selected
    on_select: EventHandler<MouseEvent>,
    // Event handler callback for changes
    on_change: EventHandler<ChangeEvent<Alias>>,
) -> impl IntoView {
    view! {
        <tr class=class on:click=move |mouse_event| on_select.run(mouse_event)>
            {row.render_row(index, on_change)}
            <td class="w-1 p-2 whitespace-nowrap text-ellipsis">
                E
                X
            </td>
        </tr>
    }
}
