use crate::Frame;

pub fn create_frames(from: usize, to: usize, duration: f32) -> Vec<Frame> {
    let mut frames = Vec::new();

    for i in from..=to {
        frames.push(Frame { index: i, duration });
    }

    frames
}
