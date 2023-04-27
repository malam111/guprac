pub enum RawNote {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

struct Note {
    raw: RawNote,
    decorators: Decorators,
}