/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Debug, Copy)]
pub struct CEXR_V2i {
    pub x: ::std::os::raw::c_int,
    pub y: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_CEXR_V2i() {
    assert_eq!(::std::mem::size_of::<CEXR_V2i>() , 8usize , concat ! (
               "Size of: " , stringify ! ( CEXR_V2i ) ));
    assert_eq! (::std::mem::align_of::<CEXR_V2i>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( CEXR_V2i ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const CEXR_V2i ) ) . x as * const _ as usize }
                , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( CEXR_V2i ) , "::" ,
                stringify ! ( x ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const CEXR_V2i ) ) . y as * const _ as usize }
                , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( CEXR_V2i ) , "::" ,
                stringify ! ( y ) ));
}
impl Clone for CEXR_V2i {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct CEXR_Box2i {
    pub min: CEXR_V2i,
    pub max: CEXR_V2i,
}
#[test]
fn bindgen_test_layout_CEXR_Box2i() {
    assert_eq!(::std::mem::size_of::<CEXR_Box2i>() , 16usize , concat ! (
               "Size of: " , stringify ! ( CEXR_Box2i ) ));
    assert_eq! (::std::mem::align_of::<CEXR_Box2i>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( CEXR_Box2i ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const CEXR_Box2i ) ) . min as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( CEXR_Box2i ) , "::" ,
                stringify ! ( min ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const CEXR_Box2i ) ) . max as * const _ as
                usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( CEXR_Box2i ) , "::" ,
                stringify ! ( max ) ));
}
impl Clone for CEXR_Box2i {
    fn clone(&self) -> Self { *self }
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CEXR_PixelType { UINT = 0, HALF = 1, FLOAT = 2, }
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CEXR_InputFile {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CEXR_OutputFile {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CEXR_Header {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CEXR_FrameBuffer {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CEXR_IStream {
    _unused: [u8; 0],
}
extern "C" {
    pub fn CEXR_IStream_from_memory(filename: *const ::std::os::raw::c_char,
                                    data: *mut ::std::os::raw::c_char,
                                    size: usize) -> *mut CEXR_IStream;
}
extern "C" {
    pub fn CEXR_IStream_delete(stream: *mut CEXR_IStream);
}
extern "C" {
    pub fn CEXR_InputFile_from_file(path: *const ::std::os::raw::c_char,
                                    threads: ::std::os::raw::c_int,
                                    out: *mut *mut CEXR_InputFile,
                                    err_out:
                                        *mut *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn CEXR_InputFile_from_stream(stream: *mut CEXR_IStream,
                                      threads: ::std::os::raw::c_int,
                                      out: *mut *mut CEXR_InputFile,
                                      err_out:
                                          *mut *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn CEXR_InputFile_delete(file: *mut CEXR_InputFile);
}
extern "C" {
    pub fn CEXR_InputFile_header(file: *mut CEXR_InputFile)
     -> *const CEXR_Header;
}
extern "C" {
    pub fn CEXR_InputFile_set_framebuffer(file: *mut CEXR_InputFile,
                                          framebuffer: *mut CEXR_FrameBuffer);
}
extern "C" {
    pub fn CEXR_InputFile_read_pixels(file: *mut CEXR_InputFile,
                                      scanline_1: ::std::os::raw::c_int,
                                      scanline_2: ::std::os::raw::c_int,
                                      err_out:
                                          *mut *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn CEXR_Header_display_window(header: *const CEXR_Header)
     -> *const CEXR_Box2i;
}
extern "C" {
    pub fn CEXR_Header_data_window(header: *const CEXR_Header)
     -> *const CEXR_Box2i;
}
extern "C" {
    pub fn CEXR_FrameBuffer_new() -> *mut CEXR_FrameBuffer;
}
extern "C" {
    pub fn CEXR_FrameBuffer_delete(framebuffer: *mut CEXR_FrameBuffer);
}
extern "C" {
    pub fn CEXR_FrameBuffer_insert(framebuffer: *mut CEXR_FrameBuffer,
                                   name: *const ::std::os::raw::c_char,
                                   type_: CEXR_PixelType,
                                   base: *mut ::std::os::raw::c_char,
                                   xStride: usize, yStride: usize,
                                   xSampling: ::std::os::raw::c_int,
                                   ySampling: ::std::os::raw::c_int,
                                   fillValue: f64,
                                   xTileCoords: ::std::os::raw::c_int,
                                   yTileCoords: ::std::os::raw::c_int);
}
