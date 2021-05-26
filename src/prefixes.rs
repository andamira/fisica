//! SI prefixes

// THE PROBLEM
//
// - Right now scalar_methods is used for both scalar and vector magnitudes
// - it's using the without_direction() constructor, in order to work for both of them
// - I can change it to new(), but I have to create `vector_methods!` macro and impl_vector_methods
// - In `vector_methods` I can
// - But, for the getters, I have to choses wheter to return:
//   - only the magnitude
//   - Self{m, d}
//   - a tuple (m, d)
//   …and it should be consistent with what scalar_methods does (currently returns the Magnitude,
//   since that it what makes most sense…
//   - also note that there are no non-SI units for vector magnitudes, I think
//
// TODO:
// - add new version: `vector_methods`
//   - use the new() method
// - support compound prefixes, for Speed, Momentum, etc.
//  - all combinations? so either of the 2 units could be scaled? 21×21=441 methods!!
//      - hide the combinations from the docs?
//
//  IDEAS:
//  - change tt to literal?
//  - I don't think it's possible to use an identifier where a tt or literal is expected
//      (in order to reuse $Q in $Qu, e.g. for m²)…
//

/// Generates 2 constructors and 2 getters, for scalar quantities.
macro_rules! scalar_methods {
    // ROOT RULE: NON const, WITH conversion factor
    //
    // LEGEND:
    // $ty = the type to impl the metods
    // $q  = abbreviated quantity in ascii
    // $Q  = quantity in ascii
    // $qu = abbreviated quantity in unicode
    // $Qu = quantity in unicode (or multiple words)
    // $pa = abbreviated prefix in ascii
    // $Pa = prefix in ascii
    // $pu = abbreviated prefix in unicode
    // $Pu = prefix in unicode
    // $f  = conversion factor in number
    // $fu = conversion factor in unicode
    // $Bu = base unit for conversion, in unicode
    //
    // NOTE:
    // - prefixes can be an empty string if you don't need them
    // - The base unit of reference is usually the quantity in unicode
    //
    // MAYBE:
    // - add fa = conversion factor in ascii?
    ($ty:ty, $q:tt, $Q:tt, qu=$qu:tt, Qu=$Qu:tt,
     pu=$pu:tt, Pu=$Pu:tt, pa=$pa:tt, Pa=$Pa:tt, f=$f:expr, fu=$fu:expr, Bu=$Bu:tt) => {
        paste::paste! {
            // constructors
            #[inline]
            #[allow(non_snake_case)]
            #[doc = "New `" $ty "` in " $Pu "" $Qu " (" $pu "" $qu ") (" $fu " " $Bu ")."]
            pub fn [<in_$pa $q>](q: crate::Magnitude) -> Self { Self::without_direction(q * $f) }
            #[inline]
            #[doc = "New `" $ty "` in " $Pu "" $Qu " (" $pu "" $qu ") (" $fu " " $Bu ")."]
            pub fn [<in_$Pa $Q>](q: crate::Magnitude) -> Self { Self::without_direction(q * $f) }
            // getters
            #[inline]
            #[allow(non_snake_case)]
            #[doc = "Returns `" $ty "` as " $Pu "" $Qu " (" $pu "" $qu ") (" $fu " " $Bu ")."]
            pub fn [<as_$pa $q>](&self) -> crate::Magnitude { self.m / $f }
            #[inline]
            #[doc = "Returns `" $ty "` as " $Pu "" $Qu " (" $pu "" $qu ") (" $fu " " $Bu ")."]
            pub fn [<as_$Pa $Q>](&self) -> crate::Magnitude { self.m / $f }
        }
    };
    // ALIAS: no need to specify: qu, Qu, Bu
    // useful for: all non-base units, with unicode prefix
    // e.g.:
    // scalar_methods![$ty, $q, $Q, pu="µ", Pu="micro", pa="u", Pa="micro", f=1e-6, fu="10⁻⁶"];
    ($ty:ty, $q:ident, $Q:ident,
     pu=$pu:tt, Pu=$Pu:tt, pa=$pa:tt, Pa=$Pa:tt, f=$f:expr, fu=$fu:expr) => {
        scalar_methods![
            $ty,
            $q,
            $Q,
            qu = $q,
            Qu = $Q,
            pu = $pa,
            Pu = $Pa,
            pa = $pa,
            Pa = $Pa,
            f = $f,
            fu = $fu,
            Bu = $q
        ];
    };
    // ALIAS: no need to specify: pu, Pu, Bu
    // useful for: squared/cubed non-base units, with ascii prefix
    // e.g.:
    // scalar_methods![$ty, $q, $Q, qu="m²", pa="T", Pa="tera", f=1.0e-3, fu="10⁻³"];
    ($ty:ty, $q:ident, $Q:ident, qu=$qu:tt, Qu=$Qu:tt,
     pa=$pa:tt, Pa=$Pa:tt, f=$f:expr, fu=$fu:expr) => {
        scalar_methods![
            $ty,
            $q,
            $Q,
            qu = $qu,
            Qu = $Qu,
            pu = $pa,
            Pu = $Pa,
            pa = $pa,
            Pa = $Pa,
            f = $f,
            fu = $fu,
            Bu = $qu
        ];
    };
    // ALIAS: no need to specify: qu, Qu, pu, Pu, Bu
    // useful for: all non-base units, with ascii prefix (doesn't support µ, micro)
    // e.g.:
    // scalar_methods![$ty, $q, $Q, "m", "milli", 1.0e-3, "10⁻³"];
    ($ty:ty, $q:ident, $Q:ident,
     pa=$pa:tt, Pa=$Pa:tt, f=$f:expr, fu=$fu:expr) => {
        scalar_methods![
            $ty,
            $q,
            $Q,
            pu = $pa,
            Pu = $Pa,
            pa = $pa,
            Pa = $Pa,
            f = $f,
            fu = $fu
        ];
    };
    // ALIAS: no need to specify: pu, Pu, pa, Pa
    // useful for: units WITHOUT prefix and unicode quantity (like Å, ångströms)
    // e.g.:
    // scalar_methods![ Length, A, angstroms, "`Å`", "`ångströms`",
    //   metres, 1.0e10, "10⁻¹⁰" ];
    ($ty:ty, $q:ident, $Q:ident, qu=$qu:tt, Qu=$Qu:tt, f=$f:expr, fu=$fu:expr,
     Bu=$Bu:tt) => {
        scalar_methods![
            $ty,
            $q,
            $Q,
            qu = $qu,
            Qu = $Qu,
            pu = "",
            Pu = "",
            pa = "",
            Pa = "",
            f = $f,
            fu = $fu,
            Bu = $Bu
        ];
    };
    // ALIAS: no need to specify: qu, Qu, pa, Pa, pu, Pu
    // e.g.:
    // scalar_methods![Time, y, years, f=60., fu="365", Bu="days"];
    ($ty:ty, $q:ident, $Q:ident, f=$f:expr, fu=$fu:tt, Bu=$Bu:tt) => {
        scalar_methods![
            $ty,
            $q,
            $Q,
            qu = $q,
            Qu = $Q,
            pu = "",
            Pu = "",
            pa = "",
            Pa = "",
            f = $f,
            fu = $fu,
            Bu = $Bu
        ];
    };
    // ALIAS: no need to specify: qu, Qu, pa, Pa, pu, Pu, Bu
    // useful for units WITHOUT prefixes, WITH conversion factor (grams)
    // e.g.:
    // scalar_methods![$ty, $q, $Q, f=1e-3, fu="10⁰"];
    ($ty:ty, $q:ident, $Q:ident, f=$f:expr, fu=$fu:expr) => {
        scalar_methods![
            $ty,
            $q,
            $Q,
            pu = "",
            Pu = "",
            pa = "",
            Pa = "",
            f = $f,
            fu = $fu
        ];
    };

    // ROOT RULE: const base unit, WITHOUT conversion factor
    ($ty:ty, $q:ident, $Q:ident, qu=$qu:tt, Qu=$Qu:tt, pa=$pa:tt, Pa=$Pa:tt, base, fu=$fu:expr) => {
        paste::paste! {
            // constructors
            #[inline]
            #[allow(non_snake_case)]
            #[doc = "New `" $ty "` in " $Pa $Qu " (" $pa $qu ") (base unit, " $fu " " $q ")."]
            pub const fn [<in_$pa $q>](q: crate::Magnitude) -> Self { Self::without_direction(q) }
            #[inline]
            #[doc = "New `" $ty "` in " $Pa $Qu " (" $pa $qu ") (base unit, " $fu " " $q ")."]
            pub const fn [<in_$Pa $Q>](q: crate::Magnitude) -> Self { Self::without_direction(q) }
            // getters
            #[inline]
            #[allow(non_snake_case)]
            #[doc = "Returns `" $ty "` as " $Pa $Qu " (" $pa $qu ") (base unit, " $fu " " $q ")."]
            pub const fn [<as_$pa $q>](&self) -> crate::Magnitude { self.m }
            #[inline]
            #[doc = "Returns `" $ty "` as " $Pa $Qu " (" $pa $qu ") (base unit, " $fu " " $q ")."]
            pub const fn [<as_$Pa $Q>](&self) -> crate::Magnitude { self.m }
        }
    };
    // ALIAS: no need to specify: qu, Qu
    // useful for: the kilogram SI base unit
    // e.g.:
    // scalar_methods![$ty, $q, $Q, pa="k", Pa="kilo", base, fu="10³"];
    ($ty:ty, $q:ident, $Q:ident, pa=$pa:tt, Pa=$Pa:tt, base, fu=$fu:expr) => {
        scalar_methods![
            $ty,
            $q,
            $Q,
            qu = $q,
            Qu = $Q,
            pa = $pa,
            Pa = $Pa,
            base,
            fu = $fu
        ];
    };
    // ALIAS: no need to specify: pa, Pa
    // useful for: squared and cubed base units
    // e.g.:
    // scalar_methods![$ty, $q, $Q, qu="m²", Qu="metres squared", base, fu="10⁰"];
    ($ty:ty, $q:ident, $Q:ident, qu=$qu:tt, Qu=$Qu:tt, base, fu=$fu:expr) => {
        scalar_methods![
            $ty,
            $q,
            $Q,
            qu = $qu,
            Qu = $Qu,
            pa = "",
            Pa = "",
            base,
            fu = $fu
        ];
    };
    // ALIAS: no need to specify: qu, Qu, pa, Pa
    // useful for: base SI units except the kilogram (metre, kelvin, pascal…)
    // e.g.:
    // scalar_methods![$ty, $q, $Q, base, fu="10⁰"];
    ($ty:ty, $q:ident, $Q:ident, base, fu=$fu:expr) => {
        scalar_methods![
            $ty,
            $q,
            $Q,
            qu = $q,
            Qu = $Q,
            pa = "",
            Pa = "",
            base,
            fu = $fu
        ];
    };
}

