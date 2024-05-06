use chrono::{DateTime, Local, Utc};
use leptos::{
    html::{Dialog, Select},
    *,
};
use leptos_icons::Icon;
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
                <span class="pointer-events-none absolute -top-10 -left-10 w-max rounded-lg bg-gray-900 dark:bg-black dark:border-[1.5px] dark:border-zinc-800 px-2 py-1 font-medium text-gray-50 opacity-0 shadow transition-opacity group-hover:opacity-100 z-50">
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

                <div class="relative w-[3.25rem] h-6 bg-gray-300 dark:bg-zinc-700 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:focus:ring-blue-300 rounded-full peer peer-checked:after:translate-x-7 rtl:peer-checked:after:-translate-x-7 after:content-[''] after:absolute after:top-[3px] after:start-[3px] after:bg-white dark:bg-black after:rounded-full after:h-[18px] after:w-[18px] after:transition-all peer-checked:bg-blue-600 dark:peer-checked:bg-blue-600 dark:checked:bg-blue-600 dark:bg-blue-600"></div>
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
                class="inline-flex items-center justify-center whitespace-nowrap px-2 -ml-2 h-12 text-gray-900 dark:text-gray-200 bg-white dark:bg-black focus:outline-none hover:bg-gray-100 dark:hover:bg-zinc-900 focus-visible:ring-4 focus-visible:ring-ring rounded-lg"
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
            "bg-white dark:bg-black hover:bg-gray-100 dark:hover:bg-zinc-900"
        } else {
            "bg-gray-50 dark:bg-zinc-950 dark:bg-black hover:bg-gray-100 dark:hover:bg-zinc-900"
        };

        format!("border-t-[1.5px] border-gray-200 dark:border-zinc-800 last:border-0 {} {}", bg_color, template_classes)
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
        <dialog ref=dialog_el open=open.get_untracked() class="rounded-lg backdrop:bg-gray-50 dark:bg-gray-900 dark:bg-black0 dark:backdrop:bg-black backdrop:bg-opacity-75 dark:backdrop:bg-opacity-75 dark:border-[1.5px] dark:border-zinc-800">
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
    let select_el = create_node_ref::<Select>();
    create_effect(move |_| {
        if let Some(select) = select_el.get_untracked() {
            select.set_value(&value());
        }
    });

    let option_class = Signal::derive(move || option_class.get());
    view! {
        <select
            ref=select_el
            class=class
            on:change=move |ev| {
                let new_value = event_target_value(&ev);
                set_value(new_value);
            }
        >

            <For each=choices key=|x| x.clone() let:child>
                <SelectOption value class=option_class id=child.clone()/>
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
        <option class=class value=id.clone() selected=move || value() == id_copy>
            {id}
        </option>
    }
}

#[component]
pub fn EditModal<T: Clone + 'static, F: Fn(&T) -> &str + 'static>(
    #[prop(into)] data: RwSignal<Option<Option<T>>>,
    #[prop(into)] errors: Signal<Vec<String>>,
    what: String,
    get_title: F,
    #[prop(into)] on_confirm: Callback<(Option<T>, Callback<String>)>,
    children: Children,
) -> impl IntoView {
    let (server_error, set_server_error) = create_signal(None);
    let (modal_waiting, set_modal_waiting) = create_signal(false);
    let modal_elem = create_node_ref::<Dialog>();
    let open = Signal::derive(move || data.get().is_some());

    let on_error = Callback::new(move |error: String| {
        set_modal_waiting(false);
        set_server_error(Some(error));
    });

    create_effect(move |_| {
        if !open() {
            set_modal_waiting(false);
            set_server_error(None);
        }
    });

    view! {
        <Modal open dialog_el=modal_elem>
            <div class="relative p-4 transform overflow-hidden rounded-lg bg-white text-black dark:bg-black dark:text-zinc-100 text-left transition-all w-full sm:min-w-[512px]">
                <h3 class="text-2xl tracking-tight mt-2 mb-4 font-semibold text-gray-900 dark:text-gray-200">
                    {move || {
                        if let Some(Some(data)) = data.get() {
                            format!("Edit {}", get_title(&data))
                        } else {
                            format!("New {}", what)
                        }
                    }}

                </h3>
                <div class="flex flex-col gap-3">
                    {children()} <Show when=move || (!errors.get().is_empty() || server_error().is_some())>
                        <div class="rounded-lg p-4 flex bg-red-100 dark:bg-red-900 mt-2">
                            <div>
                                <Icon icon=icondata::BiXCircleSolid class="w-5 h-5 text-red-400 dark:text-red-200"/>
                            </div>
                            <div class="ml-3 text-red-700 dark:text-red-200">
                                <For each=errors key=|x| x.clone() let:child>
                                    <p>{child.clone()}</p>
                                </For>
                                {move || {
                                    match server_error() {
                                        None => view! {}.into_view(),
                                        Some(error) => view! { <p>{error}</p> }.into_view(),
                                    }
                                }}

                            </div>
                        </div>
                    </Show> <div class="flex flex-col-reverse gap-3 sm:flex-row-reverse">
                        <button
                            type="button"
                            class="inline-flex w-full min-w-20 justify-center rounded-lg transition-all bg-white dark:bg-black px-3 py-2 font-semibold text-gray-900 dark:text-gray-200 focus:ring-4 dark:focus:ring-zinc-800 border-[1.5px] border-gray-300 dark:border-zinc-800 hover:bg-gray-100 dark:hover:bg-zinc-900 sm:w-auto"
                            on:click=move |_ev| {
                                data.set(None);
                            }
                        >

                            Cancel
                        </button>
                        <button
                            type="button"
                            disabled=move || (modal_waiting() || !errors.get().is_empty())
                            class="inline-flex w-full min-w-20 justify-center items-center rounded-lg transition-all px-3 py-2 bg-blue-600 dark:bg-blue-600 hover:bg-blue-500 dark:hover:bg-blue-500 font-semibold text-white dark:text-zinc-100 focus:ring-4 focus:ring-blue-300 dark:focus:ring-blue-300 sm:w-auto disabled:cursor-not-allowed disabled:opacity-50"
                            class=("!bg-blue-500", move || (modal_waiting() || !errors.get().is_empty()))
                            on:click=move |_ev| {
                                if let Some(data) = data.get() {
                                    if !modal_waiting() && errors.get().is_empty() {
                                        on_confirm((data, on_error));
                                        set_modal_waiting(true);
                                    }
                                }
                            }
                        >

                            <Show when=modal_waiting>
                                <Icon icon=icondata::CgSpinner class="inline w-5 h-5 me-2 text-blue-900 animate-spin"/>
                            </Show>
                            Save
                        </button>
                    </div>
                </div>
            </div>
        </Modal>
    }
}

