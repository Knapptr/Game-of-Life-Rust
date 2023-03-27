use std::time::Duration;

pub struct Screen {
    fps: Duration,
}

pub trait Render {
    fn do_loop(&self, act: impl FnMut() -> bool);
}

impl Render for Screen {
    fn do_loop(&self, mut act: impl FnMut() -> bool) {
        loop {
            let finished = act();
            if finished {
                break;
            }
            std::thread::sleep(self.fps)
        }
    }
}
impl Screen {
    pub fn new(fps: u64) -> Self {
        let millis = 1000 / fps;
        Self {
            fps: Duration::from_millis(millis),
        }
    }
}