/// Generates SI prefixes constructors and converter methods
macro_rules! impl_scalar_methods {
    ($ty:ty, $q:ident, $Q:ident) => {
        paste::paste! {
            /// # SI prefixes constructors: `in_*` & converters `as_*`
            #[doc = "**The `" $ty "` quantity is internally stored in `" $Q "`**."]
            #[doc = "- base *const* constructors: [`in_" $q "`](" $ty "#method.in_" "),"$q]
            #[doc = "[`in_" $Q "`](" $ty "#method.in_" $Q ")"]
            #[doc = "- base *const* converters: [`as_" $q "`](" $ty "#method.as_" "),"$q]
            #[doc = "[`as_" $Q "`](" $ty "#method.as_" $Q ")"]
            impl $ty {
                scalar_methods![$ty, $q, $Q, pa="Y", Pa="yotta", f=1e24, fu="10²⁴"];
                scalar_methods![$ty, $q, $Q, pa="Z", Pa="zetta", f=1e21, fu="10²¹"];
                scalar_methods![$ty, $q, $Q, pa="E", Pa="exa", f=1e18, fu="10¹⁸"];
                scalar_methods![$ty, $q, $Q, pa="P", Pa="peta", f=1e15, fu="10¹⁵"];
                scalar_methods![$ty, $q, $Q, pa="T", Pa="tera", f=1e12, fu="10¹²"];
                scalar_methods![$ty, $q, $Q, pa="G", Pa="giga", f=1e9, fu="10⁹"];
                scalar_methods![$ty, $q, $Q, pa="M", Pa="mega", f=1e6, fu="10⁶"];
                scalar_methods![$ty, $q, $Q, pa="k", Pa="kilo", f=1e3, fu="10³"];
                scalar_methods![$ty, $q, $Q, pa="h", Pa="hecto", f=1e2, fu="10²"];
                scalar_methods![$ty, $q, $Q, pa="da", Pa="deka", f=1e1, fu="10¹"];
                scalar_methods![$ty, $q, $Q, base, fu="10⁰"];
                scalar_methods![$ty, $q, $Q, pa="d", Pa="deci", f=1e-1, fu="10⁻¹"];
                scalar_methods![$ty, $q, $Q, pa="c", Pa="centi", f=1e-2, fu="10⁻²"];
                scalar_methods![$ty, $q, $Q, pa="m", Pa="milli", f=1e-3, fu="10⁻³"];
                scalar_methods![$ty, $q, $Q, pu="µ", Pu="micro",
                    pa="u", Pa="micro", f=1e-6, fu="10⁻⁶"];
                scalar_methods![$ty, $q, $Q, pa="n", Pa="nano", f=1e-9, fu="10⁻⁹"];
                scalar_methods![$ty, $q, $Q, pa="p", Pa="pico", f=1e-12, fu="10⁻¹²"];
                scalar_methods![$ty, $q, $Q, pa="f", Pa="femto", f=1e-15, fu="10⁻¹⁵"];
                scalar_methods![$ty, $q, $Q, pa="a", Pa="atto", f=1e-18, fu="10⁻¹⁸"];
                scalar_methods![$ty, $q, $Q, pa="z", Pa="zepto", f=1e-21, fu="10⁻²¹"];
                scalar_methods![$ty, $q, $Q, pa="y", Pa="yocto", f=1e-24, fu="10⁻²⁴"];
            }
        }
    };
}

