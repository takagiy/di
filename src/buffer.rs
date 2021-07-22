use std::{
    borrow::{Borrow, BorrowMut},
    collections::{
        linked_list::{self, CursorMut},
        LinkedList,
    },
    ptr::NonNull,
};

pub struct PieceTable {
    buf_orig: String,
    buf_add: String,
    pieces: LinkedList<Piece>,
}

pub struct Cursor<'a> {
    table: NonNull<PieceTable>,
    cursor: CursorMut<'a, Piece>,
    cursor_index: usize,
}

struct Piece {
    kind: Kind,
    begin: usize,
    end: usize,
}

#[derive(Copy, Clone)]
enum Kind {
    Original,
    Add,
}

struct Iter<'a> {
    table: NonNull<PieceTable>,
    pieces: linked_list::Iter<'a, Piece>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        match self.pieces.next() {
            None => None,
            Some(p) => Some(match p.kind {
                Kind::Add => &unsafe { self.table.as_ref() }.buf_add[p.begin..p.end],
                Kind::Original => &unsafe { self.table.as_ref() }.buf_orig[p.begin..p.end],
            }),
        }
    }
}

impl PieceTable {
    pub fn new() -> Self {
        PieceTable {
            buf_orig: String::new(),
            buf_add: String::new(),
            pieces: LinkedList::new(),
        }
    }

    pub fn cursor(&mut self) -> Cursor {
        Cursor {
            table: unsafe { NonNull::new_unchecked(self as *mut PieceTable) },
            cursor: unsafe { NonNull::new_unchecked(self as *mut PieceTable).as_mut() }
                .pieces
                .cursor_front_mut(),
            cursor_index: 0,
        }
    }
}

impl Cursor<'_> {
    pub fn iter(&self) -> impl Iterator<Item = char> + '_ {
        Iter {
            table: self.table,
            pieces: unsafe { self.table.as_ref() }.pieces.iter(),
        }
        .flat_map(|s| s.chars())
    }

    pub fn insert(&mut self, x: &str) {
        let inserted_len = x.len();
        let begin = unsafe { self.table.as_ref() }.buf_add.len();
        unsafe { self.table.as_mut() }.buf_add.push_str(x);
        let end = unsafe { self.table.as_ref() }.buf_add.len();
        let inserted = Piece {
            kind: Kind::Add,
            begin,
            end,
        };
        self.split_piece();
        self.cursor.insert_after(inserted);
        self.cursor.move_next();
        self.cursor_index = inserted_len;
    }

    pub fn move_next(&mut self) {
        match self.cursor.current() {
            Some(p) => {
                if p.end - p.begin == self.cursor_index {
                    if let Some(_) = self.cursor.peek_next() {
                        self.cursor.move_next();
                        self.cursor_index = 0;
                    }
                } else {
                    self.cursor_index += 1;
                }
            }
            None => (),
        }
    }

    pub fn move_prev(&mut self) {
        match self.cursor.current() {
            Some(_) => {
                if self.cursor_index == 0 {
                    if let Some(prev) = self.cursor.peek_prev() {
                        self.cursor_index = prev.end - prev.begin;
                        self.cursor.move_prev();
                    }
                } else {
                    self.cursor_index -= 1;
                }
            }
            None => (),
        }
    }

    fn split_piece(&mut self) {
        match self.cursor.index() {
            None => {}
            Some(_) => {
                let new = {
                    let p = self.cursor.current().unwrap();
                    if p.end - p.begin == self.cursor_index {
                        return;
                    }
                    let end = p.end;
                    p.end = p.begin + self.cursor_index;
                    Piece {
                        kind: p.kind,
                        begin: p.end,
                        end,
                    }
                };
                self.cursor.insert_after(new);
            }
        }
    }
}
