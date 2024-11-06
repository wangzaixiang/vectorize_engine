use std::simd::cmp::SimdOrd;
use std::simd::{simd_swizzle, u32x16, u32x32, u32x4, u32x8 };
use rand::random;

#[test]
fn test_sort_u32x8() {

    let v1 = [8u32,7,6,5,4,3,2,1];
    let mut nums = u32x8::from_slice(&v1);
    sort_u32x8(&mut nums);

    println!("nums = {:?}", nums)

}

#[test]
fn test_sort_u32x16() {

    for _ in 0..1024 {
        let mut nums: Vec<u32> = (0..16).map( |_| random::<u32>() % 1000 ).collect();
        let mut simd = u32x16::from_slice(&nums);
        sort_u32x16(&mut simd);

        nums.sort();
        assert_eq!(simd.as_array() as &[u32], &nums);
    }

}

#[test]
fn test_sort_u32x32() {

    for _ in 0..1024 {
        let mut nums: Vec<u32> = (0..32).map( |_| random::<u32>() % 1000 ).collect();
        let mut simd = u32x32::from_slice(&nums);
        sort_u32x32(&mut simd);

        nums.sort();
        assert_eq!(simd.as_array() as &[u32], &nums);
    }

}

#[test]
fn test_random(){

    let mut v1: Vec<u32> = (0..8).map (|_| random::<u32>() % 100).collect();
    let mut v2: Vec<u32> = (0..8).map (|_| random::<u32>() % 100).collect();
    v1.sort();
    v2.sort();

    println!("v1 = {:?}", v1);
    println!("v2 = {:?}", v2)

}

/// sort u32x8 using simd
#[inline]
pub fn sort_u32x8(nums: &mut u32x8) {
    let a: u32x4 = u32x4::from_slice(&nums[0..4]);
    let b: u32x4 = u32x4::from_slice(&nums[4..8]);

    // sort 1v1 * 4 group
    let min = a.simd_min(b);    // [4,3,2,1]
    let max = a.simd_max(b);    // [8,7,6,5]

    let a = simd_swizzle!(min, max, [0, 4, 1, 5]);  // [4,8,3,7]
    let b = simd_swizzle!(min, max, [2, 6, 3, 7]);  // [2,6,1,5]
    // end 1v1 * 4 group, now a, b is 2v2 * 2 group

    let min: u32x4 = a.simd_min(b);    // [2,6,1,5]
    let max: u32x4 = a.simd_max(b);    // [4,8,3,7]

    // round 1
    let tmp = min.rotate_elements_left::<1>(); // simd_swizzle!(min, [1, 0, 3, 2]);   // [6,2,5,1]
    let min = tmp.simd_min(max);  // [4,2,3,1]
    let max = tmp.simd_max(max);  // [6,8,5,7]

    // round 2
    let min = min.rotate_elements_left::<1>();

    let a  = simd_swizzle!(min, max, [0, 1, 4, 5]); // [2,4,6,8]
    let b = simd_swizzle!(min, max, [2, 3, 6, 7]);  // [1,3,5,7]
    // end 2v2 * 2group, now a, b is 4v4

    let min = a.simd_min(b);    // [1,3,5,7]
    let max = a.simd_max(b);    // [2,4,6,8]

    // round 1
    let tmp = min.rotate_elements_left::<1>();
    let min = tmp.simd_min(max);    // [2,4,6,1]
    let max = tmp.simd_max(max);    // [3,5,7,8]

    // round 2
    let tmp = min.rotate_elements_left::<1>();
    let min = tmp.simd_min(max);    // [3,5,1,2]
    let max = tmp.simd_max(max);    // [4,6,7,8]

    // round 3
    let tmp = min.rotate_elements_left::<1>();
    let min = tmp.simd_min(max);    // [4,1,2,3]
    let max: u32x4 = tmp.simd_max(max);    // [5,6,7,8]

    // round 4
    let min: u32x4 = min.rotate_elements_left::<1>();

    // test config is little end
    #[cfg(target_endian = "little")]
    {
        let p0: &mut u32x4 = unsafe { &mut *(&mut nums[0] as *mut u32 as *mut u32x4) };
        let p1: &mut u32x4 = unsafe { &mut *(&mut nums[4] as *mut u32 as *mut u32x4) };

        *p0 = min;
        *p1 = max;
    }

    #[cfg(target_endian = "big")]
    {
        let p0: &mut u32x4 = unsafe { &mut *(&mut nums[0] as *mut u32 as *mut u32x4) };
        let p1: &mut u32x4 = unsafe { &mut *(&mut nums[4] as *mut u32 as *mut u32x4) };

        *p0 = max;
        *p1 = min;
    }

}


