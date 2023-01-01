use bevy::prelude::*;
use ronky_components::components;
use ronky_resources::resources;

#[derive(Resource)]
pub struct DateTimeTimer<T>(Timer, std::marker::PhantomData<T>);

pub struct DateTime<T>(std::marker::PhantomData<T>);

impl<T> DateTime<T> {
    pub fn new() -> Self {
        Self(std::marker::PhantomData::<T>)
    }
}

impl<T: Send + Sync + 'static> Plugin for DateTime<T> {
    fn build(&self, app: &mut App) {
        app.insert_resource(DateTimeTimer::<T>(
            Timer::from_seconds(1.0, TimerMode::Repeating),
            std::marker::PhantomData::<T>,
        ))
        .insert_resource(resources::DateTime::<T>(
            Box::new(resources::DateTimeValue(default::<
                chrono::DateTime<chrono::Local>,
            >())),
            std::marker::PhantomData::<T>,
        ))
        .add_system(update::<T>)
        .add_system(update_text::<T>);
    }
}

fn update<T: Send + Sync + 'static>(
    time: Res<Time>,
    mut timer: ResMut<DateTimeTimer<T>>,
    mut date_time: ResMut<resources::DateTime<T>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        date_time.0 = Box::new(resources::DateTimeValue(chrono::Local::now()));
    }
}

fn update_text<T: Send + Sync + 'static>(
    timer: Res<DateTimeTimer<T>>,
    date_time: Res<resources::DateTime<T>>,
    mut query: Query<&mut components::SensorText>,
) {
    if timer.0.just_finished() {
        for mut sensor_text in query.iter_mut() {
            sensor_text.0 = format!("{}", date_time.0.second());
        }
    }
}
