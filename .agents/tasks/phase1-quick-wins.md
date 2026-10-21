# Phase 1: High Impact, Quick-Win Framework Improvements & App Adoption

## Goals
Implement Phase 1 improvements identified in the cross-app component research:
1. Introduce `Chip` in `origiri-frameworks`.
2. Fix `ProgressBar.indeterminate` animation in `origiri-frameworks`.
3. Fix hardcoded colors in `origiri::StatusBar` to use semantic tokens.
4. Adopt `Chip` in `mumeum` across 5 feature files.
5. Adopt `Breadcrumb` in `mumeum` explorer.
6. Adopt `RubberBandArea` in `orie`.

## Progress
- [x] Step 1: Implement `Chip` widget in `origiri` (`crates/origiri/ui/widgets/chip.slint`) and export in `widgets.slint`.
- [x] Step 2: Fix `ProgressBar.indeterminate` in `origiri` (`crates/origiri/ui/widgets/progress_bar.slint`) with `animation-tick()`.
- [x] Step 3: Replace hardcoded colors in `origiri::StatusBar` (`crates/origiri/ui/widgets/status_bar.slint`) with semantic tokens (`Tokens.warning`, `Tokens.success`).
- [x] Step 4: Verify with `cargo test` and showcase in `origiri-gallery`. Request user review and commit confirmation for `origiri-frameworks`.
- [ ] Step 5: Replace duplicated `Chip` definitions in `mumeum` (`painter_view`, `map_view`, `link_graph_view`, `collections_view`, `preview_view`).
- [ ] Step 6: Replace hand-rolled breadcrumb in `mumeum/features/explorer/ui/explorer_view.slint` with `origiri::Breadcrumb`.
- [ ] Step 7: Replace hand-rolled rubber-band box in `orie/apps/editor/ui/main.slint` with `origiri::RubberBandArea`.
