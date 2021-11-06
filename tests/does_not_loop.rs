// Uses
use breakable_block::breakable;

// Tests
#[test]
fn does_not_loop_without_break() {
	let mut iterations = 0;
	breakable!('_block: {
		iterations += 1;
	});
	assert_eq!(iterations, 1);
}

#[test]
#[allow(unreachable_code)]
fn does_not_loop_with_break() {
	let mut iterations = 0;
	breakable!('block: {
		iterations += 1;
		break 'block;
	});
	assert_eq!(iterations, 1);
}
