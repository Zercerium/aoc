use std::{cmp::max, fs};

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Couldnt read Input File");

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

#[derive(PartialEq)]
enum ItemType {
    Weapon,
    Armor,
    Ring,
}

struct Item {
    #[allow(dead_code)]
    itype: ItemType,
    name: String,
    cost: u8,
    damage: u8,
    armor: u8,
}

impl Item {
    fn new(itype: ItemType, name: &str, cost: u8, damage: u8, armor: u8) -> Self {
        Item {
            itype,
            name: String::from(name),
            cost,
            damage,
            armor,
        }
    }
}

fn setup_items() -> Items {
    vec![
        Item::new(ItemType::Weapon, "Dagger", 8, 4, 0),
        Item::new(ItemType::Weapon, "Shortsword", 10, 5, 0),
        Item::new(ItemType::Weapon, "Warhammer", 25, 6, 0),
        Item::new(ItemType::Weapon, "Longsword", 40, 7, 0),
        Item::new(ItemType::Weapon, "Greataxe", 74, 8, 0),
        Item::new(ItemType::Armor, "Leather", 13, 0, 1),
        Item::new(ItemType::Armor, "Chainmail", 31, 0, 2),
        Item::new(ItemType::Armor, "Splintmail", 53, 0, 3),
        Item::new(ItemType::Armor, "Bandedmail", 75, 0, 4),
        Item::new(ItemType::Armor, "Platemail", 102, 0, 5),
        Item::new(ItemType::Ring, "Defense +1", 20, 0, 1),
        Item::new(ItemType::Ring, "Damage +1", 25, 1, 0),
        Item::new(ItemType::Ring, "Defense +2", 40, 0, 2),
        Item::new(ItemType::Ring, "Damage +2", 50, 2, 0),
        Item::new(ItemType::Ring, "Defense +3", 80, 0, 3),
        Item::new(ItemType::Ring, "Damage +3", 100, 3, 0),
    ]
}

struct Player {
    name: String,
    hit_points: u8,
    damage: u8,
    armor: u8,
    spent_gold: u16,
    items: Vec<String>,
}

impl Player {
    fn add_item(&mut self, item: &Item) {
        self.armor += item.armor;
        self.damage += item.damage;
        self.spent_gold += item.cost as u16;
        self.items.push(item.name.to_owned());
    }

    fn remove_item(&mut self, item: &Item) {
        self.armor -= item.armor;
        self.damage -= item.damage;
        self.spent_gold -= item.cost as u16;
        let pos = self.items.iter().position(|s| *s == item.name).unwrap();
        self.items.remove(pos);
    }

    #[allow(dead_code)]
    fn attack(&self, opponent: &mut Player) {
        opponent.hit_points = opponent
            .hit_points
            .saturating_sub(self.get_damage(opponent));
    }

    fn get_damage(&self, opponent: &Player) -> u8 {
        max(self.damage.saturating_sub(opponent.armor), 1)
    }

    fn rounds_to_finish(&self, opponent: &Player) -> u8 {
        let dmg_per_rnd = self.get_damage(opponent);
        let mut rnds = opponent.hit_points / dmg_per_rnd;
        if opponent.hit_points - dmg_per_rnd * rnds > 0 {
            rnds += 1;
        }
        rnds
    }

    fn wins(&self, opponent: &Player) -> bool {
        let p_rnds = self.rounds_to_finish(&opponent);
        let m_rnds = opponent.rounds_to_finish(&self);

        p_rnds <= m_rnds
    }
}

#[allow(dead_code)]
fn one_round(player: &mut Player, monster: &mut Player) -> Option<String> {
    player.attack(monster);
    if monster.hit_points <= 0 {
        return Some(player.name.to_owned());
    }
    monster.attack(player);
    if player.hit_points <= 0 {
        return Some(monster.name.to_owned());
    }
    None
}

fn parse_monster(i: &str) -> Player {
    let mut i = i.lines();
    let hit_points = i.next().unwrap().split(": ").collect::<Vec<_>>()[1]
        .parse::<u8>()
        .expect("hit_points error");
    let damage = i.next().unwrap().split(": ").collect::<Vec<_>>()[1]
        .parse::<u8>()
        .expect("damage error");
    let armor = i.next().unwrap().split(": ").collect::<Vec<_>>()[1]
        .parse::<u8>()
        .expect("armor error");

    Player {
        name: String::from("Monster"),
        hit_points,
        damage,
        armor,
        spent_gold: 0,
        items: Vec::new(),
    }
}
type Items = Vec<Item>;

