pub mod one items{
	pub type Chosen_Item = fn(&mut Character, bool);
	pub enum Attuneable{
		Possible(bool),
		Not,
	}
	pub struct Item{
		pub name: String,
		pub mod_indexs: Vec<u16>,
		pub attune: Attuneable,
		pub rarity: u8,
	}
	//first is list of modifications
	pub fn pick_item(){
		fn amulet_of_resistance(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.resist, 5, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Amulet of Resistance".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn apothecary_herbs(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, index: u16){
				//buff
			}
			let these = Modify::NoLayers(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Apothecary's Herbs".to_string();
			let tune = Attuneable::Not;
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn blackjack(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16, index: u16){
				string = "stun for 2 seconds".to_string();
				effecter = Effect{text: string, chance: 10, layers: layers};
				change_effects(&mut character.primary_attack.effects, effecter, yes, mod_index);
			}
			let these = Modify::YesAll(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Blackjack".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn book_of_enlightenment(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.exp, 20, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Book of Enlightenment".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn broad_sword(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.attack, 10, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Broad Sword".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn cape_of_withdrawal(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Cape of Withdrawal".to_string();
			let tune = Attuneable::Not;
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn chainmail(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.armor, 5, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Chainmail".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn circlet_of_willpower(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_percentage(character.percent_stats.mana_cost, 75, yes, layers);
			}
			let these = Modify::NoIndex(this);
			fn that(character: Character) -> bool{
				if character.percents.remaining_hp.amount<50{
					return true;
				}
				else{
					return false;
				}
			}
			let modifies = Modification{
				conditions: Questions::Condition
					{condition: that, dependancies: vec!["current hp"]},
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Circlet of Willpower".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn coated_arrow(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16, index: u16){
				string = "poison for 4 seconds".to_string();
				effecter = Effect{text: string, chance: 20, layers: layers};
				change_effects(&mut character.primary_attack.effects, effecter, yes, mod_index);
			}
			let these = Modify::YesAll(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Coated Arrow".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn cowl_of_protection(mut character: &mut Character) -> Item{//?
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Cowl of Protection".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn curio_box(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.health, 5, yes, layers);
				change_flat(character.stats.mana, 5, yes, layers);
				change_flat(character.stats.attack, 3, yes, layers);
				change_flat(character.stats.skill, 3, yes, layers);
				change_flat(character.stats.armor, 2, yes, layers);
				change_flat(character.stats.resist, 2, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Curio's Box".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn defender_halberd(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, index: u16){
				string = "disarm for 2 seconds".to_string();
				effecter = Effect{text: string, chance: 100, layers: 1};
				change_effects(&mut character.attacked.effects, effecter, yes, mod_index);
			}
			let these = Modify::NoLayers(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Defender's Halberd".to_string();
			let tune = Attuneable::Not;
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn enchanted_dirk(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.attack_m_damage, 1, yes, layers);
				change_flat(character.stats.attack_p_damage, 1, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Enchanted Dirk".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn firm_buckler(mut character: &mut Character) -> Item{//?
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Firm Buckler".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn gladiator_net(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16, index: u16){
				string = "disarm for 3 seconds".to_string();
				effecter = Effect{text: string, chance: 10, layers: layers};
				change_effects(&mut character.primary_attack.effects, effecter, yes, mod_index);
			}
			let these = Modify::YesALl(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Gladiator's Net".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn glowing_staff(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.skill, 5, yes, layers);
				change_flat(character.stats.attack_m_damage, 1, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Glowing Staff".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn great_helm(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.armor, 7, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Great Helm".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn greaves_of_the_barbarian(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Greaves of the Barbarian".to_string();
			let tune = Attuneable::Not;
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn heavy_gauntlents(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.health, 15.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Heavy Gauntlents".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn hunter_knife(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.damage_b, 20.0, yes, layers);
			}
			fn thistwo(mut character: &mut Character, yes: bool, index: u16){
				string = ;
				effecter = Effect{text: string, chance: 10, layers: layers};
				change_effects(&mut character.primary_attack.effects, effecter, yes, mod_index);
			}
			let these = Modify::NoIndex(this);
			let thesetwo = Modify::NoLayers(thistwo);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Hunter's Knife".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn left_boot_of_speed(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.movement_speed, 0.2, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Left Boot of Speed".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn lesser_sphere_of_life(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.hp_regen, 0.75, yes, layers);
			}
			let these = Modify::NoIndex(this);
			fn that(character: Character) -> bool{
				return character.flags.combo.flag;
			}
			let modifies = Modification{
				conditions: Questions::Condition
					{condition: that, dependancies: vec!["combo".to_string()]},
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Lesser Sphere of Life".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn lesser_sphere_of_mana(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.mp_regen, 1.5, yes, layers);
			}
			let these = Modify::NoIndex(this);
			fn that(character: Character) -> bool{
				return character.flags.combo.flag;
			}
			let modifies = Modification{
				conditions: Questions::Condition
					{condition: that, dependancies: vec!["combo".to_string()]},
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Lesser Sphere of Mana".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn lesser_sphere_of_time(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			fn that(character: Character) -> bool{
				return character.flags.combo.flag;
			}
			let modifies = Modification{
				conditions: Questions::Condition
					{condition: that, dependancies: vec!["combo".to_string()]},
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Lesser Sphere of Time".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn life_stone(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16, index: u16){
				string = Text{text: "15% chance to gain 1 Health".to_string(), numbers: vec![], layers: layers};
				change_texts(character.defeat_enemy.texts, string, yes, mod_index);
			}
			let these = Modify::YesAll(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Life Stone".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn luck_charm(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.luck, 1.0, yes, layers);
				change_flat(character.stats.gold_percent, 10.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Lcuky Charm".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn mage_robe(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.mana, 20.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Mage Robe".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn mail_of_thorns(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.armor, 2.0, yes, layers);
				change_flat(character.stats.return_p_damage, 15.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Mail of Thorns".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn mana_stone(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16, index: u16){
				string = Text{text: "30% chance to gain 1 Mana".to_string(), numbers: vec![], layers: layers};
				change_texts(character.defeat_enemy.texts, string, yes, mod_index);
			}
			let these = Modify::YesAll(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Mana Stone".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn markham_purse(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let challenges = recieve_parameter(character, "item count");
			let modifies = Modification{
				conditions: Questions::DependsOnly{dependancies: challenges},
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Markham's Purse".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn monster_pamphlet_a(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.exp_a, 10.0, yes, layers);
				change_flat(character.stats.damage_a, 10.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Monster Pamphlet: Aberration".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn monster_pamphlet_b(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.exp_b, 10.0, yes, layers);
				change_flat(character.stats.damage_b, 10.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Monster Pamphlet: Beast".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn monster_pamphlet_c(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.exp_c, 10.0, yes, layers);
				change_flat(character.stats.damage_c, 10.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Monster Pamphlet: Construct".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn monster_pamphlet_d(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.exp_d, 10.0, yes, layers);
				change_flat(character.stats.damage_d, 10.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Monster Pamphlet: Undead".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn papers_of_nobility(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_percentage(character.percent_stats.tax_reduction, 25, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Papers of Nobility".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn pot_helm(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_percentage(character.percent_stats.block_projectile, 10, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Pot Helm".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn right_boot_of_speed(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.movement_speed, 0.2, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Right Boot of Speed".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn ring_of_health(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.hp_regen, 0.2, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Ring of Health".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn ring_of_mana(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.mp_regen, 0.4, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Ring of Mana".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn ring_of_rejuvenation(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.hp_regen, 0.1, yes, layers);
				change_flat(character.stats.mp_regen, 0.2, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Ring of Rejuvenation".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn sandals_of_swiftness(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.movement_speed, 0.1, yes, layers);
				change_percentage(character.percent_stats.evasion, 3.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Sandals of Swiftness".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn scroll_of_magic_missle_one(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Scroll of Magic Missle: Page 1".to_string();
			let tune = Attuneable::Not;
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn scroll_of_magic_missle_two(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Scroll of Magic Missle: Page 2".to_string();
			let tune = Attuneable::Not;
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn serrated_scimitar(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_percentage(character.percent_stats.a_crit_chance, 2.5, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Serrated Scimitar".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn slippery_cloak(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_percentage(character.percent_stats.evasion, 6, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Slippery Cloak".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn spell_book(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.skill, 10.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Spell Book".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn spiked_flail(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.attack_p_damage, 3.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Spiked Flail".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn steady_greaves(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.armor, 3.0, yes, layers);
				change_flat(character.stats.health, 10.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Steady Greaves".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn stiletto(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_percentage(character.percent_stats.physical_pen, 10.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Stiletto".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn sturdy_belt(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.health, 15.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Sturdy Belt".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn vendor_coin(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Vendor's Coin".to_string();
			let tune = Attuneable::Not;
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn wizard_wand(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.attack_m_damage, 2.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Wizard's Wand".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 0;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn adventurer_garb(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.health, 25.0, yes, layers);
				change_flat(character.stats.mana, 30.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Adventurer's Garb".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn apothecary_flask(mut character: &mut Character) -> Item{//Handle this
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Apothecary's Flask".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn apothecary_mortar_and_pestle(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Apothecary's Mortar and Prestle".to_string();
			let tune = Attuneable::Not;
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn apothecary_satchel(mut character: &mut Character) -> Item{//Handle this
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Apothecary's Satchel".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn apothecary_sphere(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Apothecary's Sphere".to_string();
			let tune = Attuneable::Not;
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn assassin_dagger(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_percentage(character.percent_stats.a_crit_chance, 7.5, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Assassin's Dagger".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn blood_dagger(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Blood Dagger".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn blowgun(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Blowgun".to_string();
			let tune = Attuneable::Not;
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn bomb_pouch(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.skill, 10.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			//bomb
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Bomb Pouch".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn bracers_of_quickness(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.attack_speed, 10.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Bracers of Quickness".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn claymore(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.attack, 10.0, yes, layers);
				change_flat(character.stats.attack_p_damage, 5.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Claymore".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn cloak_of_many_pockets(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.gold_percent, 50.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Cloak of Many Pockets".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn curio_case(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.health, 10.0, yes, layers);
				change_flat(character.stats.mana, 10.0, yes, layers);
				change_flat(character.stats.attack, 6.0, yes, layers);
				change_flat(character.stats.skill, 6.0, yes, layers);
				change_flat(character.stats.armor, 4.0, yes, layers);
				change_flat(character.stats.resist, 4.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Curio's Case".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn elven_bow(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.attack, 20.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Elven Bow".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn elven_ruby(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Elven Ruby".to_string();
			let tune = Attuneable::Not;
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn essence_collector(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.damage_d, 20.0, yes, layers);
				//
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Essence Collector".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn fancy_plume(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Fancy Plume".to_string();
			let tune = Attuneable::Not;
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn full_plate_mail(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.armor, 15.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Full Plate Mail".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn gloves_of_warding(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Gloves of Warding".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn greater_ring_of_rejuvenation(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.hp_regen, 0.5, yes, layers);
				change_flat(character.stats.mp_regen, 1.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Greater Ring of Rejuvenation".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn hero_unknown_frying_pan(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Hero Unknown's Frying Pan".to_string();
			let tune = Attuneable::Not;
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn key_ring(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Key Ring".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn lucky_horseshoe(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.luck, 2.0, yes, layers);
				change_flat(character.stats.movement_speed, 0.1, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Lucky Horseshoe".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn markham_stone(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Markham's Stone".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn monster_manual_a(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.damage_a, 20.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Monster Manual: Aberration".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn monster_manual_b(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.damage_b, 20.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Monster Manual: Beast".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn monster_manual_c(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.damage_c, 20.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Monster Manual: Construct".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn monster_manual_d(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.damage_d, 20.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Monster Manual: Undead".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn necklace_of_prowess(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Necklace of Prowess".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn old_map(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Old Map".to_string();
			let tune = Attuneable::Not;
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn pendant_of_penance(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Pendant of Penance".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn rapier_of_retaliation(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_percentage(character.percent_stats.evasion, 5.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			//another effect
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Rapier of Retaliation".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn reinforced_gloves(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Reinforced Gloves".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn ring_of_transmutation(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Ring of Transmutation".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn robe_of_the_archmagi(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.skill, 10.0, yes, layers);
				change_flat(character.stats.resist, 10.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Robe of the Arch-Magi".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn scarab_of_protection(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Scarab of Protection".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn seal_of_souls(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Seal of Souls".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn seal_of_the_martyr(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Seal of the Martyr".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn shaftlocke_pickaxe(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.ore_exp, 50.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Shaftlocke Pickaxe".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn sphere_of_life(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.hp_regen, 1.5, yes, layers);
			}
			let these = Modify::NoIndex(this);
			fn that(character: Character) -> bool{
				return character.flags.combo.flag;
			}
			let modifies = Modification{
				conditions: Questions::,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Sphere of Life".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn sphere_of_mana(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.mp_regen, 3.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			fn that(character: Character) -> bool{
				return character.flags.combo.flag;
			}
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Sphere of Mana".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn sphere_of_time(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			fn that(character: Character) -> bool{
				return character.flags.combo.flag;
			}
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Sphere of Time".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn sphere_of_warriors(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			fn that(character: Character) -> bool{
				return character.flags.combo.flag;
			}
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Sphere of Warriors".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn spiked_boots(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.p_collide, 50.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			//
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Spiked Boots".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn staff_of_unstable_casting(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_percentage(character.percent_stats.s_crit_chance, 5.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Staff of Unstable Casting".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn stinger(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_percentage(character.percent_stats.physical_pen, 20.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Stinger".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn symbol_of_zeal(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Symbol of Zeal".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn talisman_of_decay(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Talisman of Decay".to_string();
			let tune = Attuneable::Not;
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn tome_of_magic_missle(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.skill, 10.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Tome of Magic Missle".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn tower_shield(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Tower Shield".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn wand_of_spell_piercing(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_percentage(character.percent_stats.magical_pen, 15.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Wand of Spell Piercing".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 1;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn amulet_of_kings(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.resist, 30.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Amulet of Kings".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 2;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn amulet_of_vengeance(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Amulet of Vengeance".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 2;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn armor_of_kings(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.armor, 30.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Armor of Kings".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 2;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn bloodthirst_ring(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.lifesteal, 2.5, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Bloodthirst Ring".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 2;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn boots_of_the_giants(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.health, 50.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			//
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Boots of the Giant".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 2;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn cape_of_the_flamewalker(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Cape of the Flamewalker".to_string();
			let tune = Attuneable::Not;
			let rare = 2;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn chakram(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Chakram".to_string();
			let tune = Attuneable::Not;
			let rare = 2;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn curio_coffer(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.health, 20.0, yes, layers);
				change_flat(character.stats.mana, 20.0, yes, layers);
				change_flat(character.stats.attack, 12.0, yes, layers);
				change_flat(character.stats.skill, 12.0, yes, layers);
				change_flat(character.stats.armor, 8.0, yes, layers);
				change_flat(character.stats.resist, 8.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Curio's Coffer".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 2;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn duelist_edge(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_percentage(character.percent_stats.evasion, 8.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			//
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Duelist's Edge".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 2;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn duke_signet_ring(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Duke's Signet Ring".to_string();
			let tune = Attuneable::Not;
			let rare = 2;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn dwarven_pickaxe(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.damage_c, 100.0, yes, layers);
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Dwarven Pickaxe".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 2;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn earthsplitter(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Earthsplitter".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 2;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn flametongue(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.attack_p_damage, 5.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			//
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Flametongue".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 2;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn frostband(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.skill, 5.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			//
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Frostband".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 2;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn golden_lamp(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Golden Lamp".to_string();
			let tune = Attuneable::Not;
			let rare = 2;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn greater_sphere_of_regen(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			fn that(character: Character) -> bool{
				return true;
			}
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Greater Sphere of Regen".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 2;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn guardian_figurine(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Guardian Figurine".to_string();
			let tune = Attuneable::Not;
			let rare = 2;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn heartseeker(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_percentage(character.percent_stats.a_crit_chance, 12.5, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Heartseeker".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 2;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn judgement(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.attack_p_damage, 12.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			//
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Judgement".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 2;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn lucky_hat(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.luck, 4.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Lucky Hat".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 2;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn magebane(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Magebane".to_string();
			let tune = Attuneable::Not;
			let rare = 2;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn markham_amulet(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Markham's Amulet".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 2;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn markham_mace(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Markham's Mace".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 2;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn markham_wand(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Markham's Wand".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 2;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn scepter_of_kings(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.skill, 40.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Scepter of Kings".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 2;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn seal_of_rage(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Seal of Rage".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 2;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn shieldbreaker(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Shieldbreaker".to_string();
			let tune = Attuneable::Not;
			let rare = 2;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn skullsmasher(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Skullsmasher".to_string();
			let tune = Attuneable::Not;
			let rare = 2;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn spreading_corruption(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Spreading Corruption".to_string();
			let tune = Attuneable::Not;
			let rare = 2;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn staff_of_volatile_casting(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_percentage(character.percent_stats.s_crit_chance, 10.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Staff of Volatile Casting".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 2;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn stormcaller(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Stormcaller".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 2;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn sword_of_kings(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.attack, 40.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Sword of Kings".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 2;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn talisman_of_conflagration(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Talisman of Conflagration".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 2;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn waning_crescent(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.attack, 25.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Waning Crescent".to_string();
			let tune = Attuneable::Not;
			let rare = 2;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn waxing_crescent(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.skill, 25.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Waxing Crescent".to_string();
			let tune = Attuneable::Not;
			let rare = 2;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn wrath_of_the_thundergod(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Wrath of the Thundergod".to_string();
			let tune = Attuneable::Possible(false);
			let rare = 2;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn crown_of_kings(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Crown of Kings".to_string();
			let tune = Attuneable::Not;
			let rare = 3;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn dragonscale_mail(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_percentage(character.percent_stats.damage_reduction, 25.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Dragonscale Mail".to_string();
			let tune = Attuneable::Not;
			let rare = 3;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn phoenix_feather(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Phoenix Feather".to_string();
			let tune = Attuneable::Not;
			let rare = 3;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn mask_of_bewilderment(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Mask of Bewilderment".to_string();
			let tune = Attuneable::Not;
			let rare = 3;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn lantern_of_light(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Lantern of Light".to_string();
			let tune = Attuneable::Not;
			let rare = 3;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn ring_of_arcane_powers(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_percentage(character.percent_stats.mana_cost, 75.0, yes, layers);
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Ring of Arcane Powers".to_string();
			let tune = Attuneable::Not;
			let rare = 3;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn searing_sabatons(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Searing Sabatons".to_string();
			let tune = Attuneable::Not;
			let rare = 3;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn sphere_of_champions(mut character: &mut Character) -> Item{
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				change_flat(character.stats.attack, 60.0, yes, layers);
				change_flat(character.stats.skill, 60.0, yes, layers);
			}
			let these = Modify::NoIndex(this);
			fn that(character: Character) -> bool{
				return character.flags.combo.flag;
			}
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Sphere of Champions".to_string();
			let tune = Attuneable::Not;
			let rare = 3;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn wylmir_wand(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Wylmir's Wand".to_string();
			let tune = Attuneable::Not;
			let rare = 3;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn boots_of_freedom(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Boots of Freedom".to_string();
			let tune = Attuneable::Not;
			let rare = 3;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn descending_destruction(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Descending Destruction".to_string();
			let tune = Attuneable::Not;
			let rare = 4;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn dragon_tail(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Dragon's Tail".to_string();
			let tune = Attuneable::Not;
			let rare = 4;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn spirit_cloak(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Spirit Cloak".to_string();
			let tune = Attuneable::Not;
			let rare = 4;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn medusa_charm(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Medusa's Charm".to_string();
			let tune = Attuneable::Not;
			let rare = 4;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn soul_taker(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Soul Taker".to_string();
			let tune = Attuneable::Not;
			let rare = 4;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn sphere_of_heroes(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			fn that(character: Character) -> bool{
				return true;
			}
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Sphere of Heroes".to_string();
			let tune = Attuneable::Not;
			let rare = 4;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
		fn wand_of_chaos(mut character: &mut Character) -> Item{//
			fn this(mut character: &mut Character, yes: bool, layers: u16){
				
			}
			let these = Modify::NoIndex(this);
			let modifies = Modification{
				conditions: Questions::NoCondition,
				changes: these,
				activated: 0,
			};
			let one = add_modification(&mut character, modifies);
			let noun = "Wand of Chaos".to_string();
			let tune = Attuneable::Not;
			let rare = 4;
			return Item{name: noun, mod_indexs: vec![one], attune: tune, rarity: rare};
		}
	}
}