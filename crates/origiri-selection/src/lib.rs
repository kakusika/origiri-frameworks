//! Multi-select over a collection of items (such as files, notes, or entries shown as a grid or list):
//! click, Ctrl/Shift-click, rubber-band drag (marquee), and bulk selection operations.
//!
//! Multi-select operates over *paths* (or stable string identifiers), not row indices: an index goes
//! stale the moment rows shift or directories re-list, whereas a path remains valid across view transitions.

use std::collections::HashSet;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Selection {
    selected: HashSet<String>,
    /// The last row picked by a plain click or Ctrl+click: what a following Shift+click ranges from.
    /// Repeated Shift+clicks keep extending from the same start.
    anchor: Option<String>,
    /// The selection as it stood when a rubber-band drag began, so shrinking the rectangle again within the
    /// same drag cannot lose what was selected before it.
    marquee_base: HashSet<String>,
}

impl Selection {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_selected(&self, path: &str) -> bool {
        self.selected.contains(path)
    }

    pub fn len(&self) -> usize {
        self.selected.len()
    }

    pub fn is_empty(&self) -> bool {
        self.selected.is_empty()
    }

    /// The selected paths, sorted (a stable order for callers and tests).
    pub fn paths(&self) -> Vec<String> {
        let mut v: Vec<String> = self.selected.iter().cloned().collect();
        v.sort();
        v
    }

    pub fn is_sole(&self, path: &str) -> bool {
        self.selected.len() == 1 && self.selected.contains(path)
    }

    pub fn select_only(&mut self, path: &str) {
        self.selected.clear();
        self.selected.insert(path.to_string());
        self.anchor = Some(path.to_string());
    }

    pub fn toggle(&mut self, path: &str) {
        if !self.selected.remove(path) {
            self.selected.insert(path.to_string());
        }
        self.anchor = Some(path.to_string());
    }

    /// `ordered` is the on-screen row order. With no anchor (or one that has vanished from `ordered`) this starts
    /// a fresh selection at `path`. `additive` unions the range into what was selected instead of replacing it.
    pub fn select_range(&mut self, ordered: &[String], path: &str, additive: bool) {
        let anchor = self
            .anchor
            .as_deref()
            .and_then(|a| ordered.iter().position(|p| p == a));
        let target = ordered.iter().position(|p| p == path);
        let (Some(anchor), Some(target)) = (anchor, target) else {
            if additive {
                self.selected.insert(path.to_string());
                self.anchor = Some(path.to_string());
            } else {
                self.select_only(path);
            }
            return;
        };
        let (start, end) = (anchor.min(target), anchor.max(target));
        let range: HashSet<String> = ordered[start..=end].iter().cloned().collect();
        if additive {
            self.selected.extend(range);
        } else {
            self.selected = range;
        }
    }

    /// Selects `path` without touching the rest (used by select-all).
    pub fn toggle_on(&mut self, path: &str) {
        self.selected.insert(path.to_string());
    }

    /// Selects multiple paths without clearing the rest.
    pub fn select_all(&mut self, paths: impl IntoIterator<Item = impl Into<String>>) {
        for p in paths {
            self.selected.insert(p.into());
        }
    }

    pub fn clear(&mut self) {
        self.selected.clear();
        self.anchor = None;
    }

    /// Call once when a rubber-band drag starts.
    pub fn begin_marquee(&mut self) {
        self.marquee_base = self.selected.clone();
    }

    /// Marquee without a modifier: the selection becomes exactly what the rectangle covers. The anchor is left
    /// alone (a marquee has no single "last clicked" row).
    pub fn replace_with(&mut self, paths: HashSet<String>) {
        self.selected = paths;
    }

    /// Marquee with a modifier: what the rectangle covers, added to the selection from before the drag.
    pub fn union_with_marquee_base(&mut self, paths: HashSet<String>) {
        self.selected = self.marquee_base.union(&paths).cloned().collect();
    }

    /// What a context-menu action on `clicked` applies to: the whole selection if `clicked` is part of it (a
    /// right-click inside a multi-selection acts on all of it, as in file managers), else just `clicked`.
    pub fn resolve_targets(&self, clicked: &str) -> Vec<String> {
        if self.selected.contains(clicked) {
            self.paths()
        } else if !clicked.is_empty() {
            vec![clicked.to_string()]
        } else {
            self.paths()
        }
    }

    /// Drops selected paths for which `keep` is false (entries that disappeared in a re-listing).
    pub fn retain(&mut self, keep: impl Fn(&str) -> bool) {
        self.selected.retain(|p| keep(p));
        if let Some(a) = &self.anchor
            && !keep(a)
        {
            self.anchor = None;
        }
    }
}

/// Indices of the grid cells that intersect the rectangle, computed from the covered rows and columns only (not
/// by walking every item).
#[allow(clippy::too_many_arguments)]
pub fn select_grid_in_rect(
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    columns: i32,
    cell_width: f64,
    cell_height: f64,
    total: usize,
) -> Vec<usize> {
    if columns <= 0 || cell_width <= 0.0 || cell_height <= 0.0 || total == 0 {
        return Vec::new();
    }
    let (left, right) = (x.min(x + width), x.max(x + width));
    let (top, bottom) = (y.min(y + height), y.max(y + height));
    let start_col = ((left / cell_width).floor() as i32).max(0);
    let end_col = ((right / cell_width).floor() as i32).min(columns - 1);
    let start_row = ((top / cell_height).floor() as i32).max(0);
    let end_row = (bottom / cell_height).floor() as i32;
    let mut out = Vec::new();
    for r in start_row..=end_row {
        for c in start_col..=end_col {
            let idx = (r * columns + c) as usize;
            if idx >= total {
                continue;
            }
            let (cx, cy) = (c as f64 * cell_width, r as f64 * cell_height);
            if cx < right && cx + cell_width > left && cy < bottom && cy + cell_height > top {
                out.push(idx);
            }
        }
    }
    out
}