/// Generates constructors from the S.I. metre prefix. (with kilo being the base unit)
macro_rules! impl_scalar_methods_base_kilo {
    ($ty:ty, $q:ident, $Q:ident) => {
        paste::paste! {
            /// # SI prefixes constructors: `in_*` & converters `as_*`
            #[doc = "**The `" $ty "` quantity is internally stored in `kilo" $Q "`**."]
            #[doc = "- base *const* constructors: [`in_k" $q "`](" $ty "#method.in_k" "),"$q]
            #[doc = "[`in_kilo" $Q "`](" $ty "#method.in_kilo" $Q ")"]
            #[doc = "- base *const* converters: [`as_k" $q "`](" $ty "#method.as_k" "),"$q]
            #[doc = "[`as_kilo" $Q "`](" $ty "#method.as_kilo" $Q ")"]
            impl $ty {
                scalar_methods![$ty, $q, $Q, pa="Y", Pa="yotta", f=1e21, fu="10²⁴"];
                scalar_methods![$ty, $q, $Q, pa="Z", Pa="zetta", f=1e18, fu="10²¹"];
                scalar_methods![$ty, $q, $Q, pa="E", Pa="exa", f=1e15, fu="10¹⁸"];
                scalar_methods![$ty, $q, $Q, pa="P", Pa="peta", f=1e12, fu="10¹⁵"];
                scalar_methods![$ty, $q, $Q, pa="T", Pa="tera", f=1e9, fu="10¹²"];
                scalar_methods![$ty, $q, $Q, pa="G", Pa="giga", f=1e6, fu="10⁹"];
                scalar_methods![$ty, $q, $Q, pa="M", Pa="mega", f=1e3, fu="10⁶"];
                scalar_methods![$ty, $q, $Q, pa="k", Pa="kilo", base, fu="10³"];
                scalar_methods![$ty, $q, $Q, pa="h", Pa="hecto", f=1e-1, fu="10²"];
                scalar_methods![$ty, $q, $Q, pa="da", Pa="deka", f=1e-2, fu="10¹"];
                scalar_methods![$ty, $q, $Q, f=1e-3, fu="10⁰"];
                scalar_methods![$ty, $q, $Q, pa="d", Pa="deci", f=1e-4, fu="10⁻¹"];
                scalar_methods![$ty, $q, $Q, pa="c", Pa="centi", f=1e-5, fu="10⁻²"];
                scalar_methods![$ty, $q, $Q, pa="m", Pa="milli", f=1e-6, fu="10⁻³"];
                scalar_methods![$ty, $q, $Q, pu="µ", Pu="micro",
                    pa="u", Pa="micro", f=1e-9, fu="10⁻⁶"];
                scalar_methods![$ty, $q, $Q, pa="n", Pa="nano", f=1e-12, fu="10⁻⁹"];
                scalar_methods![$ty, $q, $Q, pa="p", Pa="pico", f=1e-15, fu="10⁻¹²"];
                scalar_methods![$ty, $q, $Q, pa="f", Pa="femto", f=1e-18, fu="10⁻¹⁵"];
                scalar_methods![$ty, $q, $Q, pa="a", Pa="atto", f=1e-21, fu="10⁻¹⁸"];
                scalar_methods![$ty, $q, $Q, pa="z", Pa="zepto", f=1e-24, fu="10⁻²¹"];
                scalar_methods![$ty, $q, $Q, pa="y", Pa="yocto", f=1e-27, fu="10⁻²⁴"];
            }
        }
    };
}

