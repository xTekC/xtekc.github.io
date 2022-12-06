//! &nbsp; [Github](https://github.com/xTekC) &nbsp; [Twitter](https://twitter.com/xTekC) &nbsp; [Blog](https://xtekc.hashnode.dev)
//!
//! Hello, I am xTekC.
//!
//! As an open source developer, I have a passion for creating software that is efficient, elegant, and secure. My experience with Rust, Bash, and C, along with my knowledge of data structures and algorithms, has allowed me to contribute to a variety of open source projects and create innovative solutions for complex problems.
//!
//! In addition to my technical skills, I have also proven myself as a strong leader and team player. As a team lead for over 30 associates, I have implemented initiatives that have improved departments' productivity, reduced material costs, and enhanced the safety and well-being of associates within the company.
//!
//! My interest in programming and security has driven me to explore ways to create security programs that protect the privacy and security of users and systems. I am always looking for new challenges and opportunities to grow as a developer and make a positive impact on the organization I am a part of.
//!
//! Aside from my work experience, I am also interested in content creation and am currently working on creating video content for YouTube and writing for blogs. I am also interested in participating in a podcast to discuss educational topics and am currently learning the German language.

/// Open Source Developer (2+ Years)
pub mod open_source_exp {}

/// Production Engineer (10+ Years)
pub mod production_engineer_exp {}

/// Rust
#[macro_export]
macro_rules! Rust {
    () => {};
}

/// Bash
#[macro_export]
macro_rules! Bash {
    () => {};
}

/// C
#[macro_export]
macro_rules! C {
    () => {};
}

/// Certification Road-Map
pub struct Certifications {
   pub LinuxPlus: String,
   pub ServerPlus: String,
   pub CloudPlus: String,
   pub NetworkPlus: String,
   pub SecurityPlus: String,
   pub CysaPlus: String,
   pub PentestPlus: String,
   pub CaspPlus: String,
}

/// Comptia Linux+ (pending)
pub const CERT01: &str = "Linux+";

/// A markdown to html compiler
/// [github.com/xTekC/mduc](https://github.com/xTekC/mduc)
pub fn mduc() {}

/// Delete data from shell history file(s)
/// [github.com/xTekC/shdel](https://github.com/xTekC/shdel)
pub fn shdel() {}

/// Asynchronous Password Generator
/// [github.com/xTekC/passgen](https://github.com/xTeKc/passgen)
pub fn passgen() {}
