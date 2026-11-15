use sigmaos_craft_film::{Clip, ClipType, FilmProject, Scene};

#[test]
fn film_project_integration_aaa() {
    let mut p = FilmProject::new("My Film", 24).unwrap();
    let c = Clip { id: "c1".into(), clip_type: ClipType::Video,
        source_path: "/clip.mp4".into(), start_frame: 0, duration_frames: 100, track_index: 0 };
    p.add_clip(c).unwrap();
    let s = Scene { id: "s1".into(), name: "Intro".into(), clips: vec!["c1".into()] };
    p.add_scene(s).unwrap();
    assert_eq!(p.clip_count(), 1);
    assert_eq!(p.scene_count(), 1);
}
