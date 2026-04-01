use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, HtmlElement, HtmlTemplateElement, Node, Text, Window};

fn window() -> Window {
    web_sys::window().expect("no global window")
}

fn document() -> Document {
    window().document().expect("no document")
}

fn storage() -> web_sys::Storage {
    window()
        .local_storage()
        .expect("localStorage unavailable")
        .expect("localStorage null")
}

fn storage_key(page: &str, idx: usize, suffix: &str) -> String {
    format!("pv:{}:{}:{}", page, idx, suffix)
}

// ---------------------------------------------------------------------------
// init_viewer
// Called once on page load. Builds the sidebar and renders the first problem.
// ---------------------------------------------------------------------------
#[wasm_bindgen]
pub fn init_viewer(page_key: &str, num_problems: usize) {
    build_sidebar(page_key, num_problems);
    render_problem(0);
    update_stats(page_key, num_problems);
}

// ---------------------------------------------------------------------------
// navigate_to
// Show problem at `idx`, update active state in sidebar.
// ---------------------------------------------------------------------------
#[wasm_bindgen]
pub fn navigate_to(idx: usize) {
    // Clear previous active
    let doc = document();
    if let Ok(buttons) = doc.query_selector_all(".pv-prob-btn") {
        for i in 0..buttons.length() {
            if let Some(node) = buttons.item(i) {
                let el: Element = node.dyn_into().unwrap();
                el.class_list().remove_1("active").ok();
            }
        }
    }
    // Set new active
    if let Some(el) = doc.get_element_by_id(&format!("pv-btn-{}", idx)) {
        el.class_list().add_1("active").ok();
    }
    render_problem(idx);
}

// ---------------------------------------------------------------------------
// toggle_done
// Flip the done flag for problem `idx`, persist to localStorage, refresh UI.
// ---------------------------------------------------------------------------
#[wasm_bindgen]
pub fn toggle_done(page_key: &str, idx: usize, num_problems: usize) {
    let store = storage();
    let key = storage_key(page_key, idx, "done");
    let currently_done = store.get_item(&key).unwrap_or(None).as_deref() == Some("1");
    let new_val = if currently_done { "0" } else { "1" };
    store.set_item(&key, new_val).ok();
    refresh_btn_style(page_key, idx);
    update_stats(page_key, num_problems);
}

// ---------------------------------------------------------------------------
// save_note / load_note
// Persist per-problem scratch notes to localStorage.
// ---------------------------------------------------------------------------
#[wasm_bindgen]
pub fn save_note(page_key: &str, idx: usize, text: &str) {
    let key = storage_key(page_key, idx, "note");
    storage().set_item(&key, text).ok();
}

#[wasm_bindgen]
pub fn load_note(page_key: &str, idx: usize) -> String {
    let key = storage_key(page_key, idx, "note");
    storage().get_item(&key).unwrap_or(None).unwrap_or_default()
}

