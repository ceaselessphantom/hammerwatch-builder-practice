pub mod class{
	type Base_Stat(f64, f64);
	pub struct Class{
		pub name: String,
		pub level: u16,
		pub base: Class_Base,
	}
	struct Class_Base{
		health: Base_Stat,
		mana: Base_Stat,
		hp_regen: Base_Stat,
		mp_regen: Base_Stat,
		armor: Base_Stat,
		resist: Base_Stat,
	}
	pub fn initilize_class(mut character: &mut Character) -> mut Class{
		mut target = Class{name: "Blank".to_string(), level: 1, base: Class_Base{health: (0.0, 0.0), mana: (0.0, 0.0), hp_regen: (0.0, 0.0), mp_regen: (0.0, 0.0), armor: (0.0, 0.0), resist: (0.0, 0.0)}};
		change_class(&mut character, &mut target);
		return target;
	}
	pub fn change_class(mut character: &mut Character, mut target: &mut Class){
		let new_name: String;
		//Get the new_name from the user.
		if new_name.to_string() == target.name{
			return ();
		}
		let level=target.level;
		alter_level(&mut character, &mut target, level, true, false);
		match new_name {
			"Paladin" => target = Class{name: "Paladin".to_string(), level: level, base: Class_Base{
				health: (67.0, 8.0), mana: (44.0, 6.0), hp_regen: (-0.025, 0.025), mp_regen: (0.035, 0.05), armor: (9.4, 0.6), resist: (-0.2, 0.2)}
			},
			"Priest" => target = Class{name: "Priest".to_string(), level: level, base: Class_Base{
				health: (25, 5), mana: (59, 11), hp_regen: (-0.05, 0.05), mp_regen: (1.8, 0.1), armor: (-0.2, 0.2), resist: (2.5, 0.5)}
			},
			"Ranger" => target = Class{name: "Ranger".to_string(), level: level, base: Class_Base{
				health: (44, 6), mana: (44, 6), hp_regen: (-0.025, 0.025), mp_regen: (0.425, 0.075), armor: (-0.5, 0.5), resist: (-0.2, 0.2)}
			},
			"Sorcerer" => target = Class{name: "Sorcerer".to_string(), level: level, base: Class_Base{
				health: (35, 5), mana: (59, 16), hp_regen: (-0.025, 0.025), mp_regen: (1.4, 0.1), armor: (-0.2, 0.2), resist: (1.5, 0.5)}
			},
			"Thief" => target = Class{name: "Thief".to_string(), level: level, base: Class_Base{
				health: (35, 5), mana: (32, 8), hp_regen: (-0.025, 0.025), mp_regen: (0.4, 0.1), armor: (-0.3, 0.3), resist: (-0.3, 0.3)}
			},
			"Warlock" => target = Class{name: "Warlock".to_string(), level: level, base: Class_Base{
				health: (54, 6), mana: (62, 13), hp_regen: (-0.025, 0.025), mp_regen: (1.425, 0.075), armor: (1.7, 0.3), resist: (1.7, 0.3)}
			},
			"Wizard" => target = Class{name: "Wizard".to_string(), level: level, base: Class_Base{
				health: (31, 4), mana: (60, 15), hp_regen: (-0.025, 0.025), mp_regen: (1.4, 0.1), armor: (-0.2, 0.2), resist: (1.5, 0.5)}
			},
			"Gladiator" => target = Class{name: "Gladiator".to_string(), level: level, base: Class_Base{
				health: (58, 7), mana: (43, 7), hp_regen: (-0.025, 0.025), mp_regen: (0.45, 0.05), armor: (1.7, 0.3), resist: (-0.3, 0.3)}
			},
			"Witch Hunter" => target = Class{name: "Witch Hunter".to_string(), level: level, base: Class_Base{
				health: (39, 6), mana: (53, 7), hp_regen: (-0.025, 0.025), mp_regen: (0.675, 0.075), armor: (0.7, 0.3), resist: (2.5, 0.5)}
			},
		}
		alter_level(&mut character, &mut target, level, true, true);
	}
	fn alter_level(mut character &mut Character, mut the_class: &mut Class, add: u16, use_base: bool, add_remove: bool){
		//add is the number of levels changed by.
		the_class.level+=add;
		if add_remove == true{
			the_class.level+=add;
		}
		else{
			the_class.level-=add;
		}
		
		let v = &mut character.stats;
		mut target: [&mut f64; 6] = [&mut v.health.amount, &mut v.mana.amount, &mut v.hp_regen.amount, &mut v.mp_regen.amount, &mut v.armor.amount, &mut v.resist.amount];
		
		let editors=the_class.base;
		let edit = [Base_Stat, 6] = [editors.health, editors.mana, editors.hp_regen, editors.mp_regen, editors.armor, editors.resist];
		
		if use_base == true{
			for x in 0..6{
				change_flat(&mut target[x], edit[x].0, add_remove, 1);
			}
		}
		for x in 0..6{
			change_flat(&mut target[x], edit[x].1, add_remove, add);
		}
	}
}