#[inline]
pub fn sort_u32x16(nums: &mut u32x16) {
    let a: u32x8 = u32x8::from_slice(&nums[0..8]);
    let b: u32x8 = u32x8::from_slice(&nums[8..16]);

    // sort 1v1 * 8 group
    let min = a.simd_min(b);    // [ 8, 7, 6, 5, 4, 3, 2, 1]
    let max = a.simd_max(b);    // [16,15,14,13,12,11,10, 9]

    let a = simd_swizzle!(min, max, [0, 8, 1, 9, 2, 10, 3, 11]);    // [8, 16, 7, 15, 6, 14, 5, 13],
    let b = simd_swizzle!(min, max, [4, 12, 5, 13, 6, 14, 7, 15]);  // [4, 12, 3, 11, 2, 10, 1, 9]
    // end 1v1 * 8 group, now a, b is 2v2 * 4 group

    let min: u32x8 = a.simd_min(b);    // [4, 12, 3, 11, 2, 10, 1, 9]
    let max: u32x8 = a.simd_max(b);    // [8, 16, 7, 15, 6, 14, 5, 13],

    let tmp = simd_swizzle!(min, [1, 0, 3, 2, 5, 4, 7, 6]);   // [12, 4, 11, 3, 19, 2, 9, 1]
    let min = tmp.simd_min(max);  //    [8, 4, 7, 3, 6, 2, 5, 1]
    let max = tmp.simd_max(max);  //     [12, 16, 11, 15, 10, 14, 9, 13]

    // let min = min.rotate_elements_left::<1>();  // [4, 7, 3, 6, 2, 5, 1, 8]
    let min = simd_swizzle!(min, [1, 0, 3, 2, 5, 4, 7, 6]); // [4, 8, 3, 7, 2, 6, 1, 5]

    let a = simd_swizzle!(min, max, [0, 1, 8, 9, 2, 3, 10, 11]);   // [4, 7, 12, 16, 3, 6, 11, 15]
    let b = simd_swizzle!(min, max, [4, 5, 12, 13, 6, 7, 14, 15]); //  [2, 5, 10, 14, 1, 8, 9, 13]
    // end 2v2 * 4 grpup, now a:b is 4v4 * 2 group

    let min = a.simd_min(b);
    let max = a.simd_max(b);

    // round 1
    let tmp = simd_swizzle!(min, [1,2,3,0,5,6,7,4]);
    let min = tmp.simd_min(max);
    let max = tmp.simd_max(max);

    // round 2
    let tmp = simd_swizzle!(min, [1,2,3,0,5,6,7,4]);
    let min = tmp.simd_min(max);
    let max = tmp.simd_max(max);

    // round 3
    let tmp = simd_swizzle!(min, [1,2,3,0,5,6,7,4]);
    let min = tmp.simd_min(max);
    let max = tmp.simd_max(max);

    // round 4
    let min = simd_swizzle!(min, [1,2,3,0,5,6,7,4]);

    let a = simd_swizzle!(min, max, [0,1,2,3, 8,9,10,11]);
    let b = simd_swizzle!(min, max, [4,5,6,7, 12,13,14,15]);
    // now a:b is 8v8 * 1 group

    let min = a.simd_min(b);
    let max = a.simd_max(b);

    // round 1
    let tmp = min.rotate_elements_left::<1>();
    let min = tmp.simd_min(max);    // [2,4,6,1]
    let max = tmp.simd_max(max);    // [3,5,7,8]

    // round 2
    let tmp = min.rotate_elements_left::<1>();
    let min = tmp.simd_min(max);    // [3,5,1,2]
    let max = tmp.simd_max(max);    // [4,6,7,8]

    // round 3
    let tmp = min.rotate_elements_left::<1>();
    let min = tmp.simd_min(max);    // [4,1,2,3]
    let max = tmp.simd_max(max);    // [5,6,7,8]

    // round 4
    let tmp = min.rotate_elements_left::<1>();
    let min = tmp.simd_min(max);    // [4,1,2,3]
    let max = tmp.simd_max(max);    // [5,6,7,8]

    // round 5
    let tmp = min.rotate_elements_left::<1>();
    let min = tmp.simd_min(max);    // [4,1,2,3]
    let max = tmp.simd_max(max);    // [5,6,7,8]

    // round 6
    let tmp = min.rotate_elements_left::<1>();
    let min = tmp.simd_min(max);    // [4,1,2,3]
    let max = tmp.simd_max(max);    // [5,6,7,8]

    // round 7
    let tmp = min.rotate_elements_left::<1>();
    let min = tmp.simd_min(max);    // [4,1,2,3]
    let max = tmp.simd_max(max);    // [5,6,7,8]

    // round 8
    let min = min.rotate_elements_left::<1>();

    #[cfg(target_endian = "little")]
    {
        let p0: &mut u32x8 = unsafe { &mut *(&mut nums[0] as *mut u32 as *mut u32x8) };
        let p1: &mut u32x8 = unsafe { &mut *(&mut nums[8] as *mut u32 as *mut u32x8) };

        *p0 = min;
        *p1 = max;
    }

    #[cfg(target_endian = "big")]
    {
        let p0: &mut u32x8 = unsafe { &mut *(&mut nums[0] as *mut u32 as *mut u32x8) };
        let p1: &mut u32x8 = unsafe { &mut *(&mut nums[8] as *mut u32 as *mut u32x8) };

        *p0 = max;
        *p1 = min;
    }
}


