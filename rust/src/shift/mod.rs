use godot::{
    classes::{AudioStream, AudioStreamPlayer2D, ResourceLoader},
    obj::Gd,
};
use page::Page;
use rand::seq::SliceRandom;

mod page;

pub(crate) const SCENE_URL: &str = "res://scenes/shift.tscn";
pub(crate) const SHIFT1: ShiftData<2> = ShiftData {
    pages: [
        Page::new(true, 10, 100, "page1.jpg", "page 1", "woahg"),
        Page::new(false, 100, 10, "page2.jpg", "page 2", "oh no"),
    ],
    music: "shift1.ogg",
};

pub(crate) struct ShiftData<const N: usize> {
    pages: [Page; N],
    music: &'static str,
}

pub(crate) struct Shift {
    left: Vec<Page>,
    current: Page,
    music: Gd<AudioStream>,
}

impl Default for Shift {
    fn default() -> Self {
        Self::load(SHIFT1)
    }
}

/// Select a random [Page] from the provided list and remove it.
/// Returns `None` if the list is empty.
fn select_page(pages: &mut Vec<Page>) -> Option<Page> {
    if pages.is_empty() {
        return None;
    }
    let i = rand::random_range(0..pages.len());
    Some(pages.remove(i))
}

impl Shift {
    pub(crate) fn load<const N: usize>(data: ShiftData<N>) -> Shift {
        let mut left = data.pages.to_vec();
        let mut rng = rand::rng();
        left.shuffle(&mut rng);
        let current = select_page(&mut left).expect("shift must have at least one page");
        let music = ResourceLoader::singleton()
            .load(&format!("res://assets/music/{}", data.music))
            .and_then(|res| res.try_cast::<AudioStream>().ok())
            .expect("music must be valid");
        Self {
            left,
            current,
            music,
        }
    }

    pub(crate) fn set_music(&self, player: &mut Gd<AudioStreamPlayer2D>) {
        player.stop();
        player.set_stream(&self.music);
        player.play();
    }

    /// Move to the next [Page] in the [Shift].
    pub(crate) fn next_page(&mut self) -> Option<Page> {
        select_page(&mut self.left)
    }

    /// Is the [Shift] done?
    pub(crate) fn is_done(&self) -> bool {
        self.left.is_empty()
    }

    /// The current [Page] in the [Shift].
    pub(crate) fn page(&self) -> &Page {
        &self.current
    }
}
