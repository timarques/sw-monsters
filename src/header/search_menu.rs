use crate::header::preferences::{Preferences, PreferencesRow};
use crate::traits::{ComboBoxWidget, SpinnerWidget};
use crate::DATA;
use gtk::prelude::*;

#[derive(Clone)]
pub struct SearchMenu {
    stack: gtk::Stack,
    loading: gtk::Spinner,
    pub preferences: Preferences,
    pub container: gtk::PopoverMenu
}

impl SearchMenu {

    pub fn new() -> Self {

        let preferences = Preferences::new();

        let loading = gtk::Spinner::new_loading();
        loading.show();
        loading.start();

        let stack = gtk::Stack::new();
        stack.add_named(&loading, "loading");
        stack.show();

        let container = gtk::PopoverMenu::new();
        container.set_property_width_request(400);
        container.add(&stack);

        Self {
            container,
            stack,
            loading,
            preferences
        }
    }

    pub fn build(&mut self) {
        let data = &*DATA;
        let mut families = vec!["All"];
        families.append(&mut data.iter().map(|monster| monster.family.as_str()).collect::<Vec<_>>());
        families.dedup();

        let sort_by = vec!["Date", "Name", "Family Name", "Stars", "_", "Health", "Attack", "Defense", "Speed"];
        let r#type = vec!["All", "Attack", "Defense", "Health", "Support"];
        let element = vec![
            ("All", None),
            ("Dark", Some("data/icons/elements/dark.png")),
            ("Fire", Some("data/icons/elements/fire.png")),
            ("Light", Some("data/icons/elements/light.png")),
            ("Water", Some("data/icons/elements/water.png")),
            ("Wind", Some("data/icons/elements/wind.png"))
        ];
        let effects = vec![
            ("All", None),
            ("Block Beneficial Effects", Some("data/icons/effects/block-beneficial-effects.png")),
            ("Bomb", Some("data/icons/effects/bomb.png")),
            ("Brand", Some("data/icons/effects/brand.png")),
            ("Continuous Damage", Some("data/icons/effects/continuous-damage.png")),
            ("Counter", Some("data/icons/effects/counter.png")),
            ("Critical Resist", Some("data/icons/effects/critical-resist.png")),
            ("Decrease Attack Bar", Some("data/icons/effects/decrease-attack-bar.png")),
            ("Decrease Attack", Some("data/icons/effects/decrease-attack.png")),
            ("Decrease Defense", Some("data/icons/effects/decrease-defense.png")),
            ("Decrease Speed", Some("data/icons/effects/decrease-speed.png")),
            ("Defend", Some("data/icons/effects/defend.png")),
            ("Endure", Some("data/icons/effects/endure.png")),
            ("Freeze", Some("data/icons/effects/freeze.png")),
            ("Glancing", Some("data/icons/effects/glancing.png")),
            ("Immunity", Some("data/icons/effects/immunity.png")),
            ("Increase Attack Bar", Some("data/icons/effects/increase-attack-bar.png")),
            ("Increase Attack", Some("data/icons/effects/increase-attack.png")),
            ("Increase Critical Rate", Some("data/icons/effects/increase-critical-rate.png")),
            ("Increase Defense", Some("data/icons/effects/increase-defense.png")),
            ("Increase Speed", Some("data/icons/effects/increase-speed.png")),
            ("Invincibility", Some("data/icons/effects/invincibility.png")),
            ("Oblivion", Some("data/icons/effects/oblivion.png")),
            ("Protect Soul", Some("data/icons/effects/protect-soul.png")),
            ("Provoke", Some("data/icons/effects/provoke.png")),
            ("Recovery", Some("data/icons/effects/recovery.png")),
            ("Reflect", Some("data/icons/effects/reflect.png")),
            ("Revenge", Some("data/icons/effects/revenge.png")),
            ("Shield", Some("data/icons/effects/shield.png")),
            ("Silence", Some("data/icons/effects/silence.png")),
            ("Sleep", Some("data/icons/effects/sleep.png")),
            ("Stun", Some("data/icons/effects/stun.png")),
            ("Threat", Some("data/icons/effects/threat.png")),
            ("Unrecoverable", Some("data/icons/effects/unrecoverable.png")),
            ("Vampire", Some("data/icons/effects/vampire.png"))
        ];

        let preferences_widget = self.preferences.add_row(
            PreferencesRow::new().add("Sort By", &gtk::ComboBox::new_with_data(sort_by))
        ).add_row(
            PreferencesRow::new()
                .add("Type", &gtk::ComboBox::new_with_data(r#type))
                .add("Element", &gtk::ComboBox::new_with_images(element))
                .add("Family", &gtk::ComboBox::new_with_data(families))
                .add("Effect", &gtk::ComboBox::new_with_images(effects))
        ).add_row(
            PreferencesRow::new()
                .add("Fusion", &gtk::Switch::new())
                .add("Second Awakening", &gtk::Switch::new())
        ).build();
        self.stack.add_named(&preferences_widget, "preferences");
    }

    pub fn enable(&self) {
        self.stack.set_visible_child_name("preferences");
        self.loading.stop();
    }

}
