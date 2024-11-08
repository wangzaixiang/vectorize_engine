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

/// reqyire: temp.len() >= p1.len() / 2
#[inline]
pub fn merge_sort(p1: &mut [u32], p2: &mut [u32], temp: &mut [u32]) {

    let (mut i, mut j) = (0usize, 0usize);
    let copy_size = p1.len().min(temp.len());

    // round 1, merge p1[0..copy_size] and p2[0..] and output to p1, to avoid overlap, copy p1[0..copy_size] to temp first
    temp[0..copy_size].copy_from_slice(&p1[0..copy_size]);
    let mut remain = unsafe {merge_sort_round1(&temp[0..copy_size], &mut i, &p2[0..], &mut j, &mut p1[0..], copy_size) };

    // end round 1, may remains temp[i..copy_size] and p2[j..]
    // then (temp[i..copy_size],  temp[0..i] = p1[copy_size..copy_size+i], p2[0..?] = p1[copy_size+i..]) and  p2[j..])
    // and output to p1[copy_size..] and p2[0..]

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

    // round 2
    let (mut idx1_0, mut idx1_1, mut idx1_2, mut idx2) = (0usize, 0usize, 0usize, 0usize);
    let p1_len = p1.len();
    let p2_len = p2.len();
    if p1_len > copy_size {
        remain = unsafe { merge_sort_round2(remain, &temp[i..copy_size], &mut idx1_0, &p1_1[0..], &mut idx1_1, &p1_2[0..], &mut idx1_2,
                          &p2[j..], &mut idx2,
                          &mut p1[copy_size..], p1_len - copy_size) };
    }


    // round 3
    remain = unsafe { merge_sort_round2(remain, &temp[i..copy_size], &mut idx1_0, &p1_1[0..], &mut idx1_1, &p1_2[0..], &mut idx1_2,
                      &*( &p2[j..] as *const [u32] ), &mut idx2,   // here p2 is immutable borrow which conflict with &mut p2[0..] but it is safe here
                      &mut p2[0..], p2_len) };

    debug_assert_eq!(idx1_0, copy_size - i);
    debug_assert_eq!(idx1_1, p1_1.len());
    debug_assert_eq!(idx1_2, p1_2.len());
    debug_assert_eq!(idx2, p2_len - j);

    p2[p2_len - 16..].copy_from_slice(remain.as_array());

}

#[inline]
fn write(out: &mut [u32], o: &mut usize, value: &u32x16) {
    out[*o..*o+16].copy_from_slice( value.as_array() );
    *o += 16;
}

#[inline]
fn read(buf: &[u32], i: &mut usize) -> u32x16  {
    let value = u32x16::from_slice(&buf[*i..]);
    *i += 16;
    value
}

/// merge sort from p1(sorted), p2(sorted) for count elements into out
/// require: out\[0..count] will not overlap with p1, p2
unsafe fn merge_sort_round1(
    p1: &[u32], i: &mut usize,
    p2: &[u32], j: &mut usize,
    out: &mut [u32], count: usize) -> u32x16
{
    debug_assert!(count > 0);
    debug_assert!(p1.len() > 0 && p2.len() > 0);
    debug_assert!(p1.len() % 16 == 0 && p2.len() % 16 == 0);

    let mut o = 0usize;
    let mut min = read(p1, i); // u32x16::from_slice(&p1[*i..]);
    let mut max = read(p2, j); //u32x16::from_slice(&p2[*j..]);

    loop {
        merge_sort_u32x16x2(&mut min, &mut max);
        write(out, &mut o, &min);

        if o < count  { // load another
            if *i < p1.len() && *j < p2.len()  {
                if p1[*i] < p2[*j] {
                    min = read(p1, i);
                } else {
                    min  = read(p2, j);
                }
            }
            else if *i < p1.len() {
                min = read(p1, i);
            }
            else {
                debug_assert!( *j < p2.len() );
                min = read(p2, j);
            }
        }
        else {
            break max;
        }
    }

}

/// p1: p1_0 :: p1_1 :: p1_2
unsafe fn merge_sort_round2(
    last: u32x16,
    p1_0: &[u32], idx1_0: &mut usize,
    p1_1: &[u32], idx1_1: &mut usize,
    p1_2: &[u32], idx1_2: &mut usize,
    p2: &[u32], idx2: &mut usize,
    out: &mut [u32], count: usize) -> u32x16
{

    let mut max = last;
    let mut o = 0;

    if *idx1_0 < p1_0.len() {  // merge p1_0 and p2
        max = merge2_when_part1_non_empty(p1_0, idx1_0, p2, idx2, out, &mut o, count, &mut max);
        if o >= count {
            return max;
        }
    }

    if *idx1_1 < p1_1.len() {   // merge p1_1 and p2
        max = merge2_when_part1_non_empty(p1_1, idx1_1, p2, idx2, out, &mut o, count, &mut max);
        if o >= count {
            return max;
        }
    }

    if *idx1_2 < p1_2.len() {  // merge p1_2 and p2
        max = merge2_when_part1_non_empty(p1_2, idx1_2, p2, idx2, out, &mut o, count, &mut max);
        if o >= count {
            return max;
        }
    }

    if *idx2 < p2.len() {  // p2 still non empy
        max = merge2_when_part1_non_empty(p2, idx2, &[], &mut 0, out, &mut o, count, &mut max);
        if o >= count {
            return max;
        }
    }

    max
}

/// require idx1 < p1.len()
unsafe fn merge2_when_part1_non_empty(p1: &[u32], idx1: &mut usize, p2: &[u32], idx2: &mut usize,
                                      out: &mut [u32], mut o: &mut usize, count: usize,
                                      remain: &u32x16) -> u32x16 {
    let mut max = *remain;
    loop {
        let mut min;
        if *idx2 < p2.len() {   // use p1 & p2
            if p1[*idx1] < p2[*idx2] {
                min = read(p1, idx1);
            } else {
                min = read(p2, idx2);
            }
        } else {    // no p2
            min = read(p1, idx1);
        }

        merge_sort_u32x16x2(&mut min, &mut max);
        write(out, &mut o, &min);
        if *o >= count {
            break max;
        }
        if *idx1 >= p1.len() {  // part1 complete, try the next by caller.
            break max;
        }
    }
}


#[test]
fn test_merge_sort(){

    for _ in 0..1024*16 {
        let len1 = (random::<u32>() % 512 + 1) as usize * 16;
        let len2= (random::<u32>() % 512 + 1)  as usize * 16;

        let p1: Vec<u32>= {
            let mut v: Vec<u32>= (0..len1).map(|_| random::<u32>() % 1_000_000).collect();
            v.sort();
            v
        };
        let p2: Vec<u32>= {
            let mut v: Vec<u32> = (0..len2).map( |_| random::<u32>() % 1_000_000 ).collect();
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