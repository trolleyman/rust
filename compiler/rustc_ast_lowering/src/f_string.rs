use std::iter;

use rustc_ast::{
    FStringFormatSpec, FormatAlignment, FormatCount, FormatDebugHex, FormatOptions, FormatSign,
    FormatTrait,
};

use crate::LoweringContext;

struct Parser<'a> {
    _input: &'a str,
    pub cursor: iter::Peekable<std::str::CharIndices<'a>>,
}
impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Parser<'a> {
        Parser { _input: input, cursor: input.char_indices().peekable() }
    }
    pub fn consume_pos(&mut self, c: char) -> Option<usize> {
        match self.cursor.peek().copied() {
            Some((i, peek_c)) if peek_c == c => {
                self.cursor.next();
                Some(i)
            }
            _ => None,
        }
    }

    pub fn consume(&mut self, c: char) -> bool {
        self.consume_pos(c).is_some()
    }

    pub fn consume_integer(&mut self) -> Option<usize> {
        let mut cur: usize = 0;
        let mut found = false;
        let mut overflow = false;
        while let Some(&(_, c)) = self.cursor.peek() {
            if let Some(i) = c.to_digit(10) {
                let (tmp, mul_overflow) = cur.overflowing_mul(10);
                let (tmp, add_overflow) = tmp.overflowing_add(i as usize);
                if mul_overflow || add_overflow {
                    overflow = true;
                }
                cur = tmp;
                found = true;
                self.cursor.next();
            } else {
                break;
            }
        }

        if overflow {
            // TODO: Error correctly
            panic!("Overflow");
            // let end = self.current_pos();
            // let overflowed_int = &self.input[start..end];
            // self.err(
            //     format!(
            //         "integer `{}` does not fit into the type `usize` whose range is `0..={}`",
            //         overflowed_int,
            //         usize::MAX
            //     ),
            //     "integer out of range for `usize`",
            //     self.span(start, end),
            // );
        }

        found.then_some(cur)
    }
}

impl<'hir> LoweringContext<'_, 'hir> {
    pub(crate) fn lower_f_string_format_spec(
        &mut self,
        format_spec: &Option<FStringFormatSpec>,
    ) -> (FormatTrait, FormatOptions) {
        let format_spec = if let Some(format_spec) = format_spec {
            format_spec
        } else {
            return (FormatTrait::Display, FormatOptions::default());
        };

        let mut options = FormatOptions::default();
        let mut parser = Parser::new(format_spec.sym.as_str());

        // fill character
        if let Some(&(_, c)) = parser.cursor.peek() {
            if let Some((_, '>' | '<' | '^')) = parser.cursor.clone().nth(1) {
                options.fill = Some(c);
                parser.cursor.next();
            }
        }
        // Alignment
        if parser.consume('<') {
            options.alignment = Some(FormatAlignment::Left);
        } else if parser.consume('>') {
            options.alignment = Some(FormatAlignment::Right);
        } else if parser.consume('^') {
            options.alignment = Some(FormatAlignment::Center);
        }
        // Sign flags
        if parser.consume('+') {
            options.sign = Some(FormatSign::Plus);
        } else if parser.consume('-') {
            options.sign = Some(FormatSign::Minus);
        }
        // Alternate marker
        if parser.consume('#') {
            options.alternate = true;
        }
        // Width and precision
        if parser.consume('0') {
            options.zero_pad = true;

            // Check for `$`, and flag as error
            if let Some(_end) = parser.consume_pos('$') {
                // span(end - 1, end + 1);
                // TODO
                panic!("Invalid format string");
            }
        }

        // TODO: Handle $
        if let Some(width) = parser.consume_integer() {
            options.width = Some(FormatCount::Literal(width));
        }

        if let Some(_start) = parser.consume_pos('.') {
            if parser.consume('*') {
                // TODO: Error correctly
                panic!("* not supported in f-strings");
            } else {
                // TODO: Handle $
                options.precision = parser.consume_integer().map(|i| FormatCount::Literal(i));
            }
        }

        // Optional radix followed by the actual format specifier
        let format_trait = if parser.consume('x') {
            if parser.consume('?') {
                options.debug_hex = Some(FormatDebugHex::Lower);
                FormatTrait::Debug
            } else {
                FormatTrait::LowerHex
            }
        } else if parser.consume('X') {
            if parser.consume('?') {
                options.debug_hex = Some(FormatDebugHex::Upper);
                FormatTrait::Debug
            } else {
                FormatTrait::UpperHex
            }
        } else if parser.consume('?') {
            FormatTrait::Debug
        } else {
            match parser.cursor.next().map(|(_, c)| c) {
                Some('o') => FormatTrait::Octal,
                Some('x') => FormatTrait::LowerHex,
                Some('X') => FormatTrait::UpperHex,
                Some('p') => FormatTrait::Pointer,
                Some('b') => FormatTrait::Binary,
                Some('e') => FormatTrait::LowerExp,
                Some('E') => FormatTrait::UpperExp,
                Some(c) => panic!("Invalid type: {}", c), // TODO: Fix error reporting
                None => FormatTrait::Display,
            }
        };
        // TODO: Check if there is any "leftover" chars that aren't whitespace
        eprintln!("FORMAT SPEC: {:?} => {:?} {:?}", format_spec.sym.as_str(), format_trait, options);
        (format_trait, options)
    }
}
