// Uses
use breakable_block::breakable;

// Tests
#[test]
#[allow(unreachable_code)]
fn can_break() {
	let this_should_never_be_set = true;
	breakable!('block: {
		break 'block;
		this_should_never_be_set = false;
	});
	assert!(this_should_never_be_set);
}
