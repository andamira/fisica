//! SI prefixes

// TODO WIP:
// 1. support compound prefixes, for Speed, Momentum, etc.
//   - all combinations? so either of the 2 units could be scaled?
//      21×21=441 methods!! maybe hide them from the docs?
//   - there can be a single factor argument
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
    // $bu = base unit for conversion, in unicode
    //
    // NOTE:
    // - prefixes can be an empty string if you don't need them
    // - The base unit of reference is usually the quantity in unicode
    [$ty:ty, qa=$q:ident, Qa=$Q:ident, qu=$qu:tt, Qu=$Qu:tt,
     pu=$pu:tt, Pu=$Pu:tt, pa=$pa:tt, Pa=$Pa:tt, f=$f:expr, fu=$fu:expr, bu=$bu:tt] => {
        paste::paste! {
            // constructors
            #[inline]
            #[allow(non_snake_case)]
            #[doc = "New `" $ty "` in " $Pu "" $Qu " (" $pu "" $qu ") (" $fu " " $bu ")."]
            pub fn [<in_$pa $q>](m: crate::Magnitude) -> Self { Self::new(m * $f) }
            #[inline]
            #[doc = "New `" $ty "` in " $Pu "" $Qu " (" $pu "" $qu ") (" $fu " " $bu ")."]
            pub fn [<in_$Pa $Q>](m: crate::Magnitude) -> Self { Self::new(m * $f) }
            // getters
            #[inline]
            #[allow(non_snake_case)]
            #[doc = "Returns `" $ty "` as " $Pu "" $Qu " (" $pu "" $qu ") (" $fu " " $bu ")."]
            pub fn [<as_$pa $q>](&self) -> crate::Magnitude { self.m / $f }
            #[inline]
            #[doc = "Returns `" $ty "` as " $Pu "" $Qu " (" $pu "" $qu ") (" $fu " " $bu ")."]
            pub fn [<as_$Pa $Q>](&self) -> crate::Magnitude { self.m / $f }
        }
    };
    // ALIAS: no need to specify: qu, Qu, bu
    // useful for: all non-base units, with unicode prefix
    // e.g.:
    // scalar_methods![$ty, $q, $Q, pu="µ", Pu="micro", pa="u", Pa="micro", f=1e-6, fu="10⁻⁶"];
    ($ty:ty, qa=$q:ident, Qa=$Q:ident,
     pu=$pu:tt, Pu=$Pu:tt, pa=$pa:tt, Pa=$Pa:tt, f=$f:expr, fu=$fu:expr) => {
        scalar_methods![
            $ty,
            qa=$q,
            Qa=$Q,
            qu = $q,
            Qu = $Q,
            pu = $pa,
            Pu = $Pa,
            pa = $pa,
            Pa = $Pa,
            f = $f,
            fu = $fu,
            bu = $q
        ];
    };
    // ALIAS: no need to specify: pu, Pu, bu
    // useful for: squared/cubed non-base units, with ascii prefix
    // e.g.:
    // scalar_methods![$ty, $q, $Q, qu="m²", pa="T", Pa="tera", f=1.0e-3, fu="10⁻³"];
    ($ty:ty, qa=$q:ident, Qa=$Q:ident, qu=$qu:tt, Qu=$Qu:tt,
     pa=$pa:tt, Pa=$Pa:tt, f=$f:expr, fu=$fu:expr) => {
        scalar_methods![
            $ty,
            qa=$q,
            Qa=$Q,
            qu = $qu,
            Qu = $Qu,
            pu = $pa,
            Pu = $Pa,
            pa = $pa,
            Pa = $Pa,
            f = $f,
            fu = $fu,
            bu = $qu
        ];
    };
    // ALIAS: no need to specify: qu, Qu, pu, Pu, bu
    // useful for: all non-base units, with ascii prefix (doesn't support µ, micro)
    // e.g.:
    // scalar_methods![$ty, $q, $Q, "m", "milli", 1.0e-3, "10⁻³"];
    ($ty:ty, qa=$q:ident, Qa=$Q:ident,
     pa=$pa:tt, Pa=$Pa:tt, f=$f:expr, fu=$fu:expr) => {
        scalar_methods![
            $ty,
            qa=$q,
            Qa=$Q,
            qu = $q,
            Qu = $Q,
            pu = $pa,
            Pu = $Pa,
            pa = $pa,
            Pa = $Pa,
            f = $f,
            fu = $fu,
            bu = $q
        ];
    };
    // ALIAS: no need to specify: pu, Pu, pa, Pa
    // useful for: units WITHOUT prefix and unicode quantity (like Å, ångströms)
    // e.g.:
    // scalar_methods![ Length, A, angstroms, "`Å`", "`ångströms`",
    //   metres, 1.0e10, "10⁻¹⁰" ];
    ($ty:ty, qa=$q:ident, Qa=$Q:ident, qu=$qu:tt, Qu=$Qu:tt, f=$f:expr, fu=$fu:expr,
     bu=$bu:tt) => {
        scalar_methods![
            $ty,
            qa=$q,
            Qa=$Q,
            qu = $qu,
            Qu = $Qu,
            pu = "",
            Pu = "",
            pa = "",
            Pa = "",
            f = $f,
            fu = $fu,
            bu = $bu
        ];
    };
    // ALIAS: no need to specify: qu, Qu, pa, Pa, pu, Pu
    // e.g.:
    // scalar_methods![Time, y, years, f=60., fu="365", bu="days"];
    ($ty:ty, qa=$q:ident, Qa=$Q:ident, f=$f:expr, fu=$fu:tt, bu=$bu:tt) => {
        scalar_methods![
            $ty,
            qa=$q,
            Qa=$Q,
            qu = $q,
            Qu = $Q,
            pu = "",
            Pu = "",
            pa = "",
            Pa = "",
            f = $f,
            fu = $fu,
            bu = $bu
        ];
    };
    // ALIAS: no need to specify: qu, Qu, pa, Pa, pu, Pu, bu
    // useful for units WITHOUT prefixes, WITH conversion factor (grams)
    // e.g.:
    // scalar_methods![$ty, $q, $Q, f=1e-3, fu="10⁰"];
    ($ty:ty, qa=$q:ident, Qa=$Q:ident, f=$f:expr, fu=$fu:expr) => {
        scalar_methods![
            $ty,
            qa=$q,
            Qa=$Q,
            qu = $q,
            Qu = $Q,
            pu = "",
            Pu = "",
            pa = "",
            Pa = "",
            f = $f,
            fu = $fu,
            bu = $q
        ];
    };

    // ROOT RULE: const base unit, WITHOUT conversion factor
    [$ty:ty, base, qa=$q:ident, Qa=$Q:ident, qu=$qu:tt, Qu=$Qu:tt, pa=$pa:tt, Pa=$Pa:tt, fu=$fu:expr] => {
        paste::paste! {
            // constructors
            #[inline]
            #[allow(non_snake_case)]
            #[doc = "New `" $ty "` in " $Pa $Qu " (" $pa $qu ") (base unit, " $fu " " $qu ")."]
            pub const fn [<in_$pa $q>](m: crate::Magnitude) -> Self { Self::new(m) }
            #[inline]
            #[doc = "New `" $ty "` in " $Pa $Qu " (" $pa $qu ") (base unit, " $fu " " $qu ")."]
            pub const fn [<in_$Pa $Q>](m: crate::Magnitude) -> Self { Self::new(m) }
            // getters
            #[inline]
            #[allow(non_snake_case)]
            #[doc = "Returns `" $ty "` as " $Pa $Qu " (" $pa $qu ") (base unit, " $fu " " $qu ")."]
            pub const fn [<as_$pa $q>](&self) -> crate::Magnitude { self.m }
            #[inline]
            #[doc = "Returns `" $ty "` as " $Pa $Qu " (" $pa $qu ") (base unit, " $fu " " $qu ")."]
            pub const fn [<as_$Pa $Q>](&self) -> crate::Magnitude { self.m }
        }
    };
    // ALIAS: no need to specify: qu, Qu
    // useful for: the kilogram SI base unit
    // e.g.:
    // scalar_methods![$ty, base, $q, $Q, pa="k", Pa="kilo", fu="10³"];
    ($ty:ty, base, qa=$q:ident, Qa=$Q:ident, pa=$pa:tt, Pa=$Pa:tt, fu=$fu:expr) => {
        scalar_methods![
            $ty,
            base,
            qa=$q,
            Qa=$Q,
            qu = $q,
            Qu = $Q,
            pa = $pa,
            Pa = $Pa,
            fu = $fu
        ];
    };
    // ALIAS: no need to specify: pa, Pa
    // useful for: squared and cubed base units
    // e.g.:
    // scalar_methods![$ty, base, $q, $Q, qu="m²", Qu="metres squared", fu="10⁰"];
    ($ty:ty, base, qa=$q:ident, Qa=$Q:ident, qu=$qu:tt, Qu=$Qu:tt, fu=$fu:expr) => {
        scalar_methods![
            $ty,
            base,
            qa=$q,
            Qa=$Q,
            qu = $qu,
            Qu = $Qu,
            pa = "",
            Pa = "",
            fu = $fu
        ];
    };
    // ALIAS: no need to specify: qu, Qu, pa, Pa
    // useful for: base SI units except the kilogram (metre, kelvin, pascal…)
    // e.g.:
    // scalar_methods![$ty, base, $q, $Q, fu="10⁰"];
    ($ty:ty, base, qa=$q:ident, Qa=$Q:ident, fu=$fu:expr) => {
        scalar_methods![
            $ty,
            base,
            qa=$q,
            Qa=$Q,
            qu = $q,
            Qu = $Q,
            pa = "",
            Pa = "",
            fu = $fu
        ];
    };
}

