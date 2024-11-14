use std::simd::cmp::{SimdOrd};
use std::simd::{simd_swizzle, u32x16, u32x32, u32x4, u32x8 };

#[test]
fn test_sort_u32x8() {

    let v1 = [8u32,7,6,5,4,3,2,1];
    let mut nums = u32x8::from_slice(&v1);
    sort_u32x8(&mut nums);

    println!("nums = {:?}", nums)

}

#[test]
fn test_sort_u32x16() {
    use rand::random;

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
        let mut nums: Vec<u32> = (0..32).map( |_| rand::random::<u32>() % 1000 ).collect();
        let mut simd = u32x32::from_slice(&nums);
        sort_u32x32(&mut simd);

        nums.sort();
        assert_eq!(simd.as_array() as &[u32], &nums);
    }

}


#[test]
fn test_merge_sort_u32x16x2(){
    for _ in 0..1024 {
        let mut vec1: Vec<u32> = (0..16).map(|_| rand::random::<u32>() % 100).collect();
        let mut vec2: Vec<u32> = (0..16).map(|_| rand::random::<u32>() % 100).collect();
        vec1.sort();
        vec2.sort();

        let mut expected = Vec::new();
        expected.extend_from_slice(&vec1);
        expected.extend_from_slice(&vec2);
        expected.sort();

        let mut num1 = u32x16::from_slice(&vec1);
        let mut num2 = u32x16::from_slice(&vec2);
        merge_sort_u32x16x2(&mut num1, &mut num2);

        if num1.as_array() == &expected[..16] && num2.as_array() == &expected[16..] {

        }
        else {
            println!("problem: vec1 = {:?}, vec2 = {:?}", vec1, vec2);
        }

        assert_eq!(num1.as_array() as &[u32], &expected[..16]);
        assert_eq!(num2.as_array() as &[u32], &expected[16..]);
    }
}

