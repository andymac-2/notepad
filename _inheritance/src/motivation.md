# Motivation

Rust does not have inheritance, at least not natively. This can be a problem for a lot of programmers who are migrating from other languages. They may wish to map their existing ideas or programs one-to-one into Rust.

Rust may give new programmers a bit of a hard time at first: established practice in other languages may translate into highly unidiomatic Rust code. In this book, we will explore some of the ways that inheritance can be emulated in Rust with alternatives.

I will go through some of the ways in which existing inheritance patterns can be mapped into Rust. I will go through several use cases of inheritance and how that code should be written in Rust, and finish with a pattern that emulates true inheritance in other languages. The non-rust examples that I will provide are all written in pseudocode. Mind you, this article comes with a disclaimer: some of the code that will be presented is extremely unidiomatic rust, or just plain bad. Some of these patterns should be used with extreme caution and not for beginners.