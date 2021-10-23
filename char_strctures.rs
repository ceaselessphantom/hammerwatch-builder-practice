pub mod char_structures{
	pub type Conditions = fn(Character) -> bool;
	pub enum Questions{
		NoCondition,
		Condition{condition: Conditions, dependancies: &mut Vec<u16>},
		DependsOnly{dependancies: &mut Vec<u16>},
	}
	pub enum Modify{
		YesAll(fn(&mut Character, bool, u16, u16)),//layers then index
		NoLayers(fn(&mut Character, bool, u16)),
		NoIndex(fn(&mut Character, bool, u16)),
		NoAll(fn(&mut Character, bool)),
	}
	pub struct Status{
		character: Character,
		new_game: u16,
		class: Class,
		statues: [Statue; 3],
		items: Item,
		titles: Title,
		bonuses: Gladiator,
	}
	pub struct Character{
		pub stats: Stats,
		pub percent_stats: Percentage_Stats,
		pub passive_texts: Texts,//u16 is layers.
		pub flags: Parameter_Flags,
		pub percents: Parameter_Percents,
		pub inserts: Parameter_Inserts,
		pub actives: Actives,
		pub flexible: Vec<Mod_Variable>,
		pub modifications: Vec<Modification>,
		pub openslots_m: Vec<u16>,
	}
	struct Stats{
		pub attack: Stat,
		pub skill: Stat,
		pub armor: Stat,
		pub resist: Stat,
		pub base_health: Stat,
		pub base_mana: Stat,
		pub health: Stat,
		pub mana: Stat,
		pub hp_regen: Stat,
		pub mp_regen: Stat,
		pub attack_p_damage: Stat,
		pub attack_m_damage: Stat,
		pub attack_speed: Stat,
		pub movement_speed: Stat,
		pub luck: Stat,
		pub gold_percent: Stat,
		pub ore_percent: Stat,
		pub exp: Stat,
		pub exp_a: Stat,
		pub exp_b: Stat,
		pub exp_c: Stat,
		pub exp_d: Stat,
		pub damage: Stat,
		pub damage_a: Stat,
		pub damage_b: Stat,
		pub damage_c: Stat,
		pub damage_d: Stat,
		pub critical_a_damage: Stat,
		pub critical_s_damage: Stat,
		pub lifesteal: Stat,
		pub return_p_damage: Stat,
		pub p_collide: Stat,
	}
	struct Stat{
		pub text: String,
		pub amount: f64,
		pub mod_indexs: Vec<u16>,//For fields whose information is altered by this stat.
	}
	struct Percentage_Stats{
		pub evasion: Percentage_Stat,
		pub a_crit_chance: Percentage_Stat,
		pub s_crit_chance: Percentage_Stat,
		pub mana_cost: Percentage_Stat,
		pub tax_reduction: Percentage_Stat,
		pub block_projectile: Percentage_Stat,
		pub physical_pen: Percentage_Stat,
		pub magical_pen: Percentage_Stat,
		pub damage_reduction: Percentage_Stat,
	}
	struct Percentage_Stat{
		pub text: String,
		pub amount: Vec<f64>,
	}
	struct Actives{
		pub primary_attack: Active,
		pub attacked: Active,
		pub potion: Active,
		pub defeat_enemy: Active,
	}
	struct Active{
		pub name: String,
		pub attacks: Texts,
		pub effects: Texts,
	}
	pub struct Texts{
		pub fields: Vec<String>,//Alter this string to be what you want.
		pub mod_indexs: Vec<u16>,//This seperate so it can be called on directly easily.
	}
	struct Parameter_Flags{
		pub combo: Parameter_Flag,
	}
	struct Parameter_Flag{
		pub text: String,
		pub flag: bool,
		pub mod_indexs: Vec<u16>,
	}
	struct Parameter_Percents{
		pub remaining_hp: Parameter_Percent,
		pub remaining_mp: Parameter_Percent,
	}
	struct Parameter_Percent{
		pub text: String,
		pub amount: u8,
		pub mod_indexs: Vec<u16>,
	}
	struct Parameter_Inserts{
		pub gold: Parameter_Insert,
		pub item_count: Parameter_Insert,//this the user can not manually edit.
	}
	struct Parameter_Insert{
		pub text: String,
		pub amount: f64,
		pub cap: f64,
		pub mod_indexs: Vec<u16>,
	}
	struct Mod_Variable{
		pub mod_index: u16,
		pub amount: f64,
	}
	pub struct Modification{
		pub conditions: Questions,
		pub changes: Modify,
		pub activated: bool,//maybe keep an eye out for anything that used this.
		pub layers: u16,
	}
	
		pub fn initilize_character() -> Character{
			let first = Stats{
				attack:
					Stat{text: "attack".to_string(), 					amount: 120.0, dependents: vec![]},
				skill:
					Stat{text: "skill".to_string(), 					amount: 50.0, dependents: vec![]},
				armor:
					Stat{text: "armor".to_string(), 					amount: 70.0, dependents: vec![]},
				resist:
					Stat{text: "resistance".to_string(), 				amount: 50.0, dependents: vec![]},
				base_health:
					Stat{text: "base health".to_string(), 				amount: 0.0, dependents: vec![]},
				base_mana:
					Stat{text: "base mana".to_string(), 				amount: 0.0, dependents: vec![]},
				health:
					Stat{text: "health".to_string(), 					amount: 0.0, dependents: vec![]},
				mana:
					Stat{text: "mana".to_string(), 						amount: 0.0, dependents: vec![]},
				hp_regen:
					Stat{text: "health regeneration".to_string(), 		amount: 1.0, dependents: vec![]},
				mp_regen:
					Stat{text: "mana regeneration".to_string(), 		amount: 1.0, dependents: vec![]},
				attack_p_damage:
					Stat{text: "physical attack damage".to_string(), 	amount: 0.0, dependents: vec![]},
				attack_m_damage:
					Stat{text: "magical attack damage".to_string(), 	amount: 0.0, dependents: vec![]},
				attack_speed:
					Stat{text: "attack speed".to_string(), 				amount: 100.0, dependents: vec![]},
				movement_speed:
					Stat{text: "movement speed".to_string(), 			amount: 1.6, dependents: vec![]},
				luck:
					Stat{text: "luck".to_string(), 						amount: 0.0, dependents: vec![]},
				gold_percent:
					Stat{text: "gold percentage".to_string(), 			amount: 100.0, dependents: vec![]},
				ore_percent:
					Stat{text: "ore percentage".to_string(), 			amount: 100.0, dependents: vec![]},
				exp:
					Stat{text: "exp percent".to_string(),				amount: 100.0, dependants: vec![]},
				exp_a:
					Stat{text: "aberation exp percent".to_string(),		amount: 100.0, dependants: vec![]},
				exp_b:
					Stat{text: "beast exp percent".to_string(),			amount: 100.0, dependants: vec![]},
				exp_c:
					Stat{text: "constuct exp percent".to_string(),		amount: 100.0, dependants: vec![]},
				exp_d:
					Stat{text: "undead exp percent".to_string(),		amount: 100.0, dependants: vec![]},
				damage:
					Stat{text: "damage increase".to_string(), 			amount: 100.0, dependents: vec![]},
				damage_a:
					Stat{text: "damage aberration".to_string(), 		amount: 100.0, dependents: vec![]},
				damage_b:
					Stat{text: "damage beast".to_string(), 				amount: 100.0, dependents: vec![]},
				damage_c:
					Stat{text: "damage construct".to_string(), 			amount: 100.0, dependents: vec![]},
				damage_d:
					Stat{text: "damage undead".to_string(), 			amount: 100.0, dependents: vec![]},
				critical_a_damage:
					Stat{text: "critical attack damage".to_string(), 	amount: 4.0, dependents: vec![]},
				critical_s_damage:
					Stat{text: "critical skill damage".to_string(), 	amount: 3.5, dependents: vec![]},
				lifesteal:
					Stat{text: "lifesteal".to_string(), 	amount: 0.0, dependents: vec![]},
				return_p_damage:
					Stat{text: "return x damage when hit".to_string(), 	amount: 0.0, dependents: vec![]},
				p_collide:
					Stat{text: "x damage on collision".to_string(), 	amount: 0.0, dependents: vec![]},
			};
			let second = Percentage_Stats{
				evasion: Percentage_Stat{text: "evasion".to_string(), amount: vec![]},
				a_crit_chance: Percentage_Stat{text: "attack critical chance".to_string(), amounts: vec![]},
				s_crit_chance: Percentage_Stat{text: "skill critical chance".to_string(), amounts: vec![]},
				mana_cost: Percentage_Stat{text: "mana cost".to_string(), amounts: vec![]},
				tax_reduction: Percentage_Stat{text: "tax midpoint".to_string(), amounts: vec![]},
				block_projectile: Percentage_Stat{text: "chance to block projectile".to_string(), amounts: vec![]},
				physical_pen: Percentage_Stat{text: "armor ignored by physical damage".to_string(), amounts: vec![]},
				magical_pen: Percentage_Stat{text: "resist ignored by magical damage".to_string(), amounts: vec![]},
				damage_reduction: Percentage_Stat{text: "damage reduction".to_string(), amounts: vec![]},
			};
			let third = Parameter_Flags{
				combo: Parameter_Flag{text: "combo".to_string(), flag: false, mod_indexs: vec![]},
			};
			let fourth = Parameter_Percents{
				remaining_hp: Parameter_Percent{text: "HP percent".to_string, amount: "100", mod_indexs: vec![]},
				remaining_mp: Parameter_Percent{text: "MP percent".to_string, amount: "100", mod_indexs: vec![]},
			};
			let fifth = Parameter_Inserts{
				gold: Parameter_Insert{text: "gold amount".to_string, amount: 0, cap: 9999999, mod_indexs: vec![]},//Not sure if that cap will register.
				item_count: Parameter_Insert{text: "number of items".to_string, amount: 0, cap: 10000, mod_indexs: vec![]},
			};
			let sixth = Actives{
				primary_attack: Active{name: "Primary Attack".to_string(), attacks: vec![], effects: vec![]},
				attacked: Active{name: "attacked".to_string(), attacks: vec![], effects: vec![]},
				potion: Active{name: "potion use".to_string(), attacks: vec![], effects: vec![], texts: vec![]},
				defeat_enemy: Active{name: "defeated an enemy".to_string(), attacks: vec![], effects: vec![]},
			};
			return Character{
				stats: first,
				percent_stats: second,
				passive_texts: vec![],
				flags: third,
				percents: fourth,
				inserts: fifth,
				actives: sixth,
				flexible: vec![],
				modify: vec![],
				openslots_m: vec![],
			};
		}
		pub fn change_flat(mut character: &mut Character, mut field: &mut Stat, amount: f64, yes: bool, layers: u16){
			for x in 0..field.mod_indexs.len(){
				run_modification(&mut character, false, field.mod_indexs[x]);
			}
			if yes == true{
				field.amount+=amount*layers;
			}
			else{
				field.amount-=amount*layers;
			}
			for x in 0..field.mod_indexs.len(){
				run_modification(&mut character, true, field.mod_indexs[x]);
			}
		}
		pub fn change_percentage(mut fields: &mut Percentage_Stat, amount: f64, yes: bool, mut layers: u16){
			let mut target = &mut fields.amount;
			if yes  == true{
				target.resize(target.len()+layers, amount);
			}
			else{
				for x in 0..target.len(){
					if target[x] == amount{
						target.remove(x);
						layers-=1;
						if layers == 0{
							break;
						}
					}
				}
			}
		}
		pub fn change_texts(mut targets: &mut Texts, target: String, yes: bool, mod_index: u16){
			let x = 0;
			while x < targets.mod_indexs.len(){
				if targets.mod_indexs[x] == mod_index{
					break;
				}
				x++;
			}
			if yes == true{
				if x == targets.len(){
					targets.resize.fields(targets.len()+1, target);
					targets.resize.mod_indexs(targets.len()+1, target);
				}
				else{
					targets.fields[x]=target;
				}
			}
			else{
				targets.fields.remove(x);
				target.mod_indexs.remove(x);
			}
		}
		pub fn change_parameter_flag(){
			
		}
		pub fn change_parameter_percent(){
			
		}
		pub fn change_parameter_insert(){
			
		}
		pub fn add_modification(mut character: &mut Character, this: Modification) -> u16{//
			let y: u16;
			if character.openslots_m.len()>0{//If a slot is open fill it.
				y=character.openslots_m.pop();
				character.modifications[y]=this;
			}
			else{
				y=character.modifications.len()+1;
				character.modifications.resize(y, this);
			}
			return y;
		}
		pub fn remove_modification(mut character: &mut Character, target: u16){//
			let y: u16;
			//remove from parameters.
			//Create a new slot to fill.  Do not need to remove the information as long as parameters gone.
			let mut y=character.openslots_m.len();
			character.openslots_m.resize(y+1, target);
			loop{//Moves the new slot around so that they are from highest to least.
				y-=1;
				if y<0{
					break;
				}
				if character.openslots_m[y]>target{
					break;
				}
				character.openslots_m[y+1]=character.openslots_m[y];
				character.openslots_m[y]=target;
			}
		}
		pub fn change_parameter(mut target: &mut Vec<u16>, mod_index: u16, yes: bool){
			if yes == true{//add a new one.
				target.resize(target.len()+1, mod_index);
			}
			else{//remove an old one.
				for x in 0..target.len(){
					if target[x] == mod_index{
						target.remove(x);
						break;
					}
				}
			}
		}
		pub fn recieve_parameter(mut character: &mut Character, thing: &str) -> &mut Vec<u16>{
			let mut result: Vec<u16>;
			match thing{
				"attack" => result = character.stats.attack.mod_indexs,
				"skill" => result = character.stats.skill.mod_indexs,
				"armor" => result = character.stats.armor.mod_indexs,
				"resist" => result = character.stats.resist.mod_indexs,
				"base health" => result = character.stats.base_health.mod_indexs,
				"base mana" => result = character.stats.base_mana.mod_indexs,
				"health" => result = character.stats.health.mod_indexs,
				"mana" => result = character.stats.mana.mod_indexs,
				"hp regen" => result = character.stats.hp_regen.mod_indexs,
				"mp regen" => result = character.stats.mp_regen.mod_indexs,
				"attack p damage" => result = character.stats.attack_p_damage.mod_indexs,
				"attack m damage" => result = character.stats.attack_m_damage.mod_indexs,
				"attack speed" => result = character.stats.attack_speed.mod_indexs,
				"movement speed" => result = character.stats.movement_speed.mod_indexs,
				"luck" => result = character.stats.luck.mod_indexs,
				"gold percent" => result = character.stats.gold_percent.mod_indexs,
				"exp" => result = character.stats.exp.mod_indexs,
				"exp a" => result = character.stats.exp_a.mod_indexs,
				"exp b" => result = character.stats.exp_b.mod_indexs,
				"exp c" => result = character.stats.exp_c.mod_indexs,
				"exp d" => result = character.stats.exp_d.mod_indexs,
				"damage" => result = character.stats.damage.mod_indexs,
				"damage a" => result = character.stats.damage_a.mod_indexs,
				"damage b" => result = character.stats.damage_b.mod_indexs,
				"damage c" => result = character.stats.damage_c.mod_indexs,
				"damage d" => result = character.stats.damage_d.mod_indexs,
				"critical a damage" => result = character.stats.critical_a_damage.mod_indexs,
				"critical s damage" => result = character.stats.critical_s_damage.mod_indexs,
				"lifesteal" => result = character.stats.lifesteal.mod_indexs,
				"return p damage" => result = character.stats.return_p_damage.mod_indexs,
				"p collide" => result = character.stats.p_collide.mod_indexs,
				"combo" => result = character.flags.combo.mod_indexs,
				"remaining hp" => result = character.percents.remaining_hp.mod_indexs,
				"remaining mp" => result = character.percents.remaining_mp.mod_indexs,
				"gold" => result = character.inserts.gold.mod_indexs,
				"item count" => result = character.inserts.item_count.mod_indexs,
			}
			return &mut result;
		}
		pub fn test_questions(character: Character, test: Questions) -> bool{
			match test{
				Questions::NoCondition => return true,
				Questions::Condition{condition, dependancies} => return condition(character),
			}
		}
		pub fn modify_layers(mut character: &mut Character, mod_index: u16, layers: u16){
			run_modification(&mut character, 0, mod_index);
			character.modifications[mod_index].layers = layers;
			run_modification(&mut character, 1, mod_index);
		}
		pub fn run_modification(mut character: &mut Character, yes: bool, mod_index: u16){
			//Consider a way to increment this every time it is ran to stop infinite loops.
			let mut target = &mut character.modifications[mod_index];
			fn running(mut character: &mut Character, yes: bool, mod_index: u16){
				let mut target = &mut character.modifications[mod_index];
				match target.changes{
					Modify::YesAll(a) => a(character, yes, target.layers, mod_index),
					Modify::NoLayers(b) => b(character, yes, mod_index),
					Modify::NoIndex(c) => c(character, yes, target.layers),
					Modify::YesAll(d) => d(character, yes),
				}
			}
			if yes == true && test_questions(character, target.questions) == true && target.activated == false{
				running(character, true, mod_index);
			}
			else if yes == false && target.activated == true{
				running(character, false, mod_index);
				target.activated = false;
			}
		}
}