#[inline]
pub fn sort_u32x32(nums: &mut u32x32) {
    let a: u32x16 = u32x16::from_slice(&nums[0..16]);
    let b: u32x16 = u32x16::from_slice(&nums[16..32]);

    // sort 1v1 * 16 group
    let min = a.simd_min(b);    // [ 8, 7, 6, 5, 4, 3, 2, 1]
    let max = a.simd_max(b);    // [16,15,14,13,12,11,10, 9]

    let a = simd_swizzle!(min, max, [0, 0+16, 1, 1+16, 2, 2+16,   3, 3+16, 4, 4+16,     5, 5+16,   6, 6+16,   7, 7+16]);    // [8, 16, 7, 15, 6, 14, 5, 13],
    let b = simd_swizzle!(min, max, [8, 8+16, 9, 9+16, 10, 10+16, 11, 11+16, 12, 12+16, 13, 13+16, 14, 14+16, 15, 15+16]);  // [4, 12, 3, 11, 2, 10, 1, 9]
    // end 1v1 * 16 group, now a, b is 2v2 * 8 group

    let min  = a.simd_min(b);    // [4, 12, 3, 11, 2, 10, 1, 9]
    let max = a.simd_max(b);    // [8, 16, 7, 15, 6, 14, 5, 13],

    let tmp = simd_swizzle!(min, [1, 0, 3, 2, 5, 4, 7, 6, 9, 8, 11,10, 13, 12, 15, 14]);   // [12, 4, 11, 3, 19, 2, 9, 1]
    let min = tmp.simd_min(max);  //    [8, 4, 7, 3, 6, 2, 5, 1]
    let max = tmp.simd_max(max);  //     [12, 16, 11, 15, 10, 14, 9, 13]

    let tmp = simd_swizzle!(min, [1, 0, 3, 2, 5, 4, 7, 6, 9, 8, 11,10, 13, 12, 15, 14]);   // [12, 4, 11, 3, 19, 2, 9, 1]

    let a = simd_swizzle!(tmp, max, [0, 1, 0+16, 1+16, 2, 3, 2+16, 3+16, 4, 5, 4+16, 5+16, 6, 7, 6+16, 7+16]); // [4, 8, 3, 7, 2, 6, 1, 5]
    let b = simd_swizzle!(tmp, max, [8, 9, 8+16, 9+16, 10, 11, 10+16, 11+16, 12, 13, 12+16, 13+16, 14, 15, 14+16, 15+16]); // [4, 8, 3, 7, 2, 6, 1, 5]
    // end 2v2 * 8 grpup, now a:b is 4v4 * 4 group

    let min = a.simd_min(b);
    let max = a.simd_max(b);

    // round 1
    let tmp = simd_swizzle!(min, [1,2,3,0,  5,6,7,4, 9,10,11,8, 13,14,15,12]);
    let min = tmp.simd_min(max);
    let max = tmp.simd_max(max);

    // round 2
    let tmp = simd_swizzle!(min, [1,2,3,0,  5,6,7,4, 9,10,11,8, 13,14,15,12]);
    let min = tmp.simd_min(max);
    let max = tmp.simd_max(max);

    // round 3
    let tmp = simd_swizzle!(min, [1,2,3,0,  5,6,7,4, 9,10,11,8, 13,14,15,12]);
    let min = tmp.simd_min(max);
    let max = tmp.simd_max(max);

    // round 4
    let min = simd_swizzle!(min, [1,2,3,0,  5,6,7,4, 9,10,11,8, 13,14,15,12]);

    let a = simd_swizzle!(min, max, [0,1,2,3,  0+16, 1 +16, 2+16, 3+16,  4,5,6,7, 4+16, 5+16,6+16, 7+16]);
    let b = simd_swizzle!(min, max, [8,9,10,11, 8+16, 9+16, 10+16, 11+16, 12,13,14,15, 12+16, 13+16, 14+16, 15+16]);
    // now a:b is 8v8 * 2 group

    let min = a.simd_min(b);
    let max = a.simd_max(b);

    // round 1
    let tmp = simd_swizzle!(min, [1,2,3,4,5,6,7,0, 9,10,11,12,13,14,15,8]);
    let min = tmp.simd_min(max);    // [2,4,6,1]
    let max = tmp.simd_max(max);    // [3,5,7,8]

    // round 2
    let tmp = simd_swizzle!(min, [1,2,3,4,5,6,7,0, 9,10,11,12,13,14,15,8]);
    let min = tmp.simd_min(max);    // [3,5,1,2]
    let max = tmp.simd_max(max);    // [4,6,7,8]

    // round 3
    let tmp = simd_swizzle!(min, [1,2,3,4,5,6,7,0, 9,10,11,12,13,14,15,8]);
    let min = tmp.simd_min(max);    // [4,1,2,3]
    let max = tmp.simd_max(max);    // [5,6,7,8]

    // round 4
    let tmp = simd_swizzle!(min, [1,2,3,4,5,6,7,0, 9,10,11,12,13,14,15,8]);
    let min = tmp.simd_min(max);    // [4,1,2,3]
    let max = tmp.simd_max(max);    // [5,6,7,8]

    // round 5
    let tmp = simd_swizzle!(min, [1,2,3,4,5,6,7,0, 9,10,11,12,13,14,15,8]);
    let min = tmp.simd_min(max);    // [4,1,2,3]
    let max = tmp.simd_max(max);    // [5,6,7,8]

    // round 6
    let tmp = simd_swizzle!(min, [1,2,3,4,5,6,7,0, 9,10,11,12,13,14,15,8]);
    let min = tmp.simd_min(max);    // [4,1,2,3]
    let max = tmp.simd_max(max);    // [5,6,7,8]

    // round 7
    let tmp = simd_swizzle!(min, [1,2,3,4,5,6,7,0, 9,10,11,12,13,14,15,8]);
    let min = tmp.simd_min(max);    // [4,1,2,3]
    let max = tmp.simd_max(max);    // [5,6,7,8]

    // round 8
    let min = simd_swizzle!(min, [1,2,3,4,5,6,7,0, 9,10,11,12,13,14,15,8]);

    let mut a = simd_swizzle!(min, max, [0,1,2,3,4,5,6,7, 0+16, 1+16, 2+16, 3+16, 4+16, 5+16, 6+16, 7+16]);
    let mut b = simd_swizzle!(min, max, [8,9,10, 11,12,13,14,15, 8+16, 9+16, 10+16, 11+16, 12+16, 13+16, 14+16, 15+16]);

    merge_sort_u32x16x2(&mut a, &mut b);

    #[cfg(target_endian = "little")]
    {
        let p0: &mut u32x16 = unsafe { &mut *(&mut nums[0] as *mut u32 as *mut u32x16) };
        let p1: &mut u32x16 = unsafe { &mut *(&mut nums[16] as *mut u32 as *mut u32x16) };

        *p0 = a;
        *p1 = b;
    }

    #[cfg(target_endian = "big")]
    {
        let p0: &mut u32x16 = unsafe { &mut *(&mut nums[0] as *mut u32 as *mut u32x16) };
        let p1: &mut u32x16 = unsafe { &mut *(&mut nums[16] as *mut u32 as *mut u32x16) };

        *p0 = b;
        *p1 = a;
    }
}


