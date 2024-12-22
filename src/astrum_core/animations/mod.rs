use std::{collections::HashMap, sync::{Arc, Mutex}, time::Instant};


// wow
use lazy_static::lazy_static;
use lilt::{Animated, Easing};

lazy_static!{
    static ref ALL_ANIMATIONS: Arc<Mutex<HashMap<u32, Animated<bool, Instant>>>> = Arc::new(Mutex::new(HashMap::new()));
}


pub fn make_animation(
    data: mlua::Table
) {
    let mut animation_storage = ALL_ANIMATIONS.lock().unwrap();

    let state: bool = data.get("value").unwrap_or(false);
    let mut animation = Animated::new(state)
        .duration(data.get::<_, mlua::Number>("time").unwrap_or(1.0) as f32 * 1000.0);

    if let Ok(easing) = data.get::<_, mlua::String>("easing") {
        animation = animation.easing(match easing.to_str().unwrap() {
            "linear" => Easing::Linear,
            "ease_in" => Easing::EaseIn,
            "ease_out" => Easing::EaseOut,
            "ease_in_out" => Easing::EaseInOut,
            "ease_in_quad" => Easing::EaseInQuad,
            "ease_out_quad" => Easing::EaseOutQuad,
            "ease_in_out_quad" => Easing::EaseOutQuad,
            "ease_in_cubic" => Easing::EaseInCubic,
            "ease_out_cubic" => Easing::EaseOutCubic,
            "ease_in_out_cubic" => Easing::EaseInOutCubic,
            "ease_in_quart" => Easing::EaseInQuart,
            "ease_out_quart" => Easing::EaseOutQuart,
            "ease_in_out_quart" => Easing::EaseInOutQuart,
            "ease_in_quint" => Easing::EaseInQuint,
            "ease_out_quint" => Easing::EaseOutQuint,
            "ease_in_out_quint" => Easing::EaseInOutQuint,
            "ease_in_expo" => Easing::EaseInExpo,
            "ease_out_expo" => Easing::EaseOutExpo,
            "ease_in_out_expo" => Easing::EaseInOutExpo,
            "ease_in_circ" => Easing::EaseInCirc,
            "ease_out_circ" => Easing::EaseOutCirc,
            "ease_in_out_circ" => Easing::EaseInOutCirc,
            "ease_in_back" => Easing::EaseInBack,
            "ease_out_back" => Easing::EaseOutBack,
            "ease_in_out_back" => Easing::EaseInOutBack,
            "ease_in_elastic" => Easing::EaseInElastic,
            "ease_out_elastic" => Easing::EaseOutElastic,
            "ease_in_out_elastic" => Easing::EaseInOutElastic,
            "ease_in_bounce" => Easing::EaseInBounce,
            "ease_out_bounce" => Easing::EaseOutBack,
            "ease_in_out_bounce" => Easing::EaseInOutBounce,
            &_ => unimplemented!("requested easing style is not supported, are you sure it isn't a typo?")
        });
    }

    if let Ok(repeat) = data.get::<_, mlua::Number>("repeat_count") {
        animation = animation.repeat(repeat as u32);
    }

    if data.get::<_, bool>("reverse").is_ok() {
        animation = animation.auto_reverse();
    }

    if let Ok(delay) = data.get::<_, mlua::Number>("delay") {
        animation = animation.delay(delay as f32 * 1000.0);
    }

    animation_storage.insert(data.get::<_, mlua::Number>("animation_id").unwrap() as u32, animation);
}

pub fn any_animation_in_progress() -> bool {
    let anim_storage = ALL_ANIMATIONS.lock().unwrap();
    let time = Instant::now();

    for (key, anim) in anim_storage.iter() {
        if anim.in_progress(time) {
            // println!()
            return true;
        }
    }

    false
}

pub fn animate_value(animation_id: mlua::Number, from_value: mlua::Number, to_value: mlua::Number) -> f32 {
    let mut anims = ALL_ANIMATIONS.lock().unwrap();
    // let anim = anims.get(&(animation_id as u32)).expect("Couldnt find animation");
    let time = Instant::now();

    return anims.get(&(animation_id as u32)).expect("Couldnt find animation").animate_bool(from_value as f32, to_value as f32, time);
}

pub fn set_anim(animation_id: mlua::Number, value: Option<bool>) -> bool {
    let mut anims = ALL_ANIMATIONS.lock().unwrap();

    let mut anim = anims.get_mut(&(animation_id as u32)).expect("Coudnt find animation in set anim");
    let time = Instant::now();

    anim.transition({
        if value.is_none() {
            !anim.value
        } else {
            value.unwrap()
        }
    }, time);

    return anim.value;
}

pub fn toggle_state_anim(animation_id: mlua::Number, value: Option<bool>) -> bool {
    let mut animations = ALL_ANIMATIONS.lock().unwrap();

    let mut anim = animations.get_mut(&(animation_id as u32)).expect("Couldnt find animation in toggle anim");

    if value.is_none() {
        anim.value = !anim.value;
    } else {
        anim.value = value.unwrap();
    }

    return anim.value
}