// ---------------------------------------------------------------------------
// clear_progress
// Wipe all localStorage entries for this page.
// ---------------------------------------------------------------------------
#[wasm_bindgen]
pub fn clear_progress(page_key: &str, num_problems: usize) {
    let store = storage();
    for i in 0..num_problems {
        store.remove_item(&storage_key(page_key, i, "done")).ok();
        store.remove_item(&storage_key(page_key, i, "note")).ok();
    }
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

fn build_sidebar(page_key: &str, num_problems: usize) {
    let doc = document();
    let sidebar = match doc.get_element_by_id("pv-sidebar") {
        Some(el) => el,
        None => return,
    };
    sidebar.set_inner_html("");

    // Header
    let header = create_el("div");
    header.set_attribute("class", "pv-sidebar-header").ok();
    header.set_inner_html("<h4 style='margin:0 0 12px;'>Problems</h4>");
    sidebar.append_child(&header).ok();

    // Stats row
    let stats = create_el("div");
    stats.set_attribute("id", "pv-stats").ok();
    stats.set_attribute("class", "pv-stats-row").ok();
    sidebar.append_child(&stats).ok();

    // Progress bar wrapper
    let pb_wrap = create_el("div");
    pb_wrap.set_attribute("class", "progress pv-progress").ok();
    let pb = create_el("div");
    pb.set_attribute("id", "pv-progress-bar").ok();
    pb.set_attribute("class", "progress-bar progress-bar-info").ok();
    pb.set_attribute("role", "progressbar").ok();
    pb.set_attribute("style", "width:0%").ok();
    pb_wrap.append_child(&pb).ok();
    sidebar.append_child(&pb_wrap).ok();

    // Separator
    let hr = create_el("hr");
    hr.set_attribute("style", "margin:8px 0 12px;").ok();
    sidebar.append_child(&hr).ok();

    // Problem buttons
    for i in 0..num_problems {
        let tmpl_id = format!("prob-{}", i);
        let (pid, title) = get_problem_meta(&doc, &tmpl_id);

        let btn = create_el("button");
        btn.set_attribute("id", &format!("pv-btn-{}", i)).ok();
        btn.set_attribute("class", "btn btn-default btn-block pv-prob-btn text-left").ok();
        btn.set_attribute("onclick", &format!("navigate_to({})", i)).ok();

        // Done badge
        let badge = create_el("span");
        badge.set_attribute("id", &format!("pv-badge-{}", i)).ok();
        badge.set_attribute("class", "badge pull-right").ok();
        badge.set_attribute("style", "display:none").ok();
        badge.set_inner_html("&#10003;");
        btn.append_child(&badge).ok();

        // Label
        let label_text = if !pid.is_empty() {
            format!("{} {}", pid, title)
        } else {
            format!("Problem {}", i + 1)
        };
        let txt: Text = doc.create_text_node(&label_text);
        btn.append_child(&txt).ok();

        sidebar.append_child(&btn).ok();

        // Restore saved done state
        refresh_btn_style(page_key, i);
    }

    // Clear progress button
    let clr_wrap = create_el("div");
    clr_wrap.set_attribute("style", "margin-top:16px;").ok();
    let clr_btn = create_el("button");
    clr_btn.set_attribute("class", "btn btn-xs btn-danger btn-block").ok();
    clr_btn.set_attribute(
        "onclick",
        &format!(
            "if(confirm('Reset all progress?')){{ clear_progress('{}', {}); location.reload(); }}",
            page_key, num_problems
        ),
    )
    .ok();
    clr_btn.set_inner_html("Reset Progress");
    clr_wrap.append_child(&clr_btn).ok();
    sidebar.append_child(&clr_wrap).ok();

    // Activate first button
    if let Some(el) = doc.get_element_by_id("pv-btn-0") {
        el.class_list().add_1("active").ok(); // Element has class_list
    }
}

fn render_problem(idx: usize) {
    let doc = document();
    let content = match doc.get_element_by_id("pv-content") {
        Some(el) => el,
        None => return,
    };
    content.set_inner_html("");

    let tmpl_id = format!("prob-{}", idx);
    let tmpl = match doc.get_element_by_id(&tmpl_id) {
        Some(el) => el,
        None => {
            content.set_inner_html("<p class='text-muted'>Problem not found.</p>");
            return;
        }
    };

    // Clone template content into the display area
    let tmpl_el: HtmlTemplateElement = match tmpl.dyn_into() {
        Ok(t) => t,
        Err(el) => {
            // Fallback: element is not a <template>, clone it directly
            let clone = el.clone_node_with_deep(true).unwrap();
            content.append_child(&clone).ok();
            return;
        }
    };

    let frag: Node = tmpl_el.content().into();
    let clone = frag.clone_node_with_deep(true).unwrap();
    content.append_child(&clone).ok();

    // Scroll to top
    let content_el: HtmlElement = content.unchecked_into();
    content_el.scroll_into_view_with_bool(true);
}

fn refresh_btn_style(page_key: &str, idx: usize) {
    let doc = document();
    let done = is_done(page_key, idx);

    let btn_id = format!("pv-btn-{}", idx);
    let badge_id = format!("pv-badge-{}", idx);

    if let Some(el) = doc.get_element_by_id(&btn_id) {
        if done {
            el.class_list().remove_1("btn-default").ok();
            el.class_list().add_1("btn-success").ok();
        } else {
            el.class_list().remove_1("btn-success").ok();
            el.class_list().add_1("btn-default").ok();
        }
    }

    if let Some(badge) = doc.get_element_by_id(&badge_id) {
        let el: HtmlElement = badge.unchecked_into();
        if done {
            el.style().set_property("display", "inline").ok();
        } else {
            el.style().set_property("display", "none").ok();
        }
    }
}

fn update_stats(page_key: &str, num_problems: usize) {
    let done_count = (0..num_problems)
        .filter(|&i| is_done(page_key, i))
        .count();

    let doc = document();

    if let Some(el) = doc.get_element_by_id("pv-stats") {
        el.set_inner_html(&format!(
            "<span class='text-muted' style='font-size:12px;'>{} / {} solved</span>",
            done_count, num_problems
        ));
    }

    if let Some(el) = doc.get_element_by_id("pv-progress-bar") {
        let pct = if num_problems > 0 {
            (done_count * 100) / num_problems
        } else {
            0
        };
        let el: HtmlElement = el.dyn_into().unwrap();
        el.style()
            .set_property("width", &format!("{}%", pct))
            .ok();
        el.set_attribute("aria-valuenow", &pct.to_string()).ok();
    }
}

fn is_done(page_key: &str, idx: usize) -> bool {
    let key = storage_key(page_key, idx, "done");
    storage()
        .get_item(&key)
        .unwrap_or(None)
        .as_deref()
        == Some("1")
}

fn create_el(tag: &str) -> Element {
    document().create_element(tag).unwrap()
}

/// Extract pid and title from a <template id="prob-N"> dataset or its first heading.
fn get_problem_meta(doc: &Document, tmpl_id: &str) -> (String, String) {
    if let Some(el) = doc.get_element_by_id(tmpl_id) {
        // Prefer data attributes on the template element itself
        let pid = el
            .get_attribute("data-pid")
            .unwrap_or_default();
        let title = el
            .get_attribute("data-title")
            .unwrap_or_default();
        if !title.is_empty() {
            return (pid, title);
        }
    }
    (String::new(), String::new())
}
