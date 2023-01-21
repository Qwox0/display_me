use crate::fields::StructFields;
use proc_macro2::{Delimiter, Group, TokenStream, TokenTree};
use quote::format_ident;
use quote::{quote, ToTokens};
use std::iter::FromIterator;
use syn::{
    braced, bracketed, parenthesized, parse::ParseStream, token, Ident, Index, LitInt, LitStr,
    Result, Token,
};

#[derive(Debug)]
pub(crate) struct DisplayArgs {
    pub fmt: LitStr,
    pub args: TokenStream,
}

impl DisplayArgs {
    pub fn parse_struct(input: ParseStream) -> Result<DisplayArgs> {
        Ok(DisplayArgs {
            fmt: input.parse()?,
            args: parse_token_expr(input, false, false)?,
        })
    }

    pub fn parse_tuple_struct(input: ParseStream) -> Result<DisplayArgs> {
        Ok(DisplayArgs {
            fmt: input.parse()?,
            args: parse_token_expr(input, false, true)?,
        })
    }

    pub fn get_parser<F>(is_tuple_struct: bool) -> F
    where
        F: Fn(ParseStream) -> Result<DisplayArgs>,
    {
        todo!()
    }
}

/*
impl<'a> DisplayArgs<'a> {
    pub(crate) fn parse_format_tuple_args(&mut self) -> Result<()> {
        println!("{:#?}", self);
        let fmt_str = self.fmt.value();
        let fmt_span = self.fmt.span();

        macro_rules! iter_until {
            ( $iter:ident -> $target:literal ) => {{
                let mut buf = String::new();
                let found_target: bool = loop {
                    let Some(ch) = $iter.next() else { break false; };
                    buf.push(ch);
                    if ch == $target {
                        break true;
                    };
                };

                (buf, found_target)
            }};
        }

        let mut iter = fmt_str.chars();
        let mut fmt_str = "".to_string();
        let mut idents = vec![];
        loop {
            let (buf, found_target) = iter_until!(iter -> '{');
            fmt_str.push_str(&buf);
            if found_target == false {
                break;
            }
            let (mut buf, found_target) = iter_until!(iter -> '}');
            if found_target == false {
                panic!("missing '}}'")
            }
            buf.pop();
            // buf == "" or "ident" or ":?" or "ident:?" or ":#?" or "ident:#?"
            println!("{:?}", buf);
            let (ident, modifier) = match buf.split_once(':') {
                None => (buf, "".to_string()),
                Some((ident, modifier)) => (ident.to_string(), ":".to_string() + modifier),
            };
            let ident = if ident.len() == 0 {
                None
            } else {
                Some(format_ident!("_{}", ident))
            };
            println!("{:?}   {:?}", ident, modifier);
            //let ident = ident.map(|s| s.trim_start_matches('_').to_string());
            idents.push(ident);
            fmt_str.push_str(&modifier);
            fmt_str.push('}');
        }

        self.fmt = LitStr::new(fmt_str.as_str(), fmt_span); // this span is most likely incorrect
        let mut old_args = self.args.clone().into_iter();
        self.args = TokenStream::from_iter(idents.into_iter().fold(
            vec![],
            |mut acc: Vec<TokenTree>, i| {
                acc.push(TokenTree::Punct(Punct::new(
                    ',',
                    proc_macro2::Spacing::Alone,
                )));
                acc.push(if let Some(i) = i {
                    TokenTree::Ident(i)
                } else {
                    old_args.next().expect("',' literal");
                    old_args.next().expect("argument given for format literal")
                });
                acc
            },
        ));
        println!("{:?}", self);
        Ok(())
    }

    pub(crate) fn parse_format_tuple_args2(&mut self) -> Result<()> {
        println!("{:#?}", self);
        Ok(())
    }
}
*/

pub(crate) struct Displayer {
    fmt: LitStr,
    args: TokenStream,
    struct_fields: StructFields,
}

impl Displayer {
    pub fn new(args: DisplayArgs, struct_fields: StructFields) -> Result<Self> {
        let DisplayArgs { fmt, args } = args;
        Ok(Displayer {
            fmt,
            args,
            struct_fields,
        })
    }
}

impl ToTokens for Displayer {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Displayer {
            fmt,
            args,
            struct_fields,
        } = self;

        tokens.extend(quote!(
            #[allow(unused_variables)]
            let Self #struct_fields = self;
            write!(__formatter, #fmt #args )
        ));
    }
}

/// stolen from thiserror :)
fn parse_token_expr(
    input: ParseStream,
    mut begin_expr: bool,
    in_tuple_struct: bool,
) -> Result<TokenStream> {
    let mut tokens = Vec::new();
    while !input.is_empty() {
        if in_tuple_struct && begin_expr && input.peek(LitInt) {
            let int: Index = input.parse()?;
            let ident = format_ident!("_{}", int.index, span = int.span);
            tokens.push(TokenTree::Ident(ident));
            begin_expr = false;
            continue;
        }

        if begin_expr && input.peek(Token![.]) {
            if input.peek2(Ident) {
                input.parse::<Token![.]>()?;
                begin_expr = false;
                continue;
            }
            if input.peek2(LitInt) {
                input.parse::<Token![.]>()?;
                let int: Index = input.parse()?;
                let ident = format_ident!("_{}", int.index, span = int.span);
                tokens.push(TokenTree::Ident(ident));
                begin_expr = false;
                continue;
            }
        }

        begin_expr = input.peek(Token![break])
            || input.peek(Token![continue])
            || input.peek(Token![if])
            || input.peek(Token![in])
            || input.peek(Token![match])
            || input.peek(Token![mut])
            || input.peek(Token![return])
            || input.peek(Token![while])
            || input.peek(Token![+])
            || input.peek(Token![&])
            || input.peek(Token![!])
            || input.peek(Token![^])
            || input.peek(Token![,])
            || input.peek(Token![/])
            || input.peek(Token![=])
            || input.peek(Token![>])
            || input.peek(Token![<])
            || input.peek(Token![|])
            || input.peek(Token![%])
            || input.peek(Token![;])
            || input.peek(Token![*])
            || input.peek(Token![-]);

        let token: TokenTree = if input.peek(token::Paren) {
            let content;
            let delimiter = parenthesized!(content in input);
            let nested = parse_token_expr(&content, true, in_tuple_struct)?;
            let mut group = Group::new(Delimiter::Parenthesis, nested);
            group.set_span(delimiter.span);
            TokenTree::Group(group)
        } else if input.peek(token::Brace) {
            let content;
            let delimiter = braced!(content in input);
            let nested = parse_token_expr(&content, true, in_tuple_struct)?;
            let mut group = Group::new(Delimiter::Brace, nested);
            group.set_span(delimiter.span);
            TokenTree::Group(group)
        } else if input.peek(token::Bracket) {
            let content;
            let delimiter = bracketed!(content in input);
            let nested = parse_token_expr(&content, true, in_tuple_struct)?;
            let mut group = Group::new(Delimiter::Bracket, nested);
            group.set_span(delimiter.span);
            TokenTree::Group(group)
        } else {
            input.parse()?
        };
        tokens.push(token);
    }
    Ok(TokenStream::from_iter(tokens))
}
