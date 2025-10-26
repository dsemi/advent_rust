use Atom::*;
use enum_map::{Enum, EnumMap, enum_map};
use smallvec::{SmallVec, smallvec as sv};
use std::sync::LazyLock;

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Enum)]
enum Atom {
     H,                                                                                                                         He,
    Li, Be,                                                                                                  B,  C,  N,  O,  F, Ne,
    Na, Mg,                                                                                                 Al, Si,  P,  S, Cl, Ar,
     K, Ca, Sc,                                                         Ti,  V, Cr, Mn, Fe, Co, Ni, Cu, Zn, Ga, Ge, As, Se, Br, Kr,
    Rb, Sr,  Y,                                                         Zr, Nb, Mo, Tc, Ru, Rh, Pd, Ag, Cd, In, Sn, Sb, Te,  I, Xe,
    Cs, Ba, La, Ce, Pr, Nd, Pm, Sm, Eu, Gd, Tb, Dy, Ho, Er, Tm, Yb, Lu, Hf, Ta,  W, Re, Os, Ir, Pt, Au, Hg, Tl, Pb, Bi, Po, At, Rn,
    Fr, Ra, Ac, Th, Pa,  U,
}

struct A {
    decay: SmallVec<[Atom; 6]>,
    seq: &'static str,
}

