#![feature(linked_list_cursors)]

use std::collections::{LinkedList, linked_list::CursorMut};
use std::fmt;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SizeAndPercentage {
    size: usize,
    elems: usize,
    percentage: Percentage,
}

impl fmt::Display for SizeAndPercentage {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Size {}/Elements {}/Percentage {}/",
            self.size, self.elems, self.percentage
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Percentage {
    Zero,
    One,
    Ten,
    Fifty,
}

impl fmt::Display for Percentage {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn benchmark_eight_byte_array(arr: &mut Vec<u64>, inst_mix: Percentage) -> u64 {
    let mut cursor: usize = 0;
    // NOTE: Make sure that the instructions are not compiled away.
    let mut acc: u64 = 0;
    let n = arr.len() - 1;

    match inst_mix {
        Percentage::Zero => {
            for _ in 0..(n * 4) {
                acc ^= arr[cursor];
                cursor = (cursor + 1) % n;
                arr[cursor] = acc;
                cursor = (cursor + 1) % n;
            }
        }
        Percentage::One => {
            for _ in 0..(n.div_ceil(50)) {
                for _ in 0..49 {
                    acc ^= arr[cursor];
                    cursor = (cursor + 1) % n;
                    arr[cursor] = acc;
                    cursor = (cursor + 1) % n;
                }
                acc ^= arr[cursor];
                cursor = (cursor + 1) % n;
                arr.insert(cursor, acc);
                cursor = (cursor + 1) % n;
                for _ in 0..49 {
                    arr[cursor] = acc;
                    cursor = (cursor + 1) % n;
                    acc ^= arr[cursor];
                    cursor = (cursor + 1) % n;
                }
                arr[cursor] = acc;
                cursor = (cursor + 1) % n;
                arr.remove(cursor);
                cursor = (cursor + 1) % n;
            }
        }
        Percentage::Ten => {
            for _ in 0..(n.div_ceil(5)) {
                for _ in 0..4 {
                    acc ^= arr[cursor];
                    cursor = (cursor + 1) % n;
                    arr[cursor] = acc;
                    cursor = (cursor + 1) % n;
                }
                acc ^= arr[cursor];
                cursor = (cursor + 1) % n;
                arr.insert(cursor, acc);
                cursor = (cursor + 1) % n;
                for _ in 0..4 {
                    arr[cursor] = acc;
                    cursor = (cursor + 1) % n;
                    acc ^= arr[cursor];
                    cursor = (cursor + 1) % n;
                }
                arr[cursor] = acc;
                cursor = (cursor + 1) % n;
                arr.remove(cursor);
                cursor = (cursor + 1) % n;
            }
        }
        Percentage::Fifty => {
            for _ in 0..(n * 2) {
                acc ^= arr[cursor];
                cursor = (cursor + 1) % n;
                arr.insert(cursor, acc);
                cursor = (cursor + 1) % n;
                arr[cursor] = acc;
                cursor = (cursor + 1) % n;
                arr.remove(cursor);
                cursor = (cursor + 1) % n;
            }
        }
    }

