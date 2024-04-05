use super::languages::english::ENGLISHPOLYSEED;

pub(crate) struct WordsetPolyseed {
    pub name: &'static str,
    pub prefix_length: u8,
    pub words: [&'static str; 2048],
}

pub(crate) static WORDSETSPOLYSEED : [WordsetPolyseed; 1] = [
    ENGLISHPOLYSEED,
];