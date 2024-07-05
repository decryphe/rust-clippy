use clippy_config::types::{
    SourceItemOrderingEnableFor, SourceItemOrderingModuleItemKind, SourceItemOrderingTraitAssocItemKind,
};
use clippy_utils::diagnostics::span_lint_and_note;
use rustc_hir::*;
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::impl_lint_pass;

declare_clippy_lint! {
    /// ### What it does
    /// Confirms that items are sorted in source files as per configuration.
    ///
    /// ### Why restrict this?
    /// Keeping a consistent ordering throughout the codebase helps with working
    /// as a team, and possibly improves maintainability of the codebase. The
    /// idea is that by defining a consistent and enforceable rule for how
    /// source files are structured, less time will be wasted during reviews on
    /// a topic that is (under most circumstances) not relevant to the logic
    /// implemented in the code. Sometimes this will be referred to as
    /// "bike-shedding".
    ///
    /// Keep in mind, that ordering source code alphabetically can lead to
    /// reduced performance in cases where the most commonly used enum variant
    /// isn't the first entry anymore, and similar optimizations that can reduce
    /// branch misses, cache locality and such. Either don't use this lint if
    /// that's relevant, or disable the lint in modules or items specifically
    /// where it matters.
    ///
    /// ### Default Ordering and Configuration
    /// As there is no generally applicable rule, the lint can be configured
    /// with high granularity. Item kinds can be grouped together arbitrarily,
    /// items within groups will be ordered alphabetically. The following table
    /// shows the default groupings.
    ///
    /// | Group              | Item Kinds           |
    /// |--------------------|----------------------|
    /// | `modules`          | "mod", "foreign_mod" |
    /// | `use`              | "use"                |
    /// | `macro`            | "macro"              |
    /// | `global_asm`       | "global_asm"         |
    /// | `upper_snake_case` | "static", "const"    |
    /// | `pascal_case`      | "ty_alias", "opaque_ty", "enum", "struct", "union", "trait", "trait_alias", "impl" |
    /// | `lower_snake_case` | "fn"                 |
    /// | `mod_tests`        | "mod_tests"          |
    ///
    /// Item kinds are derived from the item kinds known to the `rustc`
    /// compiler:
    ///
    /// | Item Kind     | Description |
    /// |---------------|-------------|
    /// | "mod"         | TODO: Fill out. |
    /// | "foreign_mod" | TODO: Fill out. |
    /// | "use"         | TODO: Fill out. |
    /// | "macro"       | TODO: Fill out. |
    /// | "global_asm"  | TODO: Fill out. |
    /// | "static"      | TODO: Fill out. |
    /// | "const"       | TODO: Fill out. |
    /// | "ty_alias"    | TODO: Fill out. |
    /// | "opaque_ty"   | TODO: Fill out. |
    /// | "enum"        | TODO: Fill out. |
    /// | "struct"      | TODO: Fill out. |
    /// | "union"       | TODO: Fill out. |
    /// | "trait"       | TODO: Fill out. |
    /// | "trait_alias" | TODO: Fill out. |
    /// | "impl"        | TODO: Fill out. |
    /// | "fn"          | TODO: Fill out. |
    /// | "mod_tests"   | TODO: Fill out. |
    ///
    /// ### Limitations
    /// #### Lints on a Contains basis
    /// The lint can be disabled only on a "contains" basis, but not per element
    /// within a "container", e.g. per-module, per-struct, per-enum.
    ///
    /// TODO: Disable for a module.
    ///
    /// #### Module documentation
    /// Module level rustdoc comments are not part of the resulting syntax tree
    /// and as such cannot be linted from within `check_mod`. Instead, the
    /// `rustdoc::missing_documentation` may be used.
    ///
    /// #### Configurations
    ///
    /// TODO: Figure out if the lint can be used in e.g. `#[cfg(test)]`
    ///
    /// ### Example
    /// ```no_run
    /// // example code where clippy issues a warning
    /// ```
    /// Use instead:
    /// ```no_run
    /// // example code which does not raise clippy warning
    /// ```
    #[clippy::version = "1.81.0"]
    pub SOURCE_ITEM_ORDERING,
    restriction,
    "default lint description"
}

#[derive(Debug)]
pub struct SourceItemOrdering {
    assoc_types_order: Vec<SourceItemOrderingTraitAssocItemKind>,
    enable_ordering_for_enum: bool,
    enable_ordering_for_struct: bool,
    enable_ordering_for_trait: bool,
    enable_ordering_for_impl: bool,
    order_groupings: Vec<(String, Vec<SourceItemOrderingModuleItemKind>)>,
}

impl SourceItemOrdering {
    pub fn new(
        assoc_types_order: Vec<SourceItemOrderingTraitAssocItemKind>,
        enable_ordering_for: Vec<SourceItemOrderingEnableFor>,
        order_groupings: Vec<(String, Vec<SourceItemOrderingModuleItemKind>)>,
    ) -> Self {
        // let assoc_types_order_indexed = vec![];
        // for assoc_type in assoc_types_order {
        //     if assoc_types_order_indexed.contains(assoc_type)
        // }

        Self {
            assoc_types_order,
            enable_ordering_for_enum: enable_ordering_for.contains(&SourceItemOrderingEnableFor::Enum),
            enable_ordering_for_struct: enable_ordering_for.contains(&SourceItemOrderingEnableFor::Struct),
            enable_ordering_for_trait: enable_ordering_for.contains(&SourceItemOrderingEnableFor::Trait),
            enable_ordering_for_impl: enable_ordering_for.contains(&SourceItemOrderingEnableFor::Impl),
            order_groupings,
        }
    }

