use super::*;

use rustc_ast::{self as ast, ptr::P};
use rustc_span::with_default_session_globals;
use rustc_span::{symbol::Ident, Span, Symbol, DUMMY_SP};

fn fun_to_string(
    decl: &ast::FnDecl,
    header: ast::FnHeader,
    name: Ident,
    generics: &ast::Generics,
) -> String {
    to_string(|s| {
        s.head("");
        s.print_fn(decl, header, Some(name), generics);
        s.end(); // Close the head box.
        s.end(); // Close the outer box.
    })
}

fn variant_to_string(var: &ast::Variant) -> String {
    to_string(|s| s.print_variant(var))
}

#[test]
fn test_fun_to_string() {
    with_default_session_globals(|| {
        let abba_ident = Ident::from_str("abba");

        let decl = ast::FnDecl { inputs: Vec::new(), output: ast::FnRetTy::Default(DUMMY_SP) };
        let generics = ast::Generics::default();
        assert_eq!(
            fun_to_string(&decl, ast::FnHeader::default(), abba_ident, &generics),
            "fn abba()"
        );
    })
}

#[test]
fn test_variant_to_string() {
    with_default_session_globals(|| {
        let ident = Ident::from_str("principal_skinner");

        let var = ast::Variant {
            ident,
            vis: ast::Visibility {
                span: DUMMY_SP,
                kind: ast::VisibilityKind::Inherited,
                tokens: None,
            },
            attrs: Vec::new(),
            id: ast::DUMMY_NODE_ID,
            data: ast::VariantData::Unit(ast::DUMMY_NODE_ID),
            disr_expr: None,
            span: DUMMY_SP,
            is_placeholder: false,
        };

        let varstr = variant_to_string(&var);
        assert_eq!(varstr, "principal_skinner");
    })
}

#[test]
fn test_f_string() {
    with_default_session_globals(|| {
        // `f"foo{bar + 3}quaaz{something_else("123")}"`

        fn generate_piece(string: &str) -> (Symbol, Span) {
            (Symbol::intern(string), DUMMY_SP)
        }

        fn generate_expr(kind: ast::ExprKind) -> P<ast::Expr> {
            P(ast::Expr {
                id: ast::DUMMY_NODE_ID,
                kind,
                span: DUMMY_SP,
                attrs: ast::AttrVec::default(),
                tokens: None,
            })
        }

        fn generate_simple_path(name: &str) -> ast::Path {
            ast::Path {
                span: DUMMY_SP,
                segments: vec![ast::PathSegment {
                    args: None,
                    id: ast::DUMMY_NODE_ID,
                    ident: Ident::with_dummy_span(Symbol::intern(name)),
                }],
                tokens: None,
            }
        }

        fn generate_integer_lit(integer: u128) -> ast::Lit {
            let integer_string = format!("{}", integer);
            ast::Lit {
                kind: ast::LitKind::Int(integer, ast::LitIntType::Unsuffixed),
                span: DUMMY_SP,
                token: ast::token::Lit {
                    kind: ast::token::LitKind::Integer,
                    symbol: Symbol::intern(&integer_string),
                    suffix: None,
                },
            }
        }

        let bar_plus_3 = generate_expr(ast::ExprKind::Binary(
            rustc_span::source_map::dummy_spanned(ast::BinOpKind::Add),
            generate_expr(ast::ExprKind::Path(None, generate_simple_path("bar"))),
            generate_expr(ast::ExprKind::Lit(generate_integer_lit(3))),
        ));
        let something_else_123 = generate_expr(ast::ExprKind::Call(
            generate_expr(ast::ExprKind::Path(None, generate_simple_path("something_else"))),
            vec![generate_expr(ast::ExprKind::Lit(generate_integer_lit(123)))],
        ));

        let pieces = vec![generate_piece("foo"), generate_piece("quaaz"), generate_piece("")];
        let args = vec![bar_plus_3, something_else_123];

        let f_str = ast::FStr { pieces, args, span: DUMMY_SP };
        assert_eq!(
            to_string(|s| s.print_f_str(&f_str)),
            r#"f"foo{bar + 3}quaaz{something_else(123)}""#
        );
    })
}
