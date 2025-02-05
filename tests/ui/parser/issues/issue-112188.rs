//@ run-rustfix

#![allow(unused)]

struct Foo { x: i32 }

fn main() {
    let f = Foo { x: 0 };
    let Foo { .. } = f;
    let Foo { .., } = f; //~ ERROR expected `}`, found `,`
    let Foo { x, .. } = f;
    let Foo { .., x } = f; //~ ERROR expected `}`, found `,`
    let Foo { .., x, .. } = f; //~ ERROR expected `}`, found `,`
}
