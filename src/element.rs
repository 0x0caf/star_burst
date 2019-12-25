
use nannou::prelude::*;
use crate::random_range::RandomRange;

#[derive(Clone, Copy)]
pub struct AnimationProfile {
    pub width: RandomRange,
    pub height: RandomRange,
    pub speed: RandomRange,
    pub direction_probability: RandomRange,
    pub color: Srgba,
}


pub struct Element {
    width: f32,
    height: f32,
    rot_speed: f32, // deg per second
    rotation: f32,
    profile: AnimationProfile,
}

impl Element {
    pub fn new(profile: AnimationProfile) -> Element {


        Element {
            width: profile.width.get(),
            height: profile.height.get(),
            rot_speed: profile.speed.get() * profile.direction_probability.positive_or_negative(),
            rotation: RandomRange::new(0.0, 360.0).get(),
            profile
        }
    }

    pub fn update(&mut self, duration: f32) {
        let add_rotation = self.rot_speed * duration;
        self.rotation = self.rotation + add_rotation;
        self.width = self.profile.width.get();
        self.height = self.profile.height.get();
    }

    pub fn draw(&self, draw: &nannou::draw::Draw) {
        let radians = deg_to_rad(self.rotation);

        draw.rect()
            .rotate(radians)
            .x_y(0.0, 0.0)
            .height(self.width)
            .width(self.width * 0.5)
            .color(self.profile.color.clone());
    }
}