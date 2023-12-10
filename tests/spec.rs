use rstest::rstest;
use std::time::Duration;

use franim::Animation;

#[rstest]
fn starts_at_frame_zero() {
    let animation: Animation = Animation::from_frame_duration(3, Duration::from_secs(1));
    assert_eq!(animation.current_frame(), 0);
}

#[rstest]
#[case(Duration::ZERO, 0)]
#[case(Duration::from_millis(999), 0)]
#[case(Duration::from_secs(1), 1)]
#[case(Duration::from_secs(2), 2)]
#[case(Duration::from_secs(3), 0)]
#[case(Duration::from_secs(4), 1)]
fn update_frames_according_to_elapsed_time(
    #[case] elapsed: Duration,
    #[case] expected_frame: usize,
) {
    let mut animation = Animation::from_frame_duration(3, Duration::from_secs(1));
    animation.update(elapsed);
    assert_eq!(animation.current_frame(), expected_frame);
}

#[rstest]
fn remembers_elapsed_time() {
    let mut animation = Animation::from_frame_duration(3, Duration::from_secs(1));
    animation.update(Duration::from_millis(500));
    assert_eq!(animation.current_frame(), 0);
    animation.update(Duration::from_millis(500));
    assert_eq!(animation.current_frame(), 1);
}
