//! Brazilian Portuguese spelling → the existing inventory (no new sounds, no data).
//!
//! Portuguese spelling says nearly everything about a word's sounds, so this is rules, as Russian
//! and Korean are. What it reads:
//!
//! - **Stress**: a written accent wins (café, você, música); without one, a word ending in a, e,
//!   o (with or without s), am, em or ens is stressed on the syllable before the last (casa,
//!   falam, homens) and any other on the last (falar, Brasil, aqui, irmã). A short list of
//!   words that lean on their neighbours (o, a, de, que, me, se, em…) has no stress.
//! - **The last vowel, unstressed**: e is [i] and o is [u] (noite [nojtʃi], falo [falu]), a is
//!   [ɐ]. Sung too: Brazilian singers say them so.
//! - **t and d before [i]** are [tʃ] and [dʒ] (tia, dia, noite, cidade), the mark of most
//!   Brazilian accents.
//! - **Nasal vowels**: ã, õ, and a vowel before m or n that closes its syllable (canto, bem,
//!   fim, som, um): the vowel is nasalised and the m or n is not said. A final -em is [ẽj]
//!   (também), a final unstressed -am [ɐ̃w] (falam), ão [ɐ̃w], ãe [ɐ̃j], õe [õj].
//! - **Diphthongs**: an unstressed i or u after a vowel is a glide (pai, meu, foi, muito), unless
//!   it has an accent (saída, baú), stands before nh (rainha) or closes the word with l, r, z, m
//!   or n (juiz, cair, ruim).
//! - **An unstressed i before the last, unstressed vowel** glides into it: his-tó-ria, si-lên-cio,
//!   fa-mí-lia are three notes, not four (dia and rio, where the i is stressed, are two).
//! - **Consonants**: ch [ʃ], lh [ʎ], nh [ɲ], j and g before e or i [ʒ], c before e or i and ç
//!   [s], qu and gu before e or i [k g] (before a or o [kw gw]), s between vowels [z], a
//!   syllable's last s voiced before a voiced consonant (mesmo), h silent, a syllable's last l
//!   [w] (Brasil, alto), z at a word's end [s].
//! - **r**: at a word's start, doubled, and after n, l or s it is [h] (rio, carro, honra);
//!   elsewhere a tap [ɾ] (caro, prato, falar, porta).
//!
//! Approximations, each a known gap:
//! - **Open or closed e and o** under stress are not in the spelling (avó and avô are told apart
//!   only by the accent; "ela" is open, "ele" closed). Without an accent both read closed.
//! - **x** is [ʃ] (xícara, peixe, caixa), [z] in ex- before a vowel (exame), and [ks] or [s] only
//!   in the words listed in `X_AS`.
//! - A nasal vowel before a stop has in speech a short nasal consonant after it (canto
//!   [kɐ̃ⁿtu]); here it has none.
//! - One accent of Brazilian Portuguese: final r is a tap (São Paulo's), not the [h] of Rio or
//!   dropped; s at a syllable's end is [s], not Rio's [ʃ].
//! - Words do not lean on each other: no linking of a final s or z to the next word's vowel, and
//!   two vowels meeting across words are two notes.
use super::syllables::FrontEnd;
use super::{Segment, Syllable};

/// Words said as part of the next one: no stress, so their last e and o reduce (de [dʒi], o [u]).
#[rustfmt::skip]
const CLITICS: &[&str] = &[
    "o", "a", "os", "as", "e", "de", "do", "da", "dos", "das", "no", "na", "nos", "nas", "em", "um",
    "que", "me", "te", "se", "lhe", "nos", "por", "com", "sem", "mas", "ou", "ao", "aos", "pelo", "pela",
    "meu", "teu", "seu",
];

/// Words where x is not [ʃ]: the sounds it has, by the word's start.
#[rustfmt::skip]
const X_AS: &[(&str, &[&str])] = &[
    ("táxi", &["k", "s"]), ("taxi", &["k", "s"]), ("fixo", &["k", "s"]), ("fixa", &["k", "s"]), ("sexo", &["k", "s"]),
    ("tórax", &["k", "s"]), ("reflex", &["k", "s"]), ("complex", &["k", "s"]), ("anex", &["k", "s"]), ("tóxic", &["k", "s"]),
    ("oxig", &["k", "s"]), ("máxim", &["s"]), ("maxim", &["s"]), ("próxim", &["s"]), ("proxim", &["s"]), ("troux", &["s"]),
    ("auxíli", &["s"]), ("auxili", &["s"]), ("sintax", &["s"]),
];