#[component]
pub fn DeleteModal(
    #[prop(into)] data: RwSignal<Option<String>>,
    #[prop(into)] text: View,
    #[prop(into)] on_confirm: Callback<String>,
) -> impl IntoView {
    let (modal_waiting, set_modal_waiting) = create_signal(false);
    let modal_elem = create_node_ref::<Dialog>();
    let open = Signal::derive(move || data.get().is_some());

    create_effect(move |_| {
        if !open() {
            set_modal_waiting(false);
        }
    });

    view! {
        <Modal open dialog_el=modal_elem>
            <div class="relative p-4 transform overflow-hidden rounded-lg bg-white dark:bg-black text-left transition-all sm:w-full sm:max-w-lg">
                <div class="bg-white dark:bg-black py-3">
                    <div class="sm:flex sm:items-start">
                        <div class="mx-auto flex h-12 w-12 flex-shrink-0 items-center justify-center rounded-full bg-red-100 dark:bg-red-800 sm:mx-0 sm:h-10 sm:w-10">
                            <Icon icon=icondata::AiWarningFilled class="w-6 h-6 text-red-600 dark:text-red-400"/>
                        </div>
                        <div class="mt-3 text-center sm:ml-4 sm:mt-0 sm:text-left">
                            <h3 class="text-2xl tracking-tight font-semibold text-gray-900 dark:text-gray-200">"Delete " {data}</h3>
                            <div class="mt-2">
                                <p class="text-sm text-gray-500 dark:text-gray-400">{text}</p>
                            </div>
                        </div>
                    </div>
                </div>
                <div class="flex flex-col-reverse gap-3 sm:flex-row-reverse">
                    <button
                        type="button"
                        class="inline-flex w-full min-w-20 justify-center rounded-lg transition-all bg-white dark:bg-black px-3 py-2 font-semibold text-gray-900 dark:text-gray-200 focus:ring-4 dark:focus:ring-zinc-800 border-[1.5px] border-gray-300 dark:border-zinc-800 hover:bg-gray-100 dark:hover:bg-zinc-900 sm:w-auto"
                        on:click=move |_ev| {
                            data.set(None);
                        }
                    >

                        Cancel
                    </button>
                    <button
                        type="button"
                        disabled=modal_waiting
                        class="inline-flex w-full min-w-20 justify-center items-center rounded-lg transition-all px-3 py-2 bg-red-600 dark:bg-red-600 hover:bg-red-500 dark:hover:bg-red-500 font-semibold text-white dark:text-zinc-100 focus:ring-4 focus:ring-red-300 sm:w-auto"
                        class=("!bg-red-500", modal_waiting)
                        on:click=move |_ev| {
                            if let Some(data) = data.get() {
                                if !modal_waiting() {
                                    on_confirm(data);
                                    set_modal_waiting(true);
                                }
                            }
                        }
                    >

                        <Show when=modal_waiting>
                            <Icon icon=icondata::CgSpinner class="inline w-5 h-5 me-2 text-red-900 animate-spin"/>
                        </Show>
                        Delete
                    </button>
                </div>
            </div>
        </Modal>
    }
}
