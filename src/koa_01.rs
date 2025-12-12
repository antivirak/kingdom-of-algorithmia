#![allow(clippy::unused_unit)]

use itertools::itertools::iproduct;
use crate::*;

// https://everybody.codes/event/2024/quests/1
// 3969 / 3363 / 3111


fn get_enemies() -> Dict<char, u32> {
    let mut enemies = dict();
    enemies.insert('A', 0);
    enemies.insert('B', 1);
    enemies.insert('C', 3);
    enemies.insert('D', 5);
    enemies.insert('x', 0);

    enemies
}


pub fn main_1() -> u32 {
    let mut enemies = dict();
    enemies.insert('B', 1);
    enemies.insert('C', 3);

    let input = {
        open("day01/input1.txt", "r").readlines()
    };
    input[0]
        .chars()
        .fold(0, |acc, x| acc + enemies.get(&x).unwrap_or(&0))
}


pub fn main_2() -> u32 {
    let default = get_enemies();

    let mut enemies = dict();
    let keys = default.keys().collect::<Vec<_>>();
    for (a, b) in itertools::product(&keys, &keys) {
        let buffer = [ord(*a), ord(*b)];
        if *a == 'x' || *b == 'x' {
            enemies.insert(buffer, default[a] + default[b]);
        } else {
            enemies.insert(buffer, default[a] + default[b] + 2);
        }
    }

    let input = {
        open("day01/input2.txt", "r").readlines()
    };
    input[0]
        .chars()
        //.chunks(2)
        .tuples()
        .fold(0, |acc, (a, b)| acc + enemies.get(&[ord(a), ord(b)]).unwrap_or(&0))
}


pub fn main_3() -> u32 {
    let default = get_enemies();

    let mut enemies = dict();
    let keys = default.keys().collect::<Vec<_>>();
    let x = ord('x');
    for (a, b, c) in iproduct!(&keys, &keys, &keys) {
        let buffer = [ord(**a), ord(**b), ord(**c)];
        let count = buffer.iter().filter(|item| **item == x).count();
        if count > 1 {
            enemies.insert(buffer, default[a] + default[b] + default[c]);
        } else if count == 1 {
            enemies.insert(buffer, default[a] + default[b] + default[c] + 2);
        } else {
            enemies.insert(buffer, default[a] + default[b] + default[c] + 6);
        }
    }

    let input = {
        open("day01/input3.txt", "r").readlines()
    };
    input[0]
        .chars()
        .tuples()
        .fold(0, |acc, (a, b, c)| acc + enemies.get(&[ord(a), ord(b), ord(c)]).unwrap_or(&0))
}