/// Indices of the equal-height rows that intersect the vertical span `y..y+height` (the tree's marquee).
pub fn select_rows_in_span(y: f64, height: f64, row_height: f64, total: usize) -> Vec<usize> {
    if row_height <= 0.0 {
        return Vec::new();
    }
    let (top, bottom) = (y.min(y + height), y.max(y + height));
    (0..total)
        .filter(|&i| {
            let row_top = i as f64 * row_height;
            row_top < bottom && row_top + row_height > top
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn paths(names: &[&str]) -> Vec<String> {
        names.iter().map(|s| s.to_string()).collect()
    }

    fn set(names: &[&str]) -> HashSet<String> {
        names.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn click_selects_one_and_ctrl_click_toggles() {
        let mut s = Selection::default();
        s.select_only("a");
        s.toggle("b");
        assert_eq!(s.paths(), vec!["a", "b"]);
        s.toggle("a");
        assert_eq!(s.paths(), vec!["b"]);
        assert!(s.is_sole("b"));
        s.select_only("c");
        assert_eq!(s.paths(), vec!["c"]);
    }

    #[test]
    fn shift_click_ranges_from_a_fixed_anchor_in_both_directions() {
        let order = paths(&["a", "b", "c", "d", "e"]);
        let mut s = Selection::default();
        s.select_only("b");
        s.select_range(&order, "d", false);
        assert_eq!(s.paths(), vec!["b", "c", "d"]);
        // The anchor is still "b": shrinking, then crossing over.
        s.select_range(&order, "c", false);
        assert_eq!(s.paths(), vec!["b", "c"]);
        s.select_range(&order, "a", false);
        assert_eq!(s.paths(), vec!["a", "b"]);
    }

    #[test]
    fn an_additive_range_keeps_what_was_selected_and_a_missing_anchor_starts_fresh() {
        let order = paths(&["a", "b", "c", "d"]);
        let mut s = Selection::default();
        s.select_only("a");
        s.toggle("d"); // anchor d, selected a d
        s.select_range(&order, "b", true);
        assert_eq!(s.paths(), vec!["a", "b", "c", "d"]);
        let mut fresh = Selection::default();
        fresh.select_range(&order, "c", false);
        assert_eq!(fresh.paths(), vec!["c"]);
        let mut gone = Selection::default();
        gone.select_only("zz");
        gone.select_range(&order, "b", false);
        assert_eq!(gone.paths(), vec!["b"]);
    }

    #[test]
    fn a_marquee_replaces_or_extends_the_selection_from_before_the_drag() {
        let mut s = Selection::default();
        s.select_only("keep");
        s.begin_marquee();
        s.union_with_marquee_base(set(&["x", "y"]));
        assert_eq!(s.paths(), vec!["keep", "x", "y"]);
        // Shrinking the rectangle within the same drag cannot lose "keep".
        s.union_with_marquee_base(set(&["x"]));
        assert_eq!(s.paths(), vec!["keep", "x"]);
        s.replace_with(set(&["z"]));
        assert_eq!(s.paths(), vec!["z"]);
    }

    #[test]
    fn context_targets_are_the_whole_selection_only_when_the_clicked_entry_is_in_it() {
        let mut s = Selection::default();
        s.select_only("a");
        s.toggle("b");
        assert_eq!(s.resolve_targets("a"), vec!["a", "b"]);
        assert_eq!(s.resolve_targets("other"), vec!["other"]);
        assert_eq!(s.resolve_targets(""), vec!["a", "b"]);
    }

    #[test]
    fn retain_drops_vanished_paths_and_the_anchor() {
        let mut s = Selection::default();
        s.select_only("a");
        s.toggle("b");
        s.retain(|p| p != "b");
        assert_eq!(s.paths(), vec!["a"]);
        s.retain(|_| false);
        assert!(s.is_empty());
    }

    #[test]
    fn the_grid_rectangle_covers_only_the_intersecting_cells() {
        // 3 columns of 10x10 cells, 8 items: rows 0..2 (last row has two).
        let hit = |x, y, w, h| select_grid_in_rect(x, y, w, h, 3, 10.0, 10.0, 8);
        assert_eq!(hit(1.0, 1.0, 5.0, 5.0), vec![0]);
        assert_eq!(hit(5.0, 5.0, 10.0, 10.0), vec![0, 1, 3, 4]);
        // Dragged up and to the left: negative extents give the same rectangle.
        assert_eq!(hit(15.0, 15.0, -10.0, -10.0), vec![0, 1, 3, 4]);
        // Past the last item and past the columns.
        assert_eq!(hit(0.0, 20.0, 100.0, 5.0), vec![6, 7]);
        // Touching an edge does not count.
        assert!(hit(10.0, 0.0, 0.0, 5.0).is_empty());
        assert!(hit(-50.0, -50.0, 10.0, 10.0).is_empty());
        assert!(select_grid_in_rect(0.0, 0.0, 5.0, 5.0, 0, 10.0, 10.0, 8).is_empty());
    }

    #[test]
    fn the_row_span_covers_the_intersecting_rows() {
        assert_eq!(select_rows_in_span(5.0, 20.0, 10.0, 5), vec![0, 1, 2]);
        assert_eq!(select_rows_in_span(25.0, -20.0, 10.0, 5), vec![0, 1, 2]);
        assert!(select_rows_in_span(0.0, 10.0, 0.0, 5).is_empty());
    }
}