#[test]
fn test_random(){

    let mut v1: Vec<u32> = (0..8).map (|_| rand::random::<u32>() % 100).collect();
    let mut v2: Vec<u32> = (0..8).map (|_| rand::random::<u32>() % 100).collect();
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


#[inline(always)]
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


#[inline(always)]
pub fn merge_sort_u32x16x2(p1: &mut u32x16, p2: &mut u32x16) {
    let min = p1.simd_min(*p2);
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

#[inline(always)]
fn write(out: &mut [u32], o: &mut usize, value: &u32x16) {
    out[*o..*o+16].copy_from_slice( value.as_array() );
    *o += 16;
}

#[inline(always)]
fn read(buf: &[u32], i: &mut usize) -> u32x16  {
    let value = u32x16::from_slice(&buf[*i..]);
    *i += 16;
    value
}


pub fn sort(nums: &mut [u32]) {

    debug_assert!(nums.len() % 16 == 0);

    if nums.len() == 16 {
        sort_u32x16(unsafe { &mut *(nums as *mut [u32] as *mut u32x16) });
        return ;
    }
    else if nums.len() == 32 {
        sort_u32x32(unsafe { &mut *(nums as *mut [u32] as *mut u32x32) });
        return ;
    }

    let len1 = (nums.len()/16 + 1) / 2 * 16;   debug_assert!(len1 % 16 == 0);

    let mut temp: Vec<u32> = vec![0; len1]; // nums.len() / 2

    let p1: &mut[u32] = unsafe {  &mut*( &mut nums[..len1] as *mut [u32])  }; // &mut nums[0..len1];
    let p2: &mut[u32] = unsafe {  &mut*( &mut nums[len1..] as *mut [u32])  };


    sort_round1(p1, &mut temp, true);
    {
        let temp = &mut p1[0..p2.len()];
        sort_round1(p2, temp, false);
    }

    merge_sort_2(&temp, p2, nums);
}

#[inline]
fn sort_round1( nums: &mut[u32], temp: &mut[u32], swap: bool) {
    debug_assert!(nums.len() <= temp.len());
    debug_assert!(nums.len() % 16 == 0);

    if nums.len() == 16 {
        debug_assert_eq!(swap, false);
        sort_u32x16(unsafe { &mut *(nums as *mut [u32] as *mut u32x16) });
        return;
    } else if nums.len() == 32 {
        sort_u32x32(unsafe { &mut *(nums as *mut [u32] as *mut u32x32) });
        if swap {
            temp.copy_from_slice(nums);
        }
        return;
    }

    let simds = nums.len() / 16;

    let order = {
        let msb = 63 - simds.leading_zeros() as u32;
        let lsb = simds.trailing_zeros();
        if lsb == msb {
            msb
        } else {
            msb + 1
        }
    };

    // 2^0  2^2
    // 2^1  2^3
    let need_swap_first = if order % 2 == 0 { swap } else { !swap };

    let (mut current, mut next): (&mut [u32], &mut[u32]) = (nums, temp);
    let end = simds / 2 * 2;
    for i in (0..end).step_by(2) {
        let p1: &mut u32x32 = unsafe { &mut *(&mut current[i * 16] as *mut u32 as *mut u32x32) };
        sort_u32x32(p1);
        if !need_swap_first {
            next[i * 16.. i * 16 + 32].copy_from_slice(p1.as_array());
        }
    }
    if end * 16 < current.len() {
        sort_u32x16(unsafe { &mut *(&mut current[end * 16..] as *mut [u32] as *mut u32x16) });
        if !need_swap_first {
            next[end * 16..].copy_from_slice(&current[end * 16..]);
        }
    }
    if !need_swap_first {
        (current, next) = (next, current);
    }

    let mut level = 1;      // now 2^1 is sorted

    while level < order {
        // sort current into next 2^level + 2^level -> 2^(level+1)
        let len1 = 1 << level;
        let len2 = len1 * 2;
        let end = simds >> (level + 1) << (level + 1);
        for i in (0..end).step_by(len2) {
            let off0 = i * 16;
            let off1 = off0 + len1 * 16;
            let off2 = off1 + len1 * 16;
            let p1 = unsafe {  &mut *(&mut current[ off0.. off1] as *mut [u32] ) }; // &mut current[i * len1 * 2 .. i * len1 * 2 + len1];
            let p2 = unsafe {  &mut *(&mut current[ off1.. off2] as *mut [u32] ) }; // &mut current[i * len1 * 2 .. i * len1 * 2 + len1];
            merge_sort_2(p1, p2, &mut next[off0.. off2]);
        }
        // process end part, 0..len1, len1, len1..len1*2
        let remains = &mut current[end * 16..];
        if remains.len() > len1 * 16 {
            let p1 = unsafe {  &mut *(&mut remains[0.. len1*16] as *mut [u32] ) }; // &mut current[i * len1 * 2 .. i * len1 * 2 + len1];
            let p2 = unsafe {  &mut *(&mut remains[len1*16.. ] as *mut [u32] ) }; // &mut current[i * len1 * 2 .. i * len1 * 2 + len1];

            merge_sort_2(p1 ,p2, &mut next[end*16 ..]);
        }
        else {
            next[end*16..].copy_from_slice(remains);
        }
        (current, next) = (next, current);
        level += 1;
    }

}


#[inline(always)]
fn merge_sort_2(p1: &[u32], p2: &[u32], out: &mut[u32]) {
    if p1.len() == 0 {
        debug_assert!(p2.len() > 0);
    }
    debug_assert!(p1.len() > 0 && p1.len() % 16 == 0);
    debug_assert!(p2.len() > 0 && p2.len() % 16 == 0);
    debug_assert_eq!(p1.len() + p2.len(), out.len());

    if p1[p1.len()-1] <= p2[0] {
        out[0..p1.len()].copy_from_slice(p1);
        out[p1.len()..].copy_from_slice(p2);
        return;
    }
    let (mut i, mut j, mut o) = (0usize, 0usize, 0usize);
    let mut min = read(p1, &mut i);
    let mut max = read(p2, &mut j);

    loop {
        merge_sort_u32x16x2(&mut min, &mut max);
        write(out, &mut o, &min);

        if i < p1.len() && j < p2.len() {
            if p1[i] < p2[j] {
                min = read(p1, &mut i);
            } else {
                min = read(p2, &mut j);
            }
        }
        else if i < p1.len() {
            min = read(p1, &mut i);
        }
        else if j < p2.len() {
            min = read(p2, &mut j);
        }
        else {
            break;
        }
    }

    write(out, &mut o, &max);
}

pub fn debug_print(label: &str, v: &[u32]){

    println!("{:10}",  label);
    for i in (0..v.len()).step_by(16) {
        print!("\t");
        for j in i..i+16 {
            print!("{:4} ", v[j]);
        }
        println!();
    }
    println!("\n");
}

#[test]
fn test_drift_sort(){

    let mut numbers: Vec<u32> = (0..100).map(|_| rand::random::<u32>() % 100).collect();

    numbers.sort();

}

#[test]
fn test_sort(){
    use std::hint::black_box;

    for order in 0..15 {
        let (_rust, _simd) = (0u64, 0u64);
        let len: usize = 32 << order;  // 32, 64 .. 2^20

        let vec: Vec<u32> = (0..len).map(|_| rand::random::<u32>()).collect();
        let mut expected = black_box(&vec).clone();
        expected.sort();

        let mut vec = black_box(&vec).clone();
        sort(black_box(&mut vec));
        assert_eq!(vec, expected);
    }
}

