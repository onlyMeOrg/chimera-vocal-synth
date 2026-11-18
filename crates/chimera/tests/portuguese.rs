//! Brazilian Portuguese spelling → sounds (`phonetics::portuguese`), by word and through lyrics.
use chimera::phonetics::english::EnglishG2P;
use chimera::phonetics::mandarin::MandarinLexicon;
use chimera::phonetics::portuguese;
use chimera::phonetics::syllables::{lyrics_to_syllables_in, lyrics_to_syllables_with};

/// the sounds in order; a nasal vowel is followed by ~
fn ipa(segs: &[chimera::phonetics::Segment]) -> String {
    segs.iter().map(|s| if s.nasalized == Some(true) { format!("{}~", s.ipa) } else { s.ipa.clone() }).collect::<Vec<_>>().join(" ")
}
fn said(w: &str) -> String {
    ipa(&portuguese::word(w, false))
}
fn texts(w: &str) -> String {
    portuguese::syllables(w, false).iter().map(|s| s.text.clone()).collect::<Vec<_>>().join("-")
}
fn stressed(w: &str) -> String {
    portuguese::syllables(w, false).iter().find(|s| s.nucleus.stress == Some(1)).map(|s| s.text.clone()).unwrap_or_default()
}

#[test]
fn the_last_vowel_without_stress_closes() {
    assert_eq!(said("casa"), "k a z ɐ");
    assert_eq!(said("falo"), "f a l u");
    assert_eq!(said("nome"), "n o m i");
    assert_eq!(said("casas"), "k a z ɐ s");
    assert_eq!(said("livros"), "l i v ɾ u s");
    // stressed, it keeps its vowel
    assert_eq!(said("café"), "k a f ɛ");
    assert_eq!(said("você"), "v o s e");
    assert_eq!(said("avó"), "a v ɔ");
    assert_eq!(said("avô"), "a v o");
}

#[test]
fn t_and_d_before_i_are_affricates() {
    assert_eq!(said("tia"), "tʃ i ɐ");
    assert_eq!(said("dia"), "dʒ i ɐ");
    assert_eq!(said("noite"), "n o j tʃ i");
    assert_eq!(said("cidade"), "s i d a dʒ i");
    assert_eq!(said("de"), "dʒ i");
    // not before other vowels
    assert_eq!(said("tudo"), "t u d u");
    assert_eq!(said("data"), "d a t ɐ");
}

#[test]
fn stress_follows_the_accent_or_the_ending() {
    assert_eq!(stressed("música"), "mú");
    assert_eq!(stressed("coração"), "ção");
    assert_eq!(stressed("casa"), "ca");
    assert_eq!(stressed("falam"), "fa");
    assert_eq!(stressed("homens"), "ho");
    assert_eq!(stressed("falar"), "lar");
    assert_eq!(stressed("Brasil"), "sil");
    assert_eq!(stressed("aqui"), "qui");
    assert_eq!(stressed("irmã"), "mã");
    assert_eq!(stressed("falei"), "lei");
    assert_eq!(stressed("órgão"), "ór");
    // words that lean on the next have none
    assert_eq!(stressed("que"), "");
    assert_eq!(said("que"), "k i");
    assert_eq!(said("o"), "u");
}

#[test]
fn nasal_vowels_take_the_m_or_n_that_closes_them() {
    assert_eq!(said("canto"), "k ɐ~ t u");
    assert_eq!(said("samba"), "s ɐ~ b ɐ");
    assert_eq!(said("fim"), "f i~");
    assert_eq!(said("som"), "s o~");
    assert_eq!(said("um"), "u~");
    assert_eq!(said("irmã"), "i ɾ m ɐ~");
    assert_eq!(said("também"), "t ɐ~ b eɪ~");
    assert_eq!(said("homens"), "o m eɪ~ s");
    assert_eq!(said("falam"), "f a l ɐ~ w");
    assert_eq!(said("pão"), "p ɐ~ w");
    assert_eq!(said("mãe"), "m ɐ~ j");
    assert_eq!(said("põe"), "p o~ j");
    assert_eq!(said("coração"), "k o ɾ a s ɐ~ w");
    // m and n before a vowel are consonants
    assert_eq!(said("amar"), "a m a ɾ");
    assert_eq!(said("banana"), "b a n a n ɐ");
    assert_eq!(said("tinta"), "tʃ i~ t ɐ");
}

