
use crate::element::{Element, AnimationProfile};
use crate::random_range::RandomRange;
use crate::int_color::IntColor;


pub struct ElementCollection {
    pub collection: Vec<Element>
}
impl ElementCollection {
    pub fn new(num: i32, profile: AnimationProfile) -> ElementCollection {
        let mut collection = vec![];
        for _i in 0 .. num {
            collection.push(Element::new(profile));
        }
        Self {
            collection
        }
    }

    pub fn update(&mut self, duration: f32) {
        for t in self.collection.iter_mut() {
            t.update(duration);
        }
    }

    pub fn draw(&self, draw: &nannou::draw::Draw) {
        for t in &self.collection {
            t.draw(&draw);
        }
    }
}

pub struct OrangeSun {
}

impl OrangeSun {
    pub fn profile() -> Vec<ElementCollection> {
        let mut vec = vec![];
        let size = 320.0;
        let mellow_apricot = AnimationProfile {
            width: RandomRange::new(size + 20.0, size + 30.0),
            height: RandomRange::new(120.0, 200.0),
            speed: RandomRange::new(10.0, 45.0),
            color: IntColor::new(253, 183, 119).as_srgb(),
            direction_probability: RandomRange::new(1.0, 3.0)
        };

        let rajah = AnimationProfile {
            width: RandomRange::new(size + 15.0, size + 20.0),
            height: RandomRange::new(80.0, 120.0),
            speed: RandomRange::new(10.0, 45.0),
            color: IntColor::new(253, 167, 102).as_srgb(),
            direction_probability: RandomRange::new(1.0, 3.0)
        };

        let royal_orange = AnimationProfile {
            width: RandomRange::new(size + 10.0, size + 15.0),
            height: RandomRange::new(60.0, 80.0),
            speed: RandomRange::new(10.0, 45.0),
            color: IntColor::new(253, 147, 70).as_srgb(),
            direction_probability: RandomRange::new(1.0, 3.0)
        };

        let princeton_orange = AnimationProfile {
            width: RandomRange::new(size + 5.0, size + 10.0),
            height: RandomRange::new(50.0, 60.0),
            speed: RandomRange::new(10.0, 45.0),
            color: IntColor::new(253, 127, 44).as_srgb(),
            direction_probability: RandomRange::new(1.0, 3.0)
        };

        let vivid_orange = AnimationProfile {
            width: RandomRange::new(size, size + 5.0),
            height: RandomRange::new(45.0, 50.0),
            speed: RandomRange::new(10.0, 45.0),
            color: IntColor::new(255, 98, 0).as_srgb(),
            direction_probability: RandomRange::new(1.0, 3.0)
        };

        vec.push(ElementCollection::new(128, mellow_apricot));
        vec.push(ElementCollection::new(100, rajah));
        vec.push(ElementCollection::new(90, royal_orange));
        vec.push(ElementCollection::new(64, princeton_orange));
        vec.push(ElementCollection::new(64, vivid_orange));

        vec
    }
}

pub struct BlueNight{}

impl BlueNight {
    pub fn profile() -> Vec<ElementCollection> {
        let mut vec = vec![];

        let size = 440.0;

        let navy_blue = AnimationProfile {
            width: RandomRange::new(size + 30.0, size + 50.0),
            height: RandomRange::new(120.0, 200.0),
            speed: RandomRange::new(10.0, 45.0),
            color: IntColor::new(1, 2, 128).as_srgb(),
            direction_probability: RandomRange::new(1.0, 3.0)
        };


        let indigo_dye = AnimationProfile {
            width: RandomRange::new(size + 25.0, size + 30.0),
            height: RandomRange::new(80.0, 120.0),
            speed: RandomRange::new(10.0, 45.0),
            color: IntColor::new(6, 25, 147).as_srgb(),
            direction_probability: RandomRange::new(1.0, 3.0)
        };


        let black = AnimationProfile {
            width: RandomRange::new(size + 20.0, size + 25.0),
            height: RandomRange::new(60.0, 80.0),
            speed: RandomRange::new(10.0, 45.0),
            color: IntColor::new(0, 0, 0).as_srgba(0.5),
            direction_probability: RandomRange::new(1.0, 3.0)
        };


        let denim = AnimationProfile {
            width: RandomRange::new(size + 15.0, size + 20.0),
            height: RandomRange::new(50.0, 60.0),
            speed: RandomRange::new(10.0, 45.0),
            color: IntColor::new(19, 92, 197).as_srgb(),
            direction_probability: RandomRange::new(1.0, 7.0)
        };


        let bright_navy_blue = AnimationProfile {
            width: RandomRange::new(size, size + 5.0),
            height: RandomRange::new(45.0, 50.0),
            speed: RandomRange::new(10.0, 45.0),
            color: IntColor::new(25, 115, 209).as_srgb(),
            direction_probability: RandomRange::new(1.0, 3.0)
        };

        vec.push(ElementCollection::new(128, navy_blue));
        vec.push(ElementCollection::new(128, indigo_dye));
        vec.push(ElementCollection::new(128, black));
        vec.push(ElementCollection::new(128, denim));
        vec.push(ElementCollection::new(128, bright_navy_blue));

        vec
    }
}