#[inline]
pub fn merge_sort_u32x16x2(p1: &mut u32x16, p2: &mut u32x16) {
    let min= p1.simd_min(*p2);
    let max = p1.simd_max(*p2);

    // round 1
    let tmp = min.rotate_elements_left::<1>();
    let min = tmp.simd_min(max);
    let max = tmp.simd_max(max);

    // round 2
    let tmp = min.rotate_elements_left::<1>();
    let min = tmp.simd_min(max);
    let max = tmp.simd_max(max);

    // round 3
    let tmp = min.rotate_elements_left::<1>();
    let min = tmp.simd_min(max);
    let max = tmp.simd_max(max);

    // round 4
    let tmp = min.rotate_elements_left::<1>();
    let min = tmp.simd_min(max);
    let max = tmp.simd_max(max);

    // round 5
    let tmp = min.rotate_elements_left::<1>();
    let min = tmp.simd_min(max);
    let max = tmp.simd_max(max);

    // round 6
    let tmp = min.rotate_elements_left::<1>();
    let min = tmp.simd_min(max);
    let max = tmp.simd_max(max);

    // round 7
    let tmp = min.rotate_elements_left::<1>();
    let min = tmp.simd_min(max);
    let max = tmp.simd_max(max);

    // round 8
    let tmp = min.rotate_elements_left::<1>();
    let min = tmp.simd_min(max);
    let max = tmp.simd_max(max);

    // round 9
    let tmp = min.rotate_elements_left::<1>();
    let min = tmp.simd_min(max);
    let max = tmp.simd_max(max);

    // round 10
    let tmp = min.rotate_elements_left::<1>();
    let min = tmp.simd_min(max);
    let max = tmp.simd_max(max);

    // round 11
    let tmp = min.rotate_elements_left::<1>();
    let min = tmp.simd_min(max);
    let max = tmp.simd_max(max);

    // round 12
    let tmp = min.rotate_elements_left::<1>();
    let min = tmp.simd_min(max);
    let max = tmp.simd_max(max);


    // round 13
    let tmp = min.rotate_elements_left::<1>();
    let min = tmp.simd_min(max);
    let max = tmp.simd_max(max);

    // round 14
    let tmp = min.rotate_elements_left::<1>();
    let min = tmp.simd_min(max);
    let max = tmp.simd_max(max);

    // round 15
    let tmp = min.rotate_elements_left::<1>();
    let min = tmp.simd_min(max);
    let max = tmp.simd_max(max);

    // round 16
    let min = min.rotate_elements_left::<1>();
    *p1 = min;
    *p2 = max;
}

