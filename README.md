# trait selection failure

```console
~/dev/rust/trait_selection_failure$ cd app/
~/dev/rust/trait_selection_failure/app$ cargo check -q
error[E0277]: the trait bound `Fuga: From<hoge::Hoge>` is not satisfied
 --> src/main.rs:6:27
  |
6 |     let fuga: Fuga = hoge.into();
  |                           ^^^^ the trait `From<hoge::Hoge>` is not implemented for `Fuga`
  |
  = help: the trait `From<fuga::Hoge>` is implemented for `Fuga`
  = note: required for `hoge::Hoge` to implement `Into<Fuga>`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `app` due to previous error
```

ok, look more deeply..

```console
~/dev/rust/trait_selection_failure/app$ RUSTC_LOG="rustc_trait_selection" RUSTFLAGS="-Z verbose" cargo +nightly check
(...)
INFO rustc_trait_selection::traits::error_reporting::on_unimplemented evaluate(OnUnimplementedDirective { 
condition: None, subcommands: [OnUnimplementedDirective { condition: Some(MetaItem { path: Path { span: /rustc/
2036fdd24f77d607dcfaa24c48fbe85d3f785823/library/core/src/convert/mod.rs:539:5: 539:8 (#0), segments: 
[PathSegment { ident: all#0, id: NodeId(4294967040), args: None }], tokens: None }, kind: List([MetaItem
(MetaItem { path: Path { span: /rustc/2036fdd24f77d607dcfaa24c48fbe85d3f785823/library/core/src/convert/mod.
rs:539:9: 539:14 (#0), segments: [PathSegment { ident: _Self#0, id: NodeId(4294967040), args: None }], tokens: 
None }, kind: NameValue(MetaItemLit { symbol: "&str", suffix: None, kind: Str("&str", Cooked), span: /rustc/
2036fdd24f77d607dcfaa24c48fbe85d3f785823/library/core/src/convert/mod.rs:539:17: 539:23 (#0) }), span: /rustc/
2036fdd24f77d607dcfaa24c48fbe85d3f785823/library/core/src/convert/mod.rs:539:9: 539:23 (#0) }), MetaItem
(MetaItem { path: Path { span: /rustc/2036fdd24f77d607dcfaa24c48fbe85d3f785823/library/core/src/convert/mod.
rs:539:25: 539:26 (#0), segments: [PathSegment { ident: T#0, id: NodeId(4294967040), args: None }], tokens: 
None }, kind: NameValue(MetaItemLit { symbol: "std::string::String", suffix: None, kind: Str
("std::string::String", Cooked), span: /rustc/2036fdd24f77d607dcfaa24c48fbe85d3f785823/library/core/src/convert/
mod.rs:539:29: 539:50 (#0) }), span: /rustc/2036fdd24f77d607dcfaa24c48fbe85d3f785823/library/core/src/convert/
mod.rs:539:25: 539:50 (#0) })]), span: /rustc/2036fdd24f77d607dcfaa24c48fbe85d3f785823/library/core/src/convert/
mod.rs:539:5: 539:51 (#0) }), subcommands: [], message: None, label: None, note: Some
(OnUnimplementedFormatString("to coerce a `{T}` into a `{Self}`, use `&*` as a prefix")), parent_label: None, 
append_const_msg: None }], message: None, label: None, note: None, parent_label: None, append_const_msg: 
None }, trait_ref=<fuga::Fuga as std::convert::From<hoge::Hoge>>, options=[("ItemContext", Some("a function")), 
("direct", None), ("_Self", Some("fuga::Fuga")), ("_Self", Some("fuga::Fuga")), ("Self", Some("fuga::Fuga")), 
("Self", Some("fuga::Fuga")), ("T", Some("hoge::Hoge")), ("T", Some("hoge::Hoge"))])
(...)
error[E0277]: the trait bound `Fuga: From<hoge::Hoge>` is not satisfied
 --> src/main.rs:6:27
  |
6 |     let fuga: Fuga = hoge.into();
  |                           ^^^^ the trait `From<hoge::Hoge>` is not implemented for `Fuga`
  |
  = help: the trait `From<fuga::Hoge>` is implemented for `Fuga`
  = note: required for `hoge::Hoge` to implement `Into<Fuga>`
```
