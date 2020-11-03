extern crate rattle_items_match;

use rattle_items_match::{
    ActualBuilder as Actual, Condition as Cnd, ConditionsBuilder as Cnds, Control as Co,
    ExpectedBuilder as Expected, MachineBuilder as Ma, Operator as Op, RangeIncludesMax, Repeat,
    RoutineBuilder as Ro,
};

fn main() {
    println!("Start.");

    // `Actual` is sequence only.
    // 比較対象値は シーケンスのみです。
    let ac1 = Actual::default() // "    1"
        .push(&' ')
        .push(&' ')
        .push(&' ')
        .push(&' ')
        .push(&'1')
        .build();

    let ac2 = Actual::default() // "\t   1"
        .push(&'\t')
        .push(&' ')
        .push(&' ')
        .push(&' ')
        .push(&'1')
        .build();

    let ac3 = Actual::default() // 'x   1'
        .push(&'x')
        .push(&' ')
        .push(&' ')
        .push(&' ')
        .push(&'1')
        .build();
    let ac4 = Actual::default().push(&'A').build(); // 'A'
    let ac5 = Actual::default().push(&'B').push(&'C').build(); // 'BC'
    let ac6 = Actual::default().push(&'d').push(&'e').build(); // 'de'
    let ac7 = Actual::default().push(&'f').push(&'g').push(&'h').build(); // 'fgh'
    let ac8 = Actual::default().push(&'\r').push(&'\n').build(); // "\r\n"
    let ac9 = Actual::default() // '# Comment あ.'
        .push(&'#')
        .push(&' ')
        .push(&'C')
        .push(&'o')
        .push(&'m')
        .push(&'m')
        .push(&'e')
        .push(&'n')
        .push(&'t')
        .push(&' ')
        .push(&'あ')
        .push(&'.')
        .build();
    let ac10 = Actual::default() // 'No-1_0' - Unquloted key.
        .push(&'N')
        .push(&'o')
        .push(&'-')
        .push(&'1')
        .push(&'_')
        .push(&'0')
        .build();

    // https://github.com/toml-lang/toml/blob/1.0.0-rc.3/toml.abnf
    // TODO 18 toml = expression *( newline expression )
    // TODO 20 expression =  ws [ comment ]
    // TODO 21 expression =/ ws keyval ws [ comment ]
    // TODO 22 expression =/ ws table ws [ comment ]

    // TODO 26 ws = *wschar

    // Whitespace characters.
    // 27 wschar =  %x20  ; Space
    // 28 wschar =/ %x09  ; Horizontal tab
    let wschar = Cnds::default()
        .push(&Cnd::Pin('\t'))
        .push(&Cnd::Pin(' '))
        .build();

    // Newline.
    // 32 newline =  %x0A     ; LF
    // 33 newline =/ %x0D.0A  ; CRLF
    let newline = Cnds::default()
        .push(&Cnd::Pin('\n')) // LF
        .push(&Cnd::Seq(vec!['\r', '\n'])) // CR LF
        .build();

    // 37 comment-start-symbol = %x23 ; #
    let comment_start_symbol = Cnd::Pin('#'); // #

    // 38 non-ascii = %x80-D7FF / %xE000-10FFFF
    let non_ascii = Cnds::default()
        .push(&Cnd::RangeIncludesMax(
            RangeIncludesMax::default()
                .min(&(0x80 as char))
                .max(&'\u{D7FF}')
                .build(),
        ))
        .push(&Cnd::RangeIncludesMax(
            RangeIncludesMax::default()
                .min(&'\u{E000}')
                .max(&'\u{10FFFF}')
                .build(),
        ))
        .build();

    // 39 non-eol = %x09 / %x20-7F / non-ascii
    let non_eol = Op::Or(
        Cnds::default()
            .push(&Cnd::Pin(0x09 as char))
            .push(&Cnd::RangeIncludesMax(
                RangeIncludesMax::default()
                    .min(&(0x20 as char))
                    .max(&(0x7F as char))
                    .build(),
            ))
            .extend(&non_ascii)
            .build(),
    );

    // 41 comment = comment-start-symbol *non-eol
    // TODO これを条件文として持てないか？ Control を持つ Routine レイヤーを作るか？
    let comment = &Ro::default() // "# Comment."
        .push(&Co::Once(Op::One(comment_start_symbol)))
        .push(&Co::Repeat(
            Repeat::default()
                .op(&non_eol)
                .min(0)
                .max_not_included(usize::MAX)
                .build(),
        ))
        .build();

    // TODO 45 keyval = key keyval-sep val
    // TODO 47 key = simple-key / dotted-key
    // TODO 48 simple-key = quoted-key / unquoted-key

    // Alphabet.
    // 239 ALPHA = %x41-5A / %x61-7A ; A-Z / a-z
    let alpha = Cnds::default()
        .push(&Cnd::RangeIncludesMax(
            RangeIncludesMax::default().min(&'A').max(&'Z').build(),
        ))
        .push(&Cnd::RangeIncludesMax(
            RangeIncludesMax::default().min(&'a').max(&'z').build(),
        ))
        .build();

    // Digit.
    // 240 DIGIT = %x30-39 ; 0-9
    let digit = Cnd::RangeIncludesMax(RangeIncludesMax::default().min(&'0').max(&'9').build());

    // TODO 50 unquoted-key = 1*( ALPHA / DIGIT / %x2D / %x5F ) ; A-Z / a-z / 0-9 / - / _
    let unquoted_key = &Ro::default() // 'No-1_0' - Unquloted key.
        .push(&Co::Repeat(
            Repeat::default()
                .min(1)
                .max_not_included(usize::MAX)
                .op(&Op::Or(
                    Cnds::default()
                        .extend(&alpha) // A-Z, a-z.
                        .push(&digit) // 0-9.
                        .push(&Cnd::Pin(0x2D as char)) // -
                        .push(&Cnd::Pin(0x5F as char)) // _
                        .build(),
                ))
                .build(),
        ))
        .build();

    // TODO 51 quoted-key = basic-string / literal-string
    // TODO 52 dotted-key = simple-key 1*( dot-sep simple-key )
    // TODO 54 dot-sep   = ws %x2E ws  ; . Period
    // TODO 55 keyval-sep = ws %x3D ws ; =
    // TODO 57 val = string / boolean / array / inline-table / date-time / float / integer
    // TODO 61 string = ml-basic-string / basic-string / ml-literal-string / literal-string
    // TODO 65 basic-string = quotation-mark *basic-char quotation-mark
    // TODO 67 quotation-mark = %x22            ; "
    // TODO 69 basic-char = basic-unescaped / escaped
    // TODO 70 basic-unescaped = wschar / %x21 / %x23-5B / %x5D-7E / non-ascii
    // TODO 71 escaped = escape escape-seq-char
    // TODO 73 escape = %x5C                   ; \
    // TODO 74 escape-seq-char =  %x22         ; "    quotation mark  U+0022
    // TODO 75 escape-seq-char =/ %x5C         ; \    reverse solidus U+005C
    // TODO 76 escape-seq-char =/ %x62         ; b    backspace       U+0008
    // TODO 77 escape-seq-char =/ %x66         ; f    form feed       U+000C
    // TODO 78 escape-seq-char =/ %x6E         ; n    line feed       U+000A
    // TODO 79 escape-seq-char =/ %x72         ; r    carriage return U+000D
    // TODO 80 escape-seq-char =/ %x74         ; t    tab             U+0009
    // TODO 81 escape-seq-char =/ %x75 4HEXDIG ; uXXXX                U+XXXX
    // TODO 82 escape-seq-char =/ %x55 8HEXDIG ; UXXXXXXXX            U+XXXXXXXX
    // TODO 86 ml-basic-string = ml-basic-string-delim ml-basic-body ml-basic-string-delim
    // TODO 87 ml-basic-string-delim = 3quotation-mark
    // TODO 88 ml-basic-body = *mlb-content *( mlb-quotes 1*mlb-content ) [ mlb-quotes ]
    // TODO 90 mlb-content = mlb-char / newline / mlb-escaped-nl
    // TODO 91 mlb-char = mlb-unescaped / escaped
    // TODO 92 mlb-quotes = 1*2quotation-mark
    // TODO 93 mlb-unescaped = wschar / %x21 / %x23-5B / %x5D-7E / non-ascii
    // TODO 94 mlb-escaped-nl = escape ws newline *( wschar / newline )
    // TODO 98 literal-string = apostrophe *literal-char apostrophe
    // TODO 100 apostrophe = %x27 ; ' apostrophe
    // TODO 102 literal-char = %x09 / %x20-26 / %x28-7E / non-ascii
    // TODO 106 ml-literal-string = ml-literal-string-delim ml-literal-body ml-literal-string-delim
    // TODO 107 ml-literal-string-delim = 3apostrophe
    // TODO 108 ml-literal-body = *mll-content *( mll-quotes 1*mll-content ) [ mll-quotes ]
    // TODO 110 mll-content = mll-char / newline
    // TODO 111 mll-char = %x09 / %x20-26 / %x28-7E / non-ascii
    // TODO 112 mll-quotes = 1*2apostrophe
    // TODO 116 integer = dec-int / hex-int / oct-int / bin-int
    // TODO 118 minus = %x2D                       ; -
    // TODO 119 plus = %x2B                        ; +
    // TODO 120 underscore = %x5F                  ; _
    // TODO 121 digit1-9 = %x31-39                 ; 1-9
    // TODO 122 digit0-7 = %x30-37                 ; 0-7
    // TODO 123 digit0-1 = %x30-31                 ; 0-1
    // TODO 125 hex-prefix = %x30.78               ; 0x
    // TODO 126 oct-prefix = %x30.6f               ; 0o
    // TODO 127 bin-prefix = %x30.62               ; 0b
    // TODO 129 dec-int = [ minus / plus ] unsigned-dec-int
    // TODO 130 unsigned-dec-int = DIGIT / digit1-9 1*( DIGIT / underscore DIGIT )
    // TODO 132 hex-int = hex-prefix HEXDIG *( HEXDIG / underscore HEXDIG )
    // TODO 133 oct-int = oct-prefix digit0-7 *( digit0-7 / underscore digit0-7 )
    // TODO 134 bin-int = bin-prefix digit0-1 *( digit0-1 / underscore digit0-1 )
    // TODO 138 float = float-int-part ( exp / frac [ exp ] )
    // TODO 139 float =/ special-float
    // TODO 141 float-int-part = dec-int
    // TODO 142 frac = decimal-point zero-prefixable-int
    // TODO 143 decimal-point = %x2E               ; .
    // TODO 144 zero-prefixable-int = DIGIT *( DIGIT / underscore DIGIT )
    // TODO 146 exp = "e" float-exp-part
    // TODO 147 float-exp-part = [ minus / plus ] zero-prefixable-int
    // TODO 149 special-float = [ minus / plus ] ( inf / nan )
    // TODO 150 inf = %x69.6e.66  ; inf
    // TODO 151 nan = %x6e.61.6e  ; nan
    // TODO 155 boolean = true / false
    // TODO 157 true    = %x74.72.75.65     ; true
    // TODO 158 false   = %x66.61.6C.73.65  ; false
    // TODO 162 date-time      = offset-date-time / local-date-time / local-date / local-time
    // TODO 164 date-fullyear  = 4DIGIT
    // TODO 165 date-month     = 2DIGIT  ; 01-12
    // TODO 166 date-mday      = 2DIGIT  ; 01-28, 01-29, 01-30, 01-31 based on month/year
    // TODO 167 time-delim     = "T" / %x20 ; T, t, or space
    // TODO 168 time-hour      = 2DIGIT  ; 00-23
    // TODO 169 time-minute    = 2DIGIT  ; 00-59
    // TODO 170 time-second    = 2DIGIT  ; 00-58, 00-59, 00-60 based on leap second rules
    // TODO 171 time-secfrac   = "." 1*DIGIT
    // TODO 172 time-numoffset = ( "+" / "-" ) time-hour ":" time-minute
    // TODO 173 time-offset    = "Z" / time-numoffset
    // TODO 175 partial-time   = time-hour ":" time-minute ":" time-second [ time-secfrac ]
    // TODO 176 full-date      = date-fullyear "-" date-month "-" date-mday
    // TODO 177 full-time      = partial-time time-offset
    // TODO 181 offset-date-time = full-date time-delim full-time
    // TODO 185 local-date-time = full-date time-delim partial-time
    // TODO 189 local-date = full-date
    // TODO 193 local-time = partial-time
    // TODO 197 array = array-open [ array-values ] ws-comment-newline array-close
    // TODO 199 array-open =  %x5B ; [
    // TODO 200 array-close = %x5D ; ]
    // TODO 202 array-values =  ws-comment-newline val ws-comment-newline array-sep array-values
    // TODO 203 array-values =/ ws-comment-newline val ws-comment-newline [ array-sep ]
    // TODO 205 array-sep = %x2C  ; , Comma
    // TODO 207 ws-comment-newline = *( wschar / [ comment ] newline )
    // TODO 211 table = std-table / array-table
    // TODO 215 std-table = std-table-open key std-table-close
    // TODO 217 std-table-open  = %x5B ws     ; [ Left square bracket
    // TODO 218 std-table-close = ws %x5D     ; ] Right square bracket
    // TODO 222 inline-table = inline-table-open [ inline-table-keyvals ] inline-table-close
    // TODO 224 inline-table-open  = %x7B ws     ; {
    // TODO 225 inline-table-close = ws %x7D     ; }
    // TODO 226 inline-table-sep   = ws %x2C ws  ; , Comma
    // TODO 228 inline-table-keyvals = keyval [ inline-table-sep inline-table-keyvals ]
    // TODO 232 array-table = array-table-open key array-table-close
    // TODO 234 array-table-open  = %x5B.5B ws  ; [[ Double left square bracket
    // TODO 235 array-table-close = ws %x5D.5D  ; ]] Double right square bracket

    // TODO 241 HEXDIG = DIGIT / "A" / "B" / "C" / "D" / "E" / "F"
    let ex1 = Expected::default() // "(wschar)   1"
        .routine(
            &Ro::default()
                .push(&Co::Once(Op::Or(wschar.clone())))
                .push(&Co::Once(Op::One(Cnd::Pin(' '))))
                .push(&Co::Once(Op::One(Cnd::Pin(' '))))
                .push(&Co::Once(Op::One(Cnd::Pin(' '))))
                .push(&Co::Once(Op::One(Cnd::Pin('1'))))
                .build(),
        )
        .build();

    let ex2 = Expected::default() // "+(wschar)"
        .routine(
            &Ro::default()
                .push(&Co::Repeat(
                    Repeat::default()
                        .op(&Op::Or(wschar.clone()))
                        .min(1)
                        .max_not_included(usize::MAX)
                        .build(),
                ))
                .push(&Co::Once(Op::One(Cnd::Pin('1'))))
                .build(),
        )
        .build();

    let ex3 = Expected::default() // "(wschar){5,}"
        .routine(
            &Ro::default()
                .push(&Co::Repeat(
                    Repeat::default()
                        .op(&Op::Or(wschar.clone()))
                        .min(5)
                        .max_not_included(usize::MAX)
                        .build(),
                ))
                .push(&Co::Once(Op::One(Cnd::Pin('1'))))
                .build(),
        )
        .build();

    let ex4 = Expected::default() // "(wschar){0,3}"
        .routine(
            &Ro::default()
                .push(&Co::Repeat(
                    Repeat::default()
                        .op(&Op::Or(wschar.clone()))
                        .min(0)
                        .max_not_included(3)
                        .build(),
                ))
                .push(&Co::Once(Op::One(Cnd::Pin('1'))))
                .build(),
        )
        .build();
    let ex5 = Expected::default() // "(wschar){1,}"
        .routine(
            &Ro::default()
                .push(&Co::Repeat(
                    Repeat::default()
                        .op(&Op::Or(wschar.clone()))
                        .min(1)
                        .max_not_included(usize::MAX)
                        .build(),
                ))
                .push(&Co::Once(Op::One(digit.clone())))
                .build(),
        )
        .build();
    let ex6 = Expected::default() // "(alpha)"
        .routine(&Ro::default().push(&Co::Once(Op::Or(alpha.clone()))).build())
        .build();
    let ex7 = Expected::default() // "(alpha){1,3}"
        .routine(
            &Ro::default()
                .push(&Co::Repeat(
                    Repeat::default()
                        .op(&Op::Or(alpha.clone()))
                        .min(1)
                        .max_not_included(3)
                        .build(),
                ))
                .build(),
        )
        .build();

    let ex8 = Expected::default() // "(alpha){1,}"
        .routine(
            &Ro::default()
                .push(&Co::Repeat(
                    Repeat::default()
                        .op(&Op::Or(alpha.clone()))
                        .min(1)
                        .max_not_included(usize::MAX)
                        .build(),
                ))
                .build(),
        )
        .build();

    let ex9 = Expected::default() // "(newline)"
        .routine(
            &Ro::default()
                .push(&Co::Once(Op::Or(newline.clone())))
                .build(),
        )
        .build();

    let ex10 = Expected::default().routine(&comment).build();
    let ex11 = Expected::default().routine(&unquoted_key).build();

    assert!(Ma::default().actual(&ac1).expected(&ex1).build().exec());
    assert!(Ma::default().actual(&ac2).expected(&ex1).build().exec());
    assert!(!Ma::default().actual(&ac3).expected(&ex1).build().exec());
    assert!(Ma::default().actual(&ac1).expected(&ex2).build().exec());
    assert!(!Ma::default().actual(&ac1).expected(&ex3).build().exec());
    assert!(!Ma::default().actual(&ac1).expected(&ex4).build().exec());
    assert!(Ma::default().actual(&ac1).expected(&ex5).build().exec());
    assert!(Ma::default().actual(&ac4).expected(&ex6).build().exec());
    assert!(Ma::default().actual(&ac5).expected(&ex7).build().exec());
    assert!(Ma::default().actual(&ac6).expected(&ex7).build().exec());
    assert!(Ma::default().actual(&ac7).expected(&ex8).build().exec());
    assert!(Ma::default().actual(&ac8).expected(&ex9).build().exec());
    // コメントのテスト
    assert!(Ma::default().actual(&ac9).expected(&ex10).build().exec());
    // コメントではないもののテスト
    assert!(!Ma::default().actual(&ac5).expected(&ex10).build().exec());
    // 'No-1_0' - Unquloted key.
    assert!(Ma::default().actual(&ac10).expected(&ex11).build().exec());
    // Newline is not Unquloted key.
    assert!(!Ma::default().actual(&ac8).expected(&ex11).build().exec());

    println!("Finished.");
}