fn combinations<T: Clone>(
    num: &Vec<T>,
    ot: &mut Vec<T>,
    o: &mut Vec<Vec<T>>,
    s: u8,
    e: u8,
    index: u8,
    r: u8,
) {
    if index == r {
        o.push(ot.to_vec());
        return;
    }

    let mut i = s;
    while i <= e && e - i + 1 >= r - index {
        let mut ot = ot.clone();
        ot.push(num[i as usize].clone());
        combinations(num, &mut ot, o, i + 1, e, index + 1, r);
        i += 1;
    }
}

fn create_all_permutation() -> Vec<Vec<u8>> {
    // single weapon
    let wi = (0..=4).collect::<Vec<u8>>();
    let mut ot = Vec::new();
    let mut w = Vec::new();
    let len = wi.len() as u8 - 1;
    combinations(&wi, &mut ot, &mut w, 0, len, 0, 1);

    // single or none armor
    let mut ai = (5..=9).collect::<Vec<u8>>();
    ai.push(254);
    let mut a = Vec::new();
    let len = ai.len() as u8 - 1;
    combinations(&ai, &mut ot, &mut a, 0, len, 0, 1);

    // 0 - 2 rings
    let mut ri = (10..=15).collect::<Vec<u8>>();
    ri.push(254);
    ri.push(254);
    let mut r = Vec::new();
    let len = ri.len() as u8 - 1;
    combinations(&ri, &mut ot, &mut r, 0, len, 0, 2);

    let mut all = Vec::new();

    for w in &w {
        let mut temp = Vec::new();
        temp.push(w.clone());
        for a in &a {
            let mut temp1 = temp.clone();
            temp1.push(a.clone());
            for r in &r {
                let mut temp2 = temp1.clone();
                temp2.push(r.clone());
                all.push(temp2.into_iter().flatten().collect::<Vec<u8>>());
            }
        }
    }

    all
}

fn part_one(input: &String) -> u16 {
    let monster = parse_monster(&input);
    let mut player = Player {
        name: String::from("Player"),
        hit_points: 100,
        damage: 0,
        armor: 0,
        spent_gold: 0,
        items: Vec::new(),
    };
    let items = setup_items();

    let comb = create_all_permutation();
    let mut comb1 = Vec::new();
    for v in &comb {
        comb1.push(
            v.iter()
                .filter(|&&n| n != 254)
                .map(|n| *n)
                .collect::<Vec<_>>(),
        );
    }

    comb1.sort_by(|a, b| {
        let a: u16 = a.iter().map(|i| items[*i as usize].cost as u16).sum();
        let b: u16 = b.iter().map(|i| items[*i as usize].cost as u16).sum();
        a.cmp(&b)
    });

    let mut won = false;
    for c in comb1 {
        for i in &c {
            player.add_item(&items[*i as usize]);
        }

        if player.wins(&monster) {
            won = true;
            break;
        }

        for i in &c {
            player.remove_item(&items[*i as usize]);
        }
    }
    assert!(won, "Monster is too mighty");
    println!("Items: {:#?}", player.items);
    player.spent_gold
}

fn part_two(input: &String) -> u16 {
    let monster = parse_monster(&input);
    let mut player = Player {
        name: String::from("Player"),
        hit_points: 100,
        damage: 0,
        armor: 0,
        spent_gold: 0,
        items: Vec::new(),
    };
    let items = setup_items();

    let comb = create_all_permutation();
    let mut comb1 = Vec::new();
    for v in &comb {
        comb1.push(
            v.iter()
                .filter(|&&n| n != 254)
                .map(|n| *n)
                .collect::<Vec<_>>(),
        );
    }

    comb1.sort_by(|a, b| {
        let a: u16 = a.iter().map(|i| items[*i as usize].cost as u16).sum();
        let b: u16 = b.iter().map(|i| items[*i as usize].cost as u16).sum();
        b.cmp(&a)
    });

    let mut won = true;
    for c in comb1 {
        for i in &c {
            player.add_item(&items[*i as usize]);
        }

        if !player.wins(&monster) {
            won = false;
            break;
        }

        for i in &c {
            player.remove_item(&items[*i as usize]);
        }
    }
    assert!(!won, "Monster is too weak");
    println!("Items: {:#?}", player.items);
    player.spent_gold
}
