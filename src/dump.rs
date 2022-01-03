
use std::fs::write;
use crate::vm::Context;

fn html_page(body: String) -> String {
    let mut res = String::new();
    res.push_str("<!DOCTYPE html><html><head><meta charset=\"utf-8\"></head><body>");
    res.push_str("<style>
    * {
        font-family: monospace;
        font-size: 1rem;
        box-sizing: border-box;
    }
    table, tr, td {
        border: 1px solid black;
        border-collapse: collapse;
    }
    table {
        max-width: 100%;
    }
    td {
        min-width: 3rem;
    }
    </style>");
    res.push_str(&body);
    res.push_str("</body></html>");
    return res;
}

fn dump_sp(ctx: &mut Context) -> String {
    let mut res = String::new();
    res.push_str("<p>Stack pointer: ");
    res.push_str(&format!("{}", ctx.sp));
    res.push_str("</p>");
    return res;
}

fn dump_stack(ctx: &mut Context) -> String {
    let mut res = String::new();
    res.push_str("<table><tr><td>Idx Hex</td>");    
    for i in 0..ctx.stack.len() {
        res.push_str(&format!("<td class=\"i{}\">{:#X}</td>", i, i));
    }
    res.push_str("</tr><tr><td>Idx Dec</td>");
    for i in 0..ctx.stack.len() {
        res.push_str(&format!("<td class=\"i{}\">{}</td>", i, i));
    }
    res.push_str("</tr><tr><td>Value</td>");
    for (i, v) in ctx.stack.iter().enumerate() {
        res.push_str(&format!("<td class=\"i{}\">{}</td>", i, v));
    }
    res.push_str("</tr></table>");
    res.push_str(&format!("<style>.i{} {{ background-color: #BFF  }}</style>", ctx.sp));
    return res;
}

pub fn dump(ctx: &mut Context) {
    let mut content = String::new();
    content.push_str(&dump_sp(ctx));
    content.push_str(&dump_stack(ctx));
    let page = html_page(content);
    write("dump.html", page).expect("could not write to dump.html")
}