    /// Produces a linting warning for incorrectly ordered item members.
    fn lint_member_name<T: rustc_lint::LintContext>(
        &self,
        cx: &T,
        ident: &rustc_span::symbol::Ident,
        before_ident: &rustc_span::symbol::Ident,
    ) {
        span_lint_and_note(
            cx,
            SOURCE_ITEM_ORDERING,
            ident.span,
            "incorrect ordering of items (must be alphabetically ordered)",
            Some(before_ident.span),
            format!("should be placed before `{}`", before_ident.as_str(),),
        );
    }

    /// Produces a linting warning for incorrectly ordered trait items.
    fn lint_trait_item<T: rustc_lint::LintContext>(&self, cx: &T, item: &TraitItemRef, before_item: &TraitItemRef) {
        span_lint_and_note(
            cx,
            SOURCE_ITEM_ORDERING,
            item.span,
            format!(
                "incorrect ordering of trait items (defined order: {:?})",
                self.assoc_types_order
            ),
            Some(before_item.span),
            format!("should be placed before `{}`", before_item.ident.as_str(),),
        );
    }
}

impl_lint_pass!(SourceItemOrdering => [SOURCE_ITEM_ORDERING]);

impl<'tcx> LateLintPass<'tcx> for SourceItemOrdering {
    fn check_item(&mut self, cx: &LateContext<'tcx>, item: &'tcx Item<'tcx>) {
        match &item.kind {
            ItemKind::Enum(enum_def, _generics) if self.enable_ordering_for_enum => {
                let mut cur_v: Option<&Variant<'_>> = None;
                for variant in enum_def.variants {
                    if let Some(cur_v) = cur_v {
                        if cur_v.ident.name.as_str() > variant.ident.name.as_str() {
                            self.lint_member_name(cx, &variant.ident, &cur_v.ident);
                        }
                    }
                    cur_v = Some(variant);
                }
            },
            ItemKind::Struct(variant_data, _generics) if self.enable_ordering_for_struct => match variant_data {
                VariantData::Struct { fields, .. } => {
                    let mut cur_f: Option<&FieldDef<'_>> = None;
                    for field in fields.iter() {
                        if let Some(cur_f) = cur_f {
                            if cur_f.ident.name.as_str() > field.ident.name.as_str() {
                                self.lint_member_name(cx, &field.ident, &cur_f.ident);
                            }
                        }
                        cur_f = Some(field);
                    }
                },
                _ => {},
            },
            ItemKind::Trait(is_auto, _safety, _generics, _generic_bounds, item_ref)
                if self.enable_ordering_for_trait && *is_auto == IsAuto::No =>
            {
                let mut cur_t: Option<&TraitItemRef> = None;

                for item in item_ref.iter() {
                    if let Some(cur_t) = cur_t {
                        let cur_t_kind = convert_assoc_item_kind(cur_t.kind);
                        let cur_t_kind_index = self.assoc_types_order.iter().position(|e| e == &cur_t_kind);
                        let item_kind = convert_assoc_item_kind(item.kind);
                        let item_kind_index = self.assoc_types_order.iter().position(|e| e == &item_kind);

                        if cur_t_kind == item_kind && cur_t.ident.name.as_str() > item.ident.name.as_str() {
                            self.lint_member_name(cx, &item.ident, &cur_t.ident);
                        } else if cur_t_kind_index > item_kind_index {
                            // let type1 = item
                            self.lint_trait_item(cx, &item, &cur_t);
                        }
                    }
                    cur_t = Some(item);
                }
            },
            ItemKind::Impl(trait_impl) if self.enable_ordering_for_impl => {
                let mut cur_t: Option<&TraitItemRef> = None;

                for item in trait_impl.iter() {
                    if let Some(cur_t) = cur_t {
                        let cur_t_kind = convert_assoc_item_kind(cur_t.kind);
                        let cur_t_kind_index = self.assoc_types_order.iter().position(|e| e == &cur_t_kind);
                        let item_kind = convert_assoc_item_kind(item.kind);
                        let item_kind_index = self.assoc_types_order.iter().position(|e| e == &item_kind);

                        if cur_t_kind == item_kind && cur_t.ident.name.as_str() > item.ident.name.as_str() {
                            self.lint_member_name(cx, &item.ident, &cur_t.ident);
                        } else if cur_t_kind_index > item_kind_index {
                            // let type1 = item
                            self.lint_trait_item(cx, &item, &cur_t);
                        }
                    }
                    cur_t = Some(item);
                }
            },
            _ => {}, // Catch-all for `ItemKinds` that don't have fields.
        }
    }
}

/// Converts a [`rustc_hir::AssocItemKind`] to a
/// [`SourceItemOrderingTraitAssocItemKind`].
///
/// This is implemented here because `rustc_hir` is not a dependency of
/// `clippy_config`.
fn convert_assoc_item_kind(value: AssocItemKind) -> SourceItemOrderingTraitAssocItemKind {
    use SourceItemOrderingTraitAssocItemKind::*;
    match value {
        AssocItemKind::Const { .. } => Const,
        AssocItemKind::Type { .. } => Type,
        AssocItemKind::Fn { .. } => Fn,
    }
}
