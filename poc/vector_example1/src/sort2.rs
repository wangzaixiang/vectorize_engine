use std::simd::cmp::SimdOrd;
use std::simd::{simd_swizzle, u32x16, u32x32, u32x4, u32x8, ToBytes};
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

    for i in 0..1024 {
        let mut nums: Vec<u32> = (0..16).map( |_| random::<u32>() % 1000 ).collect();
        let mut simd = u32x16::from_slice(&nums);
        sort_u32x16(&mut simd);

        nums.sort();
        assert_eq!(simd.as_array() as &[u32], &nums);
    }

}

#[test]
fn test_sort_u32x32() {

    for i in 0..1024 {
        let mut nums: Vec<u32> = (0..32).map( |_| random::<u32>() % 1000 ).collect();
        let mut simd = u32x32::from_slice(&nums);
        sort_u32x32(&mut simd);

        nums.sort();
        assert_eq!(simd.as_array() as &[u32], &nums);
    }

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

    let a = simd_swizzle!(min, max, [0,1,2,3,4,5,6,7, 0+16, 1+16, 2+16, 3+16, 4+16, 5+16, 6+16, 7+16]);
    let b = simd_swizzle!(min, max, [8,9,10, 11,12,13,14,15, 8+16, 9+16, 10+16, 11+16, 12+16, 13+16, 14+16, 15+16]);

    let min= a.simd_min(b);
    let max = a.simd_max(b);

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

    #[cfg(target_endian = "little")]
    {
        let p0: &mut u32x16 = unsafe { &mut *(&mut nums[0] as *mut u32 as *mut u32x16) };
        let p1: &mut u32x16 = unsafe { &mut *(&mut nums[16] as *mut u32 as *mut u32x16) };

        *p0 = min;
        *p1 = max;
    }

    #[cfg(target_endian = "big")]
    {
        let p0: &mut u32x16 = unsafe { &mut *(&mut nums[0] as *mut u32 as *mut u32x16) };
        let p1: &mut u32x16 = unsafe { &mut *(&mut nums[16] as *mut u32 as *mut u32x16) };

        *p0 = max;
        *p1 = min;
    }
}
//