/// Generates 2 constructors and 2 getters, for scalar quantities with 2 units.
macro_rules! scalar_methods_2units {
    // ROOT RULE: NON const, WITH conversion factor
    //
    // LEGEND:
    // $ty = the type to impl the metods
    // $q1a  = abbreviated 1st quantity in ascii
    // $q2a  = abbreviated 2nd quantity in ascii
    // $Q1a  = 1st quantity in ascii
    // $Q2a  = 2nd quantity in ascii
    // $q1u = abbreviated 1st quantity in unicode
    // $q2u = abbreviated 2nd quantity in unicode
    // $Q1u = 1st quantity in unicode (or multiple words)
    // $Q2u = 2nd quantity in unicode (or multiple words)
    // $p1a = abbreviated 1st prefix in ascii
    // $p2a = abbreviated 2nd prefix in ascii
    // $P1a = 1st prefix in ascii
    // $P2a = 2nd prefix in ascii
    // $p1u = abbreviated 1st prefix in unicode
    // $p2u = abbreviated 2nd prefix in unicode
    // $P1u = 1st prefix in unicode
    // $P2u = 2nd prefix in unicode
    // $f  = conversion factor in number
    // $fu = conversion factor in unicode
    // $b1u = first base unit for conversion, in unicode
    // $b2u = second base unit for conversion, in unicode
    [$ty:ty, q1a=$q1a:ident, q2a=$q2a:ident, Q1a=$Q1a:ident, Q2a=$Q2a:ident, Ja=$Ja:ident,
     q1u=$q1u:tt, q2u=$q2u:tt, Q1u=$Q1u:tt, Q2u=$Q2u:tt,
     p1a=$p1a:tt, p2a=$p2a:tt, P1a=$P1a:tt, P2a=$P2a:tt,
     p1u=$p1u:tt, p2u=$p2u:tt, P1u=$P1u:tt, P2u=$P2u:tt,
     f=$f:expr, fu=$fu:expr, b1u=$b1u:tt, b2u=$b2u:tt] => {
        paste::paste! {
            // constructors
            #[inline]
            #[allow(non_snake_case)]
            #[doc = "New `" $ty "` in " $P1u $Q1u " " $Ja " " $P2u $Q2u " (" $p1u $q1u "/" $p2u $q2u
                ") (" $fu " " $b1u "/" $b2u ")." ]
            pub fn [<in_$p1a $q1a _$p2a $q2a>](m: crate::Magnitude) -> Self { Self::new(m * $f) }
            #[inline]
            #[doc = "New `" $ty "` in " $P1u $Q1u " " $Ja " " $P2u $Q2u " (" $p1u $q1u "/" $p2u $q2u ")." ]
            pub fn [<in_$P1a $Q1a _$P2a $Q2a>](m: crate::Magnitude) -> Self { Self::new(m * $f) }
            // // getters
            #[allow(non_snake_case)]
            #[doc = "Returns `" $ty "` as " $P1u $Q1u " " $Ja " " $P2u $Q2u " (" $p1u $q1u "/" $p2u $q2u
                ") (" $fu " " $b1u "/" $b2u ")." ]
            pub fn [<as_$p1a $q1a _$p2a $q2a>](&self) -> crate::Magnitude { self.m / $f }
            #[inline]
            #[doc = "Returns `" $ty "` as " $P1u $Q1u " " $Ja " " $P2u $Q2u " (" $p1u $q1u "/" $p2u $q2u
                ") (" $fu " " $b1u "/" $b2u ")." ]
            pub fn [<as_$P1a $Q1a _$P2a $Q2a>](&self) -> crate::Magnitude { self.m / $f }
        }
    };
    // ALIAS: no need to specify: q1u, q2u, Q1u, Q2u, p1u, p2u, P1u, P2u
    //
    // e.g.:
    // scalar_methods_2units![$ty, q1a=$q1a, q2a=$q2a, Q1a=$Q1a, Q2a=$Q2a, Ja=$Ja,
    //   p1a="T", p2a="", P1a="tera", P2a="", f=1e12, fu="10¹²"];
    ($ty:ty, q1a=$q1a:ident, q2a=$q2a:ident, Q1a=$Q1a:ident, Q2a=$Q2a:ident, Ja=$Ja:ident,
     p1a=$p1a:tt, p2a=$p2a:tt, P1a=$P1a:tt, P2a=$P2a:tt, f=$f:expr, fu=$fu:expr,
     b1u=$b1u:tt, b2u=$b2u:tt) => {
        scalar_methods_2units![
            $ty,
            q1a=$q1a,
            q2a=$q2a,
            Q1a=$Q1a,
            Q2a=$Q2a,
            Ja=$Ja,
            q1u=$q1a,
            q2u=$q2a,
            Q1u=$Q1a,
            Q2u=$Q2a,
            p1a=$p1a,
            p2a=$p2a,
            P1a=$P1a,
            P2a=$P2a,
            p1u=$p1a,
            p2u=$p2a,
            P1u=$P1a,
            P2u=$P2a,
            f=$f,
            fu=$fu,
            b1u=$b1u,
            b2u=$b2u
        ];
    };
    // ALIAS: no need to specify: q1u, q2u, Q1u, Q2u
    //
    // e.g.:
    // scalar_methods_2units![$ty, q1a=$q1a, q2a=$q2a, Q1a=$Q1a, Q2a=$Q2a, Ja=$Ja,
    //     p1a="u", p2a="", P1a="micro", P2a="", p1u="µ", p2u="", P1u="micro", P2u="",
    //     f=1e-6, fu="10⁻⁶", b1u=$q1a, b2u=$q2a];
    ($ty:ty, q1a=$q1a:ident, q2a=$q2a:ident, Q1a=$Q1a:ident, Q2a=$Q2a:ident, Ja=$Ja:ident,
     p1a=$p1a:tt, p2a=$p2a:tt, P1a=$P1a:tt, P2a=$P2a:tt,
     p1u=$p1u:tt, p2u=$p2u:tt, P1u=$P1u:tt, P2u=$P2u:tt,
     f=$f:expr, fu=$fu:expr, b1u=$b1u:tt, b2u=$b2u:tt) => {
        scalar_methods_2units![
            $ty,
            q1a=$q1a,
            q2a=$q2a,
            Q1a=$Q1a,
            Q2a=$Q2a,
            Ja=$Ja,
            q1u=$q1a,
            q2u=$q2a,
            Q1u=$Q1a,
            Q2u=$Q2a,
            p1a=$p1a,
            p2a=$p2a,
            P1a=$P1a,
            P2a=$P2a,
            p1u=$p1u,
            p2u=$p2u,
            P1u=$P1u,
            P2u=$P2u,
            f=$f,
            fu=$fu,
            b1u=$b1u,
            b2u=$b2u
        ];
    };
    // ALIAS: no need to specify: p1u, p2u, P1u, P2u
    // useful for density normal units
    ($ty:ty, q1a=$q1a:ident, q2a=$q2a:ident, Q1a=$Q1a:ident, Q2a=$Q2a:ident, Ja=$Ja:ident,
     q1u=$q1u:tt, q2u=$q2u:tt, Q1u=$Q1u:tt, Q2u=$Q2u:tt,
     p1a=$p1a:tt, p2a=$p2a:tt, P1a=$P1a:tt, P2a=$P2a:tt,
     f=$f:expr, fu=$fu:expr, b1u=$b1u:tt, b2u=$b2u:tt) => {
        scalar_methods_2units![
            $ty,
            q1a=$q1a,
            q2a=$q2a,
            Q1a=$Q1a,
            Q2a=$Q2a,
            Ja=$Ja,
            q1u=$q1u,
            q2u=$q2u,
            Q1u=$Q1u,
            Q2u=$Q2u,
            p1a=$p1a,
            p2a=$p2a,
            P1a=$P1a,
            P2a=$P2a,
            p1u=$p1a,
            p2u=$p2a,
            P1u=$P1a,
            P2u=$P2a,
            f=$f,
            fu=$fu,
            b1u=$b1u,
            b2u=$b2u
        ];
    };

    // ROOT RULE: const base unit, WITHOUT conversion factor
    //
    [$ty:ty, base, q1a=$q1a:ident, q2a=$q2a:ident, Q1a=$Q1a:ident, Q2a=$Q2a:ident,
     Ja=$Ja:ident, q1u=$q1u:tt, q2u=$q2u:tt, Q1u=$Q1u:tt, Q2u=$Q2u:tt,
     p1a=$p1a:tt, p2a=$p2a:tt, P1a=$P1a:tt, P2a=$P2a:tt,
     p1u=$p1u:tt, p2u=$p2u:tt, P1u=$P1u:tt, P2u=$P2u:tt, fu=$fu:expr] => {
        paste::paste! {
            // constructors
            #[inline]
            #[allow(non_snake_case)]
            #[doc = "New `" $ty "` in " $P1u $Q1u " " $Ja " " $P2u $Q2u " (" $p1u $q1u "/" $p2u $q2u
                ") (base unit, " $fu " " $q1u "/" $q2u ")." ]
            pub const fn [<in_$p1a $q1a _$p2a $q2a>](m: crate::Magnitude) -> Self { Self::new(m) }
            #[inline]
            #[doc = "New `" $ty "` in " $P1u $Q1u " " $Ja " " $P2u $Q2u " (" $p1u $q1u "/" $p2u $q2u
                ") (base unit, " $fu " " $q1u "/" $q2u ")." ]
            pub const fn [<in_$P1a $Q1a _$P2a $Q2a>](m: crate::Magnitude) -> Self { Self::new(m) }
            // getters
            #[inline]
            #[allow(non_snake_case)]
            #[doc = "Returns `" $ty "` as " $P1u $Q1u " " $Ja " " $P2u $Q2u " (" $p1u $q1u "/" $p2u $q2u
                ") (base unit, " $fu " " $q1u "/" $q2u ")." ]
            pub const fn [<as_$p1a $q1a _$p2a $q2a>](&self) -> crate::Magnitude { self.m }
            #[inline]
            #[doc = "Returns `" $ty "` as " $P1u $Q1u " " $Ja " " $P2u $Q2u " (" $p1u $q1u "/" $p2u $q2u
                ") (base unit, " $fu " " $q1u "/" $q2u ")." ]
            pub const fn [<as_$P1a $Q1a _$P2a $Q2a>](&self) -> crate::Magnitude { self.m }
        }
    };
    // ALIAS: no need to specify: p1a, p2a, P1a, P2a, p1u, p2u, P1u, P2u, f
    // useful for: speed base unit
    ($ty:ty, base, q1a=$q1a:ident, q2a=$q2a:ident, Q1a=$Q1a:ident, Q2a=$Q2a:ident,
     Ja=$Ja:ident, fu=$fu:tt) => {
        scalar_methods_2units![
            $ty,
            base,
            q1a=$q1a,
            q2a=$q2a,
            Q1a=$Q1a,
            Q2a=$Q2a,
            Ja=$Ja,
            q1u=$q1a,
            q2u=$q2a,
            Q1u=$Q1a,
            Q2u=$Q2a,
            p1a="",
            p2a="",
            P1a="",
            P2a="",
            p1u="",
            p2u="",
            P1u="",
            P2u="",
            fu=$fu
        ];
    };
    // ALIAS: no need to specify: p1u, p2u, P1u, P2u, f
    // useful for: density base unit
    ($ty:ty, base, q1a=$q1a:ident, q2a=$q2a:ident, Q1a=$Q1a:ident, Q2a=$Q2a:ident,
     Ja=$Ja:ident, q1u=$q1u:tt, q2u=$q2u:tt, Q1u=$Q1u:tt, Q2u=$Q2u:tt,
     p1a=$p1a:tt, p2a=$p2a:tt, P1a=$P1a:tt, P2a=$P2a:tt,
     fu=$fu:expr) => {
        scalar_methods_2units![
            $ty,
            base,
            q1a=$q1a,
            q2a=$q2a,
            Q1a=$Q1a,
            Q2a=$Q2a,
            Ja=$Ja,
            q1u=$q1u,
            q2u=$q2u,
            Q1u=$Q1u,
            Q2u=$Q2u,
            p1a=$p1a,
            p2a=$p2a,
            P1a=$P1a,
            P2a=$P2a,
            p1u=$p1a,
            p2u=$p2a,
            P1u=$P1a,
            P2u=$P2a,
            fu=$fu
        ];
    };

}

