# rust-closure-sum-type
Auto fixing types for returning Rust closure on different closure types.

## Problem

Each closure in Rust has a unique type. If we want to return a closure on two branches of an expression (e.g. match-expression or if-expression), we will be faced with a type-mismatch problem.

```
fn main(){
    let mut counter_add=0;
    let mut counter_sub=0;
    let mut add = |a:i32, b: i32| {counter_add+=1;a+b};
    let mut sub = |a:i32, b: i32| {counter_sub+=1;a-b};
    let foo=false;
    let c=(if foo{
        add
    }else{
        sub
    })(114, 514);
    assert_eq!(c, -400);
}
```

This will lead to an error:
```
   Compiling playground v0.0.1 (/playground)
error[E0308]: `if` and `else` have incompatible types
  --> src/main.rs:9:9
   |
4  |       let sub = |a:i32, b: i32| {counter+=1;a-b};
   |                 -------------------------------- the found closure
5  |       let foo=false;
6  |       let c=(if foo{
   |  ___________-
7  | |         add
   | |         --- expected because of this
8  | |     }else{
9  | |         sub
   | |         ^^^ expected closure, found a different closure
10 | |     })(114, 514);
   | |______- `if` and `else` have incompatible types
   |
   = note: expected type `[closure@src/main.rs:3:15: 3:47]`
           found closure `[closure@src/main.rs:4:15: 4:47]`
   = note: no two closures, even if identical, have the same type
   = help: consider boxing your closure and/or using it as a trait object

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
error: could not compile `playground`

To learn more, run the command again with --verbose.
```

This uniqueness in type of closure prevents us from writing Rust more functionally.

## Bad solution: using product type.

```
fn main(){
    let mut counter_add=0;
    let mut counter_sub=0;
    let mut add = |a:i32, b: i32| {counter_add+=1;a+b};
    let mut sub = |a:i32, b: i32| {counter_sub+=1;a-b};
    let foo=false;
    let c=(|a,b| (if foo{
        add(a,b)
    }else{
        sub(a,b)
    }))(114, 514);
    assert_eq!(c, -400);
}
```

## Good solution: using sum type.