fn base(c: char) -> Option<char> {
    Some(match c {
        'a' | 'á' | 'â' | 'ã' | 'à' => 'a',
        'e' | 'é' | 'ê' => 'e',
        'i' | 'í' | 'y' => 'i',
        'o' | 'ó' | 'ô' | 'õ' => 'o',
        'u' | 'ú' | 'ü' => 'u',
        _ => return None,
    })
}
fn is_vowel_letter(c: char) -> bool {
    base(c).is_some()
}
/// e or i, with any accent: the letters that soften c, g, and silence the u of qu and gu
fn front(c: Option<char>) -> bool {
    matches!(c.and_then(base), Some('e' | 'i'))
}
pub fn is_letter(c: char) -> bool {
    c.is_ascii_alphabetic() || matches!(c.to_lowercase().next(), Some('á' | 'â' | 'ã' | 'à' | 'é' | 'ê' | 'í' | 'ó' | 'ô' | 'õ' | 'ú' | 'ü' | 'ç'))
}

/// One sound of the word, with the letters it was read from.
#[derive(Clone, Debug)]
struct Tok {
    /// the first letter of this sound and one past its last (silent letters go with a neighbour)
    from: usize,
    to: usize,
    kind: Kind,
}
#[derive(Clone, Debug, PartialEq)]
enum Kind {
    Cons(&'static str),
    /// a consonant that only ends a syllable: the [w] an l turns into
    Coda(&'static str),
    Vowel(Vowel),
}
#[derive(Clone, Debug, PartialEq)]
struct Vowel {
    letter: char,
    /// the plain vowel under the accent
    base: char,
    nasal: bool,
    /// the i or u gliding off this vowel
    glide: Option<&'static str>,
    /// this letter is itself a glide of the vowel before (set in the second pass)
    is_glide: bool,
}

/// Letters (lowercase, composed) of a word; anything else is dropped.
fn letters(input: &str) -> Vec<char> {
    let mut out: Vec<char> = Vec::new();
    for c in input.to_lowercase().chars() {
        // accents typed as separate marks join their letter
        let joined = match (out.last().copied(), c) {
            (Some('a'), '\u{301}') => Some('á'),
            (Some('a'), '\u{302}') => Some('â'),
            (Some('a'), '\u{303}') => Some('ã'),
            (Some('a'), '\u{300}') => Some('à'),
            (Some('e'), '\u{301}') => Some('é'),
            (Some('e'), '\u{302}') => Some('ê'),
            (Some('i'), '\u{301}') => Some('í'),
            (Some('o'), '\u{301}') => Some('ó'),
            (Some('o'), '\u{302}') => Some('ô'),
            (Some('o'), '\u{303}') => Some('õ'),
            (Some('u'), '\u{301}') => Some('ú'),
            (Some('c'), '\u{327}') => Some('ç'),
            _ => None,
        };
        if let Some(j) = joined {
            *out.last_mut().unwrap() = j;
        } else if is_letter(c) {
            out.push(c);
        }
    }
    out
}

fn voiced_consonant(c: Option<char>) -> bool {
    matches!(c, Some('b' | 'd' | 'g' | 'v' | 'z' | 'j' | 'm' | 'n' | 'l' | 'r'))
}

/// The word's sounds in order, before stress decides the last vowel and t, d.
fn scan(l: &[char]) -> Vec<Tok> {
    let n = l.len();
    let word: String = l.iter().collect();
    let x_as = X_AS.iter().find(|(start, _)| word.starts_with(start)).map(|(_, s)| *s);
    let at = |i: usize| l.get(i).copied();
    let mut out: Vec<Tok> = Vec::new();
    let mut i = 0;
    // letters with no sound of their own (h, the u of que) wait for the next sound to carry them
    let mut pending = 0;
    while i < n {
        let c = l[i];
        let next = at(i + 1);
        let prev_vowel = out.last().is_some_and(|t| matches!(t.kind, Kind::Vowel(_)));
        fn push_at(i: usize, out: &mut Vec<Tok>, kind: Kind, len: usize, pending: &mut usize) {
            out.push(Tok { from: i - *pending, to: i + len, kind });
            *pending = 0;
        }
        if let Some(b) = base(c) {
            // ü is said (an old spelling: lingüiça); the u of qu and gu is handled with q and g
            push_at(i, &mut out, Kind::Vowel(Vowel { letter: c, base: b, nasal: matches!(c, 'ã' | 'õ'), glide: None, is_glide: false }), 1, &mut pending);
            i += 1;
            continue;
        }
        let mut len = 1;
        let kind: Option<Kind> = match c {
            'h' => None,
            'c' if next == Some('h') => {
                len = 2;
                Some(Kind::Cons("ʃ"))
            }
            'l' if next == Some('h') => {
                len = 2;
                Some(Kind::Cons("ʎ"))
            }
            'n' if next == Some('h') => {
                len = 2;
                Some(Kind::Cons("ɲ"))
            }
            'c' => Some(Kind::Cons(if front(next) { "s" } else { "k" })),
            'ç' => Some(Kind::Cons("s")),
            'g' | 'q' => {
                let hard = if c == 'g' { "g" } else { "k" };
                if next == Some('u') && at(i + 2).is_some_and(is_vowel_letter) {
                    len = 2;
                    if front(at(i + 2)) {
                        Some(Kind::Cons(hard)) // que, gui: the u is silent
                    } else {
                        push_at(i, &mut out, Kind::Cons(hard), 1, &mut pending);
                        i += 1;
                        len = 1;
                        Some(Kind::Cons("w")) // quando, água
                    }
                } else if c == 'g' && front(next) {
                    Some(Kind::Cons("ʒ"))
                } else {
                    Some(Kind::Cons(hard))
                }
            }
            'j' => Some(Kind::Cons("ʒ")),
            'x' => match x_as {
                Some(sounds) => {
                    for s in &sounds[..sounds.len() - 1] {
                        out.push(Tok { from: i - pending, to: i, kind: Kind::Cons(s) });
                        pending = 0;
                    }
                    Some(Kind::Cons(sounds[sounds.len() - 1]))
                }
                // exame, exemplo, êxito: [z]
                None if i == 1 && matches!(l[0], 'e' | 'ê') && next.is_some_and(is_vowel_letter) => Some(Kind::Cons("z")),
                // texto, extra: the end of a syllable
                None if i > 0 && next.is_some_and(|c| !is_vowel_letter(c)) => Some(Kind::Cons("s")),
                None => Some(Kind::Cons("ʃ")),
            },
            'z' => Some(Kind::Cons(if i + 1 == n { "s" } else { "z" })),
            's' => {
                if next == Some('s') {
                    len = 2;
                    Some(Kind::Cons("s"))
                } else if next == Some('c') && front(at(i + 2)) {
                    len = 2; // nascer, piscina
                    Some(Kind::Cons("s"))
                } else if prev_vowel && next.is_some_and(is_vowel_letter) {
                    Some(Kind::Cons("z"))
                } else if voiced_consonant(next) {
                    Some(Kind::Cons("z")) // mesmo, desde
                } else {
                    Some(Kind::Cons("s"))
                }
            }
            'r' => {
                if next == Some('r') {
                    len = 2;
                    Some(Kind::Cons("h"))
                } else if i == 0 || matches!(at(i - 1), Some('n' | 'l' | 's')) {
                    Some(Kind::Cons("h"))
                } else {
                    Some(Kind::Cons("ɾ"))
                }
            }
            'l' => Some(if prev_vowel && !next.is_some_and(is_vowel_letter) { Kind::Coda("w") } else { Kind::Cons("l") }),
            'm' | 'n' => {
                // closing its syllable after a vowel, it is the vowel's nasality and is not said
                // (a doubled n or an mn keeps the second as a consonant: connosco, amnésia)
                let closes = prev_vowel && !next.is_some_and(is_vowel_letter) && next != Some('h');
                if closes {
                    if let Some(Tok { to, kind: Kind::Vowel(v), .. }) = out.last_mut() {
                        v.nasal = true;
                        *to = i + 1;
                    }
                    i += 1;
                    continue;
                }
                Some(Kind::Cons(if c == 'm' { "m" } else { "n" }))
            }
            'b' => Some(Kind::Cons("b")),
            'd' => Some(Kind::Cons("d")),
            'f' => Some(Kind::Cons("f")),
            'k' => Some(Kind::Cons("k")),
            'p' => Some(Kind::Cons("p")),
            't' => Some(Kind::Cons("t")),
            'v' => Some(Kind::Cons("v")),
            'w' => Some(Kind::Cons("w")),
            _ => None,
        };
        match kind {
            Some(k) => push_at(i, &mut out, k, len, &mut pending),
            None => pending += len,
        }
        i += len;
    }
    // letters left with no sound after them (a final h) go with the last sound
    if let Some(last) = out.last_mut() {
        last.to = n;
    }
    out
}

/// An unstressed i or u after a vowel glides off it (pai, meu, muito, ão, mãe, põe).
fn glides(toks: &mut [Tok], l: &[char]) {
    let n = toks.len();
    for k in 1..n {
        let (before, rest) = toks.split_at_mut(k);
        let Kind::Vowel(prev) = &mut before[k - 1].kind else { continue };
        if prev.is_glide || prev.glide.is_some() {
            continue;
        }
        let end = rest[0].to;
        let after = rest.get(1).map(|t| t.kind.clone());
        let Kind::Vowel(v) = &mut rest[0].kind else { continue };
        let nasal_pair = matches!((prev.letter, v.letter), ('ã', 'o' | 'e') | ('õ', 'e'));
        let plain = matches!(v.letter, 'i' | 'u') && !v.nasal && prev.letter != v.letter;
        if !nasal_pair && !plain {
            continue;
        }
        if plain {
            // rainha, moinho: before nh the i is its own syllable
            if l.get(end) == Some(&'n') && l.get(end + 1) == Some(&'h') {
                continue;
            }
            // juiz, cair, Raul, ruim: closing the word with a consonant, it is the stressed vowel
            let closes_word = k + 2 == n && matches!(after, Some(Kind::Cons(_) | Kind::Coda(_))) && !matches!(l.last(), Some('s'));
            if closes_word {
                continue;
            }
            // ia, ua (dia, rua): the first is the vowel, the second cannot glide
            if matches!(prev.base, 'i' | 'u') && prev.base == v.base {
                continue;
            }
        }
        prev.glide = Some(if matches!(v.base, 'i' | 'e') { "j" } else { "w" });
        v.is_glide = true;
    }
}

/// A consonant cluster that can start a Portuguese syllable.
fn is_onset(cluster: &[&Kind]) -> bool {
    match cluster {
        [Kind::Cons(_)] => true,
        // the glide of his-tó-ria goes with whatever starts its syllable
        [rest @ .., Kind::Cons("j")] if !rest.is_empty() => is_onset(rest),
        [Kind::Cons(a), Kind::Cons(b)] => {
            (matches!(*a, "p" | "b" | "t" | "d" | "k" | "g" | "f" | "v") && matches!(*b, "ɾ" | "l")) || (matches!(*a, "k" | "g") && *b == "w")
        }
        _ => false,
    }
}

/// One word → its syllables, each with the letters it is written with. `reduce` is unused for
/// now: sung and spoken Portuguese reduce the same last vowel, and nothing else is reduced here.
pub fn syllables(input: &str, _reduce: bool) -> Vec<Syllable> {
    let l = letters(input);
    if l.is_empty() {
        return Vec::new();
    }
    let word: String = l.iter().collect();
    let mut toks = scan(&l);
    glides(&mut toks, &l);
    // the syllables' vowels
    let mut nuclei: Vec<usize> = toks.iter().enumerate().filter(|(_, t)| matches!(&t.kind, Kind::Vowel(v) if !v.is_glide)).map(|(i, _)| i).collect();
    if nuclei.is_empty() {
        return Vec::new();
    }
    fn vowel_of(toks: &[Tok], k: usize) -> Vowel {
        match &toks[k].kind {
            Kind::Vowel(v) => v.clone(),
            _ => unreachable!(),
        }
    }
    let vowel = |k: usize| vowel_of(&toks, k);
    // stress: the last written accent (a tilde only when there is no other), else by the ending
    let accented = |c: char| matches!(c, 'á' | 'â' | 'é' | 'ê' | 'í' | 'ó' | 'ô' | 'ú');
    let marked = nuclei.iter().rposition(|&k| accented(vowel(k).letter)).or_else(|| nuclei.iter().rposition(|&k| matches!(vowel(k).letter, 'ã' | 'õ')));
    let penult_ending = ["a", "e", "o", "as", "es", "os", "am", "em", "ens"].iter().any(|e| word.ends_with(e));
    let mut last = nuclei.len() - 1;
    // a last vowel with a glide is a stressed ending (falei, chapéu, depois), as i and u are
    let last_glides = vowel(nuclei[last]).glide.is_some();
    let stress: Option<usize> = if CLITICS.contains(&word.as_str()) {
        None
    } else if let Some(m) = marked {
        Some(m)
    } else if penult_ending && !last_glides && last > 0 {
        Some(last - 1)
    } else {
        Some(last)
    };
    // história, silêncio, família: an unstressed i right before the word's last, unstressed vowel
    // is not a note of its own; it glides into that vowel
    if last > 0 && stress != Some(last) && stress != Some(last - 1) {
        let (i, v) = (nuclei[last - 1], nuclei[last]);
        let glide = vowel_of(&toks, i);
        if v == i + 1 && glide.letter == 'i' && !glide.nasal && glide.glide.is_none() && vowel_of(&toks, v).glide.is_none() && i > 0 {
            toks[i].kind = Kind::Cons("j");
            nuclei.remove(last - 1);
            last -= 1;
        }
    }
    let vowel = |k: usize| vowel_of(&toks, k);

    // each vowel's sound
    let mut sound: Vec<(String, bool, Option<&'static str>)> = Vec::with_capacity(nuclei.len()); // (ipa, nasal, coda glide)
    for (n, &k) in nuclei.iter().enumerate() {
        let v = vowel(k);
        let stressed = stress == Some(n);
        let final_open = n == last && !stressed && v.glide.is_none() && !v.nasal && toks[k + 1..].iter().all(|t| matches!(t.kind, Kind::Cons("s")));
        let mut q = match v.letter {
            'é' => "ɛ",
            'ó' => "ɔ",
            'â' | 'ã' => "ɐ",
            _ => match v.base {
                'a' => "a",
                'e' => "e",
                'i' => "i",
                'o' => "o",
                _ => "u",
            },
        };
        if final_open {
            q = match v.base {
                'e' => "i",
                'o' => "u",
                'a' => "ɐ",
                _ => q,
            };
        }
        let ends_word = k + 1 == toks.len() || (k + 2 == toks.len() && matches!(toks[k + 1].kind, Kind::Cons("s")));
        let mut glide = v.glide;
        if v.nasal {
            // a nasal vowel is closed: ã [ɐ̃], and the é of também is [ẽ]
            q = match v.base {
                'a' => "ɐ",
                'e' => "e",
                'o' => "o",
                _ => q,
            };
            // também, homens [ẽj]; falam [ɐ̃w]
            let m_closed = l.get(toks[k].to.saturating_sub(1)).is_some_and(|c| matches!(c, 'm' | 'n'));
            if glide.is_none() && ends_word && m_closed {
                if v.base == 'e' {
                    glide = Some("j");
                } else if v.base == 'a' && !stressed && word.ends_with("am") {
                    glide = Some("w");
                }
            }
        }
        // the falling diphthongs the planner holds and glides off; the rest end in a glide
        let (ipa, coda) = match (q, glide) {
            ("a", Some("j")) => ("aɪ".to_string(), None),
            ("a", Some("w")) => ("aʊ".to_string(), None),
            ("e", Some("j")) => ("eɪ".to_string(), None),
            ("o", Some("w")) => ("oʊ".to_string(), None),
            ("ɔ", Some("j")) => ("ɔɪ".to_string(), None),
            (q, g) => (q.to_string(), g),
        };
        sound.push((ipa, v.nasal, coda));
    }

    // syllables: of the consonants between two vowels, the longest legal onset goes right
    let mut out: Vec<Syllable> = Vec::new();
    let mut start = 0; // first token of this syllable
    for (n, &k) in nuclei.iter().enumerate() {
        let mut end = toks.len();
        if let Some(&next) = nuclei.get(n + 1) {
            let between: Vec<usize> = (k + 1..next).filter(|&j| !matches!(&toks[j].kind, Kind::Vowel(_))).collect();
            let mut take = 0;
            for len in (1..=between.len()).rev() {
                let cluster: Vec<&Kind> = between[between.len() - len..].iter().map(|&j| &toks[j].kind).collect();
                if is_onset(&cluster) {
                    take = len;
                    break;
                }
            }
            end = if take == 0 { next } else { between[between.len() - take] };
        }
        let cons = |range: std::ops::Range<usize>| -> Vec<&'static str> {
            range.filter_map(|j| match toks[j].kind { Kind::Cons(c) | Kind::Coda(c) => Some(c), Kind::Vowel(_) => None }).collect()
        };
        let (ipa, nasal, glide) = &sound[n];
        // t and d before [i]
        let mut onset: Vec<&'static str> = cons(start..k);
        let before_i = if onset.last() == Some(&"j") { onset.len().checked_sub(2) } else if ipa.starts_with('i') { onset.len().checked_sub(1) } else { None };
        if let Some(c) = before_i.and_then(|k| onset.get_mut(k)) {
            *c = match *c {
                "t" => "tʃ",
                "d" => "dʒ",
                other => other,
            };
        }
        let mut coda: Vec<&'static str> = glide.iter().copied().collect();
        coda.extend(cons(k + 1..end));
        let nucleus = Segment { stress: Some(u8::from(stress == Some(n))), nasalized: nasal.then_some(true), ..Segment::new(ipa) };
        let text: String = l[if n == 0 { 0 } else { toks[start].from }..if end == toks.len() { l.len() } else { toks[end].from }].iter().collect();
        out.push(Syllable { text, onset: onset.into_iter().map(Segment::new).collect(), nucleus, coda: coda.into_iter().map(Segment::new).collect(), ..Default::default() });
        start = end;
    }
    out
}

/// One word → its sounds in order (Speak, and the tests).
pub fn word(input: &str, reduce: bool) -> Vec<Segment> {
    syllables(input, reduce).into_iter().flat_map(|s| s.onset.into_iter().chain(std::iter::once(s.nucleus)).chain(s.coda)).collect()
}

/// The Portuguese front end for sung lyrics. It reads the Latin alphabet, as English and pinyin
/// do, so it is not among the front ends lyrics go through by default: the page says the lyrics
/// are Portuguese (`lyrics_to_syllables_in`), and then it comes first.
pub struct Portuguese;

impl FrontEnd for Portuguese {
    fn starts(&self, c: char) -> bool {
        is_letter(c)
    }
    fn continues(&self, c: char) -> bool {
        is_letter(c) || matches!(c, '-' | '\u{300}'..='\u{303}' | '\u{327}')
    }
    fn word(&self, tok: &str) -> Vec<Syllable> {
        // typed hyphens are Portuguese's own (guarda-chuva, disse-me): each part is a word
        let mut syls: Vec<Syllable> = Vec::new();
        for part in tok.split('-').filter(|p| !p.is_empty()) {
            let mut sy = syllables(part, false);
            // the spelling keeps its capitals
            let shown: Vec<char> = part.chars().filter(|c| !matches!(c, '\u{300}'..='\u{303}' | '\u{327}')).collect();
            let mut at = 0;
            for s in &mut sy {
                let len = s.text.chars().count();
                if at + len <= shown.len() && shown.len() == letters(part).len() {
                    s.text = shown[at..at + len].iter().collect();
                }
                at += len;
            }
            syls.append(&mut sy);
        }
        syls
    }
}