/// Generates SI prefixes constructors and converter methods (squared)
macro_rules! impl_scalar_methods_squared {
    ($ty:ty, $q:expr, $Q:expr, qu=$qu:expr, Qu=$Qu:expr) => {
        paste::paste! {
            /// # SI prefixes constructors: `in_*` & converters `as_*`
            ///
            /// The units are squared (10² for 1 step, 10⁶ for 3 steps)
            ///
            #[doc = "**The `" $ty "` quantity is internally stored in `" $Q "`**."]
            #[doc = "- base *const* constructors: [`in_" $q "`](" $ty "#method.in_" "),"$q]
            #[doc = "[`in_" $Q "`](" $ty "#method.in_" $Q ")"]
            #[doc = "- base *const* converters: [`as_" $q "`](" $ty "#method.as_" "),"$q]
            #[doc = "[`as_" $Q "`](" $ty "#method.as_" $Q ")"]
            impl $ty {
                // WIP: FIXING nomenclature: kilometres squared → square kilometres
                //
                // E.g.:
                // scalar_methods![$ty, $q, $Q, Q1=$Q1 qu="m²", Qu1="square", QuM="meters",
                //     pa="Y", Pa="yotta", f=1e48, fu="10⁴⁸"];

                scalar_methods![$ty, $q, $Q, qu=$qu, Qu=$Qu,
                    pa="Y", Pa="yotta", f=1e48, fu="10⁴⁸"];
                scalar_methods![$ty, $q, $Q, qu=$qu, Qu=$Qu,
                    pa="Z", Pa="zetta", f=1e42, fu="10⁴²"];
                scalar_methods![$ty, $q, $Q, qu=$qu, Qu=$Qu, pa="E", Pa="exa", f=1e36, fu="10³⁶"];
                scalar_methods![$ty, $q, $Q, qu=$qu, Qu=$Qu, pa="P", Pa="peta", f=1e30, fu="10³⁰"];
                scalar_methods![$ty, $q, $Q, qu=$qu, Qu=$Qu, pa="T", Pa="tera", f=1e24, fu="10²⁴"];
                scalar_methods![$ty, $q, $Q, qu=$qu, Qu=$Qu, pa="G", Pa="giga", f=1e18, fu="10¹⁸"];
                scalar_methods![$ty, $q, $Q, qu=$qu, Qu=$Qu, pa="M", Pa="mega", f=1e12, fu="10¹²"];
                scalar_methods![$ty, $q, $Q, qu=$qu, Qu=$Qu, pa="k", Pa="kilo", f=1e6, fu="10⁶"];
                scalar_methods![$ty, $q, $Q, qu=$qu, Qu=$Qu, pa="h", Pa="hecto", f=1e4, fu="10⁴"];
                scalar_methods![$ty, $q, $Q, qu=$qu, Qu=$Qu, pa="da", Pa="deka", f=1e2, fu="10²"];
                scalar_methods![$ty, $q, $Q, qu=$qu, Qu=$Qu, base, fu="10⁰"];
                scalar_methods![$ty, $q, $Q, qu=$qu, Qu=$Qu, pa="d", Pa="deci", f=1e-2, fu="10⁻²"];
                scalar_methods![$ty, $q, $Q, qu=$qu, Qu=$Qu,
                    pa="c", Pa="centi", f=1e-4, fu="10⁻⁴"];
                scalar_methods![$ty, $q, $Q, qu=$qu, Qu=$Qu,
                    pa="m", Pa="milli", f=1e-6, fu="10⁻⁶"];
                scalar_methods![$ty, $q, $Q, qu=$qu, Qu=$Qu,
                    pu="µ", Pu="micro", pa="u", Pa="micro", f=1e-12, fu="10⁻⁶", Bu=$Q];
                scalar_methods![$ty, $q, $Q, qu=$qu, Qu=$Qu,
                    pa="n", Pa="nano", f=1e-18, fu="10⁻¹⁸"];
                scalar_methods![$ty, $q, $Q, qu=$qu, Qu=$Qu,
                    pa="p", Pa="pico", f=1e-24, fu="10⁻²⁴"];
                scalar_methods![$ty, $q, $Q, qu=$qu, Qu=$Qu,
                    pa="f", Pa="femto", f=1e-30, fu="10⁻³⁰"];
                scalar_methods![$ty, $q, $Q, qu=$qu, Qu=$Qu,
                    pa="a", Pa="atto", f=1e-36, fu="10⁻³⁶"];
                scalar_methods![$ty, $q, $Q, qu=$qu, Qu=$Qu,
                    pa="z", Pa="zepto", f=1e-42, fu="10⁻⁴²"];
                scalar_methods![$ty, $q, $Q, qu=$qu, Qu=$Qu,
                    pa="y", Pa="yocto", f=1e-48, fu="10⁻⁴⁸"];
            }
        }
    };

    // ALIAS: no need to specify: Qu
    ($ty:ty, $q:expr, $Q:expr, qu=$qu:expr) => {
        impl_scalar_methods_squared![$ty, $q, $Q, qu = $qu, Qu = $Q];
    };
}

/// Generates SI prefixes constructors and converter methods (cubed)
macro_rules! impl_scalar_methods_cubed {
    ($ty:ty, $q:expr, $Q:expr, qu=$qu:expr, Qu=$Qu:expr) => {
        paste::paste! {
            /// # SI prefixes constructors: `in_*` & converters `as_*`
            ///
            /// The units are cubed (10³ for 1 step, 10⁹ for 3 steps)
            ///
            #[doc = "**The `" $ty "` quantity is internally stored in `" $Q "`**."]
            #[doc = "- base *const* constructors: [`in_" $q "`](" $ty "#method.in_" $q "),"]
            #[doc = "[`in_" $Q "`](" $ty "#method.in_" $Q ")"]
            #[doc = "- base *const* constructors: [`as_" $q "`](" $ty "#method.as_" $q "),"]
            #[doc = "[`as_" $Q "`](" $ty "#method.as_" $Q ")"]
            impl $ty {
                scalar_methods![$ty, $q, $Q, qu=$qu, Qu=$Qu,
                    pa="Y", Pa="yotta", f=1e72, fu="10⁷²"];
                scalar_methods![$ty, $q, $Q, qu=$qu, Qu=$Qu,
                    pa="Z", Pa="zetta", f=1e63, fu="10⁶³"];
                scalar_methods![$ty, $q, $Q, qu=$qu, Qu=$Qu,
                    pa="E", Pa="exa", f=1e54, fu="10⁵⁴"];
                scalar_methods![$ty, $q, $Q, qu=$qu, Qu=$Qu,
                    pa="P", Pa="peta", f=1e45, fu="10⁴⁵"];
                scalar_methods![$ty, $q, $Q, qu=$qu, Qu=$Qu,
                    pa="T", Pa="tera", f=1e36, fu="10³⁶"];
                scalar_methods![$ty, $q, $Q, qu=$qu, Qu=$Qu,
                    pa="G", Pa="giga", f=1e27, fu="10²⁷"];
                scalar_methods![$ty, $q, $Q, qu=$qu, Qu=$Qu,
                    pa="M", Pa="mega", f=1e18, fu="10¹⁸"];
                scalar_methods![$ty, $q, $Q, qu=$qu, Qu=$Qu,
                    pa="k", Pa="kilo", f=1e9, fu="10⁹"];
                scalar_methods![$ty, $q, $Q, qu=$qu, Qu=$Qu,
                    pa="h", Pa="hecto", f=1e6, fu="10⁶"];
                scalar_methods![$ty, $q, $Q, qu=$qu, Qu=$Qu,
                    pa="da", Pa="deka", f=1e3, fu="10³"];
                scalar_methods![$ty, $q, $Q, qu=$qu, Qu=$Qu, base, fu="10⁰"];
                scalar_methods![$ty, $q, $Q, qu=$qu, Qu=$Qu,
                    pa="d", Pa="deci", f=1e-3, fu="10⁻³"];
                scalar_methods![$ty, $q, $Q, qu=$qu, Qu=$Qu,
                    pa="c", Pa="centi", f=1e-6, fu="10⁻⁶"];
                scalar_methods![$ty, $q, $Q, qu=$qu, Qu=$Qu,
                    pa="m", Pa="milli", f=1e-9, fu="10⁻⁹"];
                scalar_methods![$ty, $q, $Q, qu=$qu, Qu=$Qu,
                    pu="µ", Pu="micro", pa="u", Pa="micro", f=1e-18, fu="10⁻¹⁸", Bu=$Q];
                scalar_methods![$ty, $q, $Q, qu=$qu, Qu=$Qu,
                    pa="n", Pa="nano", f=1e-27, fu="10⁻²⁷"];
                scalar_methods![$ty, $q, $Q, qu=$qu, Qu=$Qu,
                    pa="p", Pa="pico", f=1e-36, fu="10⁻³⁶"];
                scalar_methods![$ty, $q, $Q, qu=$qu, Qu=$Qu,
                    pa="f", Pa="femto", f=1e-45, fu="10⁻⁴⁵"];
                scalar_methods![$ty, $q, $Q, qu=$qu, Qu=$Qu,
                    pa="a", Pa="atto", f=1e-54, fu="10⁻⁵⁴"];
                scalar_methods![$ty, $q, $Q, qu=$qu, Qu=$Qu,
                    pa="z", Pa="zepto", f=1e-63, fu="10⁻⁶³"];
                scalar_methods![$ty, $q, $Q, qu=$qu, Qu=$Qu,
                    pa="y", Pa="yocto", f=1e-72, fu="10⁻⁷²"];
            }
        }
    };

    // ALIAS: no need to specify: Qu
    ($ty:ty, $q:expr, $Q:expr, qu=$qu:expr) => {
        impl_scalar_methods_cubed![$ty, $q, $Q, qu = $qu, Qu = $Q];
    };
}