pub struct Sparkle{}

impl Sparkle {
    pub fn profile() -> Vec<ElementCollection> {
        let mut vec = vec![];
        let white = AnimationProfile {
            width: RandomRange::new(10.0, 450.0),
            height: RandomRange::new(120.0, 200.0),
            speed: RandomRange::new(10.0, 45.0),
            color: IntColor::new(255, 255, 255).as_srgba(0.5),
            direction_probability: RandomRange::new(1.0, 3.0)
        };
        let light_gray = AnimationProfile {
            width: RandomRange::new(50.0, 320.0),
            height: RandomRange::new(120.0, 200.0),
            speed: RandomRange::new(10.0, 45.0),
            color: IntColor::new(169, 188, 208).as_srgba(0.5),
            direction_probability: RandomRange::new(1.0, 3.0)
        };

        let light_blue = AnimationProfile {
            width: RandomRange::new(50.0, 120.0),
            height: RandomRange::new(120.0, 200.0),
            speed: RandomRange::new(10.0, 45.0),
            color: IntColor::new(24, 135, 199).as_srgba(0.5),
            direction_probability: RandomRange::new(1.0, 3.0)
        };
        vec.push(ElementCollection::new(32, white));
        vec.push(ElementCollection::new(64, light_gray));
        vec.push(ElementCollection::new(32, light_blue));
        vec
    }
}


pub struct Smoke{}

impl Smoke {
    pub fn profile() -> Vec<ElementCollection> {
        let mut vec = vec![];
        let white = AnimationProfile {
            width: RandomRange::new(445.0, 450.0),
            height: RandomRange::new(120.0, 200.0),
            speed: RandomRange::new(10.0, 15.0),
            color: IntColor::new(255, 255, 255).as_srgba(0.25),
            direction_probability: RandomRange::new(1.0, 3.0)
        };
        let black = AnimationProfile {
            width: RandomRange::new(435.0, 440.0),
            height: RandomRange::new(120.0, 200.0),
            speed: RandomRange::new(1.0, 4.0),
            color: IntColor::new(0, 0, 0).as_srgba(0.5),
            direction_probability: RandomRange::new(1.0, 3.0)
        };
        let inner_white = AnimationProfile {
            width: RandomRange::new(375.0, 400.0),
            height: RandomRange::new(120.0, 200.0),
            speed: RandomRange::new(10.0, 45.0),
            color: IntColor::new(255, 255, 255).as_srgba(0.25),
            direction_probability: RandomRange::new(1.0, 3.0)
        };

        let inner_black = AnimationProfile {
            width: RandomRange::new(375.0, 475.0),
            height: RandomRange::new(120.0, 200.0),
            speed: RandomRange::new(10.0, 45.0),
            color: IntColor::new(0, 0, 0).as_srgba(0.25),
            direction_probability: RandomRange::new(1.0, 3.0)
        };
        vec.push(ElementCollection::new(128, white));
        vec.push(ElementCollection::new(64, black));
        vec.push(ElementCollection::new(128, inner_white));
        vec.push(ElementCollection::new(64, inner_black));
        vec
    }
}