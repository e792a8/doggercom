use ::libc;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type MD4_u32plus = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MD4_CTX {
    pub lo: MD4_u32plus,
    pub hi: MD4_u32plus,
    pub a: MD4_u32plus,
    pub b: MD4_u32plus,
    pub c: MD4_u32plus,
    pub d: MD4_u32plus,
    pub buffer: [libc::c_uchar; 64],
    pub block: [MD4_u32plus; 16],
}
unsafe extern "C" fn body(
    mut ctx: *mut MD4_CTX,
    mut data: *const libc::c_void,
    mut size: libc::c_ulong,
) -> *const libc::c_void {
    let mut ptr: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut a: MD4_u32plus = 0;
    let mut b: MD4_u32plus = 0;
    let mut c: MD4_u32plus = 0;
    let mut d: MD4_u32plus = 0;
    let mut saved_a: MD4_u32plus = 0;
    let mut saved_b: MD4_u32plus = 0;
    let mut saved_c: MD4_u32plus = 0;
    let mut saved_d: MD4_u32plus = 0;
    let ac1: MD4_u32plus = 0x5a827999 as libc::c_int as MD4_u32plus;
    let ac2: MD4_u32plus = 0x6ed9eba1 as libc::c_int as MD4_u32plus;
    ptr = data as *const libc::c_uchar;
    a = (*ctx).a;
    b = (*ctx).b;
    c = (*ctx).c;
    d = (*ctx).d;
    loop {
        saved_a = a;
        saved_b = b;
        saved_c = c;
        saved_d = d;
        a = (a as libc::c_uint)
            .wrapping_add(
                (d ^ b & (c ^ d))
                    .wrapping_add(
                        *(&*ptr.offset((0 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD4_u32plus),
                    ),
            ) as MD4_u32plus as MD4_u32plus;
        a = a << 3 as libc::c_int
            | (a & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 3 as libc::c_int;
        d = (d as libc::c_uint)
            .wrapping_add(
                (c ^ a & (b ^ c))
                    .wrapping_add(
                        *(&*ptr.offset((1 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD4_u32plus),
                    ),
            ) as MD4_u32plus as MD4_u32plus;
        d = d << 7 as libc::c_int
            | (d & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 7 as libc::c_int;
        c = (c as libc::c_uint)
            .wrapping_add(
                (b ^ d & (a ^ b))
                    .wrapping_add(
                        *(&*ptr.offset((2 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD4_u32plus),
                    ),
            ) as MD4_u32plus as MD4_u32plus;
        c = c << 11 as libc::c_int
            | (c & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 11 as libc::c_int;
        b = (b as libc::c_uint)
            .wrapping_add(
                (a ^ c & (d ^ a))
                    .wrapping_add(
                        *(&*ptr.offset((3 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD4_u32plus),
                    ),
            ) as MD4_u32plus as MD4_u32plus;
        b = b << 19 as libc::c_int
            | (b & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 19 as libc::c_int;
        a = (a as libc::c_uint)
            .wrapping_add(
                (d ^ b & (c ^ d))
                    .wrapping_add(
                        *(&*ptr.offset((4 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD4_u32plus),
                    ),
            ) as MD4_u32plus as MD4_u32plus;
        a = a << 3 as libc::c_int
            | (a & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 3 as libc::c_int;
        d = (d as libc::c_uint)
            .wrapping_add(
                (c ^ a & (b ^ c))
                    .wrapping_add(
                        *(&*ptr.offset((5 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD4_u32plus),
                    ),
            ) as MD4_u32plus as MD4_u32plus;
        d = d << 7 as libc::c_int
            | (d & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 7 as libc::c_int;
        c = (c as libc::c_uint)
            .wrapping_add(
                (b ^ d & (a ^ b))
                    .wrapping_add(
                        *(&*ptr.offset((6 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD4_u32plus),
                    ),
            ) as MD4_u32plus as MD4_u32plus;
        c = c << 11 as libc::c_int
            | (c & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 11 as libc::c_int;
        b = (b as libc::c_uint)
            .wrapping_add(
                (a ^ c & (d ^ a))
                    .wrapping_add(
                        *(&*ptr.offset((7 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD4_u32plus),
                    ),
            ) as MD4_u32plus as MD4_u32plus;
        b = b << 19 as libc::c_int
            | (b & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 19 as libc::c_int;
        a = (a as libc::c_uint)
            .wrapping_add(
                (d ^ b & (c ^ d))
                    .wrapping_add(
                        *(&*ptr.offset((8 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD4_u32plus),
                    ),
            ) as MD4_u32plus as MD4_u32plus;
        a = a << 3 as libc::c_int
            | (a & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 3 as libc::c_int;
        d = (d as libc::c_uint)
            .wrapping_add(
                (c ^ a & (b ^ c))
                    .wrapping_add(
                        *(&*ptr.offset((9 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD4_u32plus),
                    ),
            ) as MD4_u32plus as MD4_u32plus;
        d = d << 7 as libc::c_int
            | (d & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 7 as libc::c_int;
        c = (c as libc::c_uint)
            .wrapping_add(
                (b ^ d & (a ^ b))
                    .wrapping_add(
                        *(&*ptr.offset((10 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD4_u32plus),
                    ),
            ) as MD4_u32plus as MD4_u32plus;
        c = c << 11 as libc::c_int
            | (c & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 11 as libc::c_int;
        b = (b as libc::c_uint)
            .wrapping_add(
                (a ^ c & (d ^ a))
                    .wrapping_add(
                        *(&*ptr.offset((11 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD4_u32plus),
                    ),
            ) as MD4_u32plus as MD4_u32plus;
        b = b << 19 as libc::c_int
            | (b & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 19 as libc::c_int;
        a = (a as libc::c_uint)
            .wrapping_add(
                (d ^ b & (c ^ d))
                    .wrapping_add(
                        *(&*ptr.offset((12 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD4_u32plus),
                    ),
            ) as MD4_u32plus as MD4_u32plus;
        a = a << 3 as libc::c_int
            | (a & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 3 as libc::c_int;
        d = (d as libc::c_uint)
            .wrapping_add(
                (c ^ a & (b ^ c))
                    .wrapping_add(
                        *(&*ptr.offset((13 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD4_u32plus),
                    ),
            ) as MD4_u32plus as MD4_u32plus;
        d = d << 7 as libc::c_int
            | (d & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 7 as libc::c_int;
        c = (c as libc::c_uint)
            .wrapping_add(
                (b ^ d & (a ^ b))
                    .wrapping_add(
                        *(&*ptr.offset((14 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD4_u32plus),
                    ),
            ) as MD4_u32plus as MD4_u32plus;
        c = c << 11 as libc::c_int
            | (c & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 11 as libc::c_int;
        b = (b as libc::c_uint)
            .wrapping_add(
                (a ^ c & (d ^ a))
                    .wrapping_add(
                        *(&*ptr.offset((15 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD4_u32plus),
                    ),
            ) as MD4_u32plus as MD4_u32plus;
        b = b << 19 as libc::c_int
            | (b & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 19 as libc::c_int;
        a = (a as libc::c_uint)
            .wrapping_add(
                (b & (c | d) | c & d)
                    .wrapping_add(
                        (*(&*ptr.offset((0 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD4_u32plus))
                            .wrapping_add(ac1),
                    ),
            ) as MD4_u32plus as MD4_u32plus;
        a = a << 3 as libc::c_int
            | (a & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 3 as libc::c_int;
        d = (d as libc::c_uint)
            .wrapping_add(
                (a & (b | c) | b & c)
                    .wrapping_add(
                        (*(&*ptr.offset((4 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD4_u32plus))
                            .wrapping_add(ac1),
                    ),
            ) as MD4_u32plus as MD4_u32plus;
        d = d << 5 as libc::c_int
            | (d & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 5 as libc::c_int;
        c = (c as libc::c_uint)
            .wrapping_add(
                (d & (a | b) | a & b)
                    .wrapping_add(
                        (*(&*ptr.offset((8 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD4_u32plus))
                            .wrapping_add(ac1),
                    ),
            ) as MD4_u32plus as MD4_u32plus;
        c = c << 9 as libc::c_int
            | (c & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 9 as libc::c_int;
        b = (b as libc::c_uint)
            .wrapping_add(
                (c & (d | a) | d & a)
                    .wrapping_add(
                        (*(&*ptr.offset((12 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD4_u32plus))
                            .wrapping_add(ac1),
                    ),
            ) as MD4_u32plus as MD4_u32plus;
        b = b << 13 as libc::c_int
            | (b & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 13 as libc::c_int;
        a = (a as libc::c_uint)
            .wrapping_add(
                (b & (c | d) | c & d)
                    .wrapping_add(
                        (*(&*ptr.offset((1 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD4_u32plus))
                            .wrapping_add(ac1),
                    ),
            ) as MD4_u32plus as MD4_u32plus;
        a = a << 3 as libc::c_int
            | (a & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 3 as libc::c_int;
        d = (d as libc::c_uint)
            .wrapping_add(
                (a & (b | c) | b & c)
                    .wrapping_add(
                        (*(&*ptr.offset((5 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD4_u32plus))
                            .wrapping_add(ac1),
                    ),
            ) as MD4_u32plus as MD4_u32plus;
        d = d << 5 as libc::c_int
            | (d & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 5 as libc::c_int;
        c = (c as libc::c_uint)
            .wrapping_add(
                (d & (a | b) | a & b)
                    .wrapping_add(
                        (*(&*ptr.offset((9 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD4_u32plus))
                            .wrapping_add(ac1),
                    ),
            ) as MD4_u32plus as MD4_u32plus;
        c = c << 9 as libc::c_int
            | (c & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 9 as libc::c_int;
        b = (b as libc::c_uint)
            .wrapping_add(
                (c & (d | a) | d & a)
                    .wrapping_add(
                        (*(&*ptr.offset((13 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD4_u32plus))
                            .wrapping_add(ac1),
                    ),
            ) as MD4_u32plus as MD4_u32plus;
        b = b << 13 as libc::c_int
            | (b & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 13 as libc::c_int;
        a = (a as libc::c_uint)
            .wrapping_add(
                (b & (c | d) | c & d)
                    .wrapping_add(
                        (*(&*ptr.offset((2 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD4_u32plus))
                            .wrapping_add(ac1),
                    ),
            ) as MD4_u32plus as MD4_u32plus;
        a = a << 3 as libc::c_int
            | (a & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 3 as libc::c_int;
        d = (d as libc::c_uint)
            .wrapping_add(
                (a & (b | c) | b & c)
                    .wrapping_add(
                        (*(&*ptr.offset((6 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD4_u32plus))
                            .wrapping_add(ac1),
                    ),
            ) as MD4_u32plus as MD4_u32plus;
        d = d << 5 as libc::c_int
            | (d & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 5 as libc::c_int;
        c = (c as libc::c_uint)
            .wrapping_add(
                (d & (a | b) | a & b)
                    .wrapping_add(
                        (*(&*ptr.offset((10 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD4_u32plus))
                            .wrapping_add(ac1),
                    ),
            ) as MD4_u32plus as MD4_u32plus;
        c = c << 9 as libc::c_int
            | (c & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 9 as libc::c_int;
        b = (b as libc::c_uint)
            .wrapping_add(
                (c & (d | a) | d & a)
                    .wrapping_add(
                        (*(&*ptr.offset((14 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD4_u32plus))
                            .wrapping_add(ac1),
                    ),
            ) as MD4_u32plus as MD4_u32plus;
        b = b << 13 as libc::c_int
            | (b & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 13 as libc::c_int;
        a = (a as libc::c_uint)
            .wrapping_add(
                (b & (c | d) | c & d)
                    .wrapping_add(
                        (*(&*ptr.offset((3 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD4_u32plus))
                            .wrapping_add(ac1),
                    ),
            ) as MD4_u32plus as MD4_u32plus;
        a = a << 3 as libc::c_int
            | (a & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 3 as libc::c_int;
        d = (d as libc::c_uint)
            .wrapping_add(
                (a & (b | c) | b & c)
                    .wrapping_add(
                        (*(&*ptr.offset((7 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD4_u32plus))
                            .wrapping_add(ac1),
                    ),
            ) as MD4_u32plus as MD4_u32plus;
        d = d << 5 as libc::c_int
            | (d & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 5 as libc::c_int;
        c = (c as libc::c_uint)
            .wrapping_add(
                (d & (a | b) | a & b)
                    .wrapping_add(
                        (*(&*ptr.offset((11 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD4_u32plus))
                            .wrapping_add(ac1),
                    ),
            ) as MD4_u32plus as MD4_u32plus;
        c = c << 9 as libc::c_int
            | (c & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 9 as libc::c_int;
        b = (b as libc::c_uint)
            .wrapping_add(
                (c & (d | a) | d & a)
                    .wrapping_add(
                        (*(&*ptr.offset((15 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD4_u32plus))
                            .wrapping_add(ac1),
                    ),
            ) as MD4_u32plus as MD4_u32plus;
        b = b << 13 as libc::c_int
            | (b & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 13 as libc::c_int;
        a = (a as libc::c_uint)
            .wrapping_add(
                (b ^ c ^ d)
                    .wrapping_add(
                        (*(&*ptr.offset((0 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD4_u32plus))
                            .wrapping_add(ac2),
                    ),
            ) as MD4_u32plus as MD4_u32plus;
        a = a << 3 as libc::c_int
            | (a & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 3 as libc::c_int;
        d = (d as libc::c_uint)
            .wrapping_add(
                (a ^ b ^ c)
                    .wrapping_add(
                        (*(&*ptr.offset((8 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD4_u32plus))
                            .wrapping_add(ac2),
                    ),
            ) as MD4_u32plus as MD4_u32plus;
        d = d << 9 as libc::c_int
            | (d & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 9 as libc::c_int;
        c = (c as libc::c_uint)
            .wrapping_add(
                (d ^ a ^ b)
                    .wrapping_add(
                        (*(&*ptr.offset((4 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD4_u32plus))
                            .wrapping_add(ac2),
                    ),
            ) as MD4_u32plus as MD4_u32plus;
        c = c << 11 as libc::c_int
            | (c & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 11 as libc::c_int;
        b = (b as libc::c_uint)
            .wrapping_add(
                (c ^ d ^ a)
                    .wrapping_add(
                        (*(&*ptr.offset((12 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD4_u32plus))
                            .wrapping_add(ac2),
                    ),
            ) as MD4_u32plus as MD4_u32plus;
        b = b << 15 as libc::c_int
            | (b & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 15 as libc::c_int;
        a = (a as libc::c_uint)
            .wrapping_add(
                (b ^ c ^ d)
                    .wrapping_add(
                        (*(&*ptr.offset((2 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD4_u32plus))
                            .wrapping_add(ac2),
                    ),
            ) as MD4_u32plus as MD4_u32plus;
        a = a << 3 as libc::c_int
            | (a & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 3 as libc::c_int;
        d = (d as libc::c_uint)
            .wrapping_add(
                (a ^ b ^ c)
                    .wrapping_add(
                        (*(&*ptr.offset((10 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD4_u32plus))
                            .wrapping_add(ac2),
                    ),
            ) as MD4_u32plus as MD4_u32plus;
        d = d << 9 as libc::c_int
            | (d & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 9 as libc::c_int;
        c = (c as libc::c_uint)
            .wrapping_add(
                (d ^ a ^ b)
                    .wrapping_add(
                        (*(&*ptr.offset((6 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD4_u32plus))
                            .wrapping_add(ac2),
                    ),
            ) as MD4_u32plus as MD4_u32plus;
        c = c << 11 as libc::c_int
            | (c & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 11 as libc::c_int;
        b = (b as libc::c_uint)
            .wrapping_add(
                (c ^ d ^ a)
                    .wrapping_add(
                        (*(&*ptr.offset((14 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD4_u32plus))
                            .wrapping_add(ac2),
                    ),
            ) as MD4_u32plus as MD4_u32plus;
        b = b << 15 as libc::c_int
            | (b & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 15 as libc::c_int;
        a = (a as libc::c_uint)
            .wrapping_add(
                (b ^ c ^ d)
                    .wrapping_add(
                        (*(&*ptr.offset((1 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD4_u32plus))
                            .wrapping_add(ac2),
                    ),
            ) as MD4_u32plus as MD4_u32plus;
        a = a << 3 as libc::c_int
            | (a & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 3 as libc::c_int;
        d = (d as libc::c_uint)
            .wrapping_add(
                (a ^ b ^ c)
                    .wrapping_add(
                        (*(&*ptr.offset((9 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD4_u32plus))
                            .wrapping_add(ac2),
                    ),
            ) as MD4_u32plus as MD4_u32plus;
        d = d << 9 as libc::c_int
            | (d & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 9 as libc::c_int;
        c = (c as libc::c_uint)
            .wrapping_add(
                (d ^ a ^ b)
                    .wrapping_add(
                        (*(&*ptr.offset((5 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD4_u32plus))
                            .wrapping_add(ac2),
                    ),
            ) as MD4_u32plus as MD4_u32plus;
        c = c << 11 as libc::c_int
            | (c & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 11 as libc::c_int;
        b = (b as libc::c_uint)
            .wrapping_add(
                (c ^ d ^ a)
                    .wrapping_add(
                        (*(&*ptr.offset((13 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD4_u32plus))
                            .wrapping_add(ac2),
                    ),
            ) as MD4_u32plus as MD4_u32plus;
        b = b << 15 as libc::c_int
            | (b & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 15 as libc::c_int;
        a = (a as libc::c_uint)
            .wrapping_add(
                (b ^ c ^ d)
                    .wrapping_add(
                        (*(&*ptr.offset((3 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD4_u32plus))
                            .wrapping_add(ac2),
                    ),
            ) as MD4_u32plus as MD4_u32plus;
        a = a << 3 as libc::c_int
            | (a & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 3 as libc::c_int;
        d = (d as libc::c_uint)
            .wrapping_add(
                (a ^ b ^ c)
                    .wrapping_add(
                        (*(&*ptr.offset((11 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD4_u32plus))
                            .wrapping_add(ac2),
                    ),
            ) as MD4_u32plus as MD4_u32plus;
        d = d << 9 as libc::c_int
            | (d & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 9 as libc::c_int;
        c = (c as libc::c_uint)
            .wrapping_add(
                (d ^ a ^ b)
                    .wrapping_add(
                        (*(&*ptr.offset((7 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD4_u32plus))
                            .wrapping_add(ac2),
                    ),
            ) as MD4_u32plus as MD4_u32plus;
        c = c << 11 as libc::c_int
            | (c & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 11 as libc::c_int;
        b = (b as libc::c_uint)
            .wrapping_add(
                (c ^ d ^ a)
                    .wrapping_add(
                        (*(&*ptr.offset((15 as libc::c_int * 4 as libc::c_int) as isize)
                            as *const libc::c_uchar as *mut MD4_u32plus))
                            .wrapping_add(ac2),
                    ),
            ) as MD4_u32plus as MD4_u32plus;
        b = b << 15 as libc::c_int
            | (b & 0xffffffff as libc::c_uint) >> 32 as libc::c_int - 15 as libc::c_int;
        a = (a as libc::c_uint).wrapping_add(saved_a) as MD4_u32plus as MD4_u32plus;
        b = (b as libc::c_uint).wrapping_add(saved_b) as MD4_u32plus as MD4_u32plus;
        c = (c as libc::c_uint).wrapping_add(saved_c) as MD4_u32plus as MD4_u32plus;
        d = (d as libc::c_uint).wrapping_add(saved_d) as MD4_u32plus as MD4_u32plus;
        ptr = ptr.offset(64 as libc::c_int as isize);
        size = size.wrapping_sub(64 as libc::c_int as libc::c_ulong);
        if !(size != 0) {
            break;
        }
    }
    (*ctx).a = a;
    (*ctx).b = b;
    (*ctx).c = c;
    (*ctx).d = d;
    return ptr as *const libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn MD4_Init(mut ctx: *mut MD4_CTX) {
    (*ctx).a = 0x67452301 as libc::c_int as MD4_u32plus;
    (*ctx).b = 0xefcdab89 as libc::c_uint;
    (*ctx).c = 0x98badcfe as libc::c_uint;
    (*ctx).d = 0x10325476 as libc::c_int as MD4_u32plus;
    (*ctx).lo = 0 as libc::c_int as MD4_u32plus;
    (*ctx).hi = 0 as libc::c_int as MD4_u32plus;
}
#[no_mangle]
pub unsafe extern "C" fn MD4_Update(
    mut ctx: *mut MD4_CTX,
    mut data: *const libc::c_void,
    mut size: libc::c_ulong,
) {
    let mut saved_lo: MD4_u32plus = 0;
    let mut used: libc::c_ulong = 0;
    let mut available: libc::c_ulong = 0;
    saved_lo = (*ctx).lo;
    (*ctx)
        .lo = ((saved_lo as libc::c_ulong).wrapping_add(size)
        & 0x1fffffff as libc::c_int as libc::c_ulong) as MD4_u32plus;
    if (*ctx).lo < saved_lo {
        (*ctx).hi = ((*ctx).hi).wrapping_add(1);
        (*ctx).hi;
    }
    (*ctx)
        .hi = ((*ctx).hi as libc::c_ulong).wrapping_add(size >> 29 as libc::c_int)
        as MD4_u32plus as MD4_u32plus;
    used = (saved_lo & 0x3f as libc::c_int as libc::c_uint) as libc::c_ulong;
    if used != 0 {
        available = (64 as libc::c_int as libc::c_ulong).wrapping_sub(used);
        if size < available {
            memcpy(
                &mut *((*ctx).buffer).as_mut_ptr().offset(used as isize)
                    as *mut libc::c_uchar as *mut libc::c_void,
                data,
                size,
            );
            return;
        }
        memcpy(
            &mut *((*ctx).buffer).as_mut_ptr().offset(used as isize)
                as *mut libc::c_uchar as *mut libc::c_void,
            data,
            available,
        );
        data = (data as *const libc::c_uchar).offset(available as isize)
            as *const libc::c_void;
        size = size.wrapping_sub(available);
        body(
            ctx,
            ((*ctx).buffer).as_mut_ptr() as *const libc::c_void,
            64 as libc::c_int as libc::c_ulong,
        );
    }
    if size >= 64 as libc::c_int as libc::c_ulong {
        data = body(ctx, data, size & !(0x3f as libc::c_int as libc::c_ulong));
        size &= 0x3f as libc::c_int as libc::c_ulong;
    }
    memcpy(((*ctx).buffer).as_mut_ptr() as *mut libc::c_void, data, size);
}
#[no_mangle]
pub unsafe extern "C" fn MD4_Final(
    mut result: *mut libc::c_uchar,
    mut ctx: *mut MD4_CTX,
) {
    let mut used: libc::c_ulong = 0;
    let mut available: libc::c_ulong = 0;
    used = ((*ctx).lo & 0x3f as libc::c_int as libc::c_uint) as libc::c_ulong;
    let fresh0 = used;
    used = used.wrapping_add(1);
    (*ctx).buffer[fresh0 as usize] = 0x80 as libc::c_int as libc::c_uchar;
    available = (64 as libc::c_int as libc::c_ulong).wrapping_sub(used);
    if available < 8 as libc::c_int as libc::c_ulong {
        memset(
            &mut *((*ctx).buffer).as_mut_ptr().offset(used as isize)
                as *mut libc::c_uchar as *mut libc::c_void,
            0 as libc::c_int,
            available,
        );
        body(
            ctx,
            ((*ctx).buffer).as_mut_ptr() as *const libc::c_void,
            64 as libc::c_int as libc::c_ulong,
        );
        used = 0 as libc::c_int as libc::c_ulong;
        available = 64 as libc::c_int as libc::c_ulong;
    }
    memset(
        &mut *((*ctx).buffer).as_mut_ptr().offset(used as isize) as *mut libc::c_uchar
            as *mut libc::c_void,
        0 as libc::c_int,
        available.wrapping_sub(8 as libc::c_int as libc::c_ulong),
    );
    (*ctx).lo <<= 3 as libc::c_int;
    *(&mut *((*ctx).buffer).as_mut_ptr().offset(56 as libc::c_int as isize)
        as *mut libc::c_uchar)
        .offset(0 as libc::c_int as isize) = (*ctx).lo as libc::c_uchar;
    *(&mut *((*ctx).buffer).as_mut_ptr().offset(56 as libc::c_int as isize)
        as *mut libc::c_uchar)
        .offset(
            1 as libc::c_int as isize,
        ) = ((*ctx).lo >> 8 as libc::c_int) as libc::c_uchar;
    *(&mut *((*ctx).buffer).as_mut_ptr().offset(56 as libc::c_int as isize)
        as *mut libc::c_uchar)
        .offset(
            2 as libc::c_int as isize,
        ) = ((*ctx).lo >> 16 as libc::c_int) as libc::c_uchar;
    *(&mut *((*ctx).buffer).as_mut_ptr().offset(56 as libc::c_int as isize)
        as *mut libc::c_uchar)
        .offset(
            3 as libc::c_int as isize,
        ) = ((*ctx).lo >> 24 as libc::c_int) as libc::c_uchar;
    *(&mut *((*ctx).buffer).as_mut_ptr().offset(60 as libc::c_int as isize)
        as *mut libc::c_uchar)
        .offset(0 as libc::c_int as isize) = (*ctx).hi as libc::c_uchar;
    *(&mut *((*ctx).buffer).as_mut_ptr().offset(60 as libc::c_int as isize)
        as *mut libc::c_uchar)
        .offset(
            1 as libc::c_int as isize,
        ) = ((*ctx).hi >> 8 as libc::c_int) as libc::c_uchar;
    *(&mut *((*ctx).buffer).as_mut_ptr().offset(60 as libc::c_int as isize)
        as *mut libc::c_uchar)
        .offset(
            2 as libc::c_int as isize,
        ) = ((*ctx).hi >> 16 as libc::c_int) as libc::c_uchar;
    *(&mut *((*ctx).buffer).as_mut_ptr().offset(60 as libc::c_int as isize)
        as *mut libc::c_uchar)
        .offset(
            3 as libc::c_int as isize,
        ) = ((*ctx).hi >> 24 as libc::c_int) as libc::c_uchar;
    body(
        ctx,
        ((*ctx).buffer).as_mut_ptr() as *const libc::c_void,
        64 as libc::c_int as libc::c_ulong,
    );
    *(&mut *result.offset(0 as libc::c_int as isize) as *mut libc::c_uchar)
        .offset(0 as libc::c_int as isize) = (*ctx).a as libc::c_uchar;
    *(&mut *result.offset(0 as libc::c_int as isize) as *mut libc::c_uchar)
        .offset(
            1 as libc::c_int as isize,
        ) = ((*ctx).a >> 8 as libc::c_int) as libc::c_uchar;
    *(&mut *result.offset(0 as libc::c_int as isize) as *mut libc::c_uchar)
        .offset(
            2 as libc::c_int as isize,
        ) = ((*ctx).a >> 16 as libc::c_int) as libc::c_uchar;
    *(&mut *result.offset(0 as libc::c_int as isize) as *mut libc::c_uchar)
        .offset(
            3 as libc::c_int as isize,
        ) = ((*ctx).a >> 24 as libc::c_int) as libc::c_uchar;
    *(&mut *result.offset(4 as libc::c_int as isize) as *mut libc::c_uchar)
        .offset(0 as libc::c_int as isize) = (*ctx).b as libc::c_uchar;
    *(&mut *result.offset(4 as libc::c_int as isize) as *mut libc::c_uchar)
        .offset(
            1 as libc::c_int as isize,
        ) = ((*ctx).b >> 8 as libc::c_int) as libc::c_uchar;
    *(&mut *result.offset(4 as libc::c_int as isize) as *mut libc::c_uchar)
        .offset(
            2 as libc::c_int as isize,
        ) = ((*ctx).b >> 16 as libc::c_int) as libc::c_uchar;
    *(&mut *result.offset(4 as libc::c_int as isize) as *mut libc::c_uchar)
        .offset(
            3 as libc::c_int as isize,
        ) = ((*ctx).b >> 24 as libc::c_int) as libc::c_uchar;
    *(&mut *result.offset(8 as libc::c_int as isize) as *mut libc::c_uchar)
        .offset(0 as libc::c_int as isize) = (*ctx).c as libc::c_uchar;
    *(&mut *result.offset(8 as libc::c_int as isize) as *mut libc::c_uchar)
        .offset(
            1 as libc::c_int as isize,
        ) = ((*ctx).c >> 8 as libc::c_int) as libc::c_uchar;
    *(&mut *result.offset(8 as libc::c_int as isize) as *mut libc::c_uchar)
        .offset(
            2 as libc::c_int as isize,
        ) = ((*ctx).c >> 16 as libc::c_int) as libc::c_uchar;
    *(&mut *result.offset(8 as libc::c_int as isize) as *mut libc::c_uchar)
        .offset(
            3 as libc::c_int as isize,
        ) = ((*ctx).c >> 24 as libc::c_int) as libc::c_uchar;
    *(&mut *result.offset(12 as libc::c_int as isize) as *mut libc::c_uchar)
        .offset(0 as libc::c_int as isize) = (*ctx).d as libc::c_uchar;
    *(&mut *result.offset(12 as libc::c_int as isize) as *mut libc::c_uchar)
        .offset(
            1 as libc::c_int as isize,
        ) = ((*ctx).d >> 8 as libc::c_int) as libc::c_uchar;
    *(&mut *result.offset(12 as libc::c_int as isize) as *mut libc::c_uchar)
        .offset(
            2 as libc::c_int as isize,
        ) = ((*ctx).d >> 16 as libc::c_int) as libc::c_uchar;
    *(&mut *result.offset(12 as libc::c_int as isize) as *mut libc::c_uchar)
        .offset(
            3 as libc::c_int as isize,
        ) = ((*ctx).d >> 24 as libc::c_int) as libc::c_uchar;
    memset(
        ctx as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<MD4_CTX>() as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn MD4(
    mut data: *const libc::c_void,
    mut size: libc::c_ulong,
    mut result: *mut libc::c_uchar,
) {
    let mut ctx: MD4_CTX = MD4_CTX {
        lo: 0,
        hi: 0,
        a: 0,
        b: 0,
        c: 0,
        d: 0,
        buffer: [0; 64],
        block: [0; 16],
    };
    MD4_Init(&mut ctx);
    MD4_Update(&mut ctx, data, size);
    MD4_Final(result, &mut ctx);
}