#[cfg(test)]
mod tests {
    use float_eq::assert_float_eq;

    use crate::units::{Length, Mass};

    #[test]
    fn prefixes() {
        // base unit
        let len = Length::in_m(8.);
        assert_eq!(8., len.m);

        // getters
        assert_float_eq!(8.0_e-24, len.as_Ym(), r2nd <= crate::Magnitude::EPSILON);
        assert_float_eq!(
            8.0_e-24,
            len.as_yottametres(),
            r2nd <= crate::Magnitude::EPSILON
        );
        assert_float_eq!(8.0_e-21, len.as_Zm(), r2nd <= crate::Magnitude::EPSILON);
        assert_float_eq!(
            8.0_e-21,
            len.as_zettametres(),
            r2nd <= crate::Magnitude::EPSILON
        );
        assert_float_eq!(8.0_e-18, len.as_Em(), r2nd <= crate::Magnitude::EPSILON);
        assert_float_eq!(
            8.0_e-18,
            len.as_exametres(),
            r2nd <= crate::Magnitude::EPSILON
        );
        assert_float_eq!(8.0_e-15, len.as_Pm(), r2nd <= crate::Magnitude::EPSILON);
        assert_float_eq!(
            8.0_e-15,
            len.as_petametres(),
            r2nd <= crate::Magnitude::EPSILON
        );
        assert_float_eq!(8.0_e-12, len.as_Tm(), r2nd <= crate::Magnitude::EPSILON);
        assert_float_eq!(
            8.0_e-12,
            len.as_terametres(),
            r2nd <= crate::Magnitude::EPSILON
        );
        assert_float_eq!(8.0_e-9, len.as_Gm(), r2nd <= crate::Magnitude::EPSILON);
        assert_float_eq!(
            8.0_e-9,
            len.as_gigametres(),
            r2nd <= crate::Magnitude::EPSILON
        );
        assert_float_eq!(8.0_e-6, len.as_Mm(), r2nd <= crate::Magnitude::EPSILON);
        assert_float_eq!(
            8.0_e-6,
            len.as_megametres(),
            r2nd <= crate::Magnitude::EPSILON
        );
        assert_float_eq!(8.0_e-3, len.as_km(), r2nd <= crate::Magnitude::EPSILON);
        assert_float_eq!(
            8.0_e-3,
            len.as_kilometres(),
            r2nd <= crate::Magnitude::EPSILON
        );
        assert_float_eq!(8.0_e-2, len.as_hm(), r2nd <= crate::Magnitude::EPSILON);
        assert_float_eq!(
            8.0_e-2,
            len.as_hectometres(),
            r2nd <= crate::Magnitude::EPSILON
        );
        assert_float_eq!(8.0_e-1, len.as_dam(), r2nd <= crate::Magnitude::EPSILON);
        assert_float_eq!(
            8.0_e-1,
            len.as_dekametres(),
            r2nd <= crate::Magnitude::EPSILON
        );
        assert_float_eq!(8.0, len.as_m(), r2nd <= crate::Magnitude::EPSILON);
        assert_float_eq!(8.0, len.as_metres(), r2nd <= crate::Magnitude::EPSILON);
        assert_float_eq!(8.0_e1, len.as_dm(), r2nd <= crate::Magnitude::EPSILON);
        assert_float_eq!(
            8.0_e1,
            len.as_decimetres(),
            r2nd <= crate::Magnitude::EPSILON
        );
        assert_float_eq!(8.0_e2, len.as_cm(), r2nd <= crate::Magnitude::EPSILON);
        assert_float_eq!(
            8.0_e2,
            len.as_centimetres(),
            r2nd <= crate::Magnitude::EPSILON
        );
        assert_float_eq!(8.0_e3, len.as_mm(), r2nd <= crate::Magnitude::EPSILON);
        assert_float_eq!(
            8.0_e3,
            len.as_millimetres(),
            r2nd <= crate::Magnitude::EPSILON
        );
        assert_float_eq!(8.0_e6, len.as_um(), r2nd <= crate::Magnitude::EPSILON);
        assert_float_eq!(
            8.0_e6,
            len.as_micrometres(),
            r2nd <= crate::Magnitude::EPSILON
        );
        assert_float_eq!(8.0_e9, len.as_nm(), r2nd <= crate::Magnitude::EPSILON);
        assert_float_eq!(
            8.0_e9,
            len.as_nanometres(),
            r2nd <= crate::Magnitude::EPSILON
        );
        assert_float_eq!(8.0_e12, len.as_pm(), r2nd <= crate::Magnitude::EPSILON);
        assert_float_eq!(
            8.0_e12,
            len.as_picometres(),
            r2nd <= crate::Magnitude::EPSILON
        );
        assert_float_eq!(8.0_e15, len.as_fm(), r2nd <= crate::Magnitude::EPSILON);
        assert_float_eq!(
            8.0_e15,
            len.as_femtometres(),
            r2nd <= crate::Magnitude::EPSILON
        );
        assert_float_eq!(8.0_e18, len.as_am(), r2nd <= crate::Magnitude::EPSILON);
        assert_float_eq!(
            8.0_e18,
            len.as_attometres(),
            r2nd <= crate::Magnitude::EPSILON
        );
        assert_float_eq!(8.0_e21, len.as_zm(), r2nd <= crate::Magnitude::EPSILON);
        assert_float_eq!(
            8.0_e21,
            len.as_zeptometres(),
            r2nd <= crate::Magnitude::EPSILON
        );
        assert_float_eq!(8.0_e24, len.as_ym(), r2nd <= crate::Magnitude::EPSILON);
        assert_float_eq!(
            8.0_e24,
            len.as_yoctometres(),
            r2nd <= crate::Magnitude::EPSILON
        );

        // constructors
        let len = Length::in_Ym(8.0_e-24);
        assert_float_eq!(8.0, len.m, r2nd <= crate::Magnitude::EPSILON);
        let len = Length::in_yottametres(8.0_e-24);
        assert_float_eq!(8.0, len.m, r2nd <= crate::Magnitude::EPSILON);
        let len = Length::in_Zm(8.0_e-21);
        assert_float_eq!(8.0, len.m, r2nd <= crate::Magnitude::EPSILON);
        let len = Length::in_zettametres(8.0_e-21);
        assert_float_eq!(8.0, len.m, r2nd <= crate::Magnitude::EPSILON);
        let len = Length::in_Em(8.0_e-18);
        assert_float_eq!(8.0, len.m, r2nd <= crate::Magnitude::EPSILON);
        let len = Length::in_exametres(8.0_e-18);
        assert_float_eq!(8.0, len.m, r2nd <= crate::Magnitude::EPSILON);
        let len = Length::in_Pm(8.0_e-15);
        assert_float_eq!(8.0, len.m, r2nd <= crate::Magnitude::EPSILON);
        let len = Length::in_petametres(8.0_e-15);
        assert_float_eq!(8.0, len.m, r2nd <= crate::Magnitude::EPSILON);
        let len = Length::in_Tm(8.0_e-12);
        assert_float_eq!(8.0, len.m, r2nd <= crate::Magnitude::EPSILON);
        let len = Length::in_terametres(8.0_e-12);
        assert_float_eq!(8.0, len.m, r2nd <= crate::Magnitude::EPSILON);
        let len = Length::in_Gm(8.0_e-9);
        assert_float_eq!(8.0, len.m, r2nd <= crate::Magnitude::EPSILON);
        let len = Length::in_gigametres(8.0_e-9);
        assert_float_eq!(8.0, len.m, r2nd <= crate::Magnitude::EPSILON);
        let len = Length::in_Mm(8.0_e-6);
        assert_float_eq!(8.0, len.m, r2nd <= crate::Magnitude::EPSILON);
        let len = Length::in_megametres(8.0_e-6);
        assert_float_eq!(8.0, len.m, r2nd <= crate::Magnitude::EPSILON);
        let len = Length::in_km(8.0_e-3);
        assert_float_eq!(8.0, len.m, r2nd <= crate::Magnitude::EPSILON);
        let len = Length::in_kilometres(8.0_e-3);
        assert_float_eq!(8.0, len.m, r2nd <= crate::Magnitude::EPSILON);
        let len = Length::in_hm(8.0_e-2);
        assert_float_eq!(8.0, len.m, r2nd <= crate::Magnitude::EPSILON);
        let len = Length::in_hectometres(8.0_e-2);
        assert_float_eq!(8.0, len.m, r2nd <= crate::Magnitude::EPSILON);
        let len = Length::in_dam(8.0_e-1);
        assert_float_eq!(8.0, len.m, r2nd <= crate::Magnitude::EPSILON);
        let len = Length::in_dekametres(8.0_e-1);
        assert_float_eq!(8.0, len.m, r2nd <= crate::Magnitude::EPSILON);
        let len = Length::in_m(8.0);
        assert_float_eq!(8.0, len.m, r2nd <= crate::Magnitude::EPSILON);
        let len = Length::in_metres(8.0);
        assert_float_eq!(8.0, len.m, r2nd <= crate::Magnitude::EPSILON);
        let len = Length::in_dm(8.0_e1);
        assert_float_eq!(8.0, len.m, r2nd <= crate::Magnitude::EPSILON);
        let len = Length::in_decimetres(8.0_e1);
        assert_float_eq!(8.0, len.m, r2nd <= crate::Magnitude::EPSILON);
        let len = Length::in_cm(8.0_e2);
        assert_float_eq!(8.0, len.m, r2nd <= crate::Magnitude::EPSILON);
        let len = Length::in_centimetres(8.0_e2);
        assert_float_eq!(8.0, len.m, r2nd <= crate::Magnitude::EPSILON);
        let len = Length::in_mm(8.0_e3);
        assert_float_eq!(8.0, len.m, r2nd <= crate::Magnitude::EPSILON);
        let len = Length::in_millimetres(8.0_e3);
        assert_float_eq!(8.0, len.m, r2nd <= crate::Magnitude::EPSILON);
        let len = Length::in_um(8.0_e6);
        assert_float_eq!(8.0, len.m, r2nd <= crate::Magnitude::EPSILON);
        let len = Length::in_micrometres(8.0_e6);
        assert_float_eq!(8.0, len.m, r2nd <= crate::Magnitude::EPSILON);
        let len = Length::in_nm(8.0_e9);
        assert_float_eq!(8.0, len.m, r2nd <= crate::Magnitude::EPSILON);
        let len = Length::in_nanometres(8.0_e9);
        assert_float_eq!(8.0, len.m, r2nd <= crate::Magnitude::EPSILON);
        let len = Length::in_pm(8.0_e12);
        assert_float_eq!(8.0, len.m, r2nd <= crate::Magnitude::EPSILON);
        let len = Length::in_picometres(8.0_e12);
        assert_float_eq!(8.0, len.m, r2nd <= crate::Magnitude::EPSILON);
        let len = Length::in_fm(8.0_e15);
        assert_float_eq!(8.0, len.m, r2nd <= crate::Magnitude::EPSILON);
        let len = Length::in_femtometres(8.0_e15);
        assert_float_eq!(8.0, len.m, r2nd <= crate::Magnitude::EPSILON);
        let len = Length::in_am(8.0_e18);
        assert_float_eq!(8.0, len.m, r2nd <= crate::Magnitude::EPSILON);
        let len = Length::in_attometres(8.0_e18);
        assert_float_eq!(8.0, len.m, r2nd <= crate::Magnitude::EPSILON);
        let len = Length::in_zm(8.0_e21);
        assert_float_eq!(8.0, len.m, r2nd <= crate::Magnitude::EPSILON);
        let len = Length::in_zeptometres(8.0_e21);
        assert_float_eq!(8.0, len.m, r2nd <= crate::Magnitude::EPSILON);

        // FIXME: not enough precision with f64
        //
        // let len = Length::in_ym(8.0_e24); // == 0.0
        // assert_float_eq!(8.0, len.m, r2nd <= crate::Magnitude::EPSILON);
        // let len = Length::in_yoctometres(8.0_e24); // == 0.0
        // assert_float_eq!(8.0, len.m, r2nd <= crate::Magnitude::EPSILON);
    }

