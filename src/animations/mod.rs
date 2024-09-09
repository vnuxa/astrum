use std::{collections::HashMap, sync::{Arc, Mutex}, time::Instant};

use lazy_static::lazy_static;
use lilt::{Animated, Easing};

lazy_static!{
    static ref ALL_ANIMATIONS: Arc<Mutex<HashMap<String, Animated<bool, Instant>>>> = Arc::new(Mutex::new(HashMap::new()));
}

pub fn make_animation(
    data: mlua::Table
// ) -> Animated<bool, Instant>{
) {

    let mut anim_storage = ALL_ANIMATIONS.lock().unwrap();

    let state: bool = data.get::<_, bool>("value").unwrap_or(false);
    let mut animation = Animated::new(state)
        .duration(data.get::<_, mlua::Number>("time").unwrap_or(1.0) as f32 * 1000.0 );

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
    if data.get::<_, bool>("reverse").unwrap_or(false) {
        animation = animation.auto_reverse();
    }
    if let Ok(delay) = data.get::<_, mlua::Number>("delay") {
        animation = animation.delay(delay as f32 * 1000.0)
    }



    anim_storage.insert(data.get::<_, mlua::Number>("animation_id").unwrap().to_string(), animation);
}
pub fn any_animation_in_progress() -> bool {
    let anim_storage = ALL_ANIMATIONS.lock().unwrap();
    let time = std::time::Instant::now();

    for (key, anim) in anim_storage.iter() {
        // println!("checking anim {}", key);
        if anim.in_progress(time) {
            println!("anim is in progres... {}", key);
            return true;
        }
    }
    // println!("no anims in progress");

    false
}

pub fn get_animation(animation_id: mlua::Number) -> Animated<bool, Instant> {
    // let anim_binding = ALL_ANIMATIONS.clone();
    let mut anim_storage = ALL_ANIMATIONS.lock().unwrap();

    anim_storage.get(&animation_id.to_string()).expect("Couldn't find animation ID").clone()
}

pub fn animate_value(animation_id: mlua::Number, from_value: mlua::Number, to_value: mlua::Number) -> f32 {
    let time = std::time::Instant::now();
    return get_animation(animation_id).animate_bool(from_value as f32, to_value as f32, time);
}

pub fn set_anim(animation_id: mlua::Number, value: Option<bool>) -> bool{
    let mut anim_storage = ALL_ANIMATIONS.lock().unwrap();
    // let mut anim = get_animation(animation_id);

    let mut anim = anim_storage.get_mut(&animation_id.to_string()).expect("Couldn't find animation ID");
    let now = std::time::Instant::now(); // maybe make this an option??
    println!("anim current state");

    anim.transition({
        if value.is_none() {
            !anim.value
        } else {
            // println!("value is {:?}", value);
            value.unwrap()
        }
    }, now);

    return anim.value;
}
pub fn toggle_state_anim(animation_id: mlua::Number, value: Option<bool>) -> bool{
    let mut anim_storage = ALL_ANIMATIONS.lock().unwrap();
    // let mut anim = get_animation(animation_id);

    let mut anim = anim_storage.get_mut(&animation_id.to_string()).expect("Couldn't find animation ID");

    println!("anim value: {}", anim.value);
    if value.is_none() {
        anim.value = !anim.value;
    } else {
        // println!("value is {:?}", value);
        anim.value = value.unwrap();
    }


    println!("anim new value: {}", anim.value);
    return anim.value;
}