fn merge_sort(p1: &mut [u32], p2: &mut [u32], temp: &mut [u32]) {

    let (mut i, mut j) = (0usize, 0usize);
    // round 1
    let copy_size = p1.len().min(temp.len());
    temp[0..copy_size].copy_from_slice(&p1[0..copy_size]);
    let mut remain = merge_sort_round1(&temp[0..copy_size], &mut i, &p2[0..], &mut j, &mut p1[0..], copy_size);

    let len2 = p1.len() - copy_size ;
    let p1_1: &[u32];   // p1 second part, copy to temp
    let p1_2: &[u32];   // p1 third part, copy to p2's first area
    if len2 <= i {
        temp[0..len2].copy_from_slice(&p1[copy_size..]);
        p1_1 = &temp[0..len2];
        p1_2 = &[];
    }
    else {
        temp[0..i].copy_from_slice(&p1[copy_size..copy_size+i]);
        p2[0..(len2-i)].copy_from_slice(&p1[copy_size+i..]);
        p1_1 = &temp[0..i];
        p1_2 = unsafe { &* (&p2[0..(len2-i)] as *const [u32]) };    // here p2 is immutable borrow which conflict with &mut p2[0..] but it is safe here
    }
    let (mut idx1_0, mut idx1_1, mut idx1_2, mut idx2) = (0usize, 0usize, 0usize, 0usize);
    let p1_len = p1.len();
    let p2_len = p2.len();
    if p1_len > copy_size {
        remain = merge_sort_round2(remain, &temp[i..copy_size], &mut idx1_0, &p1_1[0..], &mut idx1_1, &p1_2[0..], &mut idx1_2,
                          &p2[j..], &mut idx2,
                          &mut p1[copy_size..], p1_len - copy_size);
    }

    remain = merge_sort_round2(remain, &temp[i..copy_size], &mut idx1_0, &p1_1[0..], &mut idx1_1, &p1_2[0..], &mut idx1_2,
                      unsafe { &*( &p2[j..] as *const [u32] ) }, &mut idx2,   // here p2 is immutable borrow which conflict with &mut p2[0..] but it is safe here
                      &mut p2[0..], p2_len);
    p2[p2_len - 16..].copy_from_slice(remain.as_array());

}

