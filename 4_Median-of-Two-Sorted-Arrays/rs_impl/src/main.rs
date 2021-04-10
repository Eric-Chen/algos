use std::{thread, time};

fn main() {
    let nums1 = vec![1, 3];
    let nums2 = vec![2];
    println!("{}", find_median_sorted_arrays(nums1, nums2))
}


fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    if nums1.len() > nums2.len() {
        return find_median_sorted_arrays(nums2, nums1)
    }
    let (mut low,mut high,k,mut nums1_mid,mut nums2_mid) = (0, nums1.len(), (nums1.len() + nums2.len() + 1) >> 1, 0, 0);

    while low <= high {
        nums1_mid = low + (high - low)/2;
        nums2_mid = k - nums1_mid;
        if nums1_mid > 0 && *nums1.get(nums1_mid-1).unwrap() > *nums2.get(nums2_mid).unwrap() {
            high = nums1_mid - 1;
        } else if nums1_mid < nums1.len() && *nums1.get(nums1_mid).unwrap() < *nums2.get(nums2_mid-1).unwrap() {
            low = nums1_mid + 1;
        } else {
            break
        }
        let ten_millis = time::Duration::from_millis(500);
        thread::sleep(ten_millis);
    }

    let (mut mid_left,mut mid_right) = (0, 0);
    if nums1_mid == 0 {
        mid_left = *nums2.get(nums2_mid-1).unwrap()
    } else if nums2_mid == 0 {
        mid_left = *nums1.get(nums1_mid-1).unwrap()
    } else {
        mid_left = max(*nums1.get(nums1_mid-1).unwrap(), *nums2.get(nums2_mid-1).unwrap())
    }
    if (nums1.len() + nums2.len())&1 == 1 {
		return mid_left as f64
	}

    if nums1_mid == nums1.len() {
        mid_right = *nums2.get(nums2_mid).unwrap()
    } else if nums2_mid == nums2.len() {
        mid_right = *nums1.get(nums1_mid).unwrap()
    } else {
        mid_right = min(*nums1.get(nums1_mid).unwrap(), *nums2.get(nums2_mid).unwrap())
    }

    return ((mid_left+mid_right)as f64)/2.0f64

}

fn max(a: i32, b: i32) -> i32 {
    if a > b {
        return a;
    }
    return b;
}

fn min(a: i32, b: i32) -> i32 {
    if a < b {
        return a;
    }
    return b;
}