//! List of the active feature gates.

use super::{State, Feature};

use crate::edition::Edition;
use crate::symbol::{Symbol, sym};

use syntax_pos::Span;

macro_rules! set {
    ($field: ident) => {{
        fn f(features: &mut Features, _: Span) {
            features.$field = true;
        }
        f as fn(&mut Features, Span)
    }}
}

macro_rules! declare_features {
    ($(
        $(#[doc = $doc:tt])* (active, $feature:ident, $ver:expr, $issue:expr, $edition:expr),
    )+) => {
        /// Represents active features that are currently being implemented or
        /// currently being considered for addition/removal.
        pub const ACTIVE_FEATURES:
            &[Feature] =
            &[$(
                // (sym::$feature, $ver, $issue, $edition, set!($feature))
                Feature {
                    state: State::Active { set: set!($feature) },
                    name: sym::$feature,
                    since: $ver,
                    issue: $issue,
                    edition: $edition,
                    description: concat!($($doc,)*),
                }
            ),+];

        /// A set of features to be used by later passes.
        #[derive(Clone)]
        pub struct Features {
            /// `#![feature]` attrs for language features, for error reporting.
            pub declared_lang_features: Vec<(Symbol, Span, Option<Symbol>)>,
            /// `#![feature]` attrs for non-language (library) features.
            pub declared_lib_features: Vec<(Symbol, Span)>,
            $(
                $(#[doc = $doc])*
                pub $feature: bool
            ),+
        }

        impl Features {
            pub fn new() -> Features {
                Features {
                    declared_lang_features: Vec::new(),
                    declared_lib_features: Vec::new(),
                    $($feature: false),+
                }
            }

            pub fn walk_feature_fields<F>(&self, mut f: F)
                where F: FnMut(&str, bool)
            {
                $(f(stringify!($feature), self.$feature);)+
            }
        }
    };
}

impl Feature {
    /// Sets this feature in `Features`. Panics if called on a non-active feature.
    pub fn set(&self, features: &mut Features, span: Span) {
        match self.state {
            State::Active { set } => set(features, span),
            _ => panic!("called `set` on feature `{}` which is not `active`", self.name)
        }
    }
}

// If you change this, please modify `src/doc/unstable-book` as well.
//
// Don't ever remove anything from this list; move them to `removed.rs`.
//
// The version numbers here correspond to the version in which the current status
// was set. This is most important for knowing when a particular feature became
// stable (active).
//
// Note that the features are grouped into internal/user-facing and then
// sorted by version inside those groups. This is enforced with tidy.
//
// N.B., `tools/tidy/src/features.rs` parses this information directly out of the
// source, so take care when modifying it.

declare_features! (
    // -------------------------------------------------------------------------
    // feature-group-start: internal feature gates
    // -------------------------------------------------------------------------

    // no-tracking-issue-start

    /// Allows using compiler's own crates.
    (active, rustc_private, "1.0.0", Some(27812), None),

    /// Allows using the `rust-intrinsic`'s "ABI".
    (active, intrinsics, "1.0.0", None, None),

    /// Allows using `#[lang = ".."]` attribute for linking items to special compiler logic.
    (active, lang_items, "1.0.0", None, None),

    /// Allows using the `#[stable]` and `#[unstable]` attributes.
    (active, staged_api, "1.0.0", None, None),

    /// Allows using `#[allow_internal_unstable]`. This is an
    /// attribute on `macro_rules!` and can't use the attribute handling
    /// below (it has to be checked before expansion possibly makes
    /// macros disappear).
    (active, allow_internal_unstable, "1.0.0", None, None),

    /// Allows using `#[allow_internal_unsafe]`. This is an
    /// attribute on `macro_rules!` and can't use the attribute handling
    /// below (it has to be checked before expansion possibly makes
    /// macros disappear).
    (active, allow_internal_unsafe, "1.0.0", None, None),

    /// Allows using `#[rustc_const_unstable(feature = "foo", ..)]` which
    /// lets a function to be `const` when opted into with `#![feature(foo)]`.
    (active, rustc_const_unstable, "1.0.0", None, None),

    /// no-tracking-issue-end

    /// Allows using `#[link_name="llvm.*"]`.
    (active, link_llvm_intrinsics, "1.0.0", Some(29602), None),

    /// Allows using `rustc_*` attributes (RFC 572).
    (active, rustc_attrs, "1.0.0", Some(29642), None),

    /// Allows using `#[on_unimplemented(..)]` on traits.
    (active, on_unimplemented, "1.0.0", Some(29628), None),

    /// Allows using the `box $expr` syntax.
    (active, box_syntax, "1.0.0", Some(49733), None),

    /// Allows using `#[main]` to replace the entrypoint `#[lang = "start"]` calls.
    (active, main, "1.0.0", Some(29634), None),

    /// Allows using `#[start]` on a function indicating that it is the program entrypoint.
    (active, start, "1.0.0", Some(29633), None),

    /// Allows using the `#[fundamental]` attribute.
    (active, fundamental, "1.0.0", Some(29635), None),

    /// Allows using the `rust-call` ABI.
    (active, unboxed_closures, "1.0.0", Some(29625), None),

    /// Allows using the `#[linkage = ".."]` attribute.
    (active, linkage, "1.0.0", Some(29603), None),

    /// Allows features specific to OIBIT (auto traits).
    (active, optin_builtin_traits, "1.0.0", Some(13231), None),

    /// Allows using `box` in patterns (RFC 469).
    (active, box_patterns, "1.0.0", Some(29641), None),

    // no-tracking-issue-start

    /// Allows using `#[prelude_import]` on glob `use` items.
    (active, prelude_import, "1.2.0", None, None),

    // no-tracking-issue-end

    // no-tracking-issue-start

    /// Allows using `#[omit_gdb_pretty_printer_section]`.
    (active, omit_gdb_pretty_printer_section, "1.5.0", None, None),

    /// Allows using the `vectorcall` ABI.
    (active, abi_vectorcall, "1.7.0", None, None),

    // no-tracking-issue-end

    /// Allows using `#[structural_match]` which indicates that a type is structurally matchable.
    (active, structural_match, "1.8.0", Some(31434), None),

    /// Allows using the `may_dangle` attribute (RFC 1327).
    (active, dropck_eyepatch, "1.10.0", Some(34761), None),

    /// Allows using the `#![panic_runtime]` attribute.
    (active, panic_runtime, "1.10.0", Some(32837), None),

    /// Allows declaring with `#![needs_panic_runtime]` that a panic runtime is needed.
    (active, needs_panic_runtime, "1.10.0", Some(32837), None),

    // no-tracking-issue-start

    /// Allows identifying the `compiler_builtins` crate.
    (active, compiler_builtins, "1.13.0", None, None),

    /// Allows using the `unadjusted` ABI; perma-unstable.
    (active, abi_unadjusted, "1.16.0", None, None),

    /// Allows identifying crates that contain sanitizer runtimes.
    (active, sanitizer_runtime, "1.17.0", None, None),

    /// Used to identify crates that contain the profiler runtime.
    (active, profiler_runtime, "1.18.0", None, None),

    /// Allows using the `thiscall` ABI.
    (active, abi_thiscall, "1.19.0", None, None),

    /// Allows using `#![needs_allocator]`, an implementation detail of `#[global_allocator]`.
    (active, allocator_internals, "1.20.0", None, None),

    // no-tracking-issue-end

    /// Added for testing E0705; perma-unstable.
    (active, test_2018_feature, "1.31.0", Some(0), Some(Edition::Edition2018)),

    // -------------------------------------------------------------------------
    // feature-group-end: internal feature gates
    // -------------------------------------------------------------------------

    // -------------------------------------------------------------------------
    // feature-group-start: actual feature gates (target features)
    // -------------------------------------------------------------------------

    // FIXME: Document these and merge with the list below.

    // Unstable `#[target_feature]` directives.
    (active, arm_target_feature, "1.27.0", Some(44839), None),
    (active, aarch64_target_feature, "1.27.0", Some(44839), None),
    (active, hexagon_target_feature, "1.27.0", Some(44839), None),
    (active, powerpc_target_feature, "1.27.0", Some(44839), None),
    (active, mips_target_feature, "1.27.0", Some(44839), None),
    (active, avx512_target_feature, "1.27.0", Some(44839), None),
    (active, mmx_target_feature, "1.27.0", Some(44839), None),
    (active, sse4a_target_feature, "1.27.0", Some(44839), None),
    (active, tbm_target_feature, "1.27.0", Some(44839), None),
    (active, wasm_target_feature, "1.30.0", Some(44839), None),
    (active, adx_target_feature, "1.32.0", Some(44839), None),
    (active, cmpxchg16b_target_feature, "1.32.0", Some(44839), None),
    (active, movbe_target_feature, "1.34.0", Some(44839), None),
    (active, rtm_target_feature, "1.35.0", Some(44839), None),
    (active, f16c_target_feature, "1.36.0", Some(44839), None),

    // -------------------------------------------------------------------------
    // feature-group-end: actual feature gates (target features)
    // -------------------------------------------------------------------------

    // -------------------------------------------------------------------------
    // feature-group-start: actual feature gates
    // -------------------------------------------------------------------------

    /// Allows using the `#[link_args]` attribute.
    (active, link_args, "1.0.0", Some(29596), None),

    /// Allows defining identifiers beyond ASCII.
    (active, non_ascii_idents, "1.0.0", Some(55467), None),

    /// Allows using `#[plugin_registrar]` on functions.
    (active, plugin_registrar, "1.0.0", Some(29597), None),

    /// Allows using `#![plugin(myplugin)]`.
    (active, plugin, "1.0.0", Some(29597), None),

    /// Allows using `#[thread_local]` on `static` items.
    (active, thread_local, "1.0.0", Some(29594), None),

    /// Allows the use of SIMD types in functions declared in `extern` blocks.
    (active, simd_ffi, "1.0.0", Some(27731), None),

    /// Allows using custom attributes (RFC 572).
    (active, custom_attribute, "1.0.0", Some(29642), None),

    /// Allows using non lexical lifetimes (RFC 2094).
    (active, nll, "1.0.0", Some(43234), None),

    /// Allows using slice patterns.
    (active, slice_patterns, "1.0.0", Some(62254), None),

    /// Allows the definition of `const` functions with some advanced features.
    (active, const_fn, "1.2.0", Some(57563), None),

    /// Allows associated type defaults.
    (active, associated_type_defaults, "1.2.0", Some(29661), None),

    /// Allows `#![no_core]`.
    (active, no_core, "1.3.0", Some(29639), None),

    /// Allows default type parameters to influence type inference.
    (active, default_type_parameter_fallback, "1.3.0", Some(27336), None),

    /// Allows `repr(simd)` and importing the various simd intrinsics.
    (active, repr_simd, "1.4.0", Some(27731), None),

    /// Allows `extern "platform-intrinsic" { ... }`.
    (active, platform_intrinsics, "1.4.0", Some(27731), None),

    /// Allows `#[unwind(..)]`.
    ///
    /// Permits specifying whether a function should permit unwinding or abort on unwind.
    (active, unwind_attributes, "1.4.0", Some(58760), None),

    /// Allows `#[no_debug]`.
    (active, no_debug, "1.5.0", Some(29721), None),

    /// Allows attributes on expressions and non-item statements.
    (active, stmt_expr_attributes, "1.6.0", Some(15701), None),

    /// Allows the use of type ascription in expressions.
    (active, type_ascription, "1.6.0", Some(23416), None),

    /// Allows `cfg(target_thread_local)`.
    (active, cfg_target_thread_local, "1.7.0", Some(29594), None),

    /// Allows specialization of implementations (RFC 1210).
    (active, specialization, "1.7.0", Some(31844), None),

    /// Allows using `#[naked]` on functions.
    (active, naked_functions, "1.9.0", Some(32408), None),

    /// Allows `cfg(target_has_atomic = "...")`.
    (active, cfg_target_has_atomic, "1.9.0", Some(32976), None),

    /// Allows `X..Y` patterns.
    (active, exclusive_range_pattern, "1.11.0", Some(37854), None),

    /// Allows the `!` type. Does not imply 'exhaustive_patterns' (below) any more.
    (active, never_type, "1.13.0", Some(35121), None),

    /// Allows exhaustive pattern matching on types that contain uninhabited types.
    (active, exhaustive_patterns, "1.13.0", Some(51085), None),

    /// Allows untagged unions `union U { ... }`.
    (active, untagged_unions, "1.13.0", Some(32836), None),

    /// Allows `#[link(..., cfg(..))]`.
    (active, link_cfg, "1.14.0", Some(37406), None),

    /// Allows `extern "ptx-*" fn()`.
    (active, abi_ptx, "1.15.0", Some(38788), None),

    /// Allows the `#[repr(i128)]` attribute for enums.
    (active, repr128, "1.16.0", Some(35118), None),

    /// Allows `#[link(kind="static-nobundle"...)]`.
    (active, static_nobundle, "1.16.0", Some(37403), None),

    /// Allows `extern "msp430-interrupt" fn()`.
    (active, abi_msp430_interrupt, "1.16.0", Some(38487), None),

    /// Allows declarative macros 2.0 (`macro`).
    (active, decl_macro, "1.17.0", Some(39412), None),

    /// Allows `extern "x86-interrupt" fn()`.
    (active, abi_x86_interrupt, "1.17.0", Some(40180), None),

    /// Allows `extern "avr-interrupt" fn()`
    (active, abi_avr_interrupt, "1.18.0", Some(000), None),

    /// Allows overlapping impls of marker traits.
    (active, overlapping_marker_traits, "1.18.0", Some(29864), None),

    /// Allows a test to fail without failing the whole suite.
    (active, allow_fail, "1.19.0", Some(46488), None),

    /// Allows unsized tuple coercion.
    (active, unsized_tuple_coercion, "1.20.0", Some(42877), None),

    /// Allows defining generators.
    (active, generators, "1.21.0", Some(43122), None),

    /// Allows `#[doc(cfg(...))]`.
    (active, doc_cfg, "1.21.0", Some(43781), None),

    /// Allows `#[doc(masked)]`.
    (active, doc_masked, "1.21.0", Some(44027), None),

    /// Allows `#[doc(spotlight)]`.
    (active, doc_spotlight, "1.22.0", Some(45040), None),

    /// Allows `#[doc(include = "some-file")]`.
    (active, external_doc, "1.22.0", Some(44732), None),

    /// Allows future-proofing enums/structs with the `#[non_exhaustive]` attribute (RFC 2008).
    (active, non_exhaustive, "1.22.0", Some(44109), None),

    /// Allows using `crate` as visibility modifier, synonymous with `pub(crate)`.
    (active, crate_visibility_modifier, "1.23.0", Some(53120), None),

    /// Allows defining `extern type`s.
    (active, extern_types, "1.23.0", Some(43467), None),

    /// Allows trait methods with arbitrary self types.
    (active, arbitrary_self_types, "1.23.0", Some(44874), None),

    /// Allows in-band quantification of lifetime bindings (e.g., `fn foo(x: &'a u8) -> &'a u8`).
    (active, in_band_lifetimes, "1.23.0", Some(44524), None),

    /// Allows associated types to be generic, e.g., `type Foo<T>;` (RFC 1598).
    (active, generic_associated_types, "1.23.0", Some(44265), None),

    /// Allows defining `trait X = A + B;` alias items.
    (active, trait_alias, "1.24.0", Some(41517), None),

    /// Allows infering `'static` outlives requirements (RFC 2093).
    (active, infer_static_outlives_requirements, "1.26.0", Some(54185), None),

    /// Allows accessing fields of unions inside `const` functions.
    (active, const_fn_union, "1.27.0", Some(51909), None),

    /// Allows casting raw pointers to `usize` during const eval.
    (active, const_raw_ptr_to_usize_cast, "1.27.0", Some(51910), None),

    /// Allows dereferencing raw pointers during const eval.
    (active, const_raw_ptr_deref, "1.27.0", Some(51911), None),

    /// Allows comparing raw pointers during const eval.
    (active, const_compare_raw_pointers, "1.27.0", Some(53020), None),

    /// Allows `#[doc(alias = "...")]`.
    (active, doc_alias, "1.27.0", Some(50146), None),

    /// Allows inconsistent bounds in where clauses.
    (active, trivial_bounds, "1.28.0", Some(48214), None),

    /// Allows `'a: { break 'a; }`.
    (active, label_break_value, "1.28.0", Some(48594), None),

    /// Allows using `#[doc(keyword = "...")]`.
    (active, doc_keyword, "1.28.0", Some(51315), None),

    /// Allows reinterpretation of the bits of a value of one type as another
    /// type during const eval.
    (active, const_transmute, "1.29.0", Some(53605), None),

    /// Allows using `try {...}` expressions.
    (active, try_blocks, "1.29.0", Some(31436), None),

    /// Allows defining an `#[alloc_error_handler]`.
    (active, alloc_error_handler, "1.29.0", Some(51540), None),

    /// Allows using the `amdgpu-kernel` ABI.
    (active, abi_amdgpu_kernel, "1.29.0", Some(51575), None),

    /// Allows panicking during const eval (producing compile-time errors).
    (active, const_panic, "1.30.0", Some(51999), None),

    /// Allows `#[marker]` on certain traits allowing overlapping implementations.
    (active, marker_trait_attr, "1.30.0", Some(29864), None),

    /// Allows macro invocations on modules expressions and statements and
    /// procedural macros to expand to non-items.
    (active, proc_macro_hygiene, "1.30.0", Some(54727), None),

    /// Allows unsized rvalues at arguments and parameters.
    (active, unsized_locals, "1.30.0", Some(48055), None),

    /// Allows custom test frameworks with `#![test_runner]` and `#[test_case]`.
    (active, custom_test_frameworks, "1.30.0", Some(50297), None),

    /// Allows non-builtin attributes in inner attribute position.
    (active, custom_inner_attributes, "1.30.0", Some(54726), None),

    /// Allows `impl Trait` in bindings (`let`, `const`, `static`).
    (active, impl_trait_in_bindings, "1.30.0", Some(63065), None),

    /// Allows using `reason` in lint attributes and the `#[expect(lint)]` lint check.
    (active, lint_reasons, "1.31.0", Some(54503), None),

    /// Allows exhaustive integer pattern matching on `usize` and `isize`.
    (active, precise_pointer_size_matching, "1.32.0", Some(56354), None),

    /// Allows relaxing the coherence rules such that
    /// `impl<T> ForeignTrait<LocalType> for ForeignType<T>` is permitted.
    (active, re_rebalance_coherence, "1.32.0", Some(55437), None),

    /// Allows using `#[ffi_returns_twice]` on foreign functions.
    (active, ffi_returns_twice, "1.34.0", Some(58314), None),

    /// Allows const generic types (e.g. `struct Foo<const N: usize>(...);`).
    (active, const_generics, "1.34.0", Some(44580), None),

    /// Allows using `#[optimize(X)]`.
    (active, optimize_attribute, "1.34.0", Some(54882), None),

    /// Allows using C-variadics.
    (active, c_variadic, "1.34.0", Some(44930), None),

    /// Allows the user of associated type bounds.
    (active, associated_type_bounds, "1.34.0", Some(52662), None),

    /// Allows calling constructor functions in `const fn`.
    (active, const_constructor, "1.37.0", Some(61456), None),

    /// Allows `if/while p && let q = r && ...` chains.
    (active, let_chains, "1.37.0", Some(53667), None),

    /// Allows #[repr(transparent)] on enums (RFC 2645).
    (active, transparent_enums, "1.37.0", Some(60405), None),

    /// Allows #[repr(transparent)] on unions (RFC 2645).
    (active, transparent_unions, "1.37.0", Some(60405), None),

    /// Allows explicit discriminants on non-unit enum variants.
    (active, arbitrary_enum_discriminant, "1.37.0", Some(60553), None),

    /// Allows `impl Trait` with multiple unrelated lifetimes.
    (active, member_constraints, "1.37.0", Some(61977), None),

    /// Allows `async || body` closures.
    (active, async_closure, "1.37.0", Some(62290), None),

    /// Allows the use of `#[cfg(doctest)]`; set when rustdoc is collecting doctests.
    (active, cfg_doctest, "1.37.0", Some(62210), None),

    /// Allows `[x; N]` where `x` is a constant (RFC 2203).
    (active, const_in_array_repeat_expressions, "1.37.0", Some(49147), None),

    /// Allows `impl Trait` to be used inside type aliases (RFC 2515).
    (active, type_alias_impl_trait, "1.38.0", Some(63063), None),

    /// Allows the use of or-patterns (e.g., `0 | 1`).
    (active, or_patterns, "1.38.0", Some(54883), None),

    // -------------------------------------------------------------------------
    // feature-group-end: actual feature gates
    // -------------------------------------------------------------------------
);

/// Some features are known to be incomplete and using them is likely to have
/// unanticipated results, such as compiler crashes. We warn the user about these
/// to alert them.
pub const INCOMPLETE_FEATURES: &[Symbol] = &[
    sym::impl_trait_in_bindings,
    sym::generic_associated_types,
    sym::const_generics,
    sym::or_patterns,
    sym::let_chains,
];