fn merge_sort_round1(
    p1: &[u32], i: &mut usize,
    p2: &[u32], j: &mut usize,
    out: &mut [u32], count: usize) -> u32x16
{
    debug_assert!(count > 0);
    debug_assert!(p1.len() > 0 && p2.len() > 0);
    debug_assert!(p1.len() % 16 == 0 && p2.len() % 16 == 0);

    let mut o = 0usize;
    let mut min = u32x16::from_slice(&p1[*i..]);
    let mut max = u32x16::from_slice(&p2[*j..]);
    *i += 16;
    *j += 16;

    loop {
        merge_sort_u32x16x2(&mut min, &mut max);
        out[o..o+16].copy_from_slice( min.as_array() );
        o += 16;

        if o < count  { // load another
            if *i < p1.len() && *j < p2.len()  {
                if p1[*i] < p2[*j] {
                    min = u32x16::from_slice(&p1[*i..]);
                    *i += 16;
                } else {
                    min = u32x16::from_slice(&p2[*j..]);
                    *j += 16;
                }
            }
            else if *i < p1.len() {
                min = u32x16::from_slice(&p1[*i..]);
                *i += 16;
            }
            else if *j < p2.len() {
                min = u32x16::from_slice(&p2[*j..]);
                *j += 16;
            }
            else {
                panic!("impossible");
            }
        }
        else {
            break max;
        }
    }

}

// TODO: 重构这个函数，消除重复代码

fn merge_sort_round2(
    last: u32x16,
    p1_0: &[u32], idx1_0: &mut usize,
    p1_1: &[u32], idx1_1: &mut usize,
    p1_2: &[u32], idx1_2: &mut usize,
    p2: &[u32], idx2: &mut usize,
    out: &mut [u32], count: usize) -> u32x16
{

    let mut max = last;
    let mut min;
    let mut o = 0;

    if *idx1_0 < p1_0.len() {
       loop {
           if *idx2 < p2.len() {
               if p1_0[*idx1_0] < p2[*idx2] {
                    min = u32x16::from_slice(&p1_0[*idx1_0..]);
                   *idx1_0 += 16;
               }
               else {
                   min = u32x16::from_slice(&p2[*idx2..]);
                   *idx2 += 16;
               }
           }
           else  {
                min = u32x16::from_slice(&p1_0[*idx1_0..]);
                *idx1_0 += 16;
           }

           merge_sort_u32x16x2(&mut min, &mut max);
           out[o..o+16].copy_from_slice( min.as_array() );
           o += 16;
           if o >= count {
               return max;
           }
           if *idx1_0 >= p1_0.len() {
               break;
           }
       }
    }
    if *idx1_1 < p1_1.len() {
        loop {
            if *idx2 < p2.len() {
                if p1_1[*idx1_1] < p2[*idx2] {
                    min = u32x16::from_slice(&p1_1[*idx1_1..]);
                    *idx1_1 += 16;
                }
                else {
                    min = u32x16::from_slice(&p2[*idx2..]);
                    *idx2 += 16;
                }
            }
            else  {
                min = u32x16::from_slice(&p1_1[*idx1_1..]);
                *idx1_1 += 16;
            }

            merge_sort_u32x16x2(&mut min, &mut max);
            out[o..o+16].copy_from_slice( min.as_array() );
            o += 16;
            if o >= count {
                return max;
            }
            if *idx1_1 >= p1_1.len() {
                break;
            }
        }
    }
    if *idx1_2 < p1_2.len() {
        loop {
            if *idx2 < p2.len() {
                if p1_2[*idx1_2] < p2[*idx2] {
                    min = u32x16::from_slice(&p1_1[*idx1_2..]);
                    *idx1_2 += 16;
                }
                else {
                    min = u32x16::from_slice(&p2[*idx2..]);
                    *idx2 += 16;
                }
            }
            else  {
                min = u32x16::from_slice(&p1_0[*idx1_2..]);
                *idx1_2 += 16;
            }

            merge_sort_u32x16x2(&mut min, &mut max);
            out[o..o+16].copy_from_slice( min.as_array() );
            o += 16;
            if o >= count {
                return max;
            }
            if *idx1_2 >= p1_2.len() {
                break;
            }
        }
    }

    loop {  // only left p2
        if *idx2 < p2.len() {
            min = u32x16::from_slice(&p2[*idx2..]);
            *idx2 += 16;
        } else {
            break;
        }

        merge_sort_u32x16x2(&mut min, &mut max);
        out[o..o + 16].copy_from_slice(min.as_array());
        o += 16;
    }

    max
}


