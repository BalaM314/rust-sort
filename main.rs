
pub fn bubble_sort_rr(items:&mut Vec<i32>){
	for i in (1..items.len()).rev() {
		for j in 0..i {
			if items[j+1] < items[j] {
				items.swap(j, j + 1);
			}
		}
	}
}

pub fn bubble_sort_rr_flag(items:&mut Vec<i32>){
	for i in (1..items.len()).rev() {
		let mut swapped = false;
		for j in 0..i {
			if items[j+1] < items[j] {
				items.swap(j, j + 1);
				swapped = true;
			}
		}
		if !swapped { break }
	}

}



fn main(){
	
}

#[cfg(test)]
mod tests {
	use crate::*;
	use rand::{thread_rng, Rng};
	
	
	/// range is exclusive
	fn get_random_vec(size:i32, min:i32, max:i32) -> Vec<i32> {
		(0..size).map(|_| thread_rng().gen_range(min..max)).collect()
	}
	
	fn test<F:Fn(&mut Vec<i32>) -> ()>(func:&F) {
		let mut data = get_random_vec(10, 0, 1000);
		let mut rust_sorted_data = data.clone();
		rust_sorted_data.sort();
		func(&mut data);
		assert_eq!(data, rust_sorted_data);
	}
	#[test]
	fn tests(){
		test(&bubble_sort_rr);
		test(&bubble_sort_rr_flag);
	}
}
