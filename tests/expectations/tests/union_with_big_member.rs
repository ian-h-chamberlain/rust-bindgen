/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[derive(Debug)]
#[repr(C)]
pub struct __BindgenUnionField<T>(::std::marker::PhantomData<T>);
impl <T> __BindgenUnionField<T> {
    #[inline]
    pub fn new() -> Self { __BindgenUnionField(::std::marker::PhantomData) }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T { ::std::mem::transmute(self) }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T { ::std::mem::transmute(self) }
}
impl <T> ::std::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self { Self::new() }
}
impl <T> ::std::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self { Self::new() }
}
impl <T> ::std::marker::Copy for __BindgenUnionField<T> { }
#[repr(C)]
#[derive(Copy)]
pub struct WithBigArray {
    pub a: __BindgenUnionField<::std::os::raw::c_int>,
    pub b: __BindgenUnionField<[::std::os::raw::c_int; 33usize]>,
    pub bindgen_union_field: [u32; 33usize],
}
#[test]
fn bindgen_test_layout_WithBigArray() {
    assert_eq!(::std::mem::size_of::<WithBigArray>() , 132usize);
    assert_eq!(::std::mem::align_of::<WithBigArray>() , 4usize);
}
impl Clone for WithBigArray {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Copy)]
pub struct WithBigMember {
    pub a: __BindgenUnionField<::std::os::raw::c_int>,
    pub b: __BindgenUnionField<WithBigArray>,
    pub bindgen_union_field: [u32; 33usize],
}
#[test]
fn bindgen_test_layout_WithBigMember() {
    assert_eq!(::std::mem::size_of::<WithBigMember>() , 132usize);
    assert_eq!(::std::mem::align_of::<WithBigMember>() , 4usize);
}
impl Clone for WithBigMember {
    fn clone(&self) -> Self { *self }
}