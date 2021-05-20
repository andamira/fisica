//! SI prefixes
//!

// TODO: devise a way to create compound prefixes, for Area, Speed, Momentum, etc.
//  - either of the 2 or 3 units could be the modified one…
//
// IMPROVE: allow using specific symbols in the docs (for `µ`, `Å` & `ångströms`…)

use crate::*;

/// Generates 2 prefix constructors (short and long)
macro_rules! fn_prefix_constructors {
    ($type:ty, $q:ident, $quantity:ident, $p:ident, $prefix:ident, $oper:expr, $pow:expr) => {
        paste::paste! {
            #[inline]
            #[allow(non_snake_case)]
            #[doc = "New `" $type "` in `" $p "" $q "` (`" $prefix "" $quantity
                "`) (10" $pow " " $quantity ")."]
            pub fn [<in_$p $q>](q: crate::F) -> Self { Self(q * $oper) }

            #[inline]
            #[doc = "New `" $type "` in `" $p "" $q "` (`" $prefix "" $quantity
                "`) (10" $pow " " $quantity ")."]
            pub fn [<in_$prefix $quantity>](q: crate::F) -> Self { Self(q * $oper) }
        }
    };

    ($type:ty, $q:ident, $quantity:ident, $oper:expr, $pow:expr) => {
        paste::paste! {
            #[inline]
            #[allow(non_snake_case)]
            #[doc = "New `" $type "` in `" $q "` (`" $quantity "`) (10" $pow " " $quantity ")."]
            pub fn [<in_$q>](q: crate::F) -> Self { Self(q * $oper) }

            #[inline]
            #[doc = "New `" $type "` in `" $q "` (`" $quantity "`) (10" $pow " " $quantity ")."]
            pub fn [<in_$quantity>](q: crate::F) -> Self { Self(q * $oper) }
        }
    };

    // base constructors are const, and stores the value as it is.
    ($type:ty, $q:ident, $quantity:ident, $p:ident, $prefix:ident, $pow:expr) => {
        paste::paste! {
            #[inline]
            #[doc = "New `" $type "` in `" $p "" $q "` (`" $prefix "" $quantity
                "`) (base unit, 10" $pow " " $quantity ")."]
            pub const fn [<in_$p $q>](q: crate::F) -> Self { Self(q) }

            #[inline]
            #[doc = "New `" $type "` in `" $p "" $q "` (`" $prefix "" $quantity
                "`) (base unit, 10" $pow " " $quantity ")."]
            pub const fn [<in_$prefix $quantity>](q: crate::F) -> Self { Self(q) }
        }
    };
    ($type:ty, $q:ident, $quantity:ident, $pow:expr) => {
        paste::paste! {
            #[inline]
            #[allow(non_snake_case)]
            #[doc = "New `" $type "` in `" $q "` (`" $quantity "`) (base unit, 10⁰ "
                $quantity ")."]
            pub const fn [<in_$q>](q: crate::F) -> Self { Self(q) }

            #[inline]
            #[doc = "New `" $type "` in `" $q "` (`" $quantity "`) (base unit, 10⁰ "
                $quantity ")."]
            pub const fn [<in_$quantity>](q: crate::F) -> Self { Self(q) }
        }
    };
}

