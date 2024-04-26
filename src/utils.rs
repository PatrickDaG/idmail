use chrono::{DateTime, Local, Utc};
use leptos::{html::Dialog, *};
use leptos_struct_table::*;

#[component]
#[allow(unused_variables, non_snake_case)]
pub fn TimediffRenderer<F>(
    class: String,
    #[prop(into)] value: MaybeSignal<DateTime<Utc>>,
    on_change: F,
    index: usize,
) -> impl IntoView
where
    F: Fn(DateTime<Utc>) + 'static,
{
    let time_tooltip = move || {
        let utc_time = value();
        let dt = utc_time - Utc::now();
        let human_time = chrono_humanize::HumanTime::from(dt);

        let local_time: DateTime<Local> = DateTime::from(utc_time);
        let approximate_time = human_time.to_string();
        let precise_time = local_time.format("%c").to_string();

        view! {
            <div class="group relative w-max">
                <span class="pointer-events-none absolute -top-10 -left-10 w-max rounded-lg bg-gray-900 px-2 py-1 font-medium text-gray-50 opacity-0 shadow transition-opacity group-hover:opacity-100 z-50">
                    {precise_time}
                </span>
                {approximate_time}
            </div>
        }
    };

    view! { <td class=class>{time_tooltip}</td> }
}

#[component]
#[allow(unused_variables, non_snake_case)]
pub fn SliderRenderer<F>(
    class: String,
    #[prop(into)] value: MaybeSignal<bool>,
    on_change: F,
    index: usize,
) -> impl IntoView
where
    F: Fn(bool) + 'static,
{
    view! {
        <td class=class>
            <label class="cursor-pointer">
                <input
                    type="checkbox"
                    class="sr-only peer"
                    checked=value
                    on:change=move |ev| {
                        on_change(event_target_checked(&ev));
                    }
                />

                <div class="relative w-[3.25rem] h-6 bg-gray-300 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-7 rtl:peer-checked:after:-translate-x-7 after:content-[''] after:absolute after:top-[3px] after:start-[3px] after:bg-white after:rounded-full after:h-[18px] after:w-[18px] after:transition-all peer-checked:bg-blue-600"></div>
            </label>
        </td>
    }
}

#[component]
pub fn THeadCellRenderer<F>(
    /// The class attribute for the head element. Generated by the classes provider.
    #[prop(into)]
    class: Signal<String>,
    /// The class attribute for the inner element. Generated by the classes provider.
    #[prop(into)]
    inner_class: String,
    /// The index of the column. Starts at 0 for the first column. The order of the columns is the same as the order of the fields in the struct.
    index: usize,
    /// The sort priority of the column. `None` if the column is not sorted. `0` means the column is the primary sort column.
    #[prop(into)]
    sort_priority: Signal<Option<usize>>,
    /// The sort direction of the column. See [`ColumnSort`].
    #[prop(into)]
    sort_direction: Signal<ColumnSort>,
    /// The event handler for the click event. Has to be called with [`TableHeadEvent`].
    on_click: F,
    children: Children,
) -> impl IntoView
where
    F: Fn(TableHeadEvent) + 'static,
{
    view! {
        <th
            class=class
            on:click=move |mouse_event| on_click(TableHeadEvent {
                index,
                mouse_event,
            })
        >

            <button
                type="button"
                class="inline-flex items-center justify-center whitespace-nowrap px-2 -ml-2 h-12 text-gray-900 bg-white focus:outline-none hover:bg-gray-100 focus-visible:ring-4 focus-visible:ring-ring rounded-lg"
            >
                <span class=inner_class>{children()}</span>
                <span class="ml-2 w-3">
                    {move || {
                        match (sort_priority(), sort_direction()) {
                            (_, ColumnSort::Ascending) => view! { "↑" },
                            (_, ColumnSort::Descending) => view! { "↓" },
                            _ => view! { "" },
                        }
                    }}

                </span>
            </button>
        </th>
    }
}

#[derive(Clone, Copy)]
pub struct TailwindClassesPreset;

impl TableClassesProvider for TailwindClassesPreset {
    fn new() -> Self {
        Self
    }

    fn thead_row(&self, template_classes: &str) -> String {
        template_classes.to_string()
    }

    fn thead_cell(&self, _sort: ColumnSort, template_classes: &str) -> String {
        format!(
            "h-14 px-4 text-left text-base align-middle font-medium {}",
            template_classes
        )
    }

    fn thead_cell_inner(&self) -> String {
        "flex items-center".to_string()
    }

    fn row(&self, row_index: usize, _selected: bool, template_classes: &str) -> String {
        let bg_color = if row_index % 2 == 0 {
            "bg-white hover:bg-gray-100"
        } else {
            "bg-gray-50 hover:bg-gray-100"
        };

        format!("border-t-[1.5px] last:border-0 {} {}", bg_color, template_classes)
    }

    fn loading_cell(&self, _row_index: usize, _col_index: usize, prop_class: &str) -> String {
        format!("px-4 py-2 {}", prop_class)
    }

    fn loading_cell_inner(&self, _row_index: usize, _col_index: usize, prop_class: &str) -> String {
        format!(
            "animate-pulse h-2 bg-gray-400 rounded-full inline-block align-middle w-[calc(60%-2.5rem)] {}",
            prop_class
        )
    }

    fn cell(&self, template_classes: &str) -> String {
        format!("px-4 py-2 whitespace-nowrap text-ellipsis {}", template_classes)
    }
}

#[component]
pub fn Modal(#[prop(into)] open: Signal<bool>, children: Children, dialog_el: NodeRef<Dialog>) -> impl IntoView {
    create_effect(move |_| {
        if let Some(dialog) = dialog_el.get_untracked() {
            if open() {
                if dialog.show_modal().is_err() {
                    dialog.set_open(true);
                }
            } else {
                dialog.close();
            }
        }
    });

    view! {
        <dialog ref=dialog_el open=open.get_untracked() class="rounded-lg backdrop:bg-gray-500 backdrop:bg-opacity-75">
            <main>{children()}</main>
        </dialog>
    }
}

#[component]
pub fn Select(
    #[prop(into, optional)] class: Option<AttributeValue>,
    #[prop(into, optional)] option_class: MaybeSignal<String>,
    choices: ReadSignal<Vec<String>>,
    value: ReadSignal<String>,
    set_value: WriteSignal<String>,
) -> impl IntoView {
    let option_class = Signal::derive(move || option_class.get());
    view! {
        <select
            class=class
            on:change=move |ev| {
                let new_value = event_target_value(&ev);
                set_value(new_value);
            }
        >
            <For
                each=choices
                key=|x| x.clone()
                let:child
            >
                <SelectOption value class=option_class id=child.clone() />
            </For>
        </select>
    }
}

#[component]
pub fn SelectOption(
    #[prop(into, optional)] class: MaybeSignal<String>,
    id: String,
    value: ReadSignal<String>,
) -> impl IntoView {
    let id_copy = id.clone();
    view! {
        <option
            class=class
            value=id.clone()
            selected=move || value() == id_copy
        >
            {id}
        </option>
    }
}