/*
#[test]
fn test_merge_sort1(){

    // let mut p1: Vec<u32> = (100..132).collect();
    // let mut p2: Vec<u32> = (0..48).collect();

    // let mut p1: Vec<u32> = (100..116).collect();
    // let mut p2: Vec<u32> = (0..48).collect();

    // let mut p1: Vec<u32> = (0..48).collect();
    // let mut p2: Vec<u32> = (100..132).collect();

    // let mut p1: Vec<u32> = (0..48).collect();
    // let mut p2: Vec<u32> = (100..116).collect();

    // let mut p1 = vec! [30, 41, 101, 103, 118, 134, 136, 143, 152, 164, 173, 209, 218, 222, 228, 237, 265, 282, 291, 304, 309, 350, 388, 394, 398, 427, 462, 480, 481, 543, 605, 614, 622, 634, 652, 654, 673, 676, 677, 684, 758, 760, 799, 803, 818, 832, 833, 841, 851, 857, 865, 870, 871, 872, 883, 888, 891, 902, 908, 927, 930, 960, 964, 974];
    // let mut p2 = vec! [11, 12, 24, 117, 118, 146, 155, 202, 248, 254, 371, 451, 470, 484, 529, 563, 566, 599, 605, 607, 618, 678, 688, 700, 704, 737, 753, 763, 773, 825, 965, 993];

    let mut p1 = vec! [13, 113, 114, 155, 341, 387, 481, 510, 581, 661, 742, 764, 772, 817, 860, 900];
    let mut p2 = vec! [11, 27, 30, 50, 68, 110, 115, 123, 186, 248, 296, 302, 302, 305, 347, 348, 356, 380, 398, 441, 454, 475, 589, 599, 645, 646, 669, 701, 741, 870, 946, 947];



    let mut all = Vec::with_capacity(p1.len() + p2.len() );
    all.extend_from_slice(&p1);
    all.extend_from_slice(&p2);

    p1.sort();
    p2.sort();
    all.sort();
    let temp_size = ((p1.len() / 16 + p2.len() / 16) + 1) / 2 * 16;
    let mut temp = vec![0u32; temp_size];

    merge_sort(&mut p1, &mut p2, &mut temp);

    println!("p1: {:?}", p1);
    println!("p2: {:?}", p2);

    assert_eq!(p1, &all[0..p1.len()]);
    assert_eq!(p2, &all[p1.len()..]);

}
*/


#[test]
fn test_merge_sort(){

    for _ in 0..1024*16 {
        let len1 = (random::<u32>() % 256 + 1) as usize * 16;
        let len2= (random::<u32>() % 256  + 1)  as usize * 16;

        let p1: Vec<u32>= {
            let mut v: Vec<u32>= (0..len1).map(|_| random::<u32>() % 1_000).collect();
            v.sort();
            v
        };
        let p2: Vec<u32>= {
            let mut v: Vec<u32> = (0..len2).map( |_| random::<u32>() % 1_000 ).collect();
            v.sort();
            v
        };
        let expected: Vec<u32> = {
            let mut v: Vec<u32> = Vec::with_capacity((len1 + len2) as usize);
            v.extend_from_slice(&p1);
            v.extend_from_slice(&p2);
            v.sort();
            v
        };

        let mut p1_copy = p1.clone();
        let mut p2_copy = p2.clone();
        p1_copy.sort();
        p2_copy.sort();
        let temp_len = (len1/16 + len2/16 +1) / 2 * 16;
        let mut temp = vec![0u32; temp_len];

        merge_sort(&mut p1_copy, &mut p2_copy, &mut temp);

        if p1_copy != expected[0..len1] || p2_copy != expected[len1 .. ]{
            println!("p1: {:?}", p1);
            println!("p2: {:?}", p2);
            println!()
        }

        assert_eq!(&p1_copy, &expected[0 .. len1]);
        assert_eq!(&p2_copy, &expected[len1 .. ]);
    }

}