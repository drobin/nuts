// MIT License
//
// Copyright (c) 2024 Robin Doer
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to
// deal in the Software without restriction, including without limitation the
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.

pub mod list {
    use predicates::reflection::PredicateReflection;
    use predicates::Predicate;
    use std::{fmt, str};

    fn split_bytes<'a>(bytes: &'a [u8]) -> Vec<&'a str> {
        bytes
            .split(|b| *b == 0x0a)
            .map(|b| str::from_utf8(b).unwrap())
            .filter(|s| !s.is_empty())
            .collect()
    }

    #[derive(Debug)]
    enum Compare {
        Eq,
        Unordered,
    }

    #[derive(Debug)]
    pub struct ListPredicate<const L: usize, T: AsRef<str>> {
        list: [T; L],
        cmp: Compare,
    }

    impl<const L: usize, T: AsRef<str> + fmt::Debug> fmt::Display for ListPredicate<L, T> {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            fmt::Debug::fmt(self, fmt)
        }
    }

    impl<const L: usize, T: AsRef<str> + fmt::Debug> PredicateReflection for ListPredicate<L, T> {}

    impl<const L: usize, T: AsRef<str> + fmt::Debug> Predicate<[u8]> for ListPredicate<L, T> {
        fn eval(&self, bytes: &[u8]) -> bool {
            let left = split_bytes(bytes);

            if left.len() == L {
                match self.cmp {
                    Compare::Eq => (0..L).all(|i| left[i] == self.list[i].as_ref()),
                    Compare::Unordered => left
                        .into_iter()
                        .all(|l| self.list.iter().find(|r| l == r.as_ref()).is_some()),
                }
            } else {
                false
            }
        }
    }

    pub fn eq<const L: usize, T: AsRef<str>>(list: [T; L]) -> ListPredicate<L, T> {
        ListPredicate {
            list,
            cmp: Compare::Eq,
        }
    }

    pub fn unordered<const L: usize, T: AsRef<str>>(list: [T; L]) -> ListPredicate<L, T> {
        ListPredicate {
            list,
            cmp: Compare::Unordered,
        }
    }
}

pub mod hash {
    use predicates::reflection::PredicateReflection;
    use predicates::Predicate;
    use std::collections::HashMap;
    use std::{fmt, str};

    fn split_bytes<'a>(bytes: &'a [u8]) -> HashMap<&'a str, &'a str> {
        let mut hash = HashMap::new();

        for s in bytes
            .split(|b| *b == 0x0a)
            .map(|b| str::from_utf8(b).unwrap())
            .filter(|s| s.len() > 0)
        {
            let (left, right) = s.trim_end().split_once(':').unwrap();

            assert!(hash.insert(left.trim(), right.trim()).is_none());
        }

        hash
    }

    #[derive(Debug)]
    enum Compare {
        Eq,
        Contains,
    }

    #[derive(Debug)]
    pub struct HashPredicate<K: AsRef<str>, V: AsRef<str>> {
        hash: HashMap<K, V>,
        cmp: Compare,
    }

    impl<K: AsRef<str> + fmt::Debug, V: AsRef<str>> HashPredicate<K, V> {
        fn eval_eq(&self, other: HashMap<&str, &str>) -> bool {
            if self.hash.len() == other.len() {
                self.hash.iter().all(|(k, v)| {
                    other
                        .get(k.as_ref())
                        .map(|ov| v.as_ref() == *ov)
                        .unwrap_or(false)
                })
            } else {
                false
            }
        }

        fn eval_contains(&self, other: HashMap<&str, &str>) -> bool {
            self.hash.iter().all(|(k, v)| {
                other
                    .get(k.as_ref())
                    .map(|ov| v.as_ref() == *ov)
                    .unwrap_or(false)
            })
        }
    }

    impl<K: AsRef<str> + fmt::Debug, V: AsRef<str> + fmt::Debug> fmt::Display for HashPredicate<K, V> {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            fmt::Debug::fmt(self, fmt)
        }
    }

    impl<K: AsRef<str> + fmt::Debug, V: AsRef<str> + fmt::Debug> PredicateReflection
        for HashPredicate<K, V>
    {
    }

    impl<K: AsRef<str> + fmt::Debug, V: AsRef<str> + fmt::Debug> Predicate<[u8]>
        for HashPredicate<K, V>
    {
        fn eval(&self, bytes: &[u8]) -> bool {
            let other = split_bytes(bytes);

            match self.cmp {
                Compare::Eq => self.eval_eq(other),
                Compare::Contains => self.eval_contains(other),
            }
        }
    }

    pub fn eq<K: AsRef<str>, V: AsRef<str>, T: Into<HashMap<K, V>>>(
        hash: T,
    ) -> HashPredicate<K, V> {
        HashPredicate {
            hash: hash.into(),
            cmp: Compare::Eq,
        }
    }

    pub fn contains<K: AsRef<str>, V: AsRef<str>, T: Into<HashMap<K, V>>>(
        hash: T,
    ) -> HashPredicate<K, V> {
        HashPredicate {
            hash: hash.into(),
            cmp: Compare::Contains,
        }
    }
}