    acc
}

fn benchmark_five_twelve_byte_array(arr: &mut Vec<[u64; 64]>, inst_mix: Percentage) -> u64 {
    let mut cursor: usize = 0;
    // NOTE: Make sure that the instructions are not compiled away.
    let mut acc: [u64; 64] = [0; 64];
    let n = arr.len() - 1;

    match inst_mix {
        Percentage::Zero => {
            for _ in 0..(n * 4) {
                acc = arr[cursor];
                cursor = (cursor + 1) % n;
                arr[cursor] = acc;
                cursor = (cursor + 1) % n;
            }
        }
        Percentage::One => {
            for _ in 0..(n.div_ceil(50)) {
                for _ in 0..49 {
                    acc = arr[cursor];
                    cursor = (cursor + 1) % n;
                    arr[cursor] = acc;
                    cursor = (cursor + 1) % n;
                }
                acc = arr[cursor];
                cursor = (cursor + 1) % n;
                arr.insert(cursor, acc);
                cursor = (cursor + 1) % n;
                for _ in 0..49 {
                    arr[cursor] = acc;
                    cursor = (cursor + 1) % n;
                    acc = arr[cursor];
                    cursor = (cursor + 1) % n;
                }
                arr[cursor] = acc;
                cursor = (cursor + 1) % n;
                arr.remove(cursor);
                cursor = (cursor + 1) % n;
            }
        }
        Percentage::Ten => {
            for _ in 0..(n.div_ceil(5)) {
                for _ in 0..4 {
                    acc = arr[cursor];
                    cursor = (cursor + 1) % n;
                    arr[cursor] = acc;
                    cursor = (cursor + 1) % n;
                }
                acc = arr[cursor];
                cursor = (cursor + 1) % n;
                arr.insert(cursor, acc);
                cursor = (cursor + 1) % n;
                for _ in 0..4 {
                    arr[cursor] = acc;
                    cursor = (cursor + 1) % n;
                    acc = arr[cursor];
                    cursor = (cursor + 1) % n;
                }
                arr[cursor] = acc;
                cursor = (cursor + 1) % n;
                arr.remove(cursor);
                cursor = (cursor + 1) % n;
            }
        }
        Percentage::Fifty => {
            for _ in 0..(n * 2) {
                acc = arr[cursor];
                cursor = (cursor + 1) % n;
                arr[cursor] = acc;
                cursor = (cursor + 1) % n;
                arr.insert(cursor, acc);
                cursor = (cursor + 1) % n;
                arr.remove(cursor);
                cursor = (cursor + 1) % n;
            }
        }
    }

    acc[0]
}

fn benchmark_eight_mega_byte_array(arr: &mut Vec<[u64; 1024]>, inst_mix: Percentage) -> u64 {
    let mut cursor: usize = 0;
    // NOTE: Make sure that the instructions are not compiled away.
    let mut acc: [u64; 1024] = [0; 1024];
    let n = arr.len() - 1;

    match inst_mix {
        Percentage::Zero => {
            for _ in 0..(n * 4) {
                acc = arr[cursor];
                cursor = (cursor + 1) % n;
                arr[cursor] = acc;
                cursor = (cursor + 1) % n;
            }
        }
        Percentage::One => {
            for _ in 0..(n.div_ceil(50)) {
                for _ in 0..49 {
                    acc = arr[cursor];
                    cursor = (cursor + 1) % n;
                    arr[cursor] = acc;
                    cursor = (cursor + 1) % n;
                }
                acc = arr[cursor];
                cursor = (cursor + 1) % n;
                arr.insert(cursor, acc);
                cursor = (cursor + 1) % n;
                for _ in 0..49 {
                    arr[cursor] = acc;
                    cursor = (cursor + 1) % n;
                    acc = arr[cursor];
                    cursor = (cursor + 1) % n;
                }
                arr[cursor] = acc;
                cursor = (cursor + 1) % n;
                arr.remove(cursor);
                cursor = (cursor + 1) % n;
            }
        }
        Percentage::Ten => {
            for _ in 0..(n.div_ceil(5)) {
                for _ in 0..4 {
                    acc = arr[cursor];
                    cursor = (cursor + 1) % n;
                    arr[cursor] = acc;
                    cursor = (cursor + 1) % n;
                }
                acc = arr[cursor];
                cursor = (cursor + 1) % n;
                arr.insert(cursor, acc);
                cursor = (cursor + 1) % n;
                for _ in 0..4 {
                    arr[cursor] = acc;
                    cursor = (cursor + 1) % n;
                    acc = arr[cursor];
                    cursor = (cursor + 1) % n;
                }
                arr[cursor] = acc;
                cursor = (cursor + 1) % n;
                arr.remove(cursor);
                cursor = (cursor + 1) % n;
            }
        }
        Percentage::Fifty => {
            for _ in 0..(n * 2) {
                acc = arr[cursor];
                cursor = (cursor + 1) % n;
                arr[cursor] = acc;
                cursor = (cursor + 1) % n;
                arr.insert(cursor, acc);
                cursor = (cursor + 1) % n;
                arr.remove(cursor);
                cursor = (cursor + 1) % n;
            }
        }
    }

    acc[0]
}

fn benchmark_eight_byte_list(l: &mut LinkedList<u64>, inst_mix: Percentage) -> u64 {
    // NOTE: Make sure that the instructions are not compiled away.
    let mut acc: u64 = 0;
    let n = l.len() - 1;

    let mut cursor = l.cursor_front_mut();

    match inst_mix {
        Percentage::Zero => {
            for _ in 0..(n * 4) {
                match cursor.current() {
                    Some(e) => acc ^= *e,
                    None => {
                        cursor.move_next();
                        acc ^= *cursor.current().unwrap()
                    }
                }
                match cursor.current() {
                    Some(e) => *e = acc,
                    None => {
                        cursor.move_next();
                        *cursor.current().unwrap() = acc;
                    }
                }
            }
        }
        Percentage::One => {
            for _ in 0..(n.div_ceil(50)) {
                for _ in 0..49 {
                    match cursor.current() {
                        Some(e) => acc ^= *e,
                        None => {
                            cursor.move_next();
                            acc ^= *cursor.current().unwrap()
                        }
                    }
                    match cursor.current() {
                        Some(e) => *e = acc,
                        None => {
                            cursor.move_next();
                            *cursor.current().unwrap() = acc;
                        }
                    }
                }
                match cursor.current() {
                    Some(e) => acc ^= *e,
                    None => {
                        cursor.move_next();
                        acc ^= *cursor.current().unwrap()
                    }
                }
                cursor.insert_before(acc);
                for _ in 0..49 {
                    match cursor.current() {
                        Some(e) => acc ^= *e,
                        None => {
                            cursor.move_next();
                            acc ^= *cursor.current().unwrap()
                        }
                    }
                    match cursor.current() {
                        Some(e) => *e = acc,
                        None => {
                            cursor.move_next();
                            *cursor.current().unwrap() = acc;
                        }
                    }
                }
                match cursor.current() {
                    Some(e) => *e = acc,
                    None => {
                        cursor.move_next();
                        *cursor.current().unwrap() = acc;
                    }
                }
                match cursor.remove_current() {
                    Some(e) => { acc = e; }
                    None => {
                        cursor.move_next();
                        let _ = cursor.remove_current();
                    }
                }
            }
        }
        Percentage::Ten => {
            for _ in 0..(n.div_ceil(5)) {
                for _ in 0..4 {
                    match cursor.current() {
                        Some(e) => acc ^= *e,
                        None => {
                            cursor.move_next();
                            acc ^= *cursor.current().unwrap()
                        }
                    }
                    match cursor.current() {
                        Some(e) => *e = acc,
                        None => {
                            cursor.move_next();
                            *cursor.current().unwrap() = acc;
                        }
                    }
                }
                match cursor.current() {
                    Some(e) => acc ^= *e,
                    None => {
                        cursor.move_next();
                        acc ^= *cursor.current().unwrap()
                    }
                }
                cursor.insert_before(acc);
                for _ in 0..4 {
                    match cursor.current() {
                        Some(e) => acc ^= *e,
                        None => {
                            cursor.move_next();
                            acc ^= *cursor.current().unwrap()
                        }
                    }
                    match cursor.current() {
                        Some(e) => *e = acc,
                        None => {
                            cursor.move_next();
                            *cursor.current().unwrap() = acc;
                        }
                    }
                }
                match cursor.current() {
                    Some(e) => *e = acc,
                    None => {
                        cursor.move_next();
                        *cursor.current().unwrap() = acc;
                    }
                }
                match cursor.remove_current() {
                    Some(e) => { acc = e; }
                    None => {
                        cursor.move_next();
                        let _ = cursor.remove_current();
                    }
                }
            }
        }
        Percentage::Fifty => {
            for _ in 0..(n * 2) {
                match cursor.current() {
                    Some(e) => acc ^= *e,
                    None => {
                        cursor.move_next();
                        acc ^= *cursor.current().unwrap()
                    }
                }
                match cursor.current() {
                    Some(e) => *e = acc,
                    None => {
                        cursor.move_next();
                        *cursor.current().unwrap() = acc;
                    }
                }
                cursor.insert_before(acc);
                match cursor.remove_current() {
                    Some(e) => { acc = e; }
                    None => {
                        cursor.move_next();
                        let _ = cursor.remove_current();
                    }
                }
            }
        }
    }

    acc
}

fn benchmark_five_twelve_byte_list(l: &mut LinkedList<[u64; 64]>, inst_mix: Percentage) -> u64 {
    // NOTE: Make sure that the instructions are not compiled away.
    let mut acc: [u64; 64] = [0; 64];
    let n = l.len() - 1;

    let mut cursor = l.cursor_front_mut();

    match inst_mix {
        Percentage::Zero => {
            for _ in 0..(n * 4) {
                match cursor.current() {
                    Some(e) => acc = *e,
                    None => {
                        cursor.move_next();
                        acc = *cursor.current().unwrap()
                    }
                }
                match cursor.current() {
                    Some(e) => *e = acc,
                    None => {
                        cursor.move_next();
                        *cursor.current().unwrap() = acc;
                    }
                }
            }
        }
        Percentage::One => {
            for _ in 0..(n.div_ceil(50)) {
                for _ in 0..49 {
                    match cursor.current() {
                        Some(e) => acc = *e,
                        None => {
                            cursor.move_next();
                            acc = *cursor.current().unwrap()
                        }
                    }
                    match cursor.current() {
                        Some(e) => *e = acc,
                        None => {
                            cursor.move_next();
                            *cursor.current().unwrap() = acc;
                        }
                    }
                }
                match cursor.current() {
                    Some(e) => acc = *e,
                    None => {
                        cursor.move_next();
                        acc = *cursor.current().unwrap()
                    }
                }
                cursor.insert_before(acc);
                for _ in 0..49 {
                    match cursor.current() {
                        Some(e) => acc = *e,
                        None => {
                            cursor.move_next();
                            acc = *cursor.current().unwrap()
                        }
                    }
                    match cursor.current() {
                        Some(e) => *e = acc,
                        None => {
                            cursor.move_next();
                            *cursor.current().unwrap() = acc;
                        }
                    }
                }
                match cursor.current() {
                    Some(e) => *e = acc,
                    None => {
                        cursor.move_next();
                        *cursor.current().unwrap() = acc;
                    }
                }
                match cursor.remove_current() {
                    Some(e) => { acc = e; }
                    None => {
                        cursor.move_next();
                        let _ = cursor.remove_current();
                    }
                }
            }
        }
        Percentage::Ten => {
            for _ in 0..(n.div_ceil(5)) {
                for _ in 0..4 {
                    match cursor.current() {
                        Some(e) => acc = *e,
                        None => {
                            cursor.move_next();
                            acc = *cursor.current().unwrap()
                        }
                    }
                    match cursor.current() {
                        Some(e) => *e = acc,
                        None => {
                            cursor.move_next();
                            *cursor.current().unwrap() = acc;
                        }
                    }
                }
                match cursor.current() {
                    Some(e) => acc = *e,
                    None => {
                        cursor.move_next();
                        acc = *cursor.current().unwrap()
                    }
                }
                cursor.insert_before(acc);
                for _ in 0..4 {
                    match cursor.current() {
                        Some(e) => acc = *e,
                        None => {
                            cursor.move_next();
                            acc = *cursor.current().unwrap()
                        }
                    }
                    match cursor.current() {
                        Some(e) => *e = acc,
                        None => {
                            cursor.move_next();
                            *cursor.current().unwrap() = acc;
                        }
                    }
                }
                match cursor.current() {
                    Some(e) => *e = acc,
                    None => {
                        cursor.move_next();
                        *cursor.current().unwrap() = acc;
                    }
                }
                match cursor.remove_current() {
                    Some(e) => { acc = e; }
                    None => {
                        cursor.move_next();
                        let _ = cursor.remove_current();
                    }
                }
            }
        }
        Percentage::Fifty => {
            for _ in 0..(n * 2) {
                match cursor.current() {
                    Some(e) => acc = *e,
                    None => {
                        cursor.move_next();
                        acc = *cursor.current().unwrap()
                    }
                }
                match cursor.current() {
                    Some(e) => *e = acc,
                    None => {
                        cursor.move_next();
                        *cursor.current().unwrap() = acc;
                    }
                }
                cursor.insert_before(acc);
                match cursor.remove_current() {
                    Some(e) => { acc = e; }
                    None => {
                        cursor.move_next();
                        let _ = cursor.remove_current();
                    }
                }
            }
        }
    }

    acc[0]
}

fn benchmark_eight_mega_byte_list(l: &mut LinkedList<[u64; 1024]>, inst_mix: Percentage) -> u64 {
    // NOTE: Make sure that the instructions are not compiled away.
    let mut acc: [u64; 1024] = [0; 1024];
    let n = l.len() - 1;

    let mut cursor = l.cursor_front_mut();

    match inst_mix {
        Percentage::Zero => {
            for _ in 0..(n * 4) {
                match cursor.current() {
                    Some(e) => acc = *e,
                    None => {
                        cursor.move_next();
                        acc = *cursor.current().unwrap()
                    }
                }
                match cursor.current() {
                    Some(e) => *e = acc,
                    None => {
                        cursor.move_next();
                        *cursor.current().unwrap() = acc;
                    }
                }
            }
        }
        Percentage::One => {
            for _ in 0..(n.div_ceil(50)) {
                for _ in 0..49 {
                    match cursor.current() {
                        Some(e) => acc = *e,
                        None => {
                            cursor.move_next();
                            acc = *cursor.current().unwrap()
                        }
                    }
                    match cursor.current() {
                        Some(e) => *e = acc,
                        None => {
                            cursor.move_next();
                            *cursor.current().unwrap() = acc;
                        }
                    }
                }
                match cursor.current() {
                    Some(e) => acc = *e,
                    None => {
                        cursor.move_next();
                        acc = *cursor.current().unwrap()
                    }
                }
                cursor.insert_before(acc);
                for _ in 0..49 {
                    match cursor.current() {
                        Some(e) => acc = *e,
                        None => {
                            cursor.move_next();
                            acc = *cursor.current().unwrap()
                        }
                    }
                    match cursor.current() {
                        Some(e) => *e = acc,
                        None => {
                            cursor.move_next();
                            *cursor.current().unwrap() = acc;
                        }
                    }
                }
                match cursor.current() {
                    Some(e) => *e = acc,
                    None => {
                        cursor.move_next();
                        *cursor.current().unwrap() = acc;
                    }
                }
                match cursor.remove_current() {
                    Some(e) => { acc = e; }
                    None => {
                        cursor.move_next();
                        let _ = cursor.remove_current();
                    }
                }
            }
        }
        Percentage::Ten => {
            for _ in 0..(n.div_ceil(5)) {
                for _ in 0..4 {
                    match cursor.current() {
                        Some(e) => acc = *e,
                        None => {
                            cursor.move_next();
                            acc = *cursor.current().unwrap()
                        }
                    }
                    match cursor.current() {
                        Some(e) => *e = acc,
                        None => {
                            cursor.move_next();
                            *cursor.current().unwrap() = acc;
                        }
                    }
                }
                match cursor.current() {
                    Some(e) => acc = *e,
                    None => {
                        cursor.move_next();
                        acc = *cursor.current().unwrap()
                    }
                }
                cursor.insert_before(acc);
                for _ in 0..4 {
                    match cursor.current() {
                        Some(e) => acc = *e,
                        None => {
                            cursor.move_next();
                            acc = *cursor.current().unwrap()
                        }
                    }
                    match cursor.current() {
                        Some(e) => *e = acc,
                        None => {
                            cursor.move_next();
                            *cursor.current().unwrap() = acc;
                        }
                    }
                }
                match cursor.current() {
                    Some(e) => *e = acc,
                    None => {
                        cursor.move_next();
                        *cursor.current().unwrap() = acc;
                    }
                }
                match cursor.remove_current() {
                    Some(e) => { acc = e; }
                    None => {
                        cursor.move_next();
                        let _ = cursor.remove_current();
                    }
                }
            }
        }
        Percentage::Fifty => {
            for _ in 0..(n * 2) {
                match cursor.current() {
                    Some(e) => acc = *e,
                    None => {
                        cursor.move_next();
                        acc = *cursor.current().unwrap()
                    }
                }
                match cursor.current() {
                    Some(e) => *e = acc,
                    None => {
                        cursor.move_next();
                        *cursor.current().unwrap() = acc;
                    }
                }
                cursor.insert_before(acc);
                match cursor.remove_current() {
                    Some(e) => { acc = e; }
                    None => {
                        cursor.move_next();
                        let _ = cursor.remove_current();
                    }
                }
            }
        }
    }

    acc[0]
}

pub fn benchmark_lists(c: &mut Criterion) {
    /*
    for n in [10usize, 1000, 100000, 10000000].iter() {
        let group_name = format!("Array/Size 8/Elements {n}");
        let mut group = c.benchmark_group(group_name);
        for p in [
            Percentage::Zero,
            Percentage::One,
            Percentage::Ten,
            Percentage::Fifty,
        ] {
            if *n == 10000000usize && p != Percentage::Zero {
                continue;
            }
            group.bench_with_input(BenchmarkId::new("Array", p), n, |b, i| {
                let mut arr = vec![0; i + 1];
                b.iter(|| {
                    let _ = benchmark_eight_byte_array(&mut arr, p);
                })
            });
        }
        group.finish();
    }

    for n in [10usize, 1000].iter() {
        let group_name = format!("Array/Size 512/Elements {n}");
        let mut group = c.benchmark_group(group_name);
        for p in [
            Percentage::Zero,
            Percentage::One,
            Percentage::Ten,
            Percentage::Fifty,
        ] {
            group.bench_with_input(BenchmarkId::new("Array", p), n, |b, i| {
                let mut arr = vec![[0; 64]; i + 1];
                b.iter(|| {
                    let _ = benchmark_five_twelve_byte_array(&mut arr, p);
                })
            });
        }
        group.finish();
    }
    */

    /*
    for n in [10usize, 1000].iter() {
        let group_name = format!("Array/Size 8192/Elements {n}");
        let mut group = c.benchmark_group(group_name);
        for p in [
            Percentage::Zero,
            Percentage::One,
            Percentage::Ten,
            Percentage::Fifty,
        ] {
            group.bench_with_input(BenchmarkId::new("Array", p), n, |b, i| {
                let mut arr = vec![[0; 1024]; i + 1];
                b.iter(|| {
                    let _ = benchmark_eight_mega_byte_array(&mut arr, p);
                })
            });
        }
        group.finish();
    }
    */
    for n in [10usize, 1000, 100000, 10000000].iter() {
        let group_name = format!("List/Size 8/Elements {n}");
        let mut group = c.benchmark_group(group_name);
        for p in [
            Percentage::Zero,
            Percentage::One,
            Percentage::Ten,
            Percentage::Fifty,
        ] {
            if *n == 10000000usize && p != Percentage::Zero {
                continue;
            }
            group.bench_with_input(BenchmarkId::new("List", p), n, |b, i| {
                let mut l = LinkedList::new();
                const TILE : usize = 5;
                for _ in 0..TILE {
                    let mut cursor = l.cursor_front_mut();
                    for i in 0..n.div_ceil(TILE) {
                        cursor.insert_before(0u64);
                        cursor.move_next();
                    }
                }
                b.iter(|| {
                    let _ = benchmark_eight_byte_list(&mut l, p);
                })
            });
        }
        group.finish();
    }

    for n in [10usize, 1000].iter() {
        let group_name = format!("List/Size 512/Elements {n}");
        let mut group = c.benchmark_group(group_name);
        for p in [
            Percentage::Zero,
            Percentage::One,
            Percentage::Ten,
            Percentage::Fifty,
        ] {
            group.bench_with_input(BenchmarkId::new("List", p), n, |b, i| {
                let mut l = LinkedList::new();
                const TILE : usize = 5;
                for _ in 0..TILE {
                    let mut cursor = l.cursor_front_mut();
                    for i in 0..n.div_ceil(TILE) {
                        cursor.insert_before([0u64; 64]);
                        cursor.move_next();
                    }
                }
                b.iter(|| {
                    let _ = benchmark_five_twelve_byte_list(&mut l, p);
                })
            });
        }
        group.finish();
    }

    for n in [10usize, 1000].iter() {
        let group_name = format!("List/Size 8192/Elements {n}");
        let mut group = c.benchmark_group(group_name);
        for p in [
            Percentage::Zero,
            Percentage::One,
            Percentage::Ten,
            Percentage::Fifty,
        ] {
            group.bench_with_input(BenchmarkId::new("List", p), n, |b, i| {
                let mut l = LinkedList::new();
                const TILE : usize = 5;
                for _ in 0..TILE {
                    let mut cursor = l.cursor_front_mut();
                    for i in 0..n.div_ceil(TILE) {
                        cursor.insert_before([0u64; 1024]);
                        cursor.move_next();
                    }
                }
                b.iter(|| {
                    let _ = benchmark_eight_mega_byte_list(&mut l, p);
                })
            });
        }
        group.finish();
    }
}

criterion_group!(benches, benchmark_lists);
criterion_main!(benches);
