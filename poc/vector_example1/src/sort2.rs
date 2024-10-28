use std::simd::cmp::SimdOrd;
use std::simd::{simd_swizzle, u32x4, u32x8, ToBytes};


#[test]
fn test_sort_u32x8() {

    let v1 = [8u32,7,6,5,4,3,2,1];
    let mut nums = u32x8::from_slice(&v1);
    sort_u32x8(&mut nums);

    println!("nums = {:?}", nums)

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

