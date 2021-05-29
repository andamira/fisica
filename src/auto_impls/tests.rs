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