#[test]
fn diphthongs_and_where_i_and_u_stay_vowels() {
    assert_eq!(said("pai"), "p aɪ");
    assert_eq!(said("meu"), "m e w");
    assert_eq!(said("foi"), "f o j");
    assert_eq!(said("muito"), "m u j t u");
    assert_eq!(said("céu"), "s ɛ w");
    assert_eq!(said("herói"), "e ɾ ɔɪ");
    assert_eq!(said("falei"), "f a l eɪ");
    assert_eq!(said("ouro"), "oʊ ɾ u");
    assert_eq!(said("aula"), "aʊ l ɐ");
    assert_eq!(texts("saída"), "sa-í-da");
    assert_eq!(texts("baú"), "ba-ú");
    assert_eq!(texts("rainha"), "ra-i-nha");
    assert_eq!(texts("juiz"), "ju-iz");
    assert_eq!(texts("cair"), "ca-ir");
    assert_eq!(texts("ruim"), "ru-im");
    assert_eq!(texts("rua"), "ru-a");
    // an unstressed i before the last vowel is not a note of its own
    assert_eq!(texts("história"), "his-tó-ria");
    assert_eq!(said("história"), "i s t ɔ ɾ j ɐ");
    assert_eq!(texts("silêncio"), "si-lên-cio");
    assert_eq!(said("pátio"), "p a tʃ j u");
    assert_eq!(texts("dia"), "di-a");
    assert_eq!(texts("rio"), "ri-o");
}

#[test]
fn consonants() {
    assert_eq!(said("chuva"), "ʃ u v ɐ");
    assert_eq!(said("filho"), "f i ʎ u");
    assert_eq!(said("manhã"), "m a ɲ ɐ~");
    assert_eq!(said("gente"), "ʒ e~ tʃ i");
    assert_eq!(said("janela"), "ʒ a n e l ɐ");
    assert_eq!(said("cedo"), "s e d u");
    assert_eq!(said("coração"), "k o ɾ a s ɐ~ w");
    assert_eq!(said("quero"), "k e ɾ u");
    assert_eq!(said("guitarra"), "g i t a h ɐ");
    assert_eq!(said("quando"), "k w ɐ~ d u");
    assert_eq!(said("água"), "a g w ɐ");
    assert_eq!(said("casa"), "k a z ɐ");
    assert_eq!(said("passo"), "p a s u");
    assert_eq!(said("mesmo"), "m e z m u");
    assert_eq!(said("nascer"), "n a s e ɾ");
    assert_eq!(said("hoje"), "o ʒ i");
    assert_eq!(said("Brasil"), "b ɾ a z i w");
    assert_eq!(said("alto"), "a w t u");
    assert_eq!(said("luz"), "l u s");
    assert_eq!(said("fazer"), "f a z e ɾ");
}

#[test]
fn the_two_rs() {
    assert_eq!(said("rio"), "h i u");
    assert_eq!(said("carro"), "k a h u");
    assert_eq!(said("honra"), "o~ h ɐ");
    assert_eq!(said("caro"), "k a ɾ u");
    assert_eq!(said("prato"), "p ɾ a t u");
    assert_eq!(said("porta"), "p o ɾ t ɐ");
    assert_eq!(said("amor"), "a m o ɾ");
}

#[test]
fn x() {
    assert_eq!(said("xícara"), "ʃ i k a ɾ ɐ");
    assert_eq!(said("peixe"), "p eɪ ʃ i");
    assert_eq!(said("exame"), "e z a m i");
    assert_eq!(said("texto"), "t e s t u");
    assert_eq!(said("táxi"), "t a k s i");
    assert_eq!(said("próximo"), "p ɾ ɔ s i m u");
}

#[test]
fn syllables_keep_their_letters() {
    assert_eq!(texts("saudade"), "sau-da-de");
    assert_eq!(texts("coração"), "co-ra-ção");
    assert_eq!(texts("palavra"), "pa-la-vra");
    assert_eq!(texts("esta"), "es-ta");
    assert_eq!(texts("carro"), "ca-rro");
    assert_eq!(texts("filho"), "fi-lho");
    assert_eq!(texts("quando"), "quan-do");
    assert_eq!(texts("homem"), "ho-mem");
    assert_eq!(texts("canto"), "can-to");
    assert_eq!(texts("Brasil"), "bra-sil");
}