#[rustfmt::skip]
static MAPPINGS: LazyLock<EnumMap<Atom, A>> = LazyLock::new(|| {
    enum_map! {
        H  => A{decay: sv![H],                     seq: "22"},
        He => A{decay: sv![Hf, Pa, H, Ca, Li],     seq: "13112221133211322112211213322112"},
        Li => A{decay: sv![He],                    seq: "312211322212221121123222112"},
        Be => A{decay: sv![Ge, Ca, Li],            seq: "111312211312113221133211322112211213322112"},
        B  => A{decay: sv![Be],                    seq: "1321132122211322212221121123222112"},
        C  => A{decay: sv![B],                     seq: "3113112211322112211213322112"},
        N  => A{decay: sv![C],                     seq: "111312212221121123222112"},
        O  => A{decay: sv![N],                     seq: "132112211213322112"},
        F  => A{decay: sv![O],                     seq: "31121123222112"},
        Ne => A{decay: sv![F],                     seq: "111213322112"},
        Na => A{decay: sv![Ne],                    seq: "123222112"},
        Mg => A{decay: sv![Pm, Na],                seq: "3113322112"},
        Al => A{decay: sv![Mg],                    seq: "1113222112"},
        Si => A{decay: sv![Al],                    seq: "1322112"},
        P  => A{decay: sv![Ho, Si],                seq: "311311222112"},
        S  => A{decay: sv![P],                     seq: "1113122112"},
        Cl => A{decay: sv![S],                     seq: "132112"},
        Ar => A{decay: sv![Cl],                    seq: "3112"},
        K  => A{decay: sv![Ar],                    seq: "1112"},
        Ca => A{decay: sv![K],                     seq: "12"},
        Sc => A{decay: sv![Ho, Pa, H, Ca, Co],     seq: "3113112221133112"},
        Ti => A{decay: sv![Sc],                    seq: "11131221131112"},
        V  => A{decay: sv![Ti],                    seq: "13211312"},
        Cr => A{decay: sv![V],                     seq: "31132"},
        Mn => A{decay: sv![Cr, Si],                seq: "111311222112"},
        Fe => A{decay: sv![Mn],                    seq: "13122112"},
        Co => A{decay: sv![Fe],                    seq: "32112"},
        Ni => A{decay: sv![Zn, Co],                seq: "11133112"},
        Cu => A{decay: sv![Ni],                    seq: "131112"},
        Zn => A{decay: sv![Cu],                    seq: "312"},
        Ga => A{decay: sv![Eu, Ca, Ac, H, Ca, Zn], seq: "13221133122211332"},
        Ge => A{decay: sv![Ho, Ga],                seq: "31131122211311122113222"},
        As => A{decay: sv![Ge, Na],                seq: "11131221131211322113322112"},
        Se => A{decay: sv![As],                    seq: "13211321222113222112"},
        Br => A{decay: sv![Se],                    seq: "3113112211322112"},
        Kr => A{decay: sv![Br],                    seq: "11131221222112"},
        Rb => A{decay: sv![Kr],                    seq: "1321122112"},
        Sr => A{decay: sv![Rb],                    seq: "3112112"},
        Y  => A{decay: sv![Sr, U],                 seq: "1112133"},
        Zr => A{decay: sv![Y, H, Ca, Tc],          seq: "12322211331222113112211"},
        Nb => A{decay: sv![Er, Zr],                seq: "1113122113322113111221131221"},
        Mo => A{decay: sv![Nb],                    seq: "13211322211312113211"},
        Tc => A{decay: sv![Mo],                    seq: "311322113212221"},
        Ru => A{decay: sv![Eu, Ca, Tc],            seq: "132211331222113112211"},
        Rh => A{decay: sv![Ho, Ru],                seq: "311311222113111221131221"},
        Pd => A{decay: sv![Rh],                    seq: "111312211312113211"},
        Ag => A{decay: sv![Pd],                    seq: "132113212221"},
        Cd => A{decay: sv![Ag],                    seq: "3113112211"},
        In => A{decay: sv![Cd],                    seq: "11131221"},
        Sn => A{decay: sv![In],                    seq: "13211"},
        Sb => A{decay: sv![Pm, Sn],                seq: "3112221"},
        Te => A{decay: sv![Eu, Ca, Sb],            seq: "1322113312211"},
        I  => A{decay: sv![Ho, Te],                seq: "311311222113111221"},
        Xe => A{decay: sv![I],                     seq: "11131221131211"},
        Cs => A{decay: sv![Xe],                    seq: "13211321"},
        Ba => A{decay: sv![Cs],                    seq: "311311"},
        La => A{decay: sv![Ba],                    seq: "11131"},
        Ce => A{decay: sv![La, H, Ca, Co],         seq: "1321133112"},
        Pr => A{decay: sv![Ce],                    seq: "31131112"},
        Nd => A{decay: sv![Pr],                    seq: "111312"},
        Pm => A{decay: sv![Nd],                    seq: "132"},
        Sm => A{decay: sv![Pm, Ca, Zn],            seq: "311332"},
        Eu => A{decay: sv![Sm],                    seq: "1113222"},
        Gd => A{decay: sv![Eu, Ca, Co],            seq: "13221133112"},
        Tb => A{decay: sv![Ho, Gd],                seq: "3113112221131112"},
        Dy => A{decay: sv![Tb],                    seq: "111312211312"},
        Ho => A{decay: sv![Dy],                    seq: "1321132"},
        Er => A{decay: sv![Ho, Pm],                seq: "311311222"},
        Tm => A{decay: sv![Er, Ca, Co],            seq: "11131221133112"},
        Yb => A{decay: sv![Tm],                    seq: "1321131112"},
        Lu => A{decay: sv![Yb],                    seq: "311312"},
        Hf => A{decay: sv![Lu],                    seq: "11132"},
        Ta => A{decay: sv![Hf, Pa, H, Ca, W],      seq: "13112221133211322112211213322113"},
        W  => A{decay: sv![Ta],                    seq: "312211322212221121123222113"},
        Re => A{decay: sv![Ge, Ca, W],             seq: "111312211312113221133211322112211213322113"},
        Os => A{decay: sv![Re],                    seq: "1321132122211322212221121123222113"},
        Ir => A{decay: sv![Os],                    seq: "3113112211322112211213322113"},
        Pt => A{decay: sv![Ir],                    seq: "111312212221121123222113"},
        Au => A{decay: sv![Pt],                    seq: "132112211213322113"},
        Hg => A{decay: sv![Au],                    seq: "31121123222113"},
        Tl => A{decay: sv![Hg],                    seq: "111213322113"},
        Pb => A{decay: sv![Tl],                    seq: "123222113"},
        Bi => A{decay: sv![Pm, Pb],                seq: "3113322113"},
        Po => A{decay: sv![Bi],                    seq: "1113222113"},
        At => A{decay: sv![Po],                    seq: "1322113"},
        Rn => A{decay: sv![Ho, At],                seq: "311311222113"},
        Fr => A{decay: sv![Rn],                    seq: "1113122113"},
        Ra => A{decay: sv![Fr],                    seq: "132113"},
        Ac => A{decay: sv![Ra],                    seq: "3113"},
        Th => A{decay: sv![Ac],                    seq: "1113"},
        Pa => A{decay: sv![Th],                    seq: "13"},
        U  => A{decay: sv![Pa],                     seq: "3"},
    }
});

fn look_and_say(n: usize, input: &str) -> usize {
    let mut state: EnumMap<Atom, usize> = EnumMap::default();
    let start = MAPPINGS.iter().find_map(|(k, v)| (input == v.seq).then_some(k)).unwrap();
    state[start] += 1;
    for _ in 0..n {
        let mut next: EnumMap<Atom, usize> = EnumMap::default();
        for (k, v) in state {
            if v > 0 {
                for &a in &MAPPINGS[k].decay {
                    next[a] += v;
                }
            }
        }
        state = next;
    }
    state.into_iter().map(|(k, v)| v * MAPPINGS[k].seq.len()).sum()
}

pub fn part1(input: &str) -> usize {
    look_and_say(40, input)
}

pub fn part2(input: &str) -> usize {
    look_and_say(50, input)
}
