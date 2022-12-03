use pulldown_cmark::{Parser, Options, Event::*, md::push_md};


fn test_latex(original: &str, blocklatex: &str) {
    // let p = Parser::new_ext(&original, Options::all());

    // for event in p {
    //     match event {
    //         e => println!("{e:?}")
    //     }
    // }

    let p = Parser::new_ext(&original, Options::all());
    let mut no_latex = true;
    for event in p {
        match event {
            BlockLaTex(s) => {
                no_latex = false;
                assert_eq!(s.trim(), blocklatex.trim())
            },
            _ => ()
        }
    }
    assert!(!no_latex, "No Block Latex found");
}


#[test]
fn latex_test_1() {
    let blocklatex = r##"
\[
\begin{aligned}
\textit{pat}:\qquad\qquad &\texttt{EXAMPLE} \\
\textit{string}:\qquad\quad &\texttt{HERE IS A SIMPLE EXAMPLE} \dots \\
&\qquad\ \ \ \, \, \Uparrow
\end{aligned}
\]"##;

    let original = r##"## 基础介绍
想象一下，如果我们的的模式字符串$pat$，被放在文本字符串$string$的左手起头部，使它们的第一个字符对齐。
"##.to_owned() + blocklatex + r##"在这里做定义，往后不赘述：

$pat$的长度为 $patlen$，特别地对于从0开始的串来说，规定$patlastpos=patlen-1$为$pat$串最后一个字符的位置;
"##;

    test_latex(&original, blocklatex)
}


#[test]
fn latex_test_2() {

    let blocklatex = r##"
$$
\\begin{aligned}
\\textit{pat}:\qquad\qquad &\texttt{AT-THAT} \\
\\textit{string}:\ \ \ \dots\ &\texttt{WHICH-FINALLY-HALTS.–AT-THAT-POINT} \dots \\
&\qquad\ \ \ , , \Uparrow
\\end{aligned}
$$
"##;

    let original = r##"## 实例说明：

箭头指向失配字符$char$："##.to_owned() + blocklatex + r##"$\texttt{F}$没有出现$pat$中，根据**观察1**，$pat$直接向下移动$patlen$个字符，也就是7​个字符:
}"##;

    test_latex(&original, blocklatex);

}

#[test]
fn latex_push_back_test() {
    let original = r##"asc`push` $\{1,2\}$ def"##;

    let parser = Parser::new_ext(&original, Options::all());
    let mut cache = String::new();

    push_md(parser, &mut cache).unwrap();

    println!("{cache}");
}


#[ignore = "debug entry"]
#[test]
fn latex_test_3() {
    let original = r##"## 实例说明：

![](asasa.mpd)

"##;

    let p = Parser::new_ext(&original, Options::all());

    for event in p {
        match event {
            e => println!("{e:?}")
        }
    }

}
