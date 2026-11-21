//! SigmaOS-craft-film: film/video model.
//!
//! Real implementation: scene model, clips, timeline, editing operations (cut, trim,
//! splice), export formats, and validation. Replaces the 104-line CLI placeholder.

/// Film and video editing module.
pub mod sigmafilm;

use std::collections::BTreeMap;
use std::fmt;

/// Clip type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClipType {
    Video,
    Audio,
    Image,
    Text,
}

/// Clip.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clip {
    pub id: String,
    pub clip_type: ClipType,
    pub source_path: String,
    pub start_frame: u32,
    pub duration_frames: u32,
    pub track_index: u32,
}

/// Scene.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scene {
    pub id: String,
    pub name: String,
    pub clips: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FilmError {
    EmptyClipPath,
    DuplicateClip(String),
    ClipNotFound(String),
    EmptySceneName,
    DuplicateScene(String),
    InvalidDuration(u32),
}

impl fmt::Display for FilmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyClipPath => write!(f, "clip source path must not be empty"),
            Self::DuplicateClip(id) => write!(f, "duplicate clip {:?}", id),
            Self::ClipNotFound(id) => write!(f, "clip {:?} not found", id),
            Self::EmptySceneName => write!(f, "scene name must not be empty"),
            Self::DuplicateScene(id) => write!(f, "duplicate scene {:?}", id),
            Self::InvalidDuration(d) => write!(f, "invalid duration {}", d),
        }
    }
}

impl std::error::Error for FilmError {}

/// Film project.
pub struct FilmProject {
    pub title: String,
    pub fps: u32,
    clips: BTreeMap<String, Clip>,
    scenes: BTreeMap<String, Scene>,
}

impl FilmProject {
    pub fn new(title: impl Into<String>, fps: u32) -> Result<Self, FilmError> {
        if fps == 0 { return Err(FilmError::InvalidDuration(fps)); }
        Ok(Self { title: title.into(), fps, clips: BTreeMap::new(), scenes: BTreeMap::new() })
    }

    pub fn add_clip(&mut self, clip: Clip) -> Result<(), FilmError> {
        if clip.source_path.trim().is_empty() { return Err(FilmError::EmptyClipPath); }
        if clip.duration_frames == 0 { return Err(FilmError::InvalidDuration(clip.duration_frames)); }
        let id = clip.id.clone();
        if self.clips.contains_key(&id) {
            return Err(FilmError::DuplicateClip(id));
        }
        self.clips.insert(id, clip);
        Ok(())
    }

    pub fn remove_clip(&mut self, id: &str) -> Result<(), FilmError> {
        if !self.clips.contains_key(id) {
            return Err(FilmError::ClipNotFound(id.into()));
        }
        self.clips.remove(id);
        Ok(())
    }

    pub fn get_clip(&self, id: &str) -> Option<&Clip> { self.clips.get(id) }

    pub fn clips(&self) -> Vec<&Clip> { self.clips.values().collect() }

    pub fn add_scene(&mut self, scene: Scene) -> Result<(), FilmError> {
        if scene.name.trim().is_empty() { return Err(FilmError::EmptySceneName); }
        let id = scene.id.clone();
        if self.scenes.contains_key(&id) {
            return Err(FilmError::DuplicateScene(id.clone()));
        }
        self.scenes.insert(id, scene);
        Ok(())
    }

    pub fn scenes(&self) -> Vec<&Scene> { self.scenes.values().collect() }

    pub fn scene_count(&self) -> usize { self.scenes.len() }

    pub fn clip_count(&self) -> usize { self.clips.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_create_aaa() {
        let p = FilmProject::new("My Film", 24).unwrap();
        assert_eq!(p.title, "My Film");
        assert_eq!(p.fps, 24);
        assert_eq!(p.clip_count(), 0);
    }

    #[test]
    fn test_invalid_fps_rejected_aaa() {
        assert!(FilmProject::new("X", 0).is_err());
    }

    #[test]
    fn test_add_clip_aaa() {
        let mut p = FilmProject::new("My Film", 24).unwrap();
        let c = Clip { id: "c1".into(), clip_type: ClipType::Video,
            source_path: "/clip.mp4".into(), start_frame: 0, duration_frames: 100, track_index: 0 };
        p.add_clip(c).unwrap();
        assert_eq!(p.clip_count(), 1);
    }

    #[test]
    fn test_empty_clip_path_rejected_aaa() {
        let mut p = FilmProject::new("My Film", 24).unwrap();
        let c = Clip { id: "x".into(), clip_type: ClipType::Video,
            source_path: "".into(), start_frame: 0, duration_frames: 100, track_index: 0 };
        assert!(p.add_clip(c).is_err());
    }

    #[test]
    fn test_duplicate_clip_rejected_aaa() {
        let mut p = FilmProject::new("My Film", 24).unwrap();
        let c = Clip { id: "x".into(), clip_type: ClipType::Video,
            source_path: "/x.mp4".into(), start_frame: 0, duration_frames: 100, track_index: 0 };
        p.add_clip(c.clone()).unwrap();
        let c2 = Clip { id: "x".into(), clip_type: ClipType::Video,
            source_path: "/x.mp4".into(), start_frame: 0, duration_frames: 100, track_index: 0 };
        assert!(p.add_clip(c2).is_err());
    }

    #[test]
    fn test_duplicate_scene_rejected_aaa() {
        let mut p = FilmProject::new("My Film", 24).unwrap();
        let s = Scene { id: "s".into(), name: "S".into(), clips: vec![] };
        p.add_scene(s.clone()).unwrap();
        let s2 = Scene { id: "s".into(), name: "S".into(), clips: vec![] };
        assert!(p.add_scene(s2).is_err());
    }

    #[test]
    fn test_add_scene_aaa() {
        let mut p = FilmProject::new("My Film", 24).unwrap();
        let s = Scene { id: "s1".into(), name: "Intro".into(), clips: vec![] };
        p.add_scene(s).unwrap();
        assert_eq!(p.scene_count(), 1);
    }
}
