---
title: "Warframe 11th anniversary easter egg"
date: 2024-03-22
taxonomies:
  tags:
    - warframe
    - rust
    - bruteforce
extra:
  guid: c24b38b0-8878-453f-a2f2-48737ae7892d
---

# Sometimes I'm bored

Then Warframe is dropping a new easter egg on their website for their 11th anniversary. (<https://www.warframe.com/anniversary>)

And because I love to play, I decided to try to reverse it, it seems simple enough.

In 3s I found the following, we must find something with the hash `-799043843` and the hash function is the following:

```js  
function hash(str) {
  let hash = 0;
  if (str.length == 0) return hash;
  for (let i = 0; i < str.length; i++) {
    ch = str.charCodeAt(i);
    hash = ((hash << 5) - hash) + ch;
    hash = hash & hash;
  }
  return hash;
}
```

Easy pz I though, I should find a collision really fast.

For fun I implemented it in rust because why not I love this lang and take pleasure writing and learning things about it so:

```rust
use std::sync::atomic::{AtomicUsize, Ordering};

use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};

const CHARS: &'static str = "abcdefghijklmnopqrstuvwxyz";

fn hash(str: &[char]) -> i32 {
  str.iter().fold(0i32, |acc, c| {
    let ch = *c as i32;
    let hash = ((acc << 5) - acc) + ch;
    hash & hash
  })
}

fn combos<'a>(max_length: usize) -> impl ParallelIterator<Item = Vec<char>> + 'a {
  (1..=max_length).into_par_iter()
    .flat_map(|length| {
      CHARS
        .chars()
        .combinations_with_replacement(length)
        .par_bridge()
        .flat_map_iter(move |comb| comb.into_iter().permutations(length).unique())
    })
}

fn main() {
  // Official password leaked on Reddit duh
  let target_hash = -799043843;
  assert!(hash(&"happynewyear".chars().collect::<Vec<_>>()) == target_hash);

  let possible_strings = combos(12);
  println!("Found all possible strings");
  let count = AtomicUsize::new(0);
  possible_strings
    .map(|s| (hash(&s), s))
    .inspect(|_| {
      let count = count.fetch_add(1, Ordering::Relaxed);
      if count > 0 && count % 100_000_000 == 0 {
        println!("Checked {}m passwords", count / 1_000_000);
      }
    })
    .filter(|(h, found)| {
      let res = *h == target_hash;
      if res {
        println!("password might be \"{}\" (hash: {h})", found.iter().collect::<String>());
      }
      res
    }).collect::<Vec<_>>();
}
```

It was quite fun to do, with a lot of small thinking on what to change to try to optimize while keeping it simple.

I finally found the following (didn't find all of them for 12 chars because I only have my Framework 13, but good enough for me)

| after N passwords | matching password |
| ----------------- | ----------------- |
| 100 millions      | `palafhe`         |
| 1.6 billions      | `kfklgoa`         |
| 5.2 billions      | `yvmjcym`         |
| 8.9 billions      | `adsgryba`        |
| 10.7 billions     | `yaamcxdx`        |
| 11.6 billions     | `aaaaaakgbc`      |
| 12.9 billions     | `taewpagt`        |
| 17.1 billions     | `lbkuuaqb`        |