/// Generates 2 prefix getters (short and long)
macro_rules! fn_prefix_getters {
    ($type:ty, $q:ident, $quantity:ident, $p:ident, $prefix:ident, $oper:expr, $pow:expr) => {
        paste::paste! {
            #[inline]
            #[doc = "Returns `" $type "` as `" $p "" $q "` (`" $prefix "" $quantity
                "`) (10" $pow " " $quantity ")."]
            #[allow(non_snake_case)]
            pub fn [<as_$p $q>](&self) -> F { self.0 / $oper }

            #[inline]
            #[doc = "Returns `" $type "` as `" $p "" $q "` (`" $prefix "" $quantity
                "`) (10" $pow " " $quantity ")."]
            pub fn [<as_$prefix $quantity>](&self) -> F { self.0 / $oper }
        }
    };

    ($type:ty, $q:ident, $quantity:ident, $oper:expr, $pow:expr) => {
        paste::paste! {
            #[inline]
            #[doc = "Returns `" $type "` as `" $q "` (`" $quantity
                "`) (10" $pow " " $quantity ")."]
            pub fn [<as_$q>](&self) -> F { self.0 / $oper }

            #[inline]
            #[doc = "Returns `" $type "` as `" $q "` (`" $quantity
                "`) (10" $pow " " $quantity ")."]
            pub fn [<as_$quantity>](&self) -> F { self.0 / $oper }
        }
    };

    // base unit getters are const, and returns the stored value as it is.
    ($type:ty, $q:ident, $quantity:ident, $p:ident, $prefix:ident, $pow:expr) => {
        paste::paste! {
            #[inline]
            #[doc = "Returns `" $type " as `  `" $p "" $q "` (`" $prefix "" $quantity
                "`) (10" $pow " " $quantity ")."]
            pub const fn [<as_$p $q>](&self) -> F { self.0 }

            #[inline]
            #[doc = "Returns `" $type " as `  `" $p "" $q "` (`" $prefix "" $quantity
                "`) (10" $pow " " $quantity ")."]
            pub const fn [<as_$prefix $quantity>](&self) -> F { self.0 }
        }
    };

    ($type:ty, $q:ident, $quantity:ident, $pow:expr) => {
        paste::paste! {
            #[inline]
            #[doc = "Returns `" $type " as `  `" $q "` (`" $quantity
                "`) (10" $pow " " $quantity ")."]
            #[allow(non_snake_case)]
            pub const fn [<as_$q>](&self) -> F { self.0 }

            #[inline]
            #[doc = "Returns `" $type " as `  `" $q "` (`" $quantity
                "`) (10" $pow " " $quantity ")."]
            pub const fn [<as_$quantity>](&self) -> F { self.0 }
        }
    };
}