    #[test]
    fn prefixes_base_kilo() {
        // base unit
        let mass = Mass::in_kg(8.);
        assert_eq!(8., mass.m);

        // getters
        assert_float_eq!(8.0_e-21, mass.as_Yg(), r2nd <= crate::Magnitude::EPSILON);
        assert_float_eq!(
            8.0_e-21,
            mass.as_yottagrams(),
            r2nd <= crate::Magnitude::EPSILON
        );
        assert_float_eq!(8.0_e-18, mass.as_Zg(), r2nd <= crate::Magnitude::EPSILON);
        assert_float_eq!(
            8.0_e-18,
            mass.as_zettagrams(),
            r2nd <= crate::Magnitude::EPSILON
        );
        assert_float_eq!(8.0_e-15, mass.as_Eg(), r2nd <= crate::Magnitude::EPSILON);
        assert_float_eq!(
            8.0_e-15,
            mass.as_exagrams(),
            r2nd <= crate::Magnitude::EPSILON
        );
        assert_float_eq!(8.0_e-12, mass.as_Pg(), r2nd <= crate::Magnitude::EPSILON);
        assert_float_eq!(
            8.0_e-12,
            mass.as_petagrams(),
            r2nd <= crate::Magnitude::EPSILON
        );
        assert_float_eq!(8.0_e-9, mass.as_Tg(), r2nd <= crate::Magnitude::EPSILON);
        assert_float_eq!(
            8.0_e-9,
            mass.as_teragrams(),
            r2nd <= crate::Magnitude::EPSILON
        );
        assert_float_eq!(8.0_e-6, mass.as_Gg(), r2nd <= crate::Magnitude::EPSILON);
        assert_float_eq!(
            8.0_e-6,
            mass.as_gigagrams(),
            r2nd <= crate::Magnitude::EPSILON
        );
        assert_float_eq!(8.0_e-3, mass.as_Mg(), r2nd <= crate::Magnitude::EPSILON);
        assert_float_eq!(
            8.0_e-3,
            mass.as_megagrams(),
            r2nd <= crate::Magnitude::EPSILON
        );
        assert_float_eq!(8.0, mass.as_kg(), r2nd <= crate::Magnitude::EPSILON);
        assert_float_eq!(8.0, mass.as_kilograms(), r2nd <= crate::Magnitude::EPSILON);
        assert_float_eq!(8.0_e1, mass.as_hg(), r2nd <= crate::Magnitude::EPSILON);
        assert_float_eq!(
            8.0_e1,
            mass.as_hectograms(),
            r2nd <= crate::Magnitude::EPSILON
        );
        assert_float_eq!(8.0_e2, mass.as_dag(), r2nd <= crate::Magnitude::EPSILON);
        assert_float_eq!(
            8.0_e2,
            mass.as_dekagrams(),
            r2nd <= crate::Magnitude::EPSILON
        );
        assert_float_eq!(8.0_e3, mass.as_g(), r2nd <= crate::Magnitude::EPSILON);
        assert_float_eq!(8.0_e3, mass.as_grams(), r2nd <= crate::Magnitude::EPSILON);
        assert_float_eq!(8.0_e4, mass.as_dg(), r2nd <= crate::Magnitude::EPSILON);
        assert_float_eq!(
            8.0_e4,
            mass.as_decigrams(),
            r2nd <= crate::Magnitude::EPSILON
        );
        assert_float_eq!(8.0_e5, mass.as_cg(), r2nd <= crate::Magnitude::EPSILON);
        assert_float_eq!(
            8.0_e5,
            mass.as_centigrams(),
            r2nd <= crate::Magnitude::EPSILON
        );
        assert_float_eq!(8.0_e6, mass.as_mg(), r2nd <= crate::Magnitude::EPSILON);
        assert_float_eq!(
            8.0_e6,
            mass.as_milligrams(),
            r2nd <= crate::Magnitude::EPSILON
        );
        assert_float_eq!(8.0_e9, mass.as_ug(), r2nd <= crate::Magnitude::EPSILON);
        assert_float_eq!(
            8.0_e9,
            mass.as_micrograms(),
            r2nd <= crate::Magnitude::EPSILON
        );
        assert_float_eq!(8.0_e12, mass.as_ng(), r2nd <= crate::Magnitude::EPSILON);
        assert_float_eq!(
            8.0_e12,
            mass.as_nanograms(),
            r2nd <= crate::Magnitude::EPSILON
        );
        assert_float_eq!(8.0_e15, mass.as_pg(), r2nd <= crate::Magnitude::EPSILON);
        assert_float_eq!(
            8.0_e15,
            mass.as_picograms(),
            r2nd <= crate::Magnitude::EPSILON
        );
        assert_float_eq!(8.0_e18, mass.as_fg(), r2nd <= crate::Magnitude::EPSILON);
        assert_float_eq!(
            8.0_e18,
            mass.as_femtograms(),
            r2nd <= crate::Magnitude::EPSILON
        );
        assert_float_eq!(8.0_e21, mass.as_ag(), r2nd <= crate::Magnitude::EPSILON);
        assert_float_eq!(
            8.0_e21,
            mass.as_attograms(),
            r2nd <= crate::Magnitude::EPSILON
        );
        assert_float_eq!(8.0_e24, mass.as_zg(), r2nd <= crate::Magnitude::EPSILON);
        assert_float_eq!(
            8.0_e24,
            mass.as_zeptograms(),
            r2nd <= crate::Magnitude::EPSILON
        );
        assert_float_eq!(8.0_e27, mass.as_yg(), r2nd <= crate::Magnitude::EPSILON);
        assert_float_eq!(
            8.0_e27,
            mass.as_yoctograms(),
            r2nd <= crate::Magnitude::EPSILON
        );

        // constructors
        let mass = Mass::in_Yg(8.0_e-21);
        assert_float_eq!(8.0, mass.m, r2nd <= crate::Magnitude::EPSILON);
        let mass = Mass::in_yottagrams(8.0_e-21);
        assert_float_eq!(8.0, mass.m, r2nd <= crate::Magnitude::EPSILON);
        let mass = Mass::in_Zg(8.0_e-18);
        assert_float_eq!(8.0, mass.m, r2nd <= crate::Magnitude::EPSILON);
        let mass = Mass::in_zettagrams(8.0_e-18);
        assert_float_eq!(8.0, mass.m, r2nd <= crate::Magnitude::EPSILON);
        let mass = Mass::in_Eg(8.0_e-15);
        assert_float_eq!(8.0, mass.m, r2nd <= crate::Magnitude::EPSILON);
        let mass = Mass::in_exagrams(8.0_e-15);
        assert_float_eq!(8.0, mass.m, r2nd <= crate::Magnitude::EPSILON);
        let mass = Mass::in_Pg(8.0_e-12);
        assert_float_eq!(8.0, mass.m, r2nd <= crate::Magnitude::EPSILON);
        let mass = Mass::in_petagrams(8.0_e-12);
        assert_float_eq!(8.0, mass.m, r2nd <= crate::Magnitude::EPSILON);
        let mass = Mass::in_Tg(8.0_e-9);
        assert_float_eq!(8.0, mass.m, r2nd <= crate::Magnitude::EPSILON);
        let mass = Mass::in_teragrams(8.0_e-9);
        assert_float_eq!(8.0, mass.m, r2nd <= crate::Magnitude::EPSILON);
        let mass = Mass::in_Gg(8.0_e-6);
        assert_float_eq!(8.0, mass.m, r2nd <= crate::Magnitude::EPSILON);
        let mass = Mass::in_gigagrams(8.0_e-6);
        assert_float_eq!(8.0, mass.m, r2nd <= crate::Magnitude::EPSILON);
        let mass = Mass::in_Mg(8.0_e-3);
        assert_float_eq!(8.0, mass.m, r2nd <= crate::Magnitude::EPSILON);
        let mass = Mass::in_megagrams(8.0_e-3);
        assert_float_eq!(8.0, mass.m, r2nd <= crate::Magnitude::EPSILON);
        let mass = Mass::in_kg(8.0);
        assert_float_eq!(8.0, mass.m, r2nd <= crate::Magnitude::EPSILON);
        let mass = Mass::in_kilograms(8.0);
        assert_float_eq!(8.0, mass.m, r2nd <= crate::Magnitude::EPSILON);
        let mass = Mass::in_hg(8.0_e1);
        assert_float_eq!(8.0, mass.m, r2nd <= crate::Magnitude::EPSILON);
        let mass = Mass::in_hectograms(8.0_e1);
        assert_float_eq!(8.0, mass.m, r2nd <= crate::Magnitude::EPSILON);
        let mass = Mass::in_dag(8.0_e2);
        assert_float_eq!(8.0, mass.m, r2nd <= crate::Magnitude::EPSILON);
        let mass = Mass::in_dekagrams(8.0_e2);
        assert_float_eq!(8.0, mass.m, r2nd <= crate::Magnitude::EPSILON);
        let mass = Mass::in_g(8.0_e3);
        assert_float_eq!(8.0, mass.m, r2nd <= crate::Magnitude::EPSILON);
        let mass = Mass::in_grams(8.0_e3);
        assert_float_eq!(8.0, mass.m, r2nd <= crate::Magnitude::EPSILON);
        let mass = Mass::in_dg(8.0_e4);
        assert_float_eq!(8.0, mass.m, r2nd <= crate::Magnitude::EPSILON);
        let mass = Mass::in_decigrams(8.0_e4);
        assert_float_eq!(8.0, mass.m, r2nd <= crate::Magnitude::EPSILON);
        let mass = Mass::in_cg(8.0_e5);
        assert_float_eq!(8.0, mass.m, r2nd <= crate::Magnitude::EPSILON);
        let mass = Mass::in_centigrams(8.0_e5);
        assert_float_eq!(8.0, mass.m, r2nd <= crate::Magnitude::EPSILON);
        let mass = Mass::in_mg(8.0_e6);
        assert_float_eq!(8.0, mass.m, r2nd <= crate::Magnitude::EPSILON);
        let mass = Mass::in_milligrams(8.0_e6);
        assert_float_eq!(8.0, mass.m, r2nd <= crate::Magnitude::EPSILON);
        let mass = Mass::in_ug(8.0_e9);
        assert_float_eq!(8.0, mass.m, r2nd <= crate::Magnitude::EPSILON);
        let mass = Mass::in_micrograms(8.0_e9);
        assert_float_eq!(8.0, mass.m, r2nd <= crate::Magnitude::EPSILON);
        let mass = Mass::in_ng(8.0_e12);
        assert_float_eq!(8.0, mass.m, r2nd <= crate::Magnitude::EPSILON);
        let mass = Mass::in_nanograms(8.0_e12);
        assert_float_eq!(8.0, mass.m, r2nd <= crate::Magnitude::EPSILON);
        let mass = Mass::in_pg(8.0_e15);
        assert_float_eq!(8.0, mass.m, r2nd <= crate::Magnitude::EPSILON);
        let mass = Mass::in_picograms(8.0_e15);
        assert_float_eq!(8.0, mass.m, r2nd <= crate::Magnitude::EPSILON);
        let mass = Mass::in_fg(8.0_e18);
        assert_float_eq!(8.0, mass.m, r2nd <= crate::Magnitude::EPSILON);
        let mass = Mass::in_femtograms(8.0_e18);
        assert_float_eq!(8.0, mass.m, r2nd <= crate::Magnitude::EPSILON);
        let mass = Mass::in_ag(8.0_e21);
        assert_float_eq!(8.0, mass.m, r2nd <= crate::Magnitude::EPSILON);
        let mass = Mass::in_attograms(8.0_e21);
        assert_float_eq!(8.0, mass.m, r2nd <= crate::Magnitude::EPSILON);
        let mass = Mass::in_zg(8.0_e24);
        assert_float_eq!(8.0, mass.m, r2nd <= crate::Magnitude::EPSILON);
        let mass = Mass::in_zeptograms(8.0_e24);
        assert_float_eq!(8.0, mass.m, r2nd <= crate::Magnitude::EPSILON);

        // FIXME: not enough precision with f64
        //
        // let mass = Mass::in_yg(8.0_e27);
        // assert_float_eq!(8.0, mass.m, r2nd <= crate::Magnitude::EPSILON);
        // let mass = Mass::in_yoctograms(8.0_e27);
        // assert_float_eq!(8.0, mass.m, r2nd <= crate::Magnitude::EPSILON);
    }
}
