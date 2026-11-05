#[derive(Debug, Clone)]
pub struct VideoClip { pub id: String, pub start_ms: u64, pub dur_ms: u64 }

#[derive(Debug, Default)]
pub struct FilmProject { clips: Vec<VideoClip> }
impl FilmProject {
    pub fn new() -> Self { Self::default() }
    pub fn add_clip(&mut self, id: impl Into<String>, start: u64, dur: u64) {
        self.clips.push(VideoClip { id: id.into(), start_ms: start, dur_ms: dur });
    }
    pub fn total_dur(&self) -> u64 {
        self.clips.iter().map(|c| c.start_ms + c.dur_ms).max().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_film() {
        let mut p = FilmProject::new();
        p.add_clip("c1", 0, 5000);
        assert_eq!(p.total_dur(), 5000);
    }
}
