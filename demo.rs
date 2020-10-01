#![feature(fn_traits)]
#![feature(unboxed_closures)]
use std::marker::*;
pub enum Temp2<A, B, F, G>{
    N0(F, PhantomData<*const (A, B)>),
    N1(G, PhantomData<*const (A, B)>)
}
impl<A, B, F: FnMut<A, Output=B>, G: FnMut<A, Output=B>> FnMut<A> for Temp2<A,B,F,G> {
    extern "rust-call" fn call_mut(&mut self, arg: A) -> B {
        match self{
            Temp2::N0(f,_)=>f.call_mut(arg),
            Temp2::N1(g,_)=>g.call_mut(arg)
        }
    }
}
impl<A, B, F: FnOnce<A, Output=B>, G: FnOnce<A, Output=B>> FnOnce<A> for Temp2<A,B,F,G> {
    type Output=B;
    extern "rust-call" fn call_once(self, arg: A) -> B {
        match self{
            Temp2::N0(f,_)=>f.call_once(arg),
            Temp2::N1(g,_)=>g.call_once(arg)
        }
    }
}
impl<A, B, F: Fn<A, Output=B>, G: Fn<A, Output=B>> Fn<A> for Temp2<A,B,F,G> {
    extern "rust-call" fn call(&self, arg: A) -> B {
        match self{
            Temp2::N0(f,_)=>f.call(arg),
            Temp2::N1(g,_)=>g.call(arg)
        }
    }
}
pub fn multiplex<A,B>(a: bool, f: impl FnMut<A,Output=B>, g: impl FnMut<A,Output=B>)
->impl FnMut<A,Output=B>{
    if a{
        Temp2::N0(f, PhantomData)
    }else{
        Temp2::N1(g, PhantomData)
    }
}
static mut y: i32=0;
fn main(){
    let mut x: i32=0;
    let by=unsafe {&mut y};
    let add = |a:i32, b| {x+=1; a+b};
    let sub= |a:i32, b| {*by+=1; a-b};
    let c=multiplex(false, add, sub)(114i32, 514i32);
    println!("{},{},{}",x,*by,c);
}