/// Generates SI prefixes constructors and converter methods
macro_rules! impl_prefixes {
    ($type:ty, $q:ident, $quantity:ident) => {
        paste::paste! {

            /// # SI prefixes constructors: `in_*` & converters `as_*`
            #[doc = "**The `" $type "` quantity is internally stored in `" $quantity "`**."]
            #[doc = "- base *const* constructors: [`in_" $q "`](" $type "#method.in_" $q"),"]
            #[doc = "[`in_" $quantity "`](" $type "#method.in_" $quantity ")"]
            #[doc = "- base *const* converters: [`as_" $q "`](" $type "#method.as_" $q"),"]
            #[doc = "[`as_" $quantity "`](" $type "#method.as_" $quantity ")"]

            impl $type {
                fn_prefix_constructors![$type, $q, $quantity, Y, yotta, 1.0e24, "²⁴"];
                fn_prefix_constructors![$type, $q, $quantity, Z, zetta, 1.0e21, "²¹"];
                fn_prefix_constructors![$type, $q, $quantity, E, exa, 1.0e18, "¹⁸"];
                fn_prefix_constructors![$type, $q, $quantity, P, peta, 1.0e15, "¹⁵"];
                fn_prefix_constructors![$type, $q, $quantity, T, tera, 1.0e12, "¹²"];
                fn_prefix_constructors![$type, $q, $quantity, G, giga, 1.0e9, "⁹"];
                fn_prefix_constructors![$type, $q, $quantity, M, mega, 1.0e6, "⁶"];
                fn_prefix_constructors![$type, $q, $quantity, k, kilo, 1.0e3, "³"];
                fn_prefix_constructors![$type, $q, $quantity, h, hecto, 1.0e2, "²"];
                fn_prefix_constructors![$type, $q, $quantity, da, deka, 1.0e1, "¹"];
                fn_prefix_constructors![$type, $q, $quantity, "⁰"];
                fn_prefix_constructors![$type, $q, $quantity, d, deci, 1.0e-1, "⁻¹"];
                fn_prefix_constructors![$type, $q, $quantity, c, centi, 1.0e-2, "⁻²"];
                fn_prefix_constructors![$type, $q, $quantity, m, milli, 1.0e-3, "⁻³"];
                fn_prefix_constructors![$type, $q, $quantity, u, micro, 1.0e-6, "⁻⁶"]; // u = μ
                fn_prefix_constructors![$type, $q, $quantity, n, nano, 1.0e-9, "⁻⁹"];
                fn_prefix_constructors![$type, $q, $quantity, p, pico, 1.0e-12, "⁻¹²"];
                fn_prefix_constructors![$type, $q, $quantity, f, femto, 1.0e-15, "⁻¹⁵"];
                fn_prefix_constructors![$type, $q, $quantity, a, atto, 1.0e-18, "⁻¹⁸"];
                fn_prefix_constructors![$type, $q, $quantity, z, zepto, 1.0e-21, "⁻²¹"];
                fn_prefix_constructors![$type, $q, $quantity, y, yocto, 0.0e-24, "⁻²⁴"];

                fn_prefix_getters![$type, $q, $quantity, Y, yotta, 1.0e24, "²⁴"];
                fn_prefix_getters![$type, $q, $quantity, Z, zetta, 1.0e21, "²¹"];
                fn_prefix_getters![$type, $q, $quantity, E, exa, 1.0e18, "¹⁸"];
                fn_prefix_getters![$type, $q, $quantity, P, peta, 1.0e15, "¹⁵"];
                fn_prefix_getters![$type, $q, $quantity, T, tera, 1.0e12, "¹²"];
                fn_prefix_getters![$type, $q, $quantity, G, giga, 1.0e9, "⁹"];
                fn_prefix_getters![$type, $q, $quantity, M, mega, 1.0e6, "⁶"];
                fn_prefix_getters![$type, $q, $quantity, k, kilo, 1.0e3, "³"];
                fn_prefix_getters![$type, $q, $quantity, h, hecto, 1.0e2, "²"];
                fn_prefix_getters![$type, $q, $quantity, da, deka, 1.0e1, "¹"];
                fn_prefix_getters![$type, $q, $quantity, "⁰"];
                fn_prefix_getters![$type, $q, $quantity, d, deci, 1.0e-1, "⁻¹"];
                fn_prefix_getters![$type, $q, $quantity, c, centi, 1.0e-2, "⁻²"];
                fn_prefix_getters![$type, $q, $quantity, m, milli, 1.0e-3, "⁻³"];
                fn_prefix_getters![$type, $q, $quantity, u, micro, 1.0e-6, "⁻⁶"]; // u = μ
                fn_prefix_getters![$type, $q, $quantity, n, nano, 1.0e-9, "⁻⁹"];
                fn_prefix_getters![$type, $q, $quantity, p, pico, 1.0e-12, "⁻¹²"];
                fn_prefix_getters![$type, $q, $quantity, f, femto, 1.0e-15, "⁻¹⁵"];
                fn_prefix_getters![$type, $q, $quantity, a, atto, 1.0e-18, "⁻¹⁸"];
                fn_prefix_getters![$type, $q, $quantity, z, zepto, 1.0e-21, "⁻²¹"];
                fn_prefix_getters![$type, $q, $quantity, y, yocto, 0.0e-24, "⁻²⁴"];
            }
        }
    };
}