#[test]
fn lyrics_are_portuguese_only_when_said_so() {
    let g = EnglishG2P::new();
    let zh = MandarinLexicon::new();
    let pt = lyrics_to_syllables_in("Você é, meu amor. Guarda-chuva", "pt", &g, &zh);
    let shown: Vec<&str> = pt.iter().map(|s| s.text.as_str()).collect();
    assert_eq!(shown, ["Vo", "cê", "é", "meu", "a", "mor", "Guar", "da", "chu", "va"]);
    assert!(pt[2].phrase_end && pt[1].word_end && !pt[0].word_end);
    // é is also a pinyin syllable: said to be Portuguese, it is Portuguese's
    assert_eq!(pt[2].nucleus.ipa, "ɛ");
    // unsaid, or said to be English, the lyrics read exactly as before
    let text = "twinkle little star, café";
    assert_eq!(lyrics_to_syllables_in(text, "en", &g, &zh), lyrics_to_syllables_with(text, &g, &zh));
    // other scripts are read as always beside Portuguese
    let mixed = lyrics_to_syllables_in("amor 별", "pt", &g, &zh);
    assert_eq!(mixed.len(), 3);
}

#[test]
fn every_sound_is_in_the_inventory() {
    use chimera::anatomy::human::BodyMix;
    use chimera::phonetics::inventory::phoneme;
    use chimera::phonetics::planner::is_vowel;
    let words = "saudade coração Brasil quando água próximo táxi também homens falam muito rainha guitarra manhã filho chuva hoje exame carro honra mesmo põe mãe céu herói ouro";
    for w in words.split(' ') {
        for s in portuguese::word(w, false) {
            assert!(is_vowel(&s.ipa, BodyMix::MALE) || phoneme(&s.ipa, BodyMix::MALE).is_some(), "{w}: no sound for {}", s.ipa);
        }
    }
}

/// A sung Portuguese line plans a note per syllable, every vowel on its beat, and the voice sounds
/// through it: nasal vowels, [ʎ], [ɲ], the tap and the affricates all reach the airway.
#[test]
fn a_portuguese_line_is_sung() {
    use chimera::anatomy::human::BODY_MALE;
    use chimera::phonetics::sing::{SongOptions, SungNote, plan_song};
    use chimera::voice::human_voice::{HumanVoice, SourcePatch, oversample_for};
    let g = EnglishG2P::new();
    let syl = lyrics_to_syllables_in("a noite cai, minha filha, e eu canto também", "pt", &g, &MandarinLexicon::new());
    assert_eq!(syl.len(), 14);
    let (bpm, beats, t0) = (100.0, 1.0, 0.5);
    let notes: Vec<SungNote> = syl.iter().enumerate().map(|(i, s)| SungNote { midi: if i % 2 == 0 { 57.0 } else { 60.0 }, beats, syl: s.clone() }).collect();
    let plan = plan_song(&notes, &SongOptions { body: "male".into(), bpm, t0, vibrato: 0.0, ..Default::default() });
    let segs = &plan.segments;
    // each note's vowel starts on its beat (two notes can meet vowel to vowel: "e eu")
    for k in 0..notes.len() {
        let want = t0 + k as f64 * beats * 60.0 / bpm;
        assert!(segs.iter().any(|s| s.kind == "vowel" && (s.t0 - want).abs() < 1e-6), "no vowel starts on beat {k} ({}), {want:.3} s", syl[k].text);
    }
    let fs = 48000.0;
    let mut v = HumanVoice::new(fs, BODY_MALE, oversample_for(fs));
    v.set_source(&SourcePatch::f0(220.0));
    v.schedule(&plan.gestures);
    let mut out = vec![0f32; (fs * (plan.end + 0.3)) as usize];
    for block in out.chunks_mut(512) {
        v.render(block);
    }
    assert!(out.iter().all(|x| x.is_finite()), "the voice broke");
    // every note sounds: the level around the middle of each
    for k in 0..notes.len() {
        let mid = ((t0 + (k as f64 + 0.5) * beats * 60.0 / bpm) * fs) as usize;
        let rms = (out[mid - 960..mid + 960].iter().map(|x| (*x as f64).powi(2)).sum::<f64>() / 1920.0).sqrt();
        assert!(rms > 1e-3, "note {k} ({}) is silent: {rms:.5}", syl[k].text);
    }
}
