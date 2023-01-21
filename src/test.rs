use crate::display;

#[cfg(test)]
mod struct_tests {
    use super::*;

    #[test]
    fn simple() {
        #[display("A has number {} and word {}", num, word)]
        struct A {
            pub num: i32,
            pub word: &'static str,
        }
        assert_eq!(
            "A has number 42 and word HelloWorld",
            A { num: 42, word: "HelloWorld" }.to_string()
        );
    }

    #[test]
    fn long() {
        #[display("A has number {} and word {}", self.num, self.word)]
        struct A {
            pub num: i32,
            pub word: &'static str,
        }
        assert_eq!(
            "A has number 42 and word HelloWorld",
            A { num: 42, word: "HelloWorld" }.to_string()
        );
    }

    #[test]
    fn in_format() {
        #[display("A has number {num} and word {word}")]
        struct A {
            pub num: i32,
            pub word: &'static str,
        }
        assert_eq!(
            "A has number 42 and word HelloWorld",
            A { num: 42, word: "HelloWorld" }.to_string()
        );
    }

    #[test]
    fn format_specifier() {
        #[display("A has number {my_num} and word {0}", word, my_num = num)]
        struct A {
            pub num: i32,
            pub word: &'static str,
        }
        assert_eq!(
            "A has number 42 and word HelloWorld",
            A { num: 42, word: "HelloWorld" }.to_string()
        );
    }

    #[test]
    fn debug() {
        #[display("A has number {:?} and word {:?}", num, word)]
        struct A {
            pub num: i32,
            pub word: &'static str,
        }
        assert_eq!(
            "A has number 42 and word \"HelloWorld\"",
            A { num: 42, word: "HelloWorld" }.to_string()
        );
    }

    #[test]
    fn debug_in_format() {
        #[display("A has number {num:?} and word {word:?}")]
        struct A {
            pub num: i32,
            pub word: &'static str,
        }
        assert_eq!(
            "A has number 42 and word \"HelloWorld\"",
            A { num: 42, word: "HelloWorld" }.to_string()
        );
    }

    #[test]
    fn generic() {
        #[display("A has number {} and word {}", num, word)]
        struct A<I>
        where
            I: std::fmt::Display,
        {
            pub num: I,
            pub word: &'static str,
        }
        assert_eq!(
            "A has number 42 and word HelloWorld",
            A { num: 42, word: "HelloWorld" }.to_string()
        );
    }

    #[test]
    fn lifetime() {
        #[display("A has number {} and word {}", num, word)]
        struct A<'a> {
            pub num: i32,
            pub word: &'a str,
        }
        assert_eq!(
            "A has number 42 and word HelloWorld",
            A { num: 42, word: "HelloWorld" }.to_string()
        );
    }
}

#[cfg(test)]
mod tuple_struct_tests {
    use super::*;

    #[test]
    fn simple() {
        #[display("A has number {} and word {}", 0, 1)]
        struct A(pub i32, pub &'static str);
        assert_eq!("A has number 42 and word HelloWorld", A(42, "HelloWorld").to_string());
    }

    #[test]
    fn long() {
        #[display("A has number {} and word {}", self.0, self.1)]
        struct A(pub i32, pub &'static str);
        assert_eq!("A has number 42 and word HelloWorld", A(42, "HelloWorld").to_string());
    }

    /* // not supported
    #[test]
    fn in_format() {
        #[display("A has number {0} and word {1}")]
        struct A(pub i32, pub &'static str);
        assert_eq!("A has number 42 and word HelloWorld", A(42, "HelloWorld").to_string());
    }
    */

    #[test]
    fn format_specifier() {
        #[display("A has number {1} and word {0}", 1, 0)]
        struct A(pub i32, pub &'static str);
        assert_eq!("A has number 42 and word HelloWorld", A(42, "HelloWorld").to_string());
    }

    #[test]
    fn debug() {
        #[display("A has number {:?} and word {:?}", 0, 1)]
        struct A(pub i32, pub &'static str);
        assert_eq!("A has number 42 and word \"HelloWorld\"", A(42, "HelloWorld").to_string());
    }

    #[test]
    fn debug_in_format() {
        #[display("A has number {:?} and word {:?}", 0, 1)]
        struct A(pub i32, pub &'static str);
        assert_eq!("A has number 42 and word \"HelloWorld\"", A(42, "HelloWorld").to_string());
    }

    #[test]
    fn generic() {
        #[display("A has number {} and word {}", 0, 1)]
        struct A<I>(I, &'static str)
        where
            I: std::fmt::Display;
        assert_eq!("A has number 42 and word HelloWorld", A(42, "HelloWorld").to_string());
    }

    #[test]
    fn lifetime() {
        #[display("A has number {} and word {}", 0, 1)]
        struct A<'a>(i32, &'a str);
        assert_eq!("A has number 42 and word HelloWorld", A(42, "HelloWorld").to_string());
    }
}

#[cfg(test)]
mod other {
    use super::*;

    #[test]
    fn unit() {
        #[display("This is a A.")]
        struct A;
        assert_eq!("This is a A.", format!("{}", A),);
    }
}