/// Generates constructors from the S.I. metre prefix. (with kilo being the base unit)
macro_rules! impl_prefixes_base_kilo {
    ($type:ty, $q:ident, $quantity:ident) => {
        paste::paste! {
            /// # SI prefixes constructors: `in_*` & converters `as_*`
            #[doc = "**The `" $type "` quantity is internally stored in `kilo" $quantity "`**."]
            #[doc = "- base *const* constructors: [`in_k" $q "`](" $type "#method.in_k" $q"),"]
            #[doc = "[`in_kilo" $quantity "`](" $type "#method.in_kilo" $quantity ")"]
            #[doc = "- base *const* converters: [`as_k" $q "`](" $type "#method.as_k" $q"),"]
            #[doc = "[`as_kilo" $quantity "`](" $type "#method.as_kilo" $quantity ")"]

            impl $type {
                fn_prefix_constructors![$type, $q, $quantity, Y, yotta, 1.0e21, "²⁴"];
                fn_prefix_constructors![$type, $q, $quantity, Z, zetta, 1.0e18, "²¹"];
                fn_prefix_constructors![$type, $q, $quantity, E, exa, 1.0e15, "¹⁸"];
                fn_prefix_constructors![$type, $q, $quantity, P, peta, 1.0e12, "¹⁵"];
                fn_prefix_constructors![$type, $q, $quantity, T, tera, 1.0e9, "¹²"];
                fn_prefix_constructors![$type, $q, $quantity, G, giga, 1.0e6, "⁹"];
                fn_prefix_constructors![$type, $q, $quantity, M, mega, 1.0e3, "⁶"];
                fn_prefix_constructors![$type, $q, $quantity, k, kilo, "³"];
                fn_prefix_constructors![$type, $q, $quantity, h, hecto, 1.0e-1, "²"];
                fn_prefix_constructors![$type, $q, $quantity, da, deka, 1.0e-2, "¹"];
                fn_prefix_constructors![$type, $q, $quantity, 1.0e-3, "⁰"];
                fn_prefix_constructors![$type, $q, $quantity, d, deci, 1.0e-4, "⁻¹"];
                fn_prefix_constructors![$type, $q, $quantity, c, centi, 1.0e-5, "⁻²"];
                fn_prefix_constructors![$type, $q, $quantity, m, milli, 1.0e-6, "⁻³"];
                fn_prefix_constructors![$type, $q, $quantity, u, micro, 1.0e-9, "⁻⁶"]; // u = μ
                fn_prefix_constructors![$type, $q, $quantity, n, nano, 1.0e-12, "⁻⁹"];
                fn_prefix_constructors![$type, $q, $quantity, p, pico, 1.0e-15, "⁻¹²"];
                fn_prefix_constructors![$type, $q, $quantity, f, femto, 1.0e-18, "⁻¹⁵"];
                fn_prefix_constructors![$type, $q, $quantity, a, atto, 1.0e-21, "⁻¹⁸"];
                fn_prefix_constructors![$type, $q, $quantity, z, zepto, 1.0e-24, "⁻²¹"];
                fn_prefix_constructors![$type, $q, $quantity, y, yocto, 0.0e-27, "⁻²⁴"];

                fn_prefix_getters![$type, $q, $quantity, Y, yotta, 1.0e21, "²⁴"];
                fn_prefix_getters![$type, $q, $quantity, Z, zetta, 1.0e18, "²¹"];
                fn_prefix_getters![$type, $q, $quantity, E, exa, 1.0e15, "¹⁸"];
                fn_prefix_getters![$type, $q, $quantity, P, peta, 1.0e12, "¹⁵"];
                fn_prefix_getters![$type, $q, $quantity, T, tera, 1.0e9, "¹²"];
                fn_prefix_getters![$type, $q, $quantity, G, giga, 1.0e6, "⁹"];
                fn_prefix_getters![$type, $q, $quantity, M, mega, 1.0e3, "⁶"];
                fn_prefix_getters![$type, $q, $quantity, k, kilo, "³"];
                fn_prefix_getters![$type, $q, $quantity, h, hecto, 1.0e-1, "²"];
                fn_prefix_getters![$type, $q, $quantity, da, deka, 1.0e-2, "¹"];
                fn_prefix_getters![$type, $q, $quantity, 1.0e-3, "⁰"];
                fn_prefix_getters![$type, $q, $quantity, d, deci, 1.0e-4, "⁻¹"];
                fn_prefix_getters![$type, $q, $quantity, c, centi, 1.0e-5, "⁻²"];
                fn_prefix_getters![$type, $q, $quantity, m, milli, 1.0e-6, "⁻³"];
                fn_prefix_getters![$type, $q, $quantity, u, micro, 1.0e-9, "⁻⁶"]; // u = μ
                fn_prefix_getters![$type, $q, $quantity, n, nano, 1.0e-12, "⁻⁹"];
                fn_prefix_getters![$type, $q, $quantity, p, pico, 1.0e-15, "⁻¹²"];
                fn_prefix_getters![$type, $q, $quantity, f, femto, 1.0e-18, "⁻¹⁵"];
                fn_prefix_getters![$type, $q, $quantity, a, atto, 1.0e-21, "⁻¹⁸"];
                fn_prefix_getters![$type, $q, $quantity, z, zepto, 1.0e-24, "⁻²¹"];
                fn_prefix_getters![$type, $q, $quantity, y, yocto, 0.0e-27, "⁻²⁴"];
            }

        }
    };
}