/// Generates 2 constructors and 2 getters, for vector quantities.
macro_rules! vector_methods {
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
    // $bu = base unit for conversion, in unicode
    [$ty:ty, qa=$q:ident, Qa=$Q:ident, qu=$qu:tt, Qu=$Qu:tt,
     pu=$pu:tt, Pu=$Pu:tt, pa=$pa:tt, Pa=$Pa:tt, f=$f:expr, fu=$fu:expr, bu=$bu:tt] => {
        paste::paste! {
            // constructors
            #[inline]
            #[allow(non_snake_case)]
            #[doc = "New `" $ty "` in " $Pu "" $Qu " (" $pu "" $qu ") (" $fu " " $bu ")."]
            pub fn [<in_$pa $q>](m: crate::Magnitude, d: crate::Direction) -> Self { Self::new(m * $f, d) }
            #[inline]
            #[doc = "New `" $ty "` in " $Pu "" $Qu " (" $pu "" $qu ") (" $fu " " $bu ")."]
            pub fn [<in_$Pa $Q>](m: crate::Magnitude, d: crate::Direction) -> Self { Self::new(m * $f, d) }
            // getters
            #[inline]
            #[allow(non_snake_case)]
            #[doc = "Returns `" $ty "` as " $Pu "" $Qu " (" $pu "" $qu ") (" $fu " " $bu ")."]
            pub fn [<as_$pa $q>](&self) -> (crate::Magnitude, crate::Direction) { (self.m / $f, self.d) }
            #[inline]
            #[allow(non_snake_case)]
            #[doc = "Returns `" $ty "` as " $Pu "" $Qu " (" $pu "" $qu ") (" $fu " " $bu ")."]
            pub fn [<as_$Pa $Q>](&self) -> (crate::Magnitude, crate::Direction) { (self.m / $f, self.d) }
        }
    };
    // ALIAS: no need to specify: qu, Qu, bu
    // useful for: all non-base units, with unicode prefix
    // e.g.:
    // vector_methods![$ty, qa=$q, Qa=$Q, pu="µ", Pu="micro", pa="u", Pa="micro", f=1e-6, fu="10⁻⁶"];
    ($ty:ty, qa=$q:ident, Qa=$Q:ident,
     pu=$pu:tt, Pu=$Pu:tt, pa=$pa:tt, Pa=$Pa:tt, f=$f:expr, fu=$fu:expr) => {
        vector_methods![
            $ty,
            qa=$q,
            Qa=$Q,
            qu = $q,
            Qu = $Q,
            pu = $pa,
            Pu = $Pa,
            pa = $pa,
            Pa = $Pa,
            f = $f,
            fu = $fu,
            bu = $q
        ];
    };
    // ALIAS: no need to specify: pu, Pu, bu
    // useful for: squared/cubed non-base units, with ascii prefix
    // e.g.:
    // vector_methods![$ty, $q, $Q, qu="m²", pa="T", Pa="tera", f=1.0e-3, fu="10⁻³"];
    ($ty:ty, qa=$q:ident, Qa=$Q:ident, qu=$qu:tt, Qu=$Qu:tt,
     pa=$pa:tt, Pa=$Pa:tt, f=$f:expr, fu=$fu:expr) => {
        vector_methods![
            $ty,
            qa=$q,
            Qa=$Q,
            qu = $qu,
            Qu = $Qu,
            pu = $pa,
            Pu = $Pa,
            pa = $pa,
            Pa = $Pa,
            f = $f,
            fu = $fu,
            bu = $qu
        ];
    };
    // ALIAS: no need to specify: qu, Qu, pu, Pu, bu
    // useful for: all non-base units, with ascii prefix (doesn't support µ, micro)
    // e.g.:
    // vector_methods![$ty, $q, $Q, "m", "milli", 1.0e-3, "10⁻³"];
    ($ty:ty, qa=$q:ident, Qa=$Q:ident,
     pa=$pa:tt, Pa=$Pa:tt, f=$f:expr, fu=$fu:expr) => {
        vector_methods![
            $ty,
            qa=$q,
            Qa=$Q,
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
    // vector_methods![ Length, A, angstroms, "`Å`", "`ångströms`",
    //   metres, 1.0e10, "10⁻¹⁰" ];
    ($ty:ty, qa=$q:ident, Qa=$Q:ident, qu=$qu:tt, Qu=$Qu:tt, f=$f:expr, fu=$fu:expr,
     bu=$bu:tt) => {
        vector_methods![
            $ty,
            qa=$q,
            Qa=$Q,
            qu = $qu,
            Qu = $Qu,
            pu = "",
            Pu = "",
            pa = "",
            Pa = "",
            f = $f,
            fu = $fu,
            bu = $bu
        ];
    };
    // ALIAS: no need to specify: qu, Qu, pa, Pa, pu, Pu
    // e.g.:
    // vector_methods![Time, y, years, f=60., fu="365", bu="days"];
    ($ty:ty, qa=$q:ident, Qa=$Q:ident, f=$f:expr, fu=$fu:tt, bu=$bu:tt) => {
        vector_methods![
            $ty,
            qa=$q,
            Qa=$Q,
            qu = $q,
            Qu = $Q,
            pu = "",
            Pu = "",
            pa = "",
            Pa = "",
            f = $f,
            fu = $fu,
            bu = $bu
        ];
    };
    // ALIAS: no need to specify: qu, Qu, pa, Pa, pu, Pu, bu
    // useful for units WITHOUT prefixes, WITH conversion factor (grams)
    // e.g.:
    // vector_methods![$ty, $q, $Q, f=1e-3, fu="10⁰"];
    ($ty:ty, qa=$q:ident, Qa=$Q:ident, f=$f:expr, fu=$fu:expr) => {
        vector_methods![
            $ty,
            qa=$q,
            Qa=$Q,
            pu = "",
            Pu = "",
            pa = "",
            Pa = "",
            f = $f,
            fu = $fu
        ];
    };

    // ROOT RULE: const base unit, WITHOUT conversion factor
    [$ty:ty, base, qa=$q:ident, Qa=$Q:ident, qu=$qu:tt, Qu=$Qu:tt, pa=$pa:tt, Pa=$Pa:tt, fu=$fu:expr] => {
        paste::paste! {
            // constructors
            #[inline]
            #[allow(non_snake_case)]
            #[doc = "New `" $ty "` in " $Pa $Qu " (" $pa $qu ") (base unit, " $fu " " $q ")."]
            pub const fn [<in_$pa $q>](m: crate::Magnitude, d: crate::Direction) -> Self { Self::new(m, d) }
            #[inline]
            #[doc = "New `" $ty "` in " $Pa $Qu " (" $pa $qu ") (base unit, " $fu " " $q ")."]
            pub const fn [<in_$Pa $Q>](m: crate::Magnitude, d: crate::Direction) -> Self { Self::new(m, d) }
            // getters
            #[inline]
            #[allow(non_snake_case)]
            #[doc = "Returns `" $ty "` as " $Pa $Qu " (" $pa $qu ") (base unit, " $fu " " $q ")."]
            pub const fn [<as_$pa $q>](&self) -> (crate::Magnitude, crate::Direction) { (self.m, self.d) }
            #[inline]
            #[doc = "Returns `" $ty "` as " $Pa $Qu " (" $pa $qu ") (base unit, " $fu " " $q ")."]
            pub const fn [<as_$Pa $Q>](&self) -> (crate::Magnitude, crate::Direction) { (self.m, self.d) }
        }
    };
    // ALIAS: no need to specify: qu, Qu
    // useful for: the kilogram SI base unit
    // e.g.:
    // vector_methods![$ty, base, $q, $Q, pa="k", Pa="kilo", fu="10³"];
    ($ty:ty, base, qa=$q:ident, Qa=$Q:ident, pa=$pa:tt, Pa=$Pa:tt, fu=$fu:expr) => {
        vector_methods![
            $ty,
            base,
            qa=$q,
            Qa=$Q,
            qu = $q,
            Qu = $Q,
            pa = $pa,
            Pa = $Pa,
            fu = $fu
        ];
    };
    // ALIAS: no need to specify: pa, Pa
    // useful for: squared and cubed base units
    // e.g.:
    // vector_methods![$ty, base, $q, $Q, qu="m²", Qu="metres squared", fu="10⁰"];
    ($ty:ty, base, qa=$q:ident, Qa=$Q:ident, qu=$qu:tt, Qu=$Qu:tt, fu=$fu:expr) => {
        vector_methods![
            $ty,
            base,
            qa=$q,
            Qa=$Q,
            qu = $qu,
            Qu = $Qu,
            pa = "",
            Pa = "",
            fu = $fu
        ];
    };
    // ALIAS: no need to specify: qu, Qu, pa, Pa
    // useful for: base SI units except the kilogram (metre, kelvin, pascal…)
    // e.g.:
    // vector_methods![$ty, base, $q, $Q, fu="10⁰"];
    ($ty:ty, base, qa=$q:ident, Qa=$Q:ident, fu=$fu:expr) => {
        vector_methods![
            $ty,
            base,
            qa=$q,
            Qa=$Q,
            qu = $q,
            Qu = $Q,
            pa = "",
            Pa = "",
            fu = $fu
        ];
    };
}

/// Generates SI prefixes constructors and converter methods
macro_rules! impl_scalar_methods {
    [$ty:ty, $q:ident, $Q:ident] => {
        paste::paste! {
            /// # SI prefixes constructors: `in_*` & converters `as_*`
            #[doc = "**The `" $ty "` quantity is internally stored in `" $Q "` (`" $q "`)**."]
            #[doc = "- base *const* constructors: [`in_" $q "`](" $ty "#method.in_" $q ")," ]
            #[doc = "[`in_" $Q "`](" $ty "#method.in_" $Q ")"]
            #[doc = "- base *const* converters: [`as_" $q "`](" $ty "#method.as_" $q ")," ]
            #[doc = "[`as_" $Q "`](" $ty "#method.as_" $Q ")"]
            impl $ty {
                scalar_methods![$ty, qa=$q, Qa=$Q, pa="Y", Pa="yotta", f=1e24, fu="10²⁴"];
                scalar_methods![$ty, qa=$q, Qa=$Q, pa="Z", Pa="zetta", f=1e21, fu="10²¹"];
                scalar_methods![$ty, qa=$q, Qa=$Q, pa="E", Pa="exa", f=1e18, fu="10¹⁸"];
                scalar_methods![$ty, qa=$q, Qa=$Q, pa="P", Pa="peta", f=1e15, fu="10¹⁵"];
                scalar_methods![$ty, qa=$q, Qa=$Q, pa="T", Pa="tera", f=1e12, fu="10¹²"];
                scalar_methods![$ty, qa=$q, Qa=$Q, pa="G", Pa="giga", f=1e9, fu="10⁹"];
                scalar_methods![$ty, qa=$q, Qa=$Q, pa="M", Pa="mega", f=1e6, fu="10⁶"];
                scalar_methods![$ty, qa=$q, Qa=$Q, pa="k", Pa="kilo", f=1e3, fu="10³"];
                scalar_methods![$ty, qa=$q, Qa=$Q, pa="h", Pa="hecto", f=1e2, fu="10²"];
                scalar_methods![$ty, qa=$q, Qa=$Q, pa="da", Pa="deka", f=1e1, fu="10¹"];
                scalar_methods![$ty, base, qa=$q, Qa=$Q, fu="10⁰"];
                scalar_methods![$ty, qa=$q, Qa=$Q, pa="d", Pa="deci", f=1e-1, fu="10⁻¹"];
                scalar_methods![$ty, qa=$q, Qa=$Q, pa="c", Pa="centi", f=1e-2, fu="10⁻²"];
                scalar_methods![$ty, qa=$q, Qa=$Q, pa="m", Pa="milli", f=1e-3, fu="10⁻³"];
                scalar_methods![$ty, qa=$q, Qa=$Q, pu="µ", Pu="micro",
                    pa="u", Pa="micro", f=1e-6, fu="10⁻⁶"];
                scalar_methods![$ty, qa=$q, Qa=$Q, pa="n", Pa="nano", f=1e-9, fu="10⁻⁹"];
                scalar_methods![$ty, qa=$q, Qa=$Q, pa="p", Pa="pico", f=1e-12, fu="10⁻¹²"];
                scalar_methods![$ty, qa=$q, Qa=$Q, pa="f", Pa="femto", f=1e-15, fu="10⁻¹⁵"];
                scalar_methods![$ty, qa=$q, Qa=$Q, pa="a", Pa="atto", f=1e-18, fu="10⁻¹⁸"];
                scalar_methods![$ty, qa=$q, Qa=$Q, pa="z", Pa="zepto", f=1e-21, fu="10⁻²¹"];
                scalar_methods![$ty, qa=$q, Qa=$Q, pa="y", Pa="yocto", f=1e-24, fu="10⁻²⁴"];
            }
        }
    };
}

/// Generates constructors from the S.I. metre prefix. (with kilo being the base unit)
macro_rules! impl_scalar_methods_base_kilo {
    [$ty:ty, $q:ident, $Q:ident] => {
        paste::paste! {
            /// # SI prefixes constructors: `in_*` & converters `as_*`
            #[doc = "**The `" $ty "` quantity is internally stored in `kilo" $Q "` (`k" $q "`)**."]
            #[doc = "- base *const* constructors: [`in_k" $q "`](" $ty "#method.in_k" $q ")," ]
            #[doc = "[`in_kilo" $Q "`](" $ty "#method.in_kilo" $Q ")"]
            #[doc = "- base *const* converters: [`as_k" $q "`](" $ty "#method.as_k" $q ")," ]
            #[doc = "[`as_kilo" $Q "`](" $ty "#method.as_kilo" $Q ")"]
            impl $ty {
                scalar_methods![$ty, qa=$q, Qa=$Q, pa="Y", Pa="yotta", f=1e21, fu="10²⁴"];
                scalar_methods![$ty, qa=$q, Qa=$Q, pa="Z", Pa="zetta", f=1e18, fu="10²¹"];
                scalar_methods![$ty, qa=$q, Qa=$Q, pa="E", Pa="exa", f=1e15, fu="10¹⁸"];
                scalar_methods![$ty, qa=$q, Qa=$Q, pa="P", Pa="peta", f=1e12, fu="10¹⁵"];
                scalar_methods![$ty, qa=$q, Qa=$Q, pa="T", Pa="tera", f=1e9, fu="10¹²"];
                scalar_methods![$ty, qa=$q, Qa=$Q, pa="G", Pa="giga", f=1e6, fu="10⁹"];
                scalar_methods![$ty, qa=$q, Qa=$Q, pa="M", Pa="mega", f=1e3, fu="10⁶"];
                scalar_methods![$ty, base, qa=$q, Qa=$Q, pa="k", Pa="kilo", fu="10³"];
                scalar_methods![$ty, qa=$q, Qa=$Q, pa="h", Pa="hecto", f=1e-1, fu="10²"];
                scalar_methods![$ty, qa=$q, Qa=$Q, pa="da", Pa="deka", f=1e-2, fu="10¹"];
                scalar_methods![$ty, qa=$q, Qa=$Q, f=1e-3, fu="10⁰"];
                scalar_methods![$ty, qa=$q, Qa=$Q, pa="d", Pa="deci", f=1e-4, fu="10⁻¹"];
                scalar_methods![$ty, qa=$q, Qa=$Q, pa="c", Pa="centi", f=1e-5, fu="10⁻²"];
                scalar_methods![$ty, qa=$q, Qa=$Q, pa="m", Pa="milli", f=1e-6, fu="10⁻³"];
                scalar_methods![$ty, qa=$q, Qa=$Q, pu="µ", Pu="micro",
                    pa="u", Pa="micro", f=1e-9, fu="10⁻⁶"];
                scalar_methods![$ty, qa=$q, Qa=$Q, pa="n", Pa="nano", f=1e-12, fu="10⁻⁹"];
                scalar_methods![$ty, qa=$q, Qa=$Q, pa="p", Pa="pico", f=1e-15, fu="10⁻¹²"];
                scalar_methods![$ty, qa=$q, Qa=$Q, pa="f", Pa="femto", f=1e-18, fu="10⁻¹⁵"];
                scalar_methods![$ty, qa=$q, Qa=$Q, pa="a", Pa="atto", f=1e-21, fu="10⁻¹⁸"];
                scalar_methods![$ty, qa=$q, Qa=$Q, pa="z", Pa="zepto", f=1e-24, fu="10⁻²¹"];
                scalar_methods![$ty, qa=$q, Qa=$Q, pa="y", Pa="yocto", f=1e-27, fu="10⁻²⁴"];
            }
        }
    };
}

/// Generates SI prefixes constructors and converter methods (squared)
macro_rules! impl_scalar_methods_squared {
    [$ty:ty, $q:ident, $Q:ident, qu=$qu:literal, Qu=$Qu:literal] => {
        paste::paste! {
            /// # SI prefixes constructors: `in_*` & converters `as_*`
            ///
            /// The units are squared (10² for 1 step, 10⁶ for 3 steps)
            ///
            #[doc = "**The `" $ty "` quantity is internally stored in `" $Q "` (`" $qu "`)**."]
            #[doc = "- base *const* constructors: [`in_" $q "`](" $ty "#method.in_" $q ")," ]
            #[doc = "[`in_" $Q "`](" $ty "#method.in_" $Q ")"]
            #[doc = "- base *const* converters: [`as_" $q "`](" $ty "#method.as_" $q ")," ]
            #[doc = "[`as_" $Q "`](" $ty "#method.as_" $Q ")"]
            impl $ty {
                // WIP: FIXING nomenclature: kilometres squared → square kilometres
                //
                // E.g.:
                // scalar_methods![$ty, qa=$q, Qa=$Q, Q1a=$Q1a qu="m²", Qu1="square", QuM="meters",
                //     pa="Y", Pa="yotta", f=1e48, fu="10⁴⁸"];

                scalar_methods![$ty, qa=$q, Qa=$Q, qu=$qu, Qu=$Qu,
                    pa="Y", Pa="yotta", f=1e48, fu="10⁴⁸"];
                scalar_methods![$ty, qa=$q, Qa=$Q, qu=$qu, Qu=$Qu,
                    pa="Z", Pa="zetta", f=1e42, fu="10⁴²"];
                scalar_methods![$ty, qa=$q, Qa=$Q, qu=$qu, Qu=$Qu, pa="E", Pa="exa", f=1e36, fu="10³⁶"];
                scalar_methods![$ty, qa=$q, Qa=$Q, qu=$qu, Qu=$Qu, pa="P", Pa="peta", f=1e30, fu="10³⁰"];
                scalar_methods![$ty, qa=$q, Qa=$Q, qu=$qu, Qu=$Qu, pa="T", Pa="tera", f=1e24, fu="10²⁴"];
                scalar_methods![$ty, qa=$q, Qa=$Q, qu=$qu, Qu=$Qu, pa="G", Pa="giga", f=1e18, fu="10¹⁸"];
                scalar_methods![$ty, qa=$q, Qa=$Q, qu=$qu, Qu=$Qu, pa="M", Pa="mega", f=1e12, fu="10¹²"];
                scalar_methods![$ty, qa=$q, Qa=$Q, qu=$qu, Qu=$Qu, pa="k", Pa="kilo", f=1e6, fu="10⁶"];
                scalar_methods![$ty, qa=$q, Qa=$Q, qu=$qu, Qu=$Qu, pa="h", Pa="hecto", f=1e4, fu="10⁴"];
                scalar_methods![$ty, qa=$q, Qa=$Q, qu=$qu, Qu=$Qu, pa="da", Pa="deka", f=1e2, fu="10²"];
                scalar_methods![$ty, base, qa=$q, Qa=$Q, qu=$qu, Qu=$Qu, fu="10⁰"];
                scalar_methods![$ty, qa=$q, Qa=$Q, qu=$qu, Qu=$Qu, pa="d", Pa="deci", f=1e-2, fu="10⁻²"];
                scalar_methods![$ty, qa=$q, Qa=$Q, qu=$qu, Qu=$Qu,
                    pa="c", Pa="centi", f=1e-4, fu="10⁻⁴"];
                scalar_methods![$ty, qa=$q, Qa=$Q, qu=$qu, Qu=$Qu,
                    pa="m", Pa="milli", f=1e-6, fu="10⁻⁶"];
                scalar_methods![$ty, qa=$q, Qa=$Q, qu=$qu, Qu=$Qu,
                    pu="µ", Pu="micro", pa="u", Pa="micro", f=1e-12, fu="10⁻⁶", bu=$Q];
                scalar_methods![$ty, qa=$q, Qa=$Q, qu=$qu, Qu=$Qu,
                    pa="n", Pa="nano", f=1e-18, fu="10⁻¹⁸"];
                scalar_methods![$ty, qa=$q, Qa=$Q, qu=$qu, Qu=$Qu,
                    pa="p", Pa="pico", f=1e-24, fu="10⁻²⁴"];
                scalar_methods![$ty, qa=$q, Qa=$Q, qu=$qu, Qu=$Qu,
                    pa="f", Pa="femto", f=1e-30, fu="10⁻³⁰"];
                scalar_methods![$ty, qa=$q, Qa=$Q, qu=$qu, Qu=$Qu,
                    pa="a", Pa="atto", f=1e-36, fu="10⁻³⁶"];
                scalar_methods![$ty, qa=$q, Qa=$Q, qu=$qu, Qu=$Qu,
                    pa="z", Pa="zepto", f=1e-42, fu="10⁻⁴²"];
                scalar_methods![$ty, qa=$q, Qa=$Q, qu=$qu, Qu=$Qu,
                    pa="y", Pa="yocto", f=1e-48, fu="10⁻⁴⁸"];
            }
        }
    };
    // ALIAS: no need to specify: Qu
    ($ty:ty, $q:ident, $Q:ident, qu=$qu:literal) => {
        impl_scalar_methods_squared![$ty, $q, $Q, qu = $qu, Qu = $Q];
    };
}

/// Generates SI prefixes constructors and converter methods (cubed)
macro_rules! impl_scalar_methods_cubed {
    [$ty:ty, $q:ident, $Q:ident, qu=$qu:literal, Qu=$Qu:literal] => {
        paste::paste! {
            /// # SI prefixes constructors: `in_*` & converters `as_*`
            ///
            /// The units are cubed (10³ for 1 step, 10⁹ for 3 steps)
            ///
            #[doc = "**The `" $ty "` quantity is internally stored in `" $Qu "` (`" $qu "`)**."]
            #[doc = "- base *const* constructors: [`in_" $q "`](" $ty "#method.in_" $q "),"]
            #[doc = "[`in_" $Q "`](" $ty "#method.in_" $Q ")"]
            #[doc = "- base *const* constructors: [`as_" $q "`](" $ty "#method.as_" $q "),"]
            #[doc = "[`as_" $Q "`](" $ty "#method.as_" $Q ")"]
            impl $ty {
                scalar_methods![$ty, qa=$q, Qa=$Q, qu=$qu, Qu=$Qu,
                    pa="Y", Pa="yotta", f=1e72, fu="10⁷²"];
                scalar_methods![$ty, qa=$q, Qa=$Q, qu=$qu, Qu=$Qu,
                    pa="Z", Pa="zetta", f=1e63, fu="10⁶³"];
                scalar_methods![$ty, qa=$q, Qa=$Q, qu=$qu, Qu=$Qu,
                    pa="E", Pa="exa", f=1e54, fu="10⁵⁴"];
                scalar_methods![$ty, qa=$q, Qa=$Q, qu=$qu, Qu=$Qu,
                    pa="P", Pa="peta", f=1e45, fu="10⁴⁵"];
                scalar_methods![$ty, qa=$q, Qa=$Q, qu=$qu, Qu=$Qu,
                    pa="T", Pa="tera", f=1e36, fu="10³⁶"];
                scalar_methods![$ty, qa=$q, Qa=$Q, qu=$qu, Qu=$Qu,
                    pa="G", Pa="giga", f=1e27, fu="10²⁷"];
                scalar_methods![$ty, qa=$q, Qa=$Q, qu=$qu, Qu=$Qu,
                    pa="M", Pa="mega", f=1e18, fu="10¹⁸"];
                scalar_methods![$ty, qa=$q, Qa=$Q, qu=$qu, Qu=$Qu,
                    pa="k", Pa="kilo", f=1e9, fu="10⁹"];
                scalar_methods![$ty, qa=$q, Qa=$Q, qu=$qu, Qu=$Qu,
                    pa="h", Pa="hecto", f=1e6, fu="10⁶"];
                scalar_methods![$ty, qa=$q, Qa=$Q, qu=$qu, Qu=$Qu,
                    pa="da", Pa="deka", f=1e3, fu="10³"];
                scalar_methods![$ty, base, qa=$q, Qa=$Q, qu=$qu, Qu=$Qu, fu="10⁰"];
                scalar_methods![$ty, qa=$q, Qa=$Q, qu=$qu, Qu=$Qu,
                    pa="d", Pa="deci", f=1e-3, fu="10⁻³"];
                scalar_methods![$ty, qa=$q, Qa=$Q, qu=$qu, Qu=$Qu,
                    pa="c", Pa="centi", f=1e-6, fu="10⁻⁶"];
                scalar_methods![$ty, qa=$q, Qa=$Q, qu=$qu, Qu=$Qu,
                    pa="m", Pa="milli", f=1e-9, fu="10⁻⁹"];
                scalar_methods![$ty, qa=$q, Qa=$Q, qu=$qu, Qu=$Qu,
                    pu="µ", Pu="micro", pa="u", Pa="micro", f=1e-18, fu="10⁻¹⁸", bu=$Q];
                scalar_methods![$ty, qa=$q, Qa=$Q, qu=$qu, Qu=$Qu,
                    pa="n", Pa="nano", f=1e-27, fu="10⁻²⁷"];
                scalar_methods![$ty, qa=$q, Qa=$Q, qu=$qu, Qu=$Qu,
                    pa="p", Pa="pico", f=1e-36, fu="10⁻³⁶"];
                scalar_methods![$ty, qa=$q, Qa=$Q, qu=$qu, Qu=$Qu,
                    pa="f", Pa="femto", f=1e-45, fu="10⁻⁴⁵"];
                scalar_methods![$ty, qa=$q, Qa=$Q, qu=$qu, Qu=$Qu,
                    pa="a", Pa="atto", f=1e-54, fu="10⁻⁵⁴"];
                scalar_methods![$ty, qa=$q, Qa=$Q, qu=$qu, Qu=$Qu,
                    pa="z", Pa="zepto", f=1e-63, fu="10⁻⁶³"];
                scalar_methods![$ty, qa=$q, Qa=$Q, qu=$qu, Qu=$Qu,
                    pa="y", Pa="yocto", f=1e-72, fu="10⁻⁷²"];
            }
        }
    };
    // ALIAS: no need to specify: Qu
    ($ty:ty, $q:ident, $Q:ident, qu=$qu:literal) => {
        impl_scalar_methods_cubed![$ty, $q, $Q, qu = $qu, Qu = $Q];
    };
}

/// Generates SI prefixes constructors and converter methods (2 units)
///
// use for: speed
macro_rules! impl_scalar_methods_2units {
    [$ty:ty, q1a=$q1a:ident, q2a=$q2a:ident, Q1a=$Q1a:ident, Q2a=$Q2a:ident, Ja=$Ja:ident] => {
        paste::paste! {
            /// # SI prefixes constructors: `in_*` & converters `as_*`
            #[doc = "**The `" $ty "` quantity is internally stored in `" $Q1a " " $Ja " " $Q2a "` (`" $q1a "/" $q2a "`)**." ]
            #[doc = "- base *const* constructors: [`in_" $q1a _ $q2a "`](" $ty "#method.in_" $q1a $q2a ")," ]
            #[doc = "[`in_" $Q1a _ $Q2a "`](" $ty "#method.in_" $Q1a _ $Q2a ")"]
            #[doc = "- base *const* converters: [`as_" $q1a _ $q2a "`](" $ty "#method.as_" $q1a $q2a ")," ]
            #[doc = "[`as_" $Q1a _ $Q2a "`](" $ty "#method.as_" $Q1a _ $Q2a ")"]
            impl $ty {
                paste::paste! {
                    scalar_methods_2units![$ty, q1a=$q1a, q2a=$q2a, Q1a=$Q1a, Q2a=$Q2a, Ja=$Ja,
                        p1a="Y", p2a="", P1a="yotta", P2a="", f=1e24, fu="10²⁴", b1u=$q1a, b2u=$q2a];
                    scalar_methods_2units![$ty, q1a=$q1a, q2a=$q2a, Q1a=$Q1a, Q2a=$Q2a, Ja=$Ja,
                        p1a="Z", p2a="", P1a="zetta", P2a="", f=1e21, fu="10²¹", b1u=$q1a, b2u=$q2a];
                    scalar_methods_2units![$ty, q1a=$q1a, q2a=$q2a, Q1a=$Q1a, Q2a=$Q2a, Ja=$Ja,
                        p1a="E", p2a="", P1a="exa", P2a="", f=1e18, fu="10¹⁸", b1u=$q1a, b2u=$q2a];
                    scalar_methods_2units![$ty, q1a=$q1a, q2a=$q2a, Q1a=$Q1a, Q2a=$Q2a, Ja=$Ja,
                        p1a="P", p2a="", P1a="peta", P2a="", f=1e15, fu="10¹⁵", b1u=$q1a, b2u=$q2a];
                    scalar_methods_2units![$ty, q1a=$q1a, q2a=$q2a, Q1a=$Q1a, Q2a=$Q2a, Ja=$Ja,
                        p1a="T", p2a="", P1a="tera", P2a="", f=1e12, fu="10¹²", b1u=$q1a, b2u=$q2a];
                    scalar_methods_2units![$ty, q1a=$q1a, q2a=$q2a, Q1a=$Q1a, Q2a=$Q2a, Ja=$Ja,
                        p1a="G", p2a="", P1a="giga", P2a="", f=1e9, fu="10⁹", b1u=$q1a, b2u=$q2a];
                    scalar_methods_2units![$ty, q1a=$q1a, q2a=$q2a, Q1a=$Q1a, Q2a=$Q2a, Ja=$Ja,
                        p1a="M", p2a="", P1a="mega", P2a="", f=1e6, fu="10⁶", b1u=$q1a, b2u=$q2a];
                    scalar_methods_2units![$ty, q1a=$q1a, q2a=$q2a, Q1a=$Q1a, Q2a=$Q2a, Ja=$Ja,
                        p1a="k", p2a="", P1a="kilo", P2a="", f=1e3, fu="10³", b1u=$q1a, b2u=$q2a];
                    scalar_methods_2units![$ty, q1a=$q1a, q2a=$q2a, Q1a=$Q1a, Q2a=$Q2a, Ja=$Ja,
                        p1a="h", p2a="", P1a="hecto", P2a="", f=1e2, fu="10²", b1u=$q1a, b2u=$q2a];
                    scalar_methods_2units![$ty, q1a=$q1a, q2a=$q2a, Q1a=$Q1a, Q2a=$Q2a, Ja=$Ja,
                        p1a="da", p2a="", P1a="deka", P2a="", f=1e1, fu="10²", b1u=$q1a, b2u=$q2a];
                    scalar_methods_2units![$ty, base, q1a=$q1a, q2a=$q2a, Q1a=$Q1a, Q2a=$Q2a, Ja=$Ja, fu="10⁰"];
                    scalar_methods_2units![$ty, q1a=$q1a, q2a=$q2a, Q1a=$Q1a, Q2a=$Q2a, Ja=$Ja,
                        p1a="d", p2a="", P1a="deci", P2a="", f=1e-1, fu="10⁻¹", b1u=$q1a, b2u=$q2a];
                    scalar_methods_2units![$ty, q1a=$q1a, q2a=$q2a, Q1a=$Q1a, Q2a=$Q2a, Ja=$Ja,
                        p1a="c", p2a="", P1a="centi", P2a="", f=1e-2, fu="10⁻²", b1u=$q1a, b2u=$q2a];
                    scalar_methods_2units![$ty, q1a=$q1a, q2a=$q2a, Q1a=$Q1a, Q2a=$Q2a, Ja=$Ja,
                        p1a="m", p2a="", P1a="milli", P2a="", f=1e-3, fu="10⁻³", b1u=$q1a, b2u=$q2a];
                    scalar_methods_2units![$ty, q1a=$q1a, q2a=$q2a, Q1a=$Q1a, Q2a=$Q2a, Ja=$Ja,
                        p1a="u", p2a="", P1a="micro", P2a="", p1u="µ", p2u="", P1u="micro", P2u="",
                        f=1e-6, fu="10⁻⁶", b1u=$q1a, b2u=$q2a];
                    scalar_methods_2units![$ty, q1a=$q1a, q2a=$q2a, Q1a=$Q1a, Q2a=$Q2a, Ja=$Ja,
                        p1a="n", p2a="", P1a="nano", P2a="", f=1e-9, fu="10⁻⁹", b1u=$q1a, b2u=$q2a];
                    scalar_methods_2units![$ty, q1a=$q1a, q2a=$q2a, Q1a=$Q1a, Q2a=$Q2a, Ja=$Ja,
                        p1a="p", p2a="", P1a="pico", P2a="", f=1e-12, fu="10⁻¹²", b1u=$q1a, b2u=$q2a];
                    scalar_methods_2units![$ty, q1a=$q1a, q2a=$q2a, Q1a=$Q1a, Q2a=$Q2a, Ja=$Ja,
                        p1a="f", p2a="", P1a="femto", P2a="", f=1e-15, fu="10⁻¹⁵", b1u=$q1a, b2u=$q2a];
                    scalar_methods_2units![$ty, q1a=$q1a, q2a=$q2a, Q1a=$Q1a, Q2a=$Q2a, Ja=$Ja,
                        p1a="a", p2a="", P1a="atto", P2a="", f=1e-18, fu="10⁻¹⁸", b1u=$q1a, b2u=$q2a];
                    scalar_methods_2units![$ty, q1a=$q1a, q2a=$q2a, Q1a=$Q1a, Q2a=$Q2a, Ja=$Ja,
                        p1a="z", p2a="", P1a="zepto", P2a="", f=1e-21, fu="10⁻²¹", b1u=$q1a, b2u=$q2a];
                    scalar_methods_2units![$ty, q1a=$q1a, q2a=$q2a, Q1a=$Q1a, Q2a=$Q2a, Ja=$Ja,
                        p1a="y", p2a="", P1a="yocto", P2a="", f=1e-24, fu="10⁻²⁴", b1u=$q1a, b2u=$q2a];
                }
            }
        }
    };
}

/// Generates SI prefixes constructors and converter methods
/// (2 units, with kilo prefix on the 1st base unit)
//
// use for: density, (vector versions: moment of inertia, linear & angular momentum…)
macro_rules! impl_scalar_methods_2units_base_kilo {
    [$ty:ty, q1a=$q1a:ident, q2a=$q2a:ident, q1u=$q1u:tt, q2u=$q2u:tt,
     Q1a=$Q1a:ident, Q2a=$Q2a:ident, Ja=$Ja:ident, Q1u=$Q1u:tt, Q2u=$Q2u:tt] => {
        paste::paste! {
            /// # SI prefixes constructors: `in_*` & converters `as_*`
            #[doc = "**The `" $ty "` quantity is internally stored in `kilo" $Q1u " " $Ja " " $Q2u
                "` (`" $q1u "/" $q2u "`)**." ]
            #[doc = "- base *const* constructors: [`in_k" $q1a _ $q2a "`](" $ty "#method.in_" $q1a $q2a ")," ]
            #[doc = "[`in_kilo" $Q1a _ $Q2a "`](" $ty "#method.in_kilo" $Q1a _ $Q2a ")"]
            #[doc = "- base *const* converters: [`as_k" $q1a _ $q2a "`](" $ty "#method.as_k" $q1a $q2a ")," ]
            #[doc = "[`as_kilo" $Q1a _ $Q2a "`](" $ty "#method.as_kilo" $Q1a _ $Q2a ")"]
            impl $ty {
                paste::paste! {
                    scalar_methods_2units![$ty, q1a=$q1a, q2a=$q2a, Q1a=$Q1a, Q2a=$Q2a, Ja=$Ja,
                        q1u=$q1u, q2u=$q2u, Q1u=$Q1u, Q2u=$Q2u, p1a="Y", p2a="",
                        P1a="yotta", P2a="", f=1e21, fu="10²⁴", b1u=$q1u, b2u=$q2u];
                    scalar_methods_2units![$ty, q1a=$q1a, q2a=$q2a, Q1a=$Q1a, Q2a=$Q2a, Ja=$Ja,
                        q1u=$q1u, q2u=$q2u, Q1u=$Q1u, Q2u=$Q2u, p1a="Z", p2a="",
                        P1a="zetta", P2a="", f=1e18, fu="10²¹", b1u=$q1u, b2u=$q2u];
                    scalar_methods_2units![$ty, q1a=$q1a, q2a=$q2a, Q1a=$Q1a, Q2a=$Q2a, Ja=$Ja,
                        q1u=$q1u, q2u=$q2u, Q1u=$Q1u, Q2u=$Q2u, p1a="E", p2a="",
                        P1a="exa", P2a="", f=1e15, fu="10¹⁸", b1u=$q1u, b2u=$q2u];
                    scalar_methods_2units![$ty, q1a=$q1a, q2a=$q2a, Q1a=$Q1a, Q2a=$Q2a, Ja=$Ja,
                        q1u=$q1u, q2u=$q2u, Q1u=$Q1u, Q2u=$Q2u, p1a="P", p2a="",
                        P1a="peta", P2a="", f=1e12, fu="10¹⁵", b1u=$q1u, b2u=$q2u];
                    scalar_methods_2units![$ty, q1a=$q1a, q2a=$q2a, Q1a=$Q1a, Q2a=$Q2a, Ja=$Ja,
                        q1u=$q1u, q2u=$q2u, Q1u=$Q1u, Q2u=$Q2u, p1a="T", p2a="",
                        P1a="tera", P2a="", f=1e9, fu="10¹²", b1u=$q1u, b2u=$q2u];
                    scalar_methods_2units![$ty, q1a=$q1a, q2a=$q2a, Q1a=$Q1a, Q2a=$Q2a, Ja=$Ja,
                        q1u=$q1u, q2u=$q2u, Q1u=$Q1u, Q2u=$Q2u, p1a="G", p2a="",
                        P1a="giga", P2a="", f=1e6, fu="10⁹", b1u=$q1u, b2u=$q2u];
                    scalar_methods_2units![$ty, q1a=$q1a, q2a=$q2a, Q1a=$Q1a, Q2a=$Q2a, Ja=$Ja,
                        q1u=$q1u, q2u=$q2u, Q1u=$Q1u, Q2u=$Q2u, p1a="M", p2a="",
                        P1a="mega", P2a="", f=1e3, fu="10⁶", b1u=$q1u, b2u=$q2u];
                    scalar_methods_2units![$ty, base, q1a=$q1a, q2a=$q2a, Q1a=$Q1a, Q2a=$Q2a, Ja=$Ja,
                        q1u=$q1u, q2u=$q2u, Q1u=$Q1u, Q2u=$Q2u,
                        p1a="k", p2a="", P1a="kilo", P2a="", fu="10³"];
                    scalar_methods_2units![$ty, q1a=$q1a, q2a=$q2a, Q1a=$Q1a, Q2a=$Q2a, Ja=$Ja,
                        q1u=$q1u, q2u=$q2u, Q1u=$Q1u, Q2u=$Q2u, p1a="h", p2a="",
                        P1a="hecto", P2a="", f=1e-1, fu="10²", b1u=$q1u, b2u=$q2u];
                    scalar_methods_2units![$ty, q1a=$q1a, q2a=$q2a, Q1a=$Q1a, Q2a=$Q2a, Ja=$Ja,
                        q1u=$q1u, q2u=$q2u, Q1u=$Q1u, Q2u=$Q2u, p1a="da", p2a="",
                        P1a="deka", P2a="", f=1e-2, fu="10¹", b1u=$q1u, b2u=$q2u];
                    scalar_methods_2units![$ty, q1a=$q1a, q2a=$q2a, Q1a=$Q1a, Q2a=$Q2a, Ja=$Ja,
                        q1u=$q1u, q2u=$q2u, Q1u=$Q1u, Q2u=$Q2u, p1a="", p2a="",
                        P1a="", P2a="", f=1e-3, fu="10⁰", b1u=$q1u, b2u=$q2u];
                    scalar_methods_2units![$ty, q1a=$q1a, q2a=$q2a, Q1a=$Q1a, Q2a=$Q2a, Ja=$Ja,
                        q1u=$q1u, q2u=$q2u, Q1u=$Q1u, Q2u=$Q2u, p1a="d", p2a="",
                        P1a="deci", P2a="", f=1e-4, fu="10⁻¹", b1u=$q1u, b2u=$q2u];
                    scalar_methods_2units![$ty, q1a=$q1a, q2a=$q2a, Q1a=$Q1a, Q2a=$Q2a, Ja=$Ja,
                        q1u=$q1u, q2u=$q2u, Q1u=$Q1u, Q2u=$Q2u, p1a="c", p2a="",
                        P1a="centi", P2a="", f=1e-5, fu="10⁻²", b1u=$q1u, b2u=$q2u];
                    scalar_methods_2units![$ty, q1a=$q1a, q2a=$q2a, Q1a=$Q1a, Q2a=$Q2a, Ja=$Ja,
                        q1u=$q1u, q2u=$q2u, Q1u=$Q1u, Q2u=$Q2u, p1a="m", p2a="",
                        P1a="milli", P2a="", f=1e-6, fu="10⁻³", b1u=$q1u, b2u=$q2u];
                    scalar_methods_2units![$ty, q1a=$q1a, q2a=$q2a, Q1a=$Q1a, Q2a=$Q2a, Ja=$Ja,
                        q1u=$q1u, q2u=$q2u, Q1u=$Q1u, Q2u=$Q2u, p1a="u", p2a="",
                        P1a="micro", P2a="", p1u="µ", p2u="", P1u="micro", P2u="",
                        f=1e-9, fu="10⁻⁶", b1u=$q1u, b2u=$q2u];
                    scalar_methods_2units![$ty, q1a=$q1a, q2a=$q2a, Q1a=$Q1a, Q2a=$Q2a, Ja=$Ja,
                        q1u=$q1u, q2u=$q2u, Q1u=$Q1u, Q2u=$Q2u, p1a="n", p2a="",
                        P1a="nano", P2a="", f=1e-12, fu="10⁻⁹", b1u=$q1u, b2u=$q2u];
                    scalar_methods_2units![$ty, q1a=$q1a, q2a=$q2a, Q1a=$Q1a, Q2a=$Q2a, Ja=$Ja,
                        q1u=$q1u, q2u=$q2u, Q1u=$Q1u, Q2u=$Q2u, p1a="p", p2a="",
                        P1a="pico", P2a="", f=1e-15, fu="10⁻¹²", b1u=$q1u, b2u=$q2u];
                    scalar_methods_2units![$ty, q1a=$q1a, q2a=$q2a, Q1a=$Q1a, Q2a=$Q2a, Ja=$Ja,
                        q1u=$q1u, q2u=$q2u, Q1u=$Q1u, Q2u=$Q2u, p1a="f", p2a="",
                        P1a="femto", P2a="", f=1e-18, fu="10⁻¹⁵", b1u=$q1u, b2u=$q2u];
                    scalar_methods_2units![$ty, q1a=$q1a, q2a=$q2a, Q1a=$Q1a, Q2a=$Q2a, Ja=$Ja,
                        q1u=$q1u, q2u=$q2u, Q1u=$Q1u, Q2u=$Q2u, p1a="a", p2a="",
                        P1a="atto", P2a="", f=1e-21, fu="10⁻¹⁸", b1u=$q1u, b2u=$q2u];
                    scalar_methods_2units![$ty, q1a=$q1a, q2a=$q2a, Q1a=$Q1a, Q2a=$Q2a, Ja=$Ja,
                        q1u=$q1u, q2u=$q2u, Q1u=$Q1u, Q2u=$Q2u, p1a="z", p2a="",
                        P1a="zepto", P2a="", f=1e-24, fu="10⁻²¹", b1u=$q1u, b2u=$q2u];
                    scalar_methods_2units![$ty, q1a=$q1a, q2a=$q2a, Q1a=$Q1a, Q2a=$Q2a, Ja=$Ja,
                        q1u=$q1u, q2u=$q2u, Q1u=$Q1u, Q2u=$Q2u, p1a="y", p2a="",
                        P1a="yocto", P2a="", f=1e-27, fu="10⁻²⁴", b1u=$q1u, b2u=$q2u];
                }
            }
        }
    };

    [$ty:ty, q1a=$q1a:ident, q2a=$q2a:ident, Q1a=$Q1a:ident, Q2a=$Q2a:ident, Ja=$Ja:ident] => {
        impl_scalar_methods_2units_base_kilo![
            $ty,
            q1a=$q1a,
            q2a=$q2a,
            q1u=$q1a,
            q2u=$q2a,
            Q1a=$Q1a,
            Q2a=$Q2a,
            Ja=$Ja,
            Q1u=$Q1a,
            Q2u=$Q2a
        ];
    };
}

/// Generates SI prefixes constructors and converter methods
macro_rules! impl_vector_methods {
    [$ty:ty, $q:ident, $Q:ident] => {
        paste::paste! {
            /// # SI prefixes constructors: `in_*` & converters `as_*`
            #[doc = "**The `" $ty "` quantity is internally stored in `" $Q "`**."]
            #[doc = "- base *const* constructors: [`in_" $q "`](" $ty "#method.in_" $q ")," ]
            #[doc = "[`in_" $Q "`](" $ty "#method.in_" $Q ")"]
            #[doc = "- base *const* converters: [`as_" $q "`](" $ty "#method.as_" $q ")," ]
            #[doc = "[`as_" $Q "`](" $ty "#method.as_" $Q ")"]
            impl $ty {
                vector_methods![$ty, qa=$q, Qa=$Q, pa="Y", Pa="yotta", f=1e24, fu="10²⁴"];
                vector_methods![$ty, qa=$q, Qa=$Q, pa="Z", Pa="zetta", f=1e21, fu="10²¹"];
                vector_methods![$ty, qa=$q, Qa=$Q, pa="E", Pa="exa", f=1e18, fu="10¹⁸"];
                vector_methods![$ty, qa=$q, Qa=$Q, pa="P", Pa="peta", f=1e15, fu="10¹⁵"];
                vector_methods![$ty, qa=$q, Qa=$Q, pa="T", Pa="tera", f=1e12, fu="10¹²"];
                vector_methods![$ty, qa=$q, Qa=$Q, pa="G", Pa="giga", f=1e9, fu="10⁹"];
                vector_methods![$ty, qa=$q, Qa=$Q, pa="M", Pa="mega", f=1e6, fu="10⁶"];
                vector_methods![$ty, qa=$q, Qa=$Q, pa="k", Pa="kilo", f=1e3, fu="10³"];
                vector_methods![$ty, qa=$q, Qa=$Q, pa="h", Pa="hecto", f=1e2, fu="10²"];
                vector_methods![$ty, qa=$q, Qa=$Q, pa="da", Pa="deka", f=1e1, fu="10¹"];
                vector_methods![$ty, base, qa=$q, Qa=$Q, fu="10⁰"];
                vector_methods![$ty, qa=$q, Qa=$Q, pa="d", Pa="deci", f=1e-1, fu="10⁻¹"];
                vector_methods![$ty, qa=$q, Qa=$Q, pa="c", Pa="centi", f=1e-2, fu="10⁻²"];
                vector_methods![$ty, qa=$q, Qa=$Q, pa="m", Pa="milli", f=1e-3, fu="10⁻³"];
                vector_methods![$ty, qa=$q, Qa=$Q, pu="µ", Pu="micro",
                    pa="u", Pa="micro", f=1e-6, fu="10⁻⁶"];
                vector_methods![$ty, qa=$q, Qa=$Q, pa="n", Pa="nano", f=1e-9, fu="10⁻⁹"];
                vector_methods![$ty, qa=$q, Qa=$Q, pa="p", Pa="pico", f=1e-12, fu="10⁻¹²"];
                vector_methods![$ty, qa=$q, Qa=$Q, pa="f", Pa="femto", f=1e-15, fu="10⁻¹⁵"];
                vector_methods![$ty, qa=$q, Qa=$Q, pa="a", Pa="atto", f=1e-18, fu="10⁻¹⁸"];
                vector_methods![$ty, qa=$q, Qa=$Q, pa="z", Pa="zepto", f=1e-21, fu="10⁻²¹"];
                vector_methods![$ty, qa=$q, Qa=$Q, pa="y", Pa="yocto", f=1e-24, fu="10⁻²⁴"];
            }
        }
    };
}

// TODO
// /// Generates SI prefixes constructors and converter methods (two units)
//
// use for: velocity, acceleration, moment, momentum, gfs
//macro_rules! impl_vector_methods_two_units {

//}

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