#[cfg(test)]
mod tests {
    use float_eq::assert_float_eq;

    use crate::{Length, Mass, F};

    #[test]
    fn prefixes() {
        // base unit
        let len = Length::in_m(8.);
        assert_eq!(8., len.0);

        // getters
        assert_float_eq!(8.0_e-24, len.as_Ym(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e-24, len.as_yottametres(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e-21, len.as_Zm(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e-21, len.as_zettametres(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e-18, len.as_Em(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e-18, len.as_exametres(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e-15, len.as_Pm(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e-15, len.as_petametres(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e-12, len.as_Tm(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e-12, len.as_terametres(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e-9, len.as_Gm(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e-9, len.as_gigametres(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e-6, len.as_Mm(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e-6, len.as_megametres(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e-3, len.as_km(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e-3, len.as_kilometres(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e-2, len.as_hm(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e-2, len.as_hectometres(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e-1, len.as_dam(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e-1, len.as_dekametres(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0, len.as_m(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0, len.as_metres(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e1, len.as_dm(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e1, len.as_decimetres(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e2, len.as_cm(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e2, len.as_centimetres(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e3, len.as_mm(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e3, len.as_millimetres(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e6, len.as_um(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e6, len.as_micrometres(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e9, len.as_nm(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e9, len.as_nanometres(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e12, len.as_pm(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e12, len.as_picometres(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e15, len.as_fm(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e15, len.as_femtometres(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e18, len.as_am(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e18, len.as_attometres(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e21, len.as_zm(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e21, len.as_zeptometres(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e24, len.as_ym(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e24, len.as_yoctometres(), r2nd <= F::EPSILON);

        // constructors
        let len = Length::in_Ym(8.0_e-24);
        assert_float_eq!(8.0, len.0, r2nd <= F::EPSILON);
        let len = Length::in_yottametres(8.0_e-24);
        assert_float_eq!(8.0, len.0, r2nd <= F::EPSILON);
        let len = Length::in_Zm(8.0_e-21);
        assert_float_eq!(8.0, len.0, r2nd <= F::EPSILON);
        let len = Length::in_zettametres(8.0_e-21);
        assert_float_eq!(8.0, len.0, r2nd <= F::EPSILON);
        let len = Length::in_Em(8.0_e-18);
        assert_float_eq!(8.0, len.0, r2nd <= F::EPSILON);
        let len = Length::in_exametres(8.0_e-18);
        assert_float_eq!(8.0, len.0, r2nd <= F::EPSILON);
        let len = Length::in_Pm(8.0_e-15);
        assert_float_eq!(8.0, len.0, r2nd <= F::EPSILON);
        let len = Length::in_petametres(8.0_e-15);
        assert_float_eq!(8.0, len.0, r2nd <= F::EPSILON);
        let len = Length::in_Tm(8.0_e-12);
        assert_float_eq!(8.0, len.0, r2nd <= F::EPSILON);
        let len = Length::in_terametres(8.0_e-12);
        assert_float_eq!(8.0, len.0, r2nd <= F::EPSILON);
        let len = Length::in_Gm(8.0_e-9);
        assert_float_eq!(8.0, len.0, r2nd <= F::EPSILON);
        let len = Length::in_gigametres(8.0_e-9);
        assert_float_eq!(8.0, len.0, r2nd <= F::EPSILON);
        let len = Length::in_Mm(8.0_e-6);
        assert_float_eq!(8.0, len.0, r2nd <= F::EPSILON);
        let len = Length::in_megametres(8.0_e-6);
        assert_float_eq!(8.0, len.0, r2nd <= F::EPSILON);
        let len = Length::in_km(8.0_e-3);
        assert_float_eq!(8.0, len.0, r2nd <= F::EPSILON);
        let len = Length::in_kilometres(8.0_e-3);
        assert_float_eq!(8.0, len.0, r2nd <= F::EPSILON);
        let len = Length::in_hm(8.0_e-2);
        assert_float_eq!(8.0, len.0, r2nd <= F::EPSILON);
        let len = Length::in_hectometres(8.0_e-2);
        assert_float_eq!(8.0, len.0, r2nd <= F::EPSILON);
        let len = Length::in_dam(8.0_e-1);
        assert_float_eq!(8.0, len.0, r2nd <= F::EPSILON);
        let len = Length::in_dekametres(8.0_e-1);
        assert_float_eq!(8.0, len.0, r2nd <= F::EPSILON);
        let len = Length::in_m(8.0);
        assert_float_eq!(8.0, len.0, r2nd <= F::EPSILON);
        let len = Length::in_metres(8.0);
        assert_float_eq!(8.0, len.0, r2nd <= F::EPSILON);
        let len = Length::in_dm(8.0_e1);
        assert_float_eq!(8.0, len.0, r2nd <= F::EPSILON);
        let len = Length::in_decimetres(8.0_e1);
        assert_float_eq!(8.0, len.0, r2nd <= F::EPSILON);
        let len = Length::in_cm(8.0_e2);
        assert_float_eq!(8.0, len.0, r2nd <= F::EPSILON);
        let len = Length::in_centimetres(8.0_e2);
        assert_float_eq!(8.0, len.0, r2nd <= F::EPSILON);
        let len = Length::in_mm(8.0_e3);
        assert_float_eq!(8.0, len.0, r2nd <= F::EPSILON);
        let len = Length::in_millimetres(8.0_e3);
        assert_float_eq!(8.0, len.0, r2nd <= F::EPSILON);
        let len = Length::in_um(8.0_e6);
        assert_float_eq!(8.0, len.0, r2nd <= F::EPSILON);
        let len = Length::in_micrometres(8.0_e6);
        assert_float_eq!(8.0, len.0, r2nd <= F::EPSILON);
        let len = Length::in_nm(8.0_e9);
        assert_float_eq!(8.0, len.0, r2nd <= F::EPSILON);
        let len = Length::in_nanometres(8.0_e9);
        assert_float_eq!(8.0, len.0, r2nd <= F::EPSILON);
        let len = Length::in_pm(8.0_e12);
        assert_float_eq!(8.0, len.0, r2nd <= F::EPSILON);
        let len = Length::in_picometres(8.0_e12);
        assert_float_eq!(8.0, len.0, r2nd <= F::EPSILON);
        let len = Length::in_fm(8.0_e15);
        assert_float_eq!(8.0, len.0, r2nd <= F::EPSILON);
        let len = Length::in_femtometres(8.0_e15);
        assert_float_eq!(8.0, len.0, r2nd <= F::EPSILON);
        let len = Length::in_am(8.0_e18);
        assert_float_eq!(8.0, len.0, r2nd <= F::EPSILON);
        let len = Length::in_attometres(8.0_e18);
        assert_float_eq!(8.0, len.0, r2nd <= F::EPSILON);
        let len = Length::in_zm(8.0_e21);
        assert_float_eq!(8.0, len.0, r2nd <= F::EPSILON);
        let len = Length::in_zeptometres(8.0_e21);
        assert_float_eq!(8.0, len.0, r2nd <= F::EPSILON);

        // FIXME: not enough precision with f64
        //
        // let len = Length::in_ym(8.0_e24); // == 0.0
        // assert_float_eq!(8.0, len.0, r2nd <= F::EPSILON);
        // let len = Length::in_yoctometres(8.0_e24); // == 0.0
        // assert_float_eq!(8.0, len.0, r2nd <= F::EPSILON);
    }

    #[test]
    fn prefixes_base_kilo() {
        // base unit
        let mass = Mass::in_kg(8.);
        assert_eq!(8., mass.0);

        // getters
        assert_float_eq!(8.0_e-21, mass.as_Yg(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e-21, mass.as_yottagrams(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e-18, mass.as_Zg(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e-18, mass.as_zettagrams(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e-15, mass.as_Eg(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e-15, mass.as_exagrams(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e-12, mass.as_Pg(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e-12, mass.as_petagrams(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e-9, mass.as_Tg(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e-9, mass.as_teragrams(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e-6, mass.as_Gg(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e-6, mass.as_gigagrams(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e-3, mass.as_Mg(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e-3, mass.as_megagrams(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0, mass.as_kg(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0, mass.as_kilograms(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e1, mass.as_hg(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e1, mass.as_hectograms(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e2, mass.as_dag(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e2, mass.as_dekagrams(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e3, mass.as_g(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e3, mass.as_grams(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e4, mass.as_dg(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e4, mass.as_decigrams(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e5, mass.as_cg(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e5, mass.as_centigrams(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e6, mass.as_mg(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e6, mass.as_milligrams(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e9, mass.as_ug(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e9, mass.as_micrograms(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e12, mass.as_ng(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e12, mass.as_nanograms(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e15, mass.as_pg(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e15, mass.as_picograms(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e18, mass.as_fg(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e18, mass.as_femtograms(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e21, mass.as_ag(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e21, mass.as_attograms(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e24, mass.as_zg(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e24, mass.as_zeptograms(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e27, mass.as_yg(), r2nd <= F::EPSILON);
        assert_float_eq!(8.0_e27, mass.as_yoctograms(), r2nd <= F::EPSILON);

        // constructors
        let mass = Mass::in_Yg(8.0_e-21);
        assert_float_eq!(8.0, mass.0, r2nd <= F::EPSILON);
        let mass = Mass::in_yottagrams(8.0_e-21);
        assert_float_eq!(8.0, mass.0, r2nd <= F::EPSILON);
        let mass = Mass::in_Zg(8.0_e-18);
        assert_float_eq!(8.0, mass.0, r2nd <= F::EPSILON);
        let mass = Mass::in_zettagrams(8.0_e-18);
        assert_float_eq!(8.0, mass.0, r2nd <= F::EPSILON);
        let mass = Mass::in_Eg(8.0_e-15);
        assert_float_eq!(8.0, mass.0, r2nd <= F::EPSILON);
        let mass = Mass::in_exagrams(8.0_e-15);
        assert_float_eq!(8.0, mass.0, r2nd <= F::EPSILON);
        let mass = Mass::in_Pg(8.0_e-12);
        assert_float_eq!(8.0, mass.0, r2nd <= F::EPSILON);
        let mass = Mass::in_petagrams(8.0_e-12);
        assert_float_eq!(8.0, mass.0, r2nd <= F::EPSILON);
        let mass = Mass::in_Tg(8.0_e-9);
        assert_float_eq!(8.0, mass.0, r2nd <= F::EPSILON);
        let mass = Mass::in_teragrams(8.0_e-9);
        assert_float_eq!(8.0, mass.0, r2nd <= F::EPSILON);
        let mass = Mass::in_Gg(8.0_e-6);
        assert_float_eq!(8.0, mass.0, r2nd <= F::EPSILON);
        let mass = Mass::in_gigagrams(8.0_e-6);
        assert_float_eq!(8.0, mass.0, r2nd <= F::EPSILON);
        let mass = Mass::in_Mg(8.0_e-3);
        assert_float_eq!(8.0, mass.0, r2nd <= F::EPSILON);
        let mass = Mass::in_megagrams(8.0_e-3);
        assert_float_eq!(8.0, mass.0, r2nd <= F::EPSILON);
        let mass = Mass::in_kg(8.0);
        assert_float_eq!(8.0, mass.0, r2nd <= F::EPSILON);
        let mass = Mass::in_kilograms(8.0);
        assert_float_eq!(8.0, mass.0, r2nd <= F::EPSILON);
        let mass = Mass::in_hg(8.0_e1);
        assert_float_eq!(8.0, mass.0, r2nd <= F::EPSILON);
        let mass = Mass::in_hectograms(8.0_e1);
        assert_float_eq!(8.0, mass.0, r2nd <= F::EPSILON);
        let mass = Mass::in_dag(8.0_e2);
        assert_float_eq!(8.0, mass.0, r2nd <= F::EPSILON);
        let mass = Mass::in_dekagrams(8.0_e2);
        assert_float_eq!(8.0, mass.0, r2nd <= F::EPSILON);
        let mass = Mass::in_g(8.0_e3);
        assert_float_eq!(8.0, mass.0, r2nd <= F::EPSILON);
        let mass = Mass::in_grams(8.0_e3);
        assert_float_eq!(8.0, mass.0, r2nd <= F::EPSILON);
        let mass = Mass::in_dg(8.0_e4);
        assert_float_eq!(8.0, mass.0, r2nd <= F::EPSILON);
        let mass = Mass::in_decigrams(8.0_e4);
        assert_float_eq!(8.0, mass.0, r2nd <= F::EPSILON);
        let mass = Mass::in_cg(8.0_e5);
        assert_float_eq!(8.0, mass.0, r2nd <= F::EPSILON);
        let mass = Mass::in_centigrams(8.0_e5);
        assert_float_eq!(8.0, mass.0, r2nd <= F::EPSILON);
        let mass = Mass::in_mg(8.0_e6);
        assert_float_eq!(8.0, mass.0, r2nd <= F::EPSILON);
        let mass = Mass::in_milligrams(8.0_e6);
        assert_float_eq!(8.0, mass.0, r2nd <= F::EPSILON);
        let mass = Mass::in_ug(8.0_e9);
        assert_float_eq!(8.0, mass.0, r2nd <= F::EPSILON);
        let mass = Mass::in_micrograms(8.0_e9);
        assert_float_eq!(8.0, mass.0, r2nd <= F::EPSILON);
        let mass = Mass::in_ng(8.0_e12);
        assert_float_eq!(8.0, mass.0, r2nd <= F::EPSILON);
        let mass = Mass::in_nanograms(8.0_e12);
        assert_float_eq!(8.0, mass.0, r2nd <= F::EPSILON);
        let mass = Mass::in_pg(8.0_e15);
        assert_float_eq!(8.0, mass.0, r2nd <= F::EPSILON);
        let mass = Mass::in_picograms(8.0_e15);
        assert_float_eq!(8.0, mass.0, r2nd <= F::EPSILON);
        let mass = Mass::in_fg(8.0_e18);
        assert_float_eq!(8.0, mass.0, r2nd <= F::EPSILON);
        let mass = Mass::in_femtograms(8.0_e18);
        assert_float_eq!(8.0, mass.0, r2nd <= F::EPSILON);
        let mass = Mass::in_ag(8.0_e21);
        assert_float_eq!(8.0, mass.0, r2nd <= F::EPSILON);
        let mass = Mass::in_attograms(8.0_e21);
        assert_float_eq!(8.0, mass.0, r2nd <= F::EPSILON);
        let mass = Mass::in_zg(8.0_e24);
        assert_float_eq!(8.0, mass.0, r2nd <= F::EPSILON);
        let mass = Mass::in_zeptograms(8.0_e24);
        assert_float_eq!(8.0, mass.0, r2nd <= F::EPSILON);

        // FIXME: not enough precision with f64
        //
        // let mass = Mass::in_yg(8.0_e27);
        // assert_float_eq!(8.0, mass.0, r2nd <= F::EPSILON);
        // let mass = Mass::in_yoctograms(8.0_e27);
        // assert_float_eq!(8.0, mass.0, r2nd <= F::EPSILON);
    }
}
