use std::fmt::Display;

#[allow(clippy::many_single_char_names)]
#[must_use]
pub fn format_message(
    category_id: u32,
    instance_id: u32,
    arguments: Vec<Box<dyn Display>>,
) -> String {
    match (category_id, instance_id) {
        (504, 184_651_438) => {
            String::from("No profession chosen yet. Can't show skillsystem.")
        }
        (504, 173_173_381) => {
            String::from("You must choose a side before you can use the skillsystem.")
        }
        (504, 144_673_134) => {
            String::from("Auto distribution not supported in this version.")
        }
        (10003, 222_302_916) => {
            String::from("Head")
        }
        (10003, 157_193_108) => {
            String::from("No item selected")
        }
        (10003, 222_312_051) => {
            String::from("Eyes")
        }
        (10003, 162_037_156) => {
            String::from("Count")
        }
        (10003, 214_317_092) => {
            String::from("It is not possible to add items to this inventory.")
        }
        (10003, 165_255_269) => {
            String::from("Price")
        }
        (10003, 240_531_619) => {
            String::from("You can't sell containers in your shop.")
        }
        (10003, 222_314_787) => {
            String::from("Ears")
        }
        (10003, 210_068_885) => {
            String::from("Not equippable")
        }
        (10003, 131_011_846) => {
            let a = &arguments[0];
            format!("Unknown playfield {a}")
        }
        (10003, 255_984_867) => {
            let a = &arguments[0];
            format!("Total: {a} credits")
        }
        (10003, 261_859_141) => {
            String::from("Name")
        }
        (10003, 261_757_230) => {
            String::from("Icon")
        }
        (10003, 72_714_596) => {
            String::from("Shop name is too short.")
        }
        (10003, 70_058_900) => {
            String::from("Are you sure you want to reset ownership of this shop?")
        }
        (10003, 3_329_365) => {
            String::from("Set price")
        }
        (10003, 154_302_101) => {
            String::from("Enter the new price for this item.")
        }
        (10003, 241_217_684) => {
            String::from("Spirit")
        }
        (10003, 141_512_835) => {
            String::from("Are you sure you want to transfer credits from this shop?")
        }
        (10003, 9_640_357) => {
            String::from("Enter a new name for the shop terminal.")
        }
        (10003, 237_118_158) => {
            String::from("Weapon")
        }
        (10003, 222_306_307) => {
            String::from("HUDs")
        }
        (10003, 222_311_028) => {
            String::from("Feet")
        }
        (10003, 58_152_855) => {
            String::from("Waiting for query result...")
        }
        (10003, 111_464_965) => {
            String::from("The shop name is invalid. Please select a different name.")
        }
        (10003, 58_656_915) => {
            String::from("Fingers")
        }
        (10003, 192_968_004) => {
            String::from("Shop name is too long. The name was truncated.")
        }
        (10003, 47_318_724) => {
            String::from("Right wrist")
        }
        (10003, 225_860_115) => {
            String::from("Shoulders")
        }
        (10003, 107_137_021) => {
            String::from("Right arm")
        }
        (10003, 101_095_525) => {
            String::from("Shop Search")
        }
        (10003, 267_969_396) => {
            String::from("The item you dropped was locked.")
        }
        (10003, 223_571_093) => {
            String::from("You can't sell unique items in your shop.")
        }
        (10003, 265_297_780) => {
            String::from("No items found.")
        }
        (10003, 197_508_636) => {
            String::from("The shop cannot hold any more items!")
        }
        (10003, 175_110_683) => {
            String::from("Items dragged from the bank can only be dropped in the inventory, or the bank itself.")
        }
        (10003, 135_836_830) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("{a} at ({b}, {c})")
        }
        (10003, 67_532_660) => {
            String::from("Chest")
        }
        (10003, 142_362_884) => {
            String::from("The shop has reached its balance limit.  You were unable to buy one or more of your selected items.")
        }
        (10003, 83_032_948) => {
            String::from("Waist")
        }
        (10003, 2_282_091) => {
            String::from("HUD/deck")
        }
        (10003, 252_884_979) => {
            String::from("Wrists")
        }
        (10003, 14_380_505) => {
            String::from("<font color=red>Only the shop owner has access to the transaction log.</font>")
        }
        (10003, 111_274_345) => {
            String::from("Quality")
        }
        (10003, 222_417_491) => {
            String::from("Legs")
        }
        (10003, 209_600_068) => {
            String::from("This shop terminal is not rented by anyone!")
        }
        (10003, 222_409_259) => {
            String::from("Neck")
        }
        (10003, 198_445_513) => {
            String::from("Any location")
        }
        (10003, 40_216_468) => {
            String::from("Left hand")
        }
        (10003, 31_446_978) => {
            String::from("Implant/symbiant")
        }
        (10003, 259_170_642) => {
            String::from("Armor")
        }
        (10003, 67_239_779) => {
            String::from("Hands")
        }
        (10003, 222_319_147) => {
            String::from("Deck")
        }
        (10003, 207_793_496) => {
            let a = &arguments[0];
            format!("Trading with {a}.")
        }
        (10003, 122_636_030) => {
            let a = &arguments[0];
            format!("Commission: {a}%")
        }
        (10003, 103_551_268) => {
            String::from("Right hand")
        }
        (10003, 64_893_843) => {
            String::from("Deck slots")
        }
        (10003, 220_617_885) => {
            String::from("Left arm")
        }
        (10003, 83_047_907) => {
            String::from("Utilities")
        }
        (10003, 128_373_554) => {
            String::from("Cluster")
        }
        (10003, 222_330_411) => {
            String::from("Back")
        }
        (10003, 223_907_443) => {
            String::from("Downloading shop info...")
        }
        (10003, 105_557_380) => {
            String::from("Left wrist")
        }
        (10003, 37_663_116) => {
            String::from("General")
        }
        (10003, 11_228_476) => {
            String::from("Nano crystals")
        }
        (10003, 222_327_539) => {
            String::from("Arms")
        }
        (10003, 56_494_805) => {
            String::from("It is not possible to remove items from this inventory.")
        }
        (2004, 4) => {
            String::from("Fixer")
        }
        (2004, 10) => {
            String::from("Doctor")
        }
        (2004, 15) => {
            String::from("Shade")
        }
        (2004, 1) => {
            String::from("Soldier")
        }
        (2004, 14) => {
            String::from("Keeper")
        }
        (2004, 13) => {
            String::from("Monster")
        }
        (2004, 3) => {
            String::from("Engineer")
        }
        (2004, 9) => {
            String::from("Enforcer")
        }
        (2004, 11) => {
            String::from("Nano-Technician")
        }
        (2004, 5) => {
            String::from("Agent")
        }
        (2004, 8) => {
            String::from("Bureaucrat")
        }
        (2004, 12) => {
            String::from("Meta-Physicist")
        }
        (2004, 7) => {
            String::from("Trader")
        }
        (2004, 2) => {
            String::from("Martial Artist")
        }
        (2004, 6) => {
            String::from("Adventurer")
        }
        (506, 5_991_036) => {
            String::from("Total: ")
        }
        (506, 5_801_173) => {
            String::from("Range: ")
        }
        (506, 265_658_836) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("The tower {a} in {b} was just reduced to {c} % health!")
        }
        (506, 304_804) => {
            String::from("Cost: ")
        }
        (506, 5_491_324) => {
            String::from("Metal")
        }
        (506, 59_540_591) => {
            String::from("Max ammo: ")
        }
        (506, 139_033_868) => {
            String::from("SPECIAL")
        }
        (506, 125_440_323) => {
            String::from("Killed by Invaders: ")
        }
        (506, 297_109) => {
            String::from("Base: ")
        }
        (506, 118_352_306) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            let e = &arguments[4];
            format!("{a} just initiated an attack in {b} at location ({c},{d}). That area is controlled by {e}. All districts controlled by your organization are open to attack! You are in a state of war. Leader chat informed.")
        }
        (506, 195_152_259) => {
            String::from("Defence skills: ")
        }
        (506, 52_489_877) => {
            String::from("Skill To Use: ")
        }
        (506, 136_163_732) => {
            String::from("Dual wield: ")
        }
        (506, 44_473_587) => {
            String::from("Machines. ")
        }
        (506, 193_102_827) => {
            String::from("Team rank:")
        }
        (506, 149_539_071) => {
            String::from("Energy Weapon Ammo")
        }
        (506, 96_779_263) => {
            String::from("Nano Execution Init")
        }
        (506, 127_637_748) => {
            String::from("Detachment: ")
        }
        (506, 138_100_997) => {
            String::from("Clan title:")
        }
        (506, 105_547_781) => {
            String::from("Concrete")
        }
        (506, 166_235_523) => {
            String::from("Charges: ")
        }
        (506, 94_492_169) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            let e = &arguments[4];
            format!("The tower {a} in {b} was just reduced to {c} % health by {d} from the {e} organization!")
        }
        (506, 1_605_541) => {
            String::from("It will be almost impossible to kill this alone.")
        }
        (506, 307_348) => {
            String::from("Dirt")
        }
        (506, 266_695_796) => {
            String::from("Alignment: ")
        }
        (506, 265_415_915) => {
            String::from("SneakAttack ")
        }
        (506, 91_903_877) => {
            let a = &arguments[0];
            format!("<div indent=wrapped><font color=CCRed>{a}% Effective</font></div>")
        }
        (506, 21331) => {
            String::from("Location: ")
        }
        (506, 95_406_722) => {
            String::from("Modifier:<br>")
        }
        (506, 87_267_136) => {
            String::from("NODROP ")
        }
        (506, 5_991_874) => {
            String::from("Tower: ")
        }
        (506, 128_624_465) => {
            String::from("<font color=CCInfoHeader><br>Item contains a formula:<br></font>")
        }
        (506, 143_617_851) => {
            String::from("FastAttack ")
        }
        (506, 229_878_309) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("{a} days, {b:02} hours, {c:02} minutes.")
        }
        (506, 8_124_532) => {
            String::from("Ranged Weapons Initiative")
        }
        (506, 342_131) => {
            String::from("Maps: ")
        }
        (506, 5_738_389) => {
            String::from("Price: ")
        }
        (506, 24_174_231) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            let e = &arguments[4];
            format!("{a} just initiated an attack on playfield {b} at location ({c},{d}). That area is controlled by {e}. All districts controlled by your organization are open to attack! You are in a state of war. Leader chat informed.")
        }
        (506, 163_785_971) => {
            String::from("Bullets")
        }
        (506, 260_586_565) => {
            String::from("ToxicWaste")
        }
        (506, 241_782_716) => {
            String::from("Quality level: ")
        }
        (506, 59_088_885) => {
            String::from("Side. ")
        }
        (506, 9_983_890) => {
            String::from("Killing it poses no danger.")
        }
        (506, 6_130_370) => {
            String::from("Water")
        }
        (506, 1_698_740) => {
            String::from("Killing this by yourself might be possible, but it will be tough.")
        }
        (506, 218_166_418) => {
            String::from("Vicinity friendly modifier:<br>")
        }
        (506, 131_999_829) => {
            String::from("Damage type: ")
        }
        (506, 137_665_476) => {
            let a = &arguments[0];
            format!("(Up to) {a} SK. ")
        }
        (506, 5_147_209) => {
            String::from("Equip delay: ")
        }
        (506, 214_240_127) => {
            String::from("This item can be placed on the ground.")
        }
        (506, 21684) => {
            String::from("Mud")
        }
        (506, 196_323_444) => {
            String::from("Realtime left: ")
        }
        (506, 224_009_576) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            format!("The tower {a} in {b} was just reduced to {c} % health by {d}!")
        }
        (506, 142_393_411) => {
            String::from("Team mission rules:<br>")
        }
        (506, 4_835_748) => {
            String::from("Burst ")
        }
        (506, 349_765) => {
            String::from("None.")
        }
        (506, 85_289_205) => {
            String::from("You might be able to kill this by yourself.")
        }
        (506, 5_918_821) => {
            String::from("Snipe ")
        }
        (506, 4) => {
            String::from("On Item:")
        }
        (506, 42_586_692) => {
            String::from("<font color=CCInfoHeader>Item reward:</font><br>")
        }
        (506, 92_836_025) => {
            String::from("Rarity: ")
        }
        (506, 106_449_679) => {
            String::from("Blood")
        }
        (506, 223_300_019) => {
            String::from("Requirements:<br>")
        }
        (506, 147_836_847) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("<font color=CCInfoHeader>Organization:</font>\r\n<font color=CCInfoText>{a}</font>\r\n<font color=CCInfoHeader>Created at UTC:</font>\r\n<font color=CCInfoText>{b}</font>\r\n")
        }
        (506, 303_742) => {
            String::from("Clan: ")
        }
        (506, 59_661_348) => {
            String::from("Number of Invaders killed: ")
        }
        (506, 241_978_379) => {
            String::from("Defender rank: ")
        }
        (506, 366_660) => {
            String::from("Sand")
        }
        (506, 53_799_977) => {
            String::from("Owners of city in: ")
        }
        (506, 206_338_533) => {
            String::from("Melee Weapons Initiative")
        }
        (506, 138_976_855) => {
            String::from("Arrow.")
        }
        (506, 235_432_415) => {
            String::from("Flamethrower Ammo")
        }
        (506, 196_662_245) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Attack {a:.2}s<br>Recharge {b:.2}s")
        }
        (506, 45_657_746) => {
            String::from("Leather")
        }
        (506, 76_134_158) => {
            String::from("Nanoline Cooldown: ")
        }
        (506, 172_859_061) => {
            String::from("<div indent=wrapped><font color=CCRed>Item can be looted if killed in PvP.</font></div>")
        }
        (506, 104_964_995) => {
            String::from("Personal tower modifiers:<br><br>")
        }
        (506, 33_027_413) => {
            String::from("Profession&nbsp;title: ")
        }
        (506, 63_878_708) => {
            String::from("Not chosen yet.")
        }
        (506, 43_535_580) => {
            String::from("On Cancel: ")
        }
        (506, 128_000_901) => {
            String::from(" (Stackable)")
        }
        (506, 139_699_172) => {
            String::from("<font color=CCInfoHeader>Nano Cost: </font>")
        }
        (506, 156_067_838) => {
            String::from("Description:<br>")
        }
        (506, 243_500_084) => {
            String::from("<br>Might attack you on sight.")
        }
        (506, 71_746_789) => {
            String::from("Ammo type: ")
        }
        (506, 2) => {
            String::from("On User:")
        }
        (506, 62_667_244) => {
            String::from("Physical Prowess and Martial Arts Initiative")
        }
        (506, 207_470_706) => {
            String::from("DeepWater")
        }
        (506, 265_216_626) => {
            String::from("Personal modifier:<br>")
        }
        (506, 338_113) => {
            String::from("Lava")
        }
        (506, 210_283_758) => {
            String::from("Duration: ")
        }
        (506, 205_015_148) => {
            String::from("Danger level:<br>")
        }
        (506, 95_820_883) => {
            String::from("Side tokens: ")
        }
        (506, 175_049_148) => {
            String::from("BowSpecial ")
        }
        (506, 252_886_883) => {
            String::from("Side XP bonus: ")
        }
        (506, 177_147_043) => {
            let a = &arguments[0];
            format!("{a} credits")
        }
        (506, 94_561_941) => {
            String::from("UNIQUE ")
        }
        (506, 127_807_260) => {
            String::from("Max beneficial skill: ")
        }
        (506, 193_923_153) => {
            String::from("Playshift requirements")
        }
        (506, 89_561_540) => {
            String::from("Not&nbsp;Set")
        }
        (506, 301_208) => {
            String::from("Cash: ")
        }
        (506, 202_933_349) => {
            String::from("Skill locked on use: ")
        }
        (506, 41_267_315) => {
            let a = &arguments[0];
            format!("(Up to) {a} XP. ")
        }
        (506, 42_706_676) => {
            String::from("NCU cost: ")
        }
        (506, 54_638_931) => {
            String::from("Solo kills:")
        }
        (506, 105_909_336) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            let e = &arguments[4];
            format!("Your tower {a} at X:{b} and Z:{c} in {d} was just destroyed by {e}!")
        }
        (506, 1) => {
            String::from("On Self:")
        }
        (506, 202_492_547) => {
            String::from("Duel wins:")
        }
        (506, 10_603_395) => {
            String::from("Attack skills: ")
        }
        (506, 174_825_260) => {
            String::from("Special: ")
        }
        (506, 132_649_087) => {
            String::from("Grenade Ammo")
        }
        (506, 166_396_948) => {
            String::from("Self supplied")
        }
        (506, 78_133_205) => {
            String::from("Damage: ")
        }
        (506, 76_125_923) => {
            String::from("Arrows")
        }
        (506, 1_686_425) => {
            String::from("Killing it by yourself will be relatively easy.")
        }
        (506, 12_753_364) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            let e = &arguments[4];
            let f = &arguments[5];
            let g = &arguments[6];
            let h = &arguments[7];
            format!("The {a} organization {b} just entered a state of war! {c} attacked the {d} organization {e}'s tower in {f} at location ({g},{h}).")
        }
        (506, 11_481_395) => {
            String::from("Duel losses:")
        }
        (506, 70_914_326) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Ammo: {a}/{b}")
        }
        (506, 82_101_762) => {
            String::from("Fixture: ")
        }
        (506, 4_819_892) => {
            String::from("Breed: ")
        }
        (506, 119_223_822) => {
            String::from("Local cooldown: ")
        }
        (506, 136_427_907) => {
            String::from("Kills in team:")
        }
        (506, 210_471_925) => {
            String::from("Experience: ")
        }
        (506, 14) => {
            String::from("On Fighting Target:")
        }
        (506, 92_778_691) => {
            String::from("Radius: ")
        }
        (506, 125_532_947) => {
            String::from("Players. ")
        }
        (506, 231_209_733) => {
            String::from("Tower type: ")
        }
        (506, 5_426_364) => {
            String::from("Level: ")
        }
        (506, 112_983_811) => {
            String::from("Map reader upgrades: ")
        }
        (506, 12_957_456) => {
            String::from("Building Upkeep: ")
        }
        (506, 5_925_812) => {
            String::from("Speed:<br>")
        }
        (506, 93_116_548) => {
            String::from("<font color=CCInfoHeader>Reward:</font><br>")
        }
        (506, 93_976_156) => {
            String::from("School: ")
        }
        (506, 138_689_044) => {
            let a = &arguments[0];
            format!("Ammo: {a} (Stackable)")
        }
        (506, 204_507_263) => {
            String::from("FullAuto ")
        }
        (506, 185_982_243) => {
            String::from("Specializations:<br>")
        }
        (506, 23_254_227) => {
            String::from("Monsters. ")
        }
        (506, 216_700_242) => {
            String::from("Much higher than you.")
        }
        (506, 237_054_834) => {
            String::from("Stacking Order: ")
        }
        (506, 106_820_447) => {
            String::from("Rockets")
        }
        (506, 13_590_638) => {
            String::from("This is a Team Mission! It is not meant for one person, and small teams are unlikely to survive it!<br><br> A Team Mission rewards only those who were in your team when you accepted the mission. When you complete the mission, those same members must still be in your team to receive their rewards. Each member will get one of the rewards - randomly picked from the available selection - but you may choose to swap rewards afterward.<br><br>There are level restrictions on the team members. You must be within team-experience-points-sharing-range to be able to do a Team Mission together!")
        }
        (506, 5_035_092) => {
            String::from("FlingShot ")
        }
        (506, 77_345_525) => {
            String::from("Controller type: ")
        }
        (506, 105_909_337) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            let e = &arguments[4];
            let f = &arguments[5];
            format!("Your tower {a} at X:{b} and Z:{c} in {d} was just destroyed by {e}, from the {f} organization!")
        }
        (506, 103_463_160) => {
            String::from("On Death: ")
        }
        (506, 5_057_688) => {
            String::from("Flesh")
        }
        (506, 19315) => {
            String::from("Dps: ")
        }
        (506, 174_832_085) => {
            let a = &arguments[0];
            format!("Recharge {a:.2}s")
        }
        (506, 143_294_329) => {
            String::from("GrassDry")
        }
        (506, 174_599_619) => {
            String::from("Shotgun Shells")
        }
        (506, 74_895_961) => {
            String::from("Temporary: ")
        }
        (506, 173_458_082) => {
            String::from("Vicinity hostile modifier:<br>")
        }
        (506, 214_999_973) => {
            String::from("Building Size: ")
        }
        (506, 3) => {
            String::from("On Target:")
        }
        (506, 81_545_922) => {
            String::from("Gender: ")
        }
        (506, 207_945_924) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            format!("Your tower {a} at X:{b} and Z:{c} in {d} was just destroyed!")
        }
        (506, 266_646_693) => {
            String::from("Detachment title: ")
        }
        (506, 50_996_053) => {
            String::from(" (Splitable)")
        }
        (506, 76_340_692) => {
            String::from("<font color=CCInfoText>None</font>")
        }
        (506, 83_936_697) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            let e = &arguments[4];
            let f = &arguments[5];
            format!("Your tower {a} at X:{b} and Z:{c} in {d} was just destroyed by {e} from the {f} organization!")
        }
        (506, 197_231_867) => {
            String::from("Duel rank:")
        }
        (506, 104_104_939) => {
            String::from("Solo rank:")
        }
        (506, 179_094_998) => {
            String::from("Masters of:<br>")
        }
        (506, 184_399_301) => {
            String::from("Initiative: ")
        }
        (506, 174_831_828) => {
            let a = &arguments[0];
            format!("Attack {a:.2}s")
        }
        (506, 80_189_683) => {
            String::from("Fabric: ")
        }
        (506, 365_979) => {
            String::from("Rock")
        }
        (506, 214_951_694) => {
            String::from("Profession: ")
        }
        (506, 4_863_656) => {
            String::from("Cloth")
        }
        (506, 112_459_845) => {
            String::from("Trying to kill this alone will almost certainly result in your death.")
        }
        (506, 117_123_852) => {
            String::from("Shadowlevel: ")
        }
        (506, 208_168_789) => {
            String::from("You can probably kill this by yourself.")
        }
        (506, 246_830_219) => {
            String::from("MA for combined attack: ")
        }
        (506, 79_916_692) => {
            String::from("Insurance cost: ")
        }
        (506, 139_864_837) => {
            String::from("Nanoline: ")
        }
        (506, 147_506_468) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("Notum Wars Update: The {a} organization {b} lost their base in {c}.")
        }
        (2000, 228) => {
            String::from("ShieldEnergyAC")
        }
        (2000, 223) => {
            String::from("CanChangeClothes")
        }
        (2000, 365) => {
            String::from("ReclaimItem")
        }
        (2000, 8) => {
            String::from("TimeExist")
        }
        (2000, 888) => {
            String::from("VisualLODLevel")
        }
        (2000, 191) => {
            String::from("LastConcretePlayfieldInstance")
        }
        (2000, 204) => {
            String::from("BreedHostility")
        }
        (2000, 151) => {
            String::from("AimedShot")
        }
        (2000, 370) => {
            String::from("RitualTargetInst")
        }
        (2000, 416) => {
            String::from("CorpseInstance")
        }
        (2000, 442) => {
            String::from("CharTmp2")
        }
        (2000, 52) => {
            String::from("XP")
        }
        (2000, 201) => {
            String::from("Aggressiveness")
        }
        (2000, 453) => {
            String::from("NPCVicinityChars")
        }
        (2000, 563) => {
            String::from("ClanGaia")
        }
        (2000, 98) => {
            String::from("StateAction")
        }
        (2000, 409) => {
            String::from("TrainSkillCost")
        }
        (2000, 172) => {
            String::from("HealthChange")
        }
        (2000, 540) => {
            String::from("ProcInitiative2")
        }
        (2000, 703) => {
            String::from("ACGItemTemplateID2")
        }
        (2000, 573) => {
            String::from("SK")
        }
        (2000, 142) => {
            String::from("Brawl")
        }
        (2000, 564) => {
            String::from("OTTrans")
        }
        (2000, 446) => {
            String::from("NameTemplate")
        }
        (2000, 21) => {
            String::from("Psychic")
        }
        (2000, 118) => {
            String::from("CloseCombatInitiative")
        }
        (2000, 353) => {
            String::from("AnimSet")
        }
        (2000, 389) => {
            String::from("Expansion")
        }
        (2000, 427) => {
            String::from("SellModifier")
        }
        (2000, 509) => {
            String::from("QuestIndex0")
        }
        (2000, 430) => {
            String::from("WaitState")
        }
        (2000, 291) => {
            String::from("NumAttackEffects")
        }
        (2000, 216) => {
            String::from("ReflectRadiationAC")
        }
        (2000, 525) => {
            String::from("PercentRemainingHealth")
        }
        (2000, 443) => {
            String::from("CharTmp3")
        }
        (2000, 346) => {
            String::from("TradeLimit")
        }
        (2000, 454) => {
            String::from("ProximityRangeOutdoors")
        }
        (2000, 467) => {
            String::from("PetReq1")
        }
        (2000, 515) => {
            String::from("TowerInstance")
        }
        (2000, 89) => {
            String::from("Race")
        }
        (2000, 194) => {
            String::from("InPlay")
        }
        (2000, 328) => {
            String::from("Compulsion")
        }
        (2000, 318) => {
            String::from("NPCostModifier")
        }
        (2000, 395) => {
            String::from("PrimaryTemplateID")
        }
        (2000, 5) => {
            String::from("Clan")
        }
        (2000, 293) => {
            String::from("ItemSkill")
        }
        (2000, 39) => {
            String::from("ShoulderMesh")
        }
        (2000, 388) => {
            String::from("TowerType")
        }
        (2000, 9) => {
            String::from("MapFlags")
        }
        (2000, 1_234_567_890) => {
            String::from("NoStatValue")
        }
        (2000, 56) => {
            String::from("TimeSinceCreation")
        }
        (2000, 371) => {
            String::from("SkillTimeOnSelectedTarget")
        }
        (2000, 187) => {
            String::from("CardOwnerInstance")
        }
        (2000, 432) => {
            String::from("ErrorCode")
        }
        (2000, 572) => {
            String::from("ClanRedeemed")
        }
        (2000, 601) => {
            String::from("HPLevelUp")
        }
        (2000, 230) => {
            String::from("ShieldRadiationAC")
        }
        (2000, 577) => {
            String::from("LastPerkResetTime")
        }
        (2000, 283) => {
            String::from("ItemHateValue")
        }
        (2000, 402) => {
            String::from("CATAnimFlags")
        }
        (2000, 463) => {
            String::from("ODMaxSizeAdd")
        }
        (2000, 193) => {
            String::from("ExtenalDoorInstance")
        }
        (2000, 107) => {
            String::from("2hBluntWeapons")
        }
        (2000, 561) => {
            String::from("ClanSentinels")
        }
        (2000, 266) => {
            String::from("MartialArtsWeapon")
        }
        (2000, 163) => {
            String::from("Chemistry")
        }
        (2000, 627) => {
            String::from("PercentEnergyDamage")
        }
        (2000, 579) => {
            String::from("ShadowBreedTemplate")
        }
        (2000, 302) => {
            String::from("ClanPrice")
        }
        (2000, 69) => {
            String::from("MaxConstructedQuest")
        }
        (2000, 243) => {
            String::from("AbsorbColdAC")
        }
        (2000, 364) => {
            String::from("NanoDelta")
        }
        (2000, 475) => {
            String::from("ReflectReturnedProjectileAC")
        }
        (2000, 477) => {
            String::from("ReflectReturnedEnergyAC")
        }
        (2000, 449) => {
            String::from("NPCIsSurrendering")
        }
        (2000, 611) => {
            String::from("ShopLastUsed")
        }
        (2000, 188) => {
            String::from("BuildingComplexInst")
        }
        (2000, 224) => {
            String::from("Features")
        }
        (2000, 465) => {
            String::from("NPCCryForHelpRange")
        }
        (2000, 369) => {
            String::from("VisualSex")
        }
        (2000, 159) => {
            String::from("Pharmaceuticals")
        }
        (2000, 418) => {
            String::from("UnarmedTemplateInstance")
        }
        (2000, 161) => {
            String::from("ComputerLiteracy")
        }
        (2000, 480) => {
            String::from("ReflectReturnedColdAC")
        }
        (2000, 585) => {
            String::from("MapAreaPart3")
        }
        (2000, 602) => {
            String::from("HPPerSkill")
        }
        (2000, 128) => {
            String::from("BiologicalMetamorphose")
        }
        (2000, 166) => {
            String::from("DriveGround")
        }
        (2000, 415) => {
            String::from("CorpseType")
        }
        (2000, 261) => {
            String::from("QuestStat")
        }
        (2000, 105) => {
            String::from("2hEdgedWeapons")
        }
        (2000, 178) => {
            String::from("AlienNextXP")
        }
        (2000, 325) => {
            String::from("WeaponDisallowedType")
        }
        (2000, 393) => {
            String::from("ResistModifier")
        }
        (2000, 14) => {
            String::from("Name")
        }
        (2000, 54) => {
            String::from("Level")
        }
        (2000, 252) => {
            String::from("MeetersWalked")
        }
        (2000, 326) => {
            String::from("WeaponDisallowedInstance")
        }
        (2000, 384) => {
            String::from("ACGEntranceStyles")
        }
        (2000, 547) => {
            String::from("StackingLine3")
        }
        (2000, 61) => {
            String::from("Cash")
        }
        (2000, 220) => {
            String::from("CurrBodyLocation")
        }
        (2000, 404) => {
            String::from("DisplayCATMesh")
        }
        (2000, 533) => {
            String::from("NPCFovStatus")
        }
        (2000, 625) => {
            String::from("PercentPoisonDamage")
        }
        (2000, 390) => {
            String::from("LowresMesh")
        }
        (2000, 488) => {
            String::from("TargetFacing")
        }
        (2000, 551) => {
            String::from("StackingOrder")
        }
        (2000, 164) => {
            String::from("Concealment")
        }
        (2000, 482) => {
            String::from("ReflectReturnedFireAC")
        }
        (2000, 375) => {
            String::from("FullAutoRecharge")
        }
        (2000, 441) => {
            String::from("CharTmp1")
        }
        (2000, 566) => {
            String::from("GOS")
        }
        (2000, 40) => {
            String::from("AlienXP")
        }
        (2000, 87) => {
            String::from("AreaInstance")
        }
        (2000, 129) => {
            String::from("PsychologicalModification")
        }
        (2000, 344) => {
            String::from("MonsterTexture")
        }
        (2000, 554) => {
            String::from("ProcNano3")
        }
        (2000, 656) => {
            String::from("ShopIndex")
        }
        (2000, 444) => {
            String::from("CharTmp4")
        }
        (2000, 612) => {
            String::from("ShopType")
        }
        (2000, 624) => {
            String::from("PercentProjectileDamage")
        }
        (2000, 653) => {
            String::from("AdvantageHash3")
        }
        (2000, 603) => {
            String::from("NewbieNP")
        }
        (2000, 311) => {
            String::from("ColdDamageModifier")
        }
        (2000, 437) => {
            String::from("CollideCheckInterval")
        }
        (2000, 532) => {
            String::from("ShadowBreed")
        }
        (2000, 241) => {
            String::from("AbsorbChemicalAC")
        }
        (2000, 348) => {
            String::from("SpecialCondition")
        }
        (2000, 123) => {
            String::from("FirstAid")
        }
        (2000, 199) => {
            String::from("RP")
        }
        (2000, 642) => {
            String::from("CityTerminalRechargePercent")
        }
        (2000, 331) => {
            String::from("ClanItemInstance")
        }
        (2000, 626) => {
            String::from("PercentRadiationDamage")
        }
        (2000, 225) => {
            String::from("ReflectPoisonAC")
        }
        (2000, 368) => {
            String::from("VisualProfession")
        }
        (2000, 569) => {
            String::from("OTUnredeemed")
        }
        (2000, 623) => {
            String::from("PercentMeleeDamage")
        }
        (2000, 701) => {
            String::from("ACGItemLevel")
        }
        (2000, 175) => {
            String::from("AutoLockTimeDefault")
        }
        (2000, 298) => {
            String::from("Placement")
        }
        (2000, 236) => {
            String::from("InsurancePercentage")
        }
        (2000, 2) => {
            String::from("VolumeMass")
        }
        (2000, 383) => {
            String::from("InterruptModifier")
        }
        (2000, 411) => {
            String::from("NextFormula")
        }
        (2000, 550) => {
            String::from("StackingLine6")
        }
        (2000, 639) => {
            String::from("NPCSpellRet1")
        }
        (2000, 575) => {
            String::from("NextSK")
        }
        (2000, 655) => {
            String::from("AdvantageHash5")
        }
        (2000, 651) => {
            String::from("AdvantageHash1")
        }
        (2000, 254) => {
            String::from("MonsterLevelsKilled")
        }
        (2000, 84) => {
            String::from("UserType")
        }
        (2000, 319) => {
            String::from("XPModifier")
        }
        (2000, 66) => {
            String::from("ShoulderTexture")
        }
        (2000, 119) => {
            String::from("DistanceWeaponInitiative")
        }
        (2000, 103) => {
            String::from("1hEdgedWeapon")
        }
        (2000, 116) => {
            String::from("AssaultRifle")
        }
        (2000, 239) => {
            String::from("AbsorbMeleeAC")
        }
        (2000, 282) => {
            String::from("RadiationDamageModifier")
        }
        (2000, 413) => {
            String::from("EffectType")
        }
        (2000, 323) => {
            String::from("PlayerKilling")
        }
        (2000, 447) => {
            String::from("DesiredTargetDistance")
        }
        (2000, 520) => {
            String::from("Rnd")
        }
        (2000, 244) => {
            String::from("AbsorbFireAC")
        }
        (2000, 424) => {
            String::from("ArmourType")
        }
        (2000, 536) => {
            String::from("NanoDamageMultiplier")
        }
        (2000, 638) => {
            String::from("NPCSpellArg1")
        }
        (2000, 3) => {
            String::from("AttackSpeed")
        }
        (2000, 203) => {
            String::from("Extroverty")
        }
        (2000, 381) => {
            String::from("RangeIncreaserNF")
        }
        (2000, 268) => {
            String::from("LiquidType")
        }
        (2000, 18) => {
            String::from("Stamina")
        }
        (2000, 330) => {
            String::from("ClanItemType")
        }
        (2000, 478) => {
            String::from("ReflectReturnedChemicalAC")
        }
        (2000, 519) => {
            String::from("NPCUseFightModeRegenRate")
        }
        (2000, 524) => {
            String::from("RechargeDelayCap")
        }
        (2000, 35) => {
            String::from("AccessCount")
        }
        (2000, 59) => {
            String::from("Sex")
        }
        (2000, 102) => {
            String::from("1hBluntWeapons")
        }
        (2000, 398) => {
            String::from("Corpse_Hash")
        }
        (2000, 492) => {
            String::from("QuestIndex1")
        }
        (2000, 538) => {
            String::from("AmsCap")
        }
        (2000, 269) => {
            String::from("GatherSound")
        }
        (2000, 450) => {
            String::from("StateMachine")
        }
        (2000, 345) => {
            String::from("HasAlwaysLootable")
        }
        (2000, 361) => {
            String::from("HitEffectType")
        }
        (2000, 802) => {
            String::from("QuestASMaximumRange")
        }
        (2000, 162) => {
            String::from("Psychology")
        }
        (2000, 16) => {
            String::from("Strength")
        }
        (2000, 113) => {
            String::from("Rifle")
        }
        (2000, 33) => {
            String::from("Side")
        }
        (2000, 542) => {
            String::from("ProcInitiative4")
        }
        (2000, 38) => {
            String::from("BackMesh")
        }
        (2000, 451) => {
            String::from("NPCSurrenderInstance")
        }
        (2000, 209) => {
            String::from("WeaponMesh")
        }
        (2000, 493) => {
            String::from("QuestIndex2")
        }
        (2000, 498) => {
            String::from("QTNumMonsters")
        }
        (2000, 131) => {
            String::from("MaterialLocation")
        }
        (2000, 489) => {
            String::from("Backstab")
        }
        (2000, 555) => {
            String::from("ProcNano4")
        }
        (2000, 491) => {
            String::from("QuestInstance")
        }
        (2000, 297) => {
            String::from("InteractionRadius")
        }
        (2000, 251) => {
            String::from("PetCounter")
        }
        (2000, 222) => {
            String::from("AccumulatedDamage")
        }
        (2000, 279) => {
            String::from("MeleeDamageModifier")
        }
        (2000, 333) => {
            String::from("PvP_Rating")
        }
        (2000, 558) => {
            String::from("ProcChance3")
        }
        (2000, 322) => {
            String::from("LevelLimit")
        }
        (2000, 598) => {
            String::from("ExtendedFlags")
        }
        (2000, 640) => {
            String::from("CityInstance")
        }
        (2000, 210) => {
            String::from("RechargeDelay")
        }
        (2000, 304) => {
            String::from("ClanType")
        }
        (2000, 144) => {
            String::from("Dimach")
        }
        (2000, 373) => {
            String::from("ExtendedTime")
        }
        (2000, 20) => {
            String::from("Sense")
        }
        (2000, 111) => {
            String::from("Bow")
        }
        (2000, 275) => {
            String::from("XPKillRange")
        }
        (2000, 397) => {
            String::from("SelectedTargetType")
        }
        (2000, 155) => {
            String::from("Evade")
        }
        (2000, 494) => {
            String::from("QuestIndex3")
        }
        (2000, 109) => {
            String::from("Grenade")
        }
        (2000, 582) => {
            String::from("ApartmentsAllowed")
        }
        (2000, 329) => {
            String::from("SkillDisabled")
        }
        (2000, 549) => {
            String::from("StackingLine5")
        }
        (2000, 635) => {
            String::from("TrackColdDamage")
        }
        (2000, 633) => {
            String::from("TrackChemicalDamage")
        }
        (2000, 523) => {
            String::from("ItemDelayCap")
        }
        (2000, 276) => {
            String::from("AMSModifier")
        }
        (2000, 578) => {
            String::from("CurrentTime")
        }
        (2000, 60) => {
            String::from("Profession")
        }
        (2000, 260) => {
            String::from("ClanHierarchy")
        }
        (2000, 349) => {
            String::from("AutoAttackFlags")
        }
        (2000, 153) => {
            String::from("Duck")
        }
        (2000, 43) => {
            String::from("ParentType")
        }
        (2000, 152) => {
            String::from("BodyDevelopment")
        }
        (2000, 387) => {
            String::from("DieAnim")
        }
        (2000, 400) => {
            String::from("Rotation")
        }
        (2000, 434) => {
            String::from("CharState")
        }
        (2000, 136) => {
            String::from("Perception")
        }
        (2000, 202) => {
            String::from("Stability")
        }
        (2000, 429) => {
            String::from("NPCBrainState")
        }
        (2000, 531) => {
            String::from("ExpansionPlayfield")
        }
        (2000, 476) => {
            String::from("ReflectReturnedMeleeAC")
        }
        (2000, 541) => {
            String::from("ProcInitiative3")
        }
        (2000, 516) => {
            String::from("AttackShield")
        }
        (2000, 584) => {
            String::from("ApartmentAccessCard")
        }
        (2000, 338) => {
            String::from("DeathReason")
        }
        (2000, 423) => {
            String::from("CurrentState")
        }
        (2000, 242) => {
            String::from("AbsorbRadiationAC")
        }
        (2000, 23) => {
            String::from("StaticInstance")
        }
        (2000, 25) => {
            String::from("StaticType")
        }
        (2000, 900) => {
            String::from("TideRequiredDynelID")
        }
        (2000, 169) => {
            String::from("AlienLevel")
        }
        (2000, 590) => {
            String::from("DistrictNano")
        }
        (2000, 386) => {
            String::from("ChanceOfBreakOnDebuff")
        }
        (2000, 350) => {
            String::from("NextXP")
        }
        (2000, 197) => {
            String::from("OrientationMode")
        }
        (2000, 246) => {
            String::from("AbsorbNanoAC")
        }
        (2000, 468) => {
            String::from("PetReq2")
        }
        (2000, 24) => {
            String::from("MaxMass")
        }
        (2000, 173) => {
            String::from("CurrentMovementMode")
        }
        (2000, 74) => {
            String::from("Price")
        }
        (2000, 521) => {
            String::from("SocialStatus")
        }
        (2000, 0) => {
            String::from("Flags")
        }
        (2000, 435) => {
            String::from("ReadOnly")
        }
        (2000, 280) => {
            String::from("EnergyDamageModifier")
        }
        (2000, 12) => {
            String::from("Mesh")
        }
        (2000, 156) => {
            String::from("RunSpeed")
        }
        (2000, 546) => {
            String::from("StackingLine2")
        }
        (2000, 379) => {
            String::from("CriticalIncrease")
        }
        (2000, 599) => {
            String::from("ShopPrice")
        }
        (2000, 588) => {
            String::from("ActionCategory")
        }
        (2000, 377) => {
            String::from("CastTargetAbstractAnim")
        }
        (2000, 391) => {
            String::from("CriticalDecrease")
        }
        (2000, 237) => {
            String::from("ChangeSideCount")
        }
        (2000, 284) => {
            String::from("DamageBonus")
        }
        (2000, 50) => {
            String::from("InventoryTimeout")
        }
        (2000, 537) => {
            String::from("NanoVulnerability")
        }
        (2000, 385) => {
            String::from("ChanceOfBreakOnSpellAttack")
        }
        (2000, 288) => {
            String::from("HateValueModifyer")
        }
        (2000, 94) => {
            String::from("RadiationAC")
        }
        (2000, 503) => {
            String::from("QTKillNumMonsterID1")
        }
        (2000, 589) => {
            String::from("CurrentPlayfield")
        }
        (2000, 343) => {
            String::from("HealDelta")
        }
        (2000, 630) => {
            String::from("TrackProjectileDamage")
        }
        (2000, 63) => {
            String::from("Attitude")
        }
        (2000, 263) => {
            String::from("Brawl1Weapon")
        }
        (2000, 452) => {
            String::from("NPCHasPatrolList")
        }
        (2000, 458) => {
            String::from("NPCNumPets")
        }
        (2000, 336) => {
            String::from("OverrideTexture")
        }
        (2000, 316) => {
            String::from("FireDamageModifier")
        }
        (2000, 425) => {
            String::from("RestModifier")
        }
        (2000, 479) => {
            String::from("ReflectReturnedRadiationAC")
        }
        (2000, 274) => {
            String::from("EquippedWeapons")
        }
        (2000, 78) => {
            String::from("CurrentMass")
        }
        (2000, 570) => {
            String::from("ClanDevoted")
        }
        (2000, 126) => {
            String::from("ElectricalEngineering")
        }
        (2000, 528) => {
            String::from("TeamCloseness")
        }
        (2000, 593) => {
            String::from("RegainXPPercentage")
        }
        (2000, 28) => {
            String::from("Height")
        }
        (2000, 53) => {
            String::from("IP")
        }
        (2000, 165) => {
            String::from("BreakingEntry")
        }
        (2000, 347) => {
            String::from("FaceTexture")
        }
        (2000, 80) => {
            String::from("PrimaryItemType")
        }
        (2000, 394) => {
            String::from("ChestFlags")
        }
        (2000, 229) => {
            String::from("ShieldChemicalAC")
        }
        (2000, 287) => {
            String::from("AttackRange")
        }
        (2000, 433) => {
            String::from("OwnerInstance")
        }
        (2000, 535) => {
            String::from("HealMultiplier")
        }
        (2000, 513) => {
            String::from("OnTowerCreation")
        }
        (2000, 483) => {
            String::from("ReflectReturnedPoisonAC")
        }
        (2000, 607) => {
            String::from("PlayerID")
        }
        (2000, 704) => {
            String::from("ACGItemCategoryID")
        }
        (2000, 271) => {
            String::from("TravelSound")
        }
        (2000, 356) => {
            String::from("NPCHash")
        }
        (2000, 576) => {
            String::from("PlayerOptions")
        }
        (2000, 71) => {
            String::from("TotalMass")
        }
        (2000, 122) => {
            String::from("SenseImprovement")
        }
        (2000, 497) => {
            String::from("QTDungeonInstance")
        }
        (2000, 609) => {
            String::from("SynergyHash")
        }
        (2000, 438) => {
            String::from("PlayfieldType")
        }
        (2000, 106) => {
            String::from("Piercing")
        }
        (2000, 417) => {
            String::from("CorpseAnimKey")
        }
        (2000, 195) => {
            String::from("AccessKey")
        }
        (2000, 46) => {
            String::from("BandolierSlots")
        }
        (2000, 335) => {
            String::from("DoorBlockTime")
        }
        (2000, 317) => {
            String::from("PoisonDamageModifier")
        }
        (2000, 405) => {
            String::from("School")
        }
        (2000, 233) => {
            String::from("ShieldFireAC")
        }
        (2000, 351) => {
            String::from("TeleportPauseMilliSeconds")
        }
        (2000, 530) => {
            String::from("ConditionState")
        }
        (2000, 376) => {
            String::from("GatherAbstractAnim")
        }
        (2000, 138) => {
            String::from("Swim")
        }
        (2000, 290) => {
            String::from("StatOne")
        }
        (2000, 27) => {
            String::from("Health")
        }
        (2000, 292) => {
            String::from("DefaultAttackType")
        }
        (2000, 399) => {
            String::from("AmmoName")
        }
        (2000, 19) => {
            String::from("Intelligence")
        }
        (2000, 372) => {
            String::from("LastSaveXP")
        }
        (2000, 299) => {
            String::from("LockDifficulty")
        }
        (2000, 403) => {
            String::from("DisplayCATAnim")
        }
        (2000, 507) => {
            String::from("QTKillNumMonsterID3")
        }
        (2000, 272) => {
            String::from("HitSound")
        }
        (2000, 517) => {
            String::from("SpecialAttackShield")
        }
        (2000, 174) => {
            String::from("PrevMovementMode")
        }
        (2000, 410) => {
            String::from("IsFightingMe")
        }
        (2000, 529) => {
            String::from("NumberOnHateList")
        }
        (2000, 556) => {
            String::from("ProcChance1")
        }
        (2000, 586) => {
            String::from("MapAreaPart4")
        }
        (2000, 700) => {
            String::from("ACGItemSeed")
        }
        (2000, 462) => {
            String::from("EffectBlue")
        }
        (2000, 186) => {
            String::from("CardOwnerType")
        }
        (2000, 240) => {
            String::from("AbsorbEnergyAC")
        }
        (2000, 265) => {
            String::from("DimachWeapon")
        }
        (2000, 474) => {
            String::from("FallDamage")
        }
        (2000, 253) => {
            String::from("QuestLevelsSolved")
        }
        (2000, 583) => {
            String::from("ApartmentsOwned")
        }
        (2000, 213) => {
            String::from("TeamSide")
        }
        (2000, 340) => {
            String::from("BrainType")
        }
        (2000, 500) => {
            String::from("AnimPos")
        }
        (2000, 502) => {
            String::from("AnimSpeed")
        }
        (2000, 592) => {
            String::from("UnsavedXP")
        }
        (2000, 83) => {
            String::from("SecondaryItemInstance")
        }
        (2000, 190) => {
            String::from("NextDoorInBuilding")
        }
        (2000, 406) => {
            String::from("NanoSpeed")
        }
        (2000, 176) => {
            String::from("AutoUnlockTimeDefault")
        }
        (2000, 481) => {
            String::from("ReflectReturnedNanoAC")
        }
        (2000, 622) => {
            String::from("PercentColdDamage")
        }
        (2000, 768) => {
            String::from("HasKnubotData")
        }
        (2000, 436) => {
            String::from("DamageType")
        }
        (2000, 218) => {
            String::from("ReflectNanoAC")
        }
        (2000, 206) => {
            String::from("ReflectMeleeAC")
        }
        (2000, 422) => {
            String::from("ChanceOfUse")
        }
        (2000, 631) => {
            String::from("TrackMeleeDamage")
        }
        (2000, 800) => {
            String::from("QuestBoothDifficulty")
        }
        (2000, 281) => {
            String::from("ChemicalDamageModifier")
        }
        (2000, 90) => {
            String::from("ProjectileAC")
        }
        (2000, 137) => {
            String::from("Climb")
        }
        (2000, 334) => {
            String::from("SavedXP")
        }
        (2000, 455) => {
            String::from("NPCFamily")
        }
        (2000, 567) => {
            String::from("OTFollowers")
        }
        (2000, 654) => {
            String::from("AdvantageHash4")
        }
        (2000, 221) => {
            String::from("MaxNanoEnergy")
        }
        (2000, 277) => {
            String::from("DMSModifier")
        }
        (2000, 58) => {
            String::from("Age")
        }
        (2000, 110) => {
            String::from("ThrownGrapplingWeapons")
        }
        (2000, 205) => {
            String::from("ReflectProjectileAC")
        }
        (2000, 79) => {
            String::from("Icon")
        }
        (2000, 100) => {
            String::from("MartialArts")
        }
        (2000, 527) => {
            String::from("TargetDistance")
        }
        (2000, 150) => {
            String::from("FlingShot")
        }
        (2000, 359) => {
            String::from("MonsterData")
        }
        (2000, 171) => {
            String::from("HealthChangeWorst")
        }
        (2000, 658) => {
            String::from("IsVehicle")
        }
        (2000, 34) => {
            String::from("DeadTimer")
        }
        (2000, 104) => {
            String::from("MeleeEnergyWeapon")
        }
        (2000, 634) => {
            String::from("TrackRadiationDamage")
        }
        (2000, 504) => {
            String::from("QTKillNumMonsterCount1")
        }
        (2000, 285) => {
            String::from("MaxDamage")
        }
        (2000, 133) => {
            String::from("LR_EnergyWeapon")
        }
        (2000, 167) => {
            String::from("FullAuto")
        }
        (2000, 412) => {
            String::from("MultipleCount")
        }
        (2000, 108) => {
            String::from("ThrowingKnife")
        }
        (2000, 552) => {
            String::from("ProcNano1")
        }
        (2000, 22) => {
            String::from("AMS")
        }
        (2000, 73) => {
            String::from("RepairDifficulty")
        }
        (2000, 396) => {
            String::from("NumberOfItems")
        }
        (2000, 314) => {
            String::from("ClanFinalized")
        }
        (2000, 250) => {
            String::from("SoundVolume")
        }
        (2000, 457) => {
            String::from("NPCHatelistSize")
        }
        (2000, 553) => {
            String::from("ProcNano2")
        }
        (2000, 324) => {
            String::from("TeamAllowed")
        }
        (2000, 337) => {
            String::from("OverrideMaterial")
        }
        (2000, 421) => {
            String::from("CharRadius")
        }
        (2000, 440) => {
            String::from("InitiativeType")
        }
        (2000, 307) => {
            String::from("MemberType")
        }
        (2000, 47) => {
            String::from("Fatness")
        }
        (2000, 249) => {
            String::from("LastSaved")
        }
        (2000, 315) => {
            String::from("NanoDamageModifier")
        }
        (2000, 339) => {
            String::from("DamageOverrideType")
        }
        (2000, 652) => {
            String::from("AdvantageHash2")
        }
        (2000, 363) => {
            String::from("NanoInterval")
        }
        (2000, 539) => {
            String::from("ProcInitiative1")
        }
        (2000, 296) => {
            String::from("ItemSIS")
        }
        (2000, 76) => {
            String::from("ItemClass")
        }
        (2000, 247) => {
            String::from("TemporarySkillReduction")
        }
        (2000, 312) => {
            String::from("ClanUpkeepInterval")
        }
        (2000, 306) => {
            String::from("VoteCount")
        }
        (2000, 378) => {
            String::from("CastSelfAbstractAnim")
        }
        (2000, 179) => {
            String::from("NPCFlags")
        }
        (2000, 419) => {
            String::from("TracerEffectType")
        }
        (2000, 45) => {
            String::from("BeltSlots")
        }
        (2000, 115) => {
            String::from("Shotgun")
        }
        (2000, 499) => {
            String::from("QTKilledMonsters")
        }
        (2000, 518) => {
            String::from("NPCVicinityPlayers")
        }
        (2000, 496) => {
            String::from("QuestIndex5")
        }
        (2000, 75) => {
            String::from("MetaType")
        }
        (2000, 616) => {
            String::from("KilledByInvaders")
        }
        (2000, 286) => {
            String::from("MinDamage")
        }
        (2000, 358) => {
            String::from("OuterRadius")
        }
        (2000, 217) => {
            String::from("ReflectColdAC")
        }
        (2000, 401) => {
            String::from("CATAnim")
        }
        (2000, 145) => {
            String::from("Deflect")
        }
        (2000, 408) => {
            String::from("TrainSkill")
        }
        (2000, 615) => {
            String::from("InvadersKilled")
        }
        (2000, 88) => {
            String::from("DefaultPos")
        }
        (2000, 95) => {
            String::from("ColdAC")
        }
        (2000, 125) => {
            String::from("MechanicalEngineering")
        }
        (2000, 560) => {
            String::from("OTArmedForces")
        }
        (2000, 4) => {
            String::from("Breed")
        }
        (2000, 64) => {
            String::from("HeadMesh")
        }
        (2000, 157) => {
            String::from("FieldQuantumPhysics")
        }
        (2000, 580) => {
            String::from("NPCVicinityFamily")
        }
        (2000, 234) => {
            String::from("ShieldPoisonAC")
        }
        (2000, 42) => {
            String::from("CATMesh")
        }
        (2000, 170) => {
            String::from("HealthChangeBest")
        }
        (2000, 548) => {
            String::from("StackingLine4")
        }
        (2000, 469) => {
            String::from("PetReq3")
        }
        (2000, 270) => {
            String::from("CastSound")
        }
        (2000, 68) => {
            String::from("NumConstructedQuest")
        }
        (2000, 702) => {
            String::from("ACGItemTemplateID")
        }
        (2000, 472) => {
            String::from("MapAreaPart2")
        }
        (2000, 130) => {
            String::from("MaterialCreation")
        }
        (2000, 235) => {
            String::from("BerserkMode")
        }
        (2000, 321) => {
            String::from("GenderLimit")
        }
        (2000, 177) => {
            String::from("MoreFlags")
        }
        (2000, 466) => {
            String::from("LOSHeight")
        }
        (2000, 121) => {
            String::from("BowSpecialAttack")
        }
        (2000, 300) => {
            String::from("Members")
        }
        (2000, 245) => {
            String::from("AbsorbPoisonAC")
        }
        (2000, 587) => {
            String::from("NumberOfTeamMembers")
        }
        (2000, 91) => {
            String::from("MeleeAC")
        }
        (2000, 614) => {
            String::from("LeaderLockDownTime")
        }
        (2000, 26) => {
            String::from("Energy")
        }
        (2000, 198) => {
            String::from("SessionTime")
        }
        (2000, 255) => {
            String::from("PvPLevelsKilled")
        }
        (2000, 295) => {
            String::from("ItemOpposedSkill")
        }
        (2000, 332) => {
            String::from("DebuffFormula")
        }
        (2000, 470) => {
            String::from("MapOptions")
        }
        (2000, 301) => {
            String::from("MinMembers")
        }
        (2000, 141) => {
            String::from("Tutoring")
        }
        (2000, 801) => {
            String::from("QuestASMinimumRange")
        }
        (2000, 657) => {
            String::from("ShopID")
        }
        (2000, 305) => {
            String::from("ClanInstance")
        }
        (2000, 49) => {
            String::from("InsuranceTime")
        }
        (2000, 114) => {
            String::from("SubMachineGun")
        }
        (2000, 342) => {
            String::from("HealInterval")
        }
        (2000, 606) => {
            String::from("MaxShopItems")
        }
        (2000, 506) => {
            String::from("QTKillNumMonsterCount2")
        }
        (2000, 439) => {
            String::from("NPCCommand")
        }
        (2000, 471) => {
            String::from("MapAreaPart1")
        }
        (2000, 257) => {
            String::from("MissionBits2")
        }
        (2000, 132) => {
            String::from("NanoEnergyPool")
        }
        (2000, 82) => {
            String::from("SecondaryItemType")
        }
        (2000, 97) => {
            String::from("FireAC")
        }
        (2000, 140) => {
            String::from("MapNavigation")
        }
        (2000, 309) => {
            String::from("GlobalClanType")
        }
        (2000, 352) => {
            String::from("SISCap")
        }
        (2000, 226) => {
            String::from("ShieldProjectileAC")
        }
        (2000, 534) => {
            String::from("DudChance")
        }
        (2000, 574) => {
            String::from("LastSK")
        }
        (2000, 62) => {
            String::from("Alignment")
        }
        (2000, 360) => {
            String::from("MonsterScale")
        }
        (2000, 13) => {
            String::from("Anim")
        }
        (2000, 568) => {
            String::from("OTOperator")
        }
        (2000, 594) => {
            String::from("TempSaveTeamID")
        }
        (2000, 231) => {
            String::from("ShieldColdAC")
        }
        (2000, 215) => {
            String::from("GmLevel")
        }
        (2000, 182) => {
            String::from("Specialization")
        }
        (2000, 357) => {
            String::from("CollisionRadius")
        }
        (2000, 610) => {
            String::from("ShopFlags")
        }
        (2000, 48) => {
            String::from("ClanLevel")
        }
        (2000, 77) => {
            String::from("RepairSkill")
        }
        (2000, 310) => {
            String::from("GlobalClanInstance")
        }
        (2000, 486) => {
            String::from("PetReqVal2")
        }
        (2000, 36) => {
            String::from("AttackCount")
        }
        (2000, 632) => {
            String::from("TrackEnergyDamage")
        }
        (2000, 15) => {
            String::from("Info")
        }
        (2000, 426) => {
            String::from("BuyModifier")
        }
        (2000, 366) => {
            String::from("GatherEffectType")
        }
        (2000, 565) => {
            String::from("ClanVanguards")
        }
        (2000, 278) => {
            String::from("ProjectileDamageModifier")
        }
        (2000, 581) => {
            String::from("NPCScriptAMSScale")
        }
        (2000, 505) => {
            String::from("QTKillNumMonsterID2")
        }
        (2000, 591) => {
            String::from("DistrictNanoInterval")
        }
        (2000, 200) => {
            String::from("Conformity")
        }
        (2000, 511) => {
            String::from("Tower_NPCHash")
        }
        (2000, 101) => {
            String::from("MeleeMultiple")
        }
        (2000, 99) => {
            String::from("ItemAnim")
        }
        (2000, 464) => {
            String::from("DurationModifier")
        }
        (2000, 219) => {
            String::from("ReflectFireAC")
        }
        (2000, 273) => {
            String::from("SecondaryItemTemplate")
        }
        (2000, 135) => {
            String::from("DisarmTrap")
        }
        (2000, 313) => {
            String::from("TimeSinceUpkeep")
        }
        (2000, 543) => {
            String::from("FactionModifier")
        }
        (2000, 96) => {
            String::from("PoisonAC")
        }
        (2000, 184) => {
            String::from("BuildingType")
        }
        (2000, 628) => {
            String::from("PercentChemicalDamage")
        }
        (2000, 31) => {
            String::from("Face")
        }
        (2000, 44) => {
            String::from("ParentInstance")
        }
        (2000, 258) => {
            String::from("AccessGrant")
        }
        (2000, 362) => {
            String::from("ResurrectDest")
        }
        (2000, 428) => {
            String::from("CastEffectType")
        }
        (2000, 72) => {
            String::from("ItemType")
        }
        (2000, 148) => {
            String::from("Burst")
        }
        (2000, 605) => {
            String::from("NPPerSkill")
        }
        (2000, 11) => {
            String::from("PreviousHealth")
        }
        (2000, 30) => {
            String::from("Can")
        }
        (2000, 55) => {
            String::from("InventoryId")
        }
        (2000, 232) => {
            String::from("ShieldNanoAC")
        }
        (2000, 487) => {
            String::from("PetReqVal3")
        }
        (2000, 57) => {
            String::from("LastXP")
        }
        (2000, 86) => {
            String::from("AreaType")
        }
        (2000, 120) => {
            String::from("PhysicalProwessInitiative")
        }
        (2000, 134) => {
            String::from("LR_MultipleWeapon")
        }
        (2000, 289) => {
            String::from("TrapDifficulty")
        }
        (2000, 526) => {
            String::from("PercentRemainingNano")
        }
        (2000, 70) => {
            String::from("SpeedPenalty")
        }
        (2000, 181) => {
            String::from("MaxNCU")
        }
        (2000, 354) => {
            String::from("AttackType")
        }
        (2000, 485) => {
            String::from("PetReqVal1")
        }
        (2000, 303) => {
            String::from("ClanUpkeep")
        }
        (2000, 7) => {
            String::from("State")
        }
        (2000, 212) => {
            String::from("MaxEnergy")
        }
        (2000, 214) => {
            String::from("CurrentNano")
        }
        (2000, 341) => {
            String::from("XPBonus")
        }
        (2000, 510) => {
            String::from("QuestTimeout")
        }
        (2000, 636) => {
            String::from("TrackPoisonDamage")
        }
        (2000, 355) => {
            String::from("NanoFocusLevel")
        }
        (2000, 37) => {
            String::from("TitleLevel")
        }
        (2000, 207) => {
            String::from("ReflectEnergyAC")
        }
        (2000, 367) => {
            String::from("VisualBreed")
        }
        (2000, 473) => {
            String::from("FixtureFlags")
        }
        (2000, 431) => {
            String::from("SelectedTarget")
        }
        (2000, 93) => {
            String::from("ChemicalAC")
        }
        (2000, 85) => {
            String::from("UserInstance")
        }
        (2000, 112) => {
            String::from("Pistol")
        }
        (2000, 185) => {
            String::from("BuildingInstance")
        }
        (2000, 81) => {
            String::from("PrimaryItemInstance")
        }
        (2000, 613) => {
            String::from("LockDownTime")
        }
        (2000, 392) => {
            String::from("OldTimeExist")
        }
        (2000, 29) => {
            String::from("DMS")
        }
        (2000, 414) => {
            String::from("ImpactEffectType")
        }
        (2000, 445) => {
            String::from("NPCCommandArg")
        }
        (2000, 490) => {
            String::from("OriginatorType")
        }
        (2000, 196) => {
            String::from("ConflictReputation")
        }
        (2000, 6) => {
            String::from("Team")
        }
        (2000, 460) => {
            String::from("EffectRed")
        }
        (2000, 484) => {
            String::from("ProximityRangeIndoors")
        }
        (2000, 117) => {
            String::from("DriveWater")
        }
        (2000, 562) => {
            String::from("OTMed")
        }
        (2000, 557) => {
            String::from("ProcChance2")
        }
        (2000, 629) => {
            String::from("TotalDamage")
        }
        (2000, 320) => {
            String::from("BreedLimit")
        }
        (2000, 495) => {
            String::from("QuestIndex4")
        }
        (2000, 227) => {
            String::from("ShieldMeleeAC")
        }
        (2000, 571) => {
            String::from("ClanConserver")
        }
        (2000, 608) => {
            String::from("ShopRent")
        }
        (2000, 208) => {
            String::from("ReflectChemicalAC")
        }
        (2000, 264) => {
            String::from("Brawl2Weapon")
        }
        (2000, 192) => {
            String::from("ExtenalPlayfieldInstance")
        }
        (2000, 267) => {
            String::from("RiposteWeapon")
        }
        (2000, 380) => {
            String::from("RangeIncreaserWeapon")
        }
        (2000, 456) => {
            String::from("CommandRange")
        }
        (2000, 508) => {
            String::from("QTKillNumMonsterCount3")
        }
        (2000, 620) => {
            String::from("HouseTemplate")
        }
        (2000, 641) => {
            String::from("DistanceToSpawnpoint")
        }
        (2000, 180) => {
            String::from("CurrentNCU")
        }
        (2000, 595) => {
            String::from("TempSavePlayfield")
        }
        (2000, 92) => {
            String::from("EnergyAC")
        }
        (2000, 1) => {
            String::from("Life")
        }
        (2000, 501) => {
            String::from("AnimPlay")
        }
        (2000, 999) => {
            String::from("StreamCheckMagic")
        }
        (2000, 327) => {
            String::from("Taboo")
        }
        (2000, 420) => {
            String::from("AmmoType")
        }
        (2000, 143) => {
            String::from("Riposte")
        }
        (2000, 248) => {
            String::from("BirthDate")
        }
        (2000, 256) => {
            String::from("MissionBits1")
        }
        (2000, 294) => {
            String::from("ItemDelay")
        }
        (2000, 51) => {
            String::from("AggDef")
        }
        (2000, 127) => {
            String::from("MaterialMetamorphose")
        }
        (2000, 514) => {
            String::from("OwnedTowers")
        }
        (2000, 10) => {
            String::from("ProfessionLevel")
        }
        (2000, 183) => {
            String::from("EffectIcon")
        }
        (2000, 600) => {
            String::from("NewbieHP")
        }
        (2000, 211) => {
            String::from("EquipDelay")
        }
        (2000, 308) => {
            String::from("MemberInstance")
        }
        (2000, 146) => {
            String::from("SneakAttack")
        }
        (2000, 168) => {
            String::from("NanoAC")
        }
        (2000, 262) => {
            String::from("ClientActivated")
        }
        (2000, 189) => {
            String::from("ExitInstance")
        }
        (2000, 139) => {
            String::from("DriveAir")
        }
        (2000, 154) => {
            String::from("Dodge")
        }
        (2000, 65) => {
            String::from("HairTexture")
        }
        (2000, 147) => {
            String::from("FastAttack")
        }
        (2000, 382) => {
            String::from("SkillLockModifier")
        }
        (2000, 374) => {
            String::from("BurstRecharge")
        }
        (2000, 637) => {
            String::from("TrackFireDamage")
        }
        (2000, 41) => {
            String::from("FabricType")
        }
        (2000, 407) => {
            String::from("NanoPoints")
        }
        (2000, 461) => {
            String::from("EffectGreen")
        }
        (2000, 158) => {
            String::from("WeaponSmithing")
        }
        (2000, 17) => {
            String::from("Agility")
        }
        (2000, 160) => {
            String::from("NanoProgramming")
        }
        (2000, 67) => {
            String::from("HairColourRGB")
        }
        (2000, 596) => {
            String::from("TempSaveX")
        }
        (2000, 597) => {
            String::from("TempSaveY")
        }
        (2000, 889) => {
            String::from("TargetDistanceChange")
        }
        (2000, 32) => {
            String::from("HairMesh")
        }
        (2000, 259) => {
            String::from("DoorFlags")
        }
        (2000, 149) => {
            String::from("NanoProwessInitiative")
        }
        (2000, 448) => {
            String::from("VicinityRange")
        }
        (2000, 459) => {
            String::from("ODMinSizeAdd")
        }
        (2000, 559) => {
            String::from("ProcChance4")
        }
        (2000, 124) => {
            String::from("Treatment")
        }
        (2000, 604) => {
            String::from("NPLevelUp")
        }
        (2000, 238) => {
            String::from("AbsorbProjectileAC")
        }
        (2000, 522) => {
            String::from("LastRnd")
        }
        (2000, 512) => {
            String::from("PetType")
        }
        (2000, 621) => {
            String::from("PercentFireDamage")
        }
        (507, 99_004_953) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("The Quality Level of the Controller is outside the level range of this area! The lowest level controller this area can hold is {a} and the highest {b}.")
        }
        (507, 180_544_675) => {
            let a = &arguments[0];
            format!("This item is not usable in 75{a}uppression gas.")
        }
        (507, 163_210_435) => {
            String::from("seconds")
        }
        (507, 73_417_228) => {
            String::from("You are too high level to engage in a fight with this tower.\r\n")
        }
        (507, 119_666_357) => {
            String::from("You are trying to build too close to another tower.")
        }
        (507, 64_087_156) => {
            String::from("The defense shield is not enabled.")
        }
        (507, 166_313_474) => {
            String::from("Oops. An unknown error occured while placing this tower.")
        }
        (507, 246_682_036) => {
            String::from("Your organization can't have any more active controllers.")
        }
        (507, 173_058_612) => {
            String::from("You have successfully built a new tower.")
        }
        (507, 206_524_449) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("You volunteered to remove your controller from this area. You will have to wait {a} {b} before you can place another controller here.")
        }
        (507, 35_264_956) => {
            String::from("The defense shield disabler item will have to be of at least the Quality Level of the controller in this district.\r\n")
        }
        (507, 182_794_066) => {
            String::from("\r\n<font color=CCCCHeaderColor>You own this tower!</font>")
        }
        (507, 91_048_564) => {
            String::from("No more towers can be built in this area.")
        }
        (507, 112_571_019) => {
            String::from("Your organization rank does not allow you to build a controller.")
        }
        (507, 96_322_805) => {
            String::from("You can not place a tower here. Another organization controls this area.")
        }
        (507, 107_062_452) => {
            String::from("You can not settle any more permanent towers at your level.")
        }
        (507, 67_488_435) => {
            String::from("minutes")
        }
        (507, 204_107_045) => {
            String::from("There already is a controller in this area. Destroy that first to place another.")
        }
        (507, 208_892_434) => {
            let a = &arguments[0];
            format!("\r\n<font color=CCInfoHeader>Owner:</font>{a}")
        }
        (507, 227_013_842) => {
            String::from("You must have the \"Notum Wars\" booster pack installed to build landcontrol towers.")
        }
        (507, 68_317_620) => {
            String::from("Your organization can't have any more controllers of this type.")
        }
        (507, 19_058_661) => {
            String::from("The area is unstable. You cannot build a tower now.")
        }
        (507, 100_446_316) => {
            String::from("You don't meet the skill requirement to build another controller.")
        }
        (507, 45_410_946) => {
            String::from("You need to be a member of an organization to build a tower.")
        }
        (507, 251_324_914) => {
            String::from("Oops. An unknown error occured.")
        }
        (507, 178_651_879) => {
            String::from("This item has to be used in a district with a controller belonging to another organization.\r\n")
        }
        (507, 77_694_884) => {
            String::from("Land control is not allowed in this district.")
        }
        (507, 115_922_734) => {
            String::from("This position is not suitable for placing a tower.")
        }
        (507, 236_429_923) => {
            String::from("You cannot place a tower here. The area of effect from this tower overlaps with 3 other effects. Move to another spot.")
        }
        (507, 14_558_980) => {
            String::from("The defense shield can be disabled using the item.\r\n")
        }
        (507, 77_073_173) => {
            String::from("You can not place a tower here, as your organization has no controller in this area.")
        }
        (507, 88_758_034) => {
            String::from("You can't build a tower that is outside the level range of this district.")
        }
        (507, 117_310_593) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Your organization has just lost the area, and must wait {a} {b} before you can place another controller here.")
        }
        (507, 247_833_570) => {
            String::from("A data error occurred; please report this with your current position in Rubi-Ka!\r\n")
        }
        (2005, 9) => {
            String::from("blue")
        }
        (2005, 6) => {
            String::from("gm")
        }
        (2005, 4) => {
            String::from("advisor")
        }
        (2005, 3) => {
            String::from("monster")
        }
        (2005, 8) => {
            String::from("red")
        }
        (2005, 2) => {
            String::from("omni")
        }
        (2005, 7) => {
            String::from("mixed")
        }
        (2005, 1) => {
            String::from("clan")
        }
        (2005, 5) => {
            String::from("guardian")
        }
        (2005, 0) => {
            String::from("neutral")
        }
        (2006, 2) => {
            String::from("fat")
        }
        (2006, 0) => {
            String::from("thin")
        }
        (2006, 1) => {
            String::from("none")
        }
        (102, 91_170_781) => {
            String::from("You need to be in a team to use a team mission booth.")
        }
        (102, 246_920_037) => {
            String::from("This mission can not be deleted.")
        }
        (508, 259_056_068) => {
            String::from("You have already cast your vote. You cannot vote more than once on this matter.")
        }
        (508, 233_638_763) => {
            let a = &arguments[0];
            format!("{a} more members need to kick player.")
        }
        (508, 247_974_149) => {
            String::from("The target is not from your side.  All players must have the same side in your organization.")
        }
        (508, 106) => {
            String::from("disband your organization")
        }
        (508, 81_177_367) => {
            String::from("There is no voting currently in progress.")
        }
        (508, 89_226_052) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("Your leader, {a}, just changed the organizational tax. The new tax is {b} credits (the old value was {c}).")
        }
        (508, 179_440_135) => {
            String::from("No voting in progress.")
        }
        (508, 113_969_922) => {
            String::from("  The attacker is not a member of an organization.")
        }
        (508, 143_442_663) => {
            let a = &arguments[0];
            format!("Org Message: Your leader, {a} has disbanded your organization.")
        }
        (508, 147_071_208) => {
            let a = &arguments[0];
            format!("GM removed character {a} from your organization.")
        }
        (508, 61_873_061) => {
            String::from("Previous voting results...\r\n")
        }
        (508, 147_575_765) => {
            String::from(" Previous voting results...\r\n")
        }
        (508, 153_045_876) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("You are not allowed to {a} while suspended. You have {b} minutes left on your suspension.¨\r\nYour debt is {c} credits.\r\nNOTE: Your debt is automatically paid back at the end of your suspension time when you have enough credits.")
        }
        (508, 100) => {
            String::from("use this command")
        }
        (508, 145_730_516) => {
            let a = &arguments[0];
            format!(":Your Organization cannot initiate a bombing raid at the moment. Please wait at least {a} seconds.")
        }
        (508, 122) => {
            String::from("show your debt to the organization")
        }
        (508, 182_058_216) => {
            let a = &arguments[0];
            format!("Your debt to the organization is {a} credits.")
        }
        (508, 197_560_331) => {
            let a = &arguments[0];
            format!("Governing form: {a}\r\nRanks: ")
        }
        (508, 120_551_141) => {
            let a = &arguments[0];
            format!("{a} Voting can begin in a few seconds. Use '/org vote <voteentry id>' to cast your vote.")
        }
        (508, 121) => {
            String::from("distribute org bank credits to members")
        }
        (508, 37_093_479) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("{a} kicked {b} from your organization.")
        }
        (508, 210_090_949) => {
            String::from("GM overriding auto-leadership-election.")
        }
        (508, 34_568_918) => {
            String::from("Only the leader can change the governing form of the organization. The leader can not be a free player.")
        }
        (508, 75_378_580) => {
            String::from(" Inactive character removed from your organization.")
        }
        (508, 5_931_833) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = {
                if b.to_string() == "1" {
                    "vote "
                } else {
                    " votes"
                }
            };
            format!("{a} ( {b} {c} ):")
        }
        (508, 162_192_978) => {
            String::from("You cannot leave the organization without appointing another leader first!")
        }
        (508, 16_378_820) => {
            let a = &arguments[0];
            format!("Results for \"{a}\"...\r\nStatus: ")
        }
        (508, 165_930_356) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Organizational tax changed from {a} to {b} credits.")
        }
        (508, 142_934_233) => {
            let a = &arguments[0];
            format!(" Your rank is {a}.")
        }
        (508, 111) => {
            String::from("demote a member")
        }
        (508, 185_052_340) => {
            String::from("Election for new leader failed and a new leader could not be automatically assigned. Petition a GM to assign a new leader.")
        }
        (508, 105) => {
            String::from("get organization info")
        }
        (508, 76_010_659) => {
            String::from("Voting choices contain duplicates. Choices must be unique.")
        }
        (508, 44_637_365) => {
            String::from("The specified character name is not valid.")
        }
        (508, 206) => {
            String::from("Character already an org member or the org's rank group is full.")
        }
        (508, 15_846_784) => {
            String::from("The vote options need to be one word only preceded by the choice number, like 1:Yes 2:No. You can not include a sentence. You need to include at least two votable options. The maximum is ten.")
        }
        (508, 116) => {
            String::from("leave your organization")
        }
        (508, 120) => {
            String::from("take money out of the organization bank")
        }
        (508, 2) => {
            String::from("Third place")
        }
        (508, 216) => {
            String::from("Invalid error code.")
        }
        (508, 65_798_404) => {
            String::from("Voting can begin in a few seconds. Use '/org vote info' to show the voting status.")
        }
        (508, 17_467_336) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            format!("Your controller tower in {a} in {b} has had its defense shield disabled by {c} ({d}).")
        }
        (508, 68_628_373) => {
            String::from("The legal vote time is between 2 and 10080 minutes. ( 10080 minutes is 1 week ).")
        }
        (508, 112_284_196) => {
            let a = &arguments[0];
            let b = {
                if a.to_string() == "1" {
                    "member "
                } else {
                    " members"
                }
            };
            format!("{a} {b} voted.")
        }
        (508, 95_062_979) => {
            String::from("\r\nStatus: ")
        }
        (508, 103) => {
            String::from("open the organization contract container")
        }
        (508, 123_361_268) => {
            String::from("Organization members have been informed. ")
        }
        (508, 124_161_096) => {
            let a = &arguments[0];
            format!("  The attacker is a member of the organization {a}.")
        }
        (508, 130_903_587) => {
            String::from("Only the leader is allowed to change the description.")
        }
        (508, 138_965_334) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("{a} changed governing form to '{b}'.")
        }
        (508, 173_558_247) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("{a} invited {b} to your organization.")
        }
        (508, 215) => {
            String::from("Unknown error.")
        }
        (508, 187_598_404) => {
            String::from("Not ready to accept votes yet. Voting can begin in a few seconds.")
        }
        (508, 6) => {
            String::from("Seventh place")
        }
        (508, 198_384_040) => {
            String::from("The specified tax is not allowed.")
        }
        (508, 33_408_158) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            format!("{a}, {b} and {c} kicked {d} from the organization.")
        }
        (508, 88_959_635) => {
            String::from("No votes were cast.")
        }
        (508, 181_849_080) => {
            let a = &arguments[0];
            format!(" You have voted for {a}")
        }
        (508, 34_648_027) => {
            String::from("The defense shield is disabled.  The towers are attackable!")
        }
        (508, 119_211_733) => {
            String::from("Only the leader or a GM can stop the voting for your organization.")
        }
        (508, 149_256_015) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            format!("<font color=CCInfoHeader>In: </font><font color=CCInfoText>\"{a}\"</font>\r\n<font color=CCInfoHeader>Area: </font><font color=CCInfoText>{b}</font>\r\n<font color=CCInfoHeader>Type: </font><font color=CCInfoText>{c}</font>\r\n<font color=CCInfoHeader>Level: </font><font color=CCInfoText>{d}</font>")
        }
        (508, 3) => {
            String::from("Fourth place")
        }
        (508, 201_631_363) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("\r\n<font color=CCInfoHeader>Controlled areas: </font><font color=CCInfoText>Currently in control of {a} out of max {b} areas.</font>\r\n")
        }
        (508, 222_804_051) => {
            String::from("Only leaders can take credits out of the organization.")
        }
        (508, 233_483_144) => {
            let a = &arguments[0];
            format!("Could not start vote for new leader! {a} has been appointed as leader of this organization.")
        }
        (508, 63_867_572) => {
            String::from("Only the leader or a GM can disband your organization.")
        }
        (508, 9) => {
            String::from("Tenth place")
        }
        (508, 110) => {
            String::from("promote a member")
        }
        (508, 118) => {
            String::from("show amount of cash in the organization bank")
        }
        (508, 211) => {
            String::from("Cannot kick the leader.")
        }
        (508, 176_308_692) => {
            let a = &arguments[0];
            format!("Blammo! {a} has launched an orbital attack!")
        }
        (508, 181_351_343) => {
            String::from("Your rank is not high enough for you to be able to invite new members. You must be at least 3 ranks above the lowest rank in your organization.  (The new member entry rank.) Use \"/org ranks\" to see the available ranks in your organization.")
        }
        (508, 192_685_736) => {
            String::from("Only leaders can change the organization tax.")
        }
        (508, 201_946_636) => {
            let a = &arguments[0];
            format!("You cannot demote {a} to that rank because it is filled up.")
        }
        (508, 113) => {
            String::from("kick member by name")
        }
        (508, 67_129_906) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("You demoted {a} to the rank of {b}.")
        }
        (508, 254_100_788) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Insufficient credits ({a}) available in the organization bank to pay the city upkeep of {b} credits. Your city has been demolished.")
        }
        (508, 109) => {
            String::from("cast your vote")
        }
        (508, 139_452_332) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("You cannot promote {a} to that rank because it is filled up. Please demote someone from rank {b} first...")
        }
        (508, 5) => {
            String::from("Sixth place")
        }
        (508, 126) => {
            String::from("change organization name")
        }
        (508, 174_854_260) => {
            String::from("Tax owed to your organization has not been paid. You are now on suspension for 30 minutes.")
        }
        (508, 212) => {
            String::from("No character ( active or inactive ) was kicked.")
        }
        (508, 190_422_101) => {
            String::from(" You did not vote.")
        }
        (508, 110_670_569) => {
            String::from("Not ready to accept votes yet. Voting can begin in a few seconds.")
        }
        (508, 121_757_266) => {
            String::from("Vote for new leader.")
        }
        (508, 213) => {
            String::from("Rank out of range or rank level saturated.")
        }
        (508, 97_773_172) => {
            String::from(" You have not voted yet.\r\nUse '/org vote &lt;voteentry id&gt;' to cast your vote.")
        }
        (508, 202) => {
            String::from("Name given has no engine ID.")
        }
        (508, 77_728_843) => {
            let a = &arguments[0];
            format!("<font color=CCInfoHeader>Organization Rank: </font><fontcolor=CCInfoText>{a}</font>\r\n")
        }
        (508, 165_999_076) => {
            String::from("Governing form changed for the organization.")
        }
        (508, 184_602_788) => {
            String::from("Voting has finished.")
        }
        (508, 242_017_255) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("Online members will receive {a} credits immediately. Members not online will receive {b} credits when they login. Suspended members will have their debt reduced by {c} credits.")
        }
        (508, 10_672_340) => {
            String::from("Voting has finished. Results will be shown in a few seconds.")
        }
        (508, 71_822_052) => {
            String::from("Only leaders can pay out organization credits to members.")
        }
        (508, 89_364_632) => {
            String::from("You cannot add more credits than you own.")
        }
        (508, 207) => {
            String::from("Character is already in an organization - can only be a member of one org at a time.")
        }
        (508, 166_906_756) => {
            let a = &arguments[0];
            format!("Organization leader has stopped the voting with message : \"{a}\"")
        }
        (508, 213_809_944) => {
            let a = &arguments[0];
            format!("Your vote has been cast. You voted for {a}")
        }
        (508, 5_146_599) => {
            let a = &arguments[0];
            format!("{a} has joined your organization.")
        }
        (508, 240_339_044) => {
            let a = &arguments[0];
            format!("Your Organization cannot initiate a laser strike at the moment. Please wait at least {a} seconds.")
        }
        (508, 124) => {
            String::from("change organization objective")
        }
        (508, 127) => {
            String::from("change organization governing form")
        }
        (508, 54_457_652) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = {
                if b.to_string() == "1" {
                    "vote "
                } else {
                    " votes"
                }
            };
            format!("{a} ( {b} {c} each ):")
        }
        (508, 119) => {
            String::from("add money to the organization bank")
        }
        (508, 172_832_727) => {
            String::from("You've joined the organization.")
        }
        (508, 34_521_794) => {
            String::from("You are not a member of an organization.")
        }
        (508, 112_019_568) => {
            String::from(" Do a '/org ranks' to know the ranks of your hierarchy. ")
        }
        (508, 217_117_108) => {
            String::from("Organization disbanded.")
        }
        (508, 195_442_548) => {
            String::from("Voting has finished. Results will be shown in a few seconds.")
        }
        (508, 225_271_141) => {
            String::from("You must target another player to invite.")
        }
        (508, 236_103_944) => {
            let a = &arguments[0];
            format!("You were just promoted by a GM to the rank of {a}.")
        }
        (508, 5_429_092) => {
            String::from("The specified amount of credits is not allowed.")
        }
        (508, 178_191_954) => {
            String::from("Could not understand voting data. First parameter should be a description like \"Vote for new leader.\".")
        }
        (508, 192_652_356) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("{a} credits have been deducted from the organization bank for the upkeep of your city. Next payment is due in {b} days.")
        }
        (508, 33_183_704) => {
            let a = &arguments[0];
            format!("GM kicked {a} from your organization.")
        }
        (508, 168_300_623) => {
            String::from("Voting is curently in progress. You cannot start voting on a new issue until the current voring has finished. If you enter /stopvote, you cancel the current voting without knowing the result.")
        }
        (508, 45_978_487) => {
            let a = &arguments[0];
            format!("{a} just left your organization.")
        }
        (508, 114_148_739) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("There is a minumum period of {a} minutes between voting. Voting may be started in {b} minutes from now.")
        }
        (508, 206_008_469) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("Voting notice for \"{a}\"...\r\nDuration: {b} minutes\r\nCandidates: {c}\r\nVoting can begin in a few seconds. Use '/org vote <voteentry id>' to cast your vote.\r\n")
        }
        (508, 208_492_008) => {
            String::from("show organization tax")
        }
        (508, 241_047_288) => {
            let a = &arguments[0];
            format!("Leadership has been given to {a}.")
        }
        (508, 114) => {
            String::from("invite a player to your organization")
        }
        (508, 105_314_766) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("You were just promoted by {a} to the rank of {b}.")
        }
        (508, 236_096_388) => {
            String::from("Tax owed to your organization has been paid. ( If you were on suspension, you have been reinstated. )")
        }
        (508, 7) => {
            String::from("Eighth place")
        }
        (508, 112) => {
            String::from("kick selected member")
        }
        (508, 4) => {
            String::from("Fifth place")
        }
        (508, 127_838_135) => {
            String::from("You couldn't join the organization.")
        }
        (508, 130_975_365) => {
            String::from("Only the leader is allowed to change the name.")
        }
        (508, 108) => {
            String::from("request current active vote info")
        }
        (508, 131_644_071) => {
            String::from("You must target a player of your own organization to demote her/him.")
        }
        (508, 209) => {
            String::from("Character not a member.")
        }
        (508, 98_440_303) => {
            String::from("Voting should have a minimum of 2 and a maximum of 10 choices.")
        }
        (508, 261_298_643) => {
            String::from("Only the leader can kick this character.")
        }
        (508, 201) => {
            String::from("Target's org ID is invalid.")
        }
        (508, 214) => {
            String::from("Invalid characters in command.")
        }
        (508, 45_846_567) => {
            String::from("You've left the organization.")
        }
        (508, 119_220_807) => {
            let a = &arguments[0];
            format!("Org Message: The GM {a} has disbanded your organization.")
        }
        (508, 179_467_989) => {
            String::from("Could not understand vote. Example uses : /org vote info, /org vote 1")
        }
        (508, 197_548_405) => {
            String::from("Only the leader is allowed to change the objective.")
        }
        (508, 268_283_653) => {
            String::from("Voting could not be stopped.")
        }
        (508, 129) => {
            String::from("pay tax")
        }
        (508, 116_442_029) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Leadership has been given to {a} by GM {b}.")
        }
        (508, 67_317_284) => {
            let a = &arguments[0];
            let b = {
                if a.to_string() == "1" {
                    "minute "
                } else {
                    " minutes"
                }
            };
            format!("{a} {b} remaining.")
        }
        (508, 192_779_843) => {
            String::from("Your organization already owns a city. You only have limited access to this city terminal.")
        }
        (508, 20_908_201) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("{a} removed inactive character {b} from your organization.")
        }
        (508, 102) => {
            String::from("get organization rank info")
        }
        (508, 108_633_128) => {
            let a = &arguments[0];
            format!("You were deducted {a} credits organization tax.")
        }
        (508, 101) => {
            String::from("create a new organization")
        }
        (508, 53_724_837) => {
            let a = &arguments[0];
            format!("There is currently no active character called '{a}'.")
        }
        (508, 89_144_094) => {
            let a = &arguments[0];
            format!("Voting information for \"{a}\"...\r\nCandidates: ")
        }
        (508, 78_644_776) => {
            let a = &arguments[0];
            format!("Organization bank account: {a} credits.")
        }
        (508, 258_658_757) => {
            String::from("Only the leader or a GM can initiate voting for your organization.")
        }
        (508, 28_601_677) => {
            String::from("Your organization has been disbanded ( by a GM ).")
        }
        (508, 131_953_188) => {
            String::from("Voting stopped.")
        }
        (508, 177_338_229) => {
            let a = &arguments[0];
            format!("Organization leader has stopped the voting with message \"{a}\"")
        }
        (508, 104) => {
            String::from("close the organization contract container")
        }
        (508, 205_958_398) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("You were just demoted by {a} to the rank of {b}.")
        }
        (508, 137_012_854) => {
            String::from("Invalid governing form. Valid governing forms are:\r\n")
        }
        (508, 187_320_549) => {
            String::from("You cannot vote because you joined this organization after voting began.")
        }
        (508, 248_298_914) => {
            let a = &arguments[0];
            format!("{a} is not a member.")
        }
        (508, 117) => {
            String::from("set new organization tax for members")
        }
        (508, 6_568_276) => {
            String::from("Requested display of advantages.")
        }
        (508, 113_648_514) => {
            String::from("Target is not a member of an organization.")
        }
        (508, 131) => {
            String::from("show city advantages held by the organization")
        }
        (508, 33_524_638) => {
            let a = &arguments[0];
            format!("Only the leader is allowed to {a}.")
        }
        (508, 125) => {
            String::from("change organization description")
        }
        (508, 109_839_519) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            let e = &arguments[4];
            let f = &arguments[5];
            format!("<font color=CCInfoHeadline>Organization information</font>\r\n<font color=CCInfoHeader>Name: </font><font color=CCInfoText>{a}</font>\r\n<font color=CCInfoHeader>Leader: </font><font color=CCInfoText>{b}</font>\r\n<font color=CCInfoHeader>GoverningForm: </font><font color=CCInfoText>{c}</font>\r\n<font color=CCInfoHeader>Description: </font><font color=CCInfoText>{d}</font>\r\n<font color=CCInfoHeader>Objective: </font><font color=CCInfoText>{e}</font>\r\n<font color=CCInfoHeader>History: </font><font color=CCInfoText>{f}</font>\r\n")
        }
        (508, 111_800_288) => {
            String::from("Could not understand voting data. Usage : /org startvote <desc> <duration> <choices>. e.g. /org startvote \"Keep me as leader?\" 60 1:yes 2:no\r\n")
        }
        (508, 208) => {
            String::from("Target is not an a member.")
        }
        (508, 140_104_245) => {
            String::from(" You cannot vote because you joined this organization after voting began.")
        }
        (508, 196_585_349) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Your city upkeep payment of {a} credits is due in {b} hour(s). Please make sure the full upkeep amount is available in the organization bank or you will lose your city.")
        }
        (508, 0) => {
            String::from("First place")
        }
        (508, 204) => {
            String::from("Charcter not on same side as target's org.")
        }
        (508, 224_583_380) => {
            let a = &arguments[0];
            format!("Members were deducted {a} credits each to pay your organizational debt.")
        }
        (508, 87_033_703) => {
            let a = &arguments[0];
            format!("{a} is already member of another organization. Players must leave their current organizations before joining another.")
        }
        (508, 135_224_629) => {
            String::from("This organization has a governing form with no structure for voting.")
        }
        (508, 173_242_677) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Voting notice for \"{a}\"\r\nDuration: {b}\r\nCandidates: ")
        }
        (508, 203_390_621) => {
            String::from("You must target a player of your own organization to promote her/him.")
        }
        (508, 203) => {
            String::from("Target ID is not a player character.")
        }
        (508, 182_683_299) => {
            String::from("You can only demote someone who is one rank-level below your own level.")
        }
        (508, 231_762_094) => {
            String::from("Voting for new leader can begin in a few seconds.")
        }
        (508, 115) => {
            String::from("accept invitiation to the organization")
        }
        (508, 1) => {
            String::from("Second place")
        }
        (508, 123) => {
            String::from("change organization history")
        }
        (508, 186_172_214) => {
            String::from("You can't kick yourself. Use '/org leave' instead.")
        }
        (508, 237_299_844) => {
            String::from("Your rank in this organization is too low to view bank account details.")
        }
        (508, 83_761_406) => {
            String::from("Your organization leader is no longer an active character. An election for a new leader will commence shortly...")
        }
        (508, 44_673_573) => {
            String::from("Invalid voting parameters.")
        }
        (508, 18_639_941) => {
            String::from("Could not start voting.")
        }
        (508, 128) => {
            String::from("end voting")
        }
        (508, 130) => {
            String::from("avoid tax")
        }
        (508, 210) => {
            String::from("Target is not the org leader.")
        }
        (508, 134_643_352) => {
            let a = &arguments[0];
            format!("Governing form changed to '{a}'.")
        }
        (508, 147_506_147) => {
            let a = &arguments[0];
            format!("\"{a}\"...")
        }
        (508, 147_882_695) => {
            let a = &arguments[0];
            format!("The current level of organizational tax is {a}.")
        }
        (508, 167_722_847) => {
            String::from("You have to be the leader, or someone 3 organizational ranks above the player to kick him from the organization. Do /org ranks to see a list of the available ranks in your organization. If your organization does not support many organizational ranks, use /org contract to change the organizational hierarchy.")
        }
        (508, 224_259_411) => {
            String::from("You can only promote someone to one rank-level below your own level (The exception is the leader, who - when a new leader is appointed through promotion - will lose the leadership). Free players can not be organization leaders.\r\nType '/org ranks' to list the ranks of your hierarchy.\r\n")
        }
        (508, 200) => {
            String::from("Operation was successful.")
        }
        (508, 39_868_162) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("You promoted {a} to the rank of {b}.")
        }
        (508, 52_554_313) => {
            String::from("Only the leader is allowed to change the history.")
        }
        (508, 163_777_230) => {
            String::from("Result is corrupt. ( found invalid vote )")
        }
        (508, 99_189_171) => {
            let a = &arguments[0];
            format!("The number [{a}] exceeds the maximum number of options on vote. Please do a '/org vote info' for details.\r\n")
        }
        (508, 110_113_751) => {
            String::from("You're not in an organization.")
        }
        (508, 107) => {
            String::from("start a new vote")
        }
        (508, 8) => {
            String::from("Ninth place")
        }
        (508, 205) => {
            String::from("Character not in an organization.")
        }
        (508, 230_985_621) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("{a}, {b} and {c} kicked you from the organization.")
        }
        (508, 210_489_620) => {
            let a = &arguments[0];
            format!("You cannot withdraw more credits from the organizational bank account than is currently in it. The account holds {a} credits right now.")
        }
        (101, 8) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("You unlearned a Perk {a} hours ago. Perks can only be unlearned once every {b} hours.")
        }
        (101, 113_533_356) => {
            String::from("Your target is normally not allowed to attack you, due to level restrictions in PvP.  Would you still like to attack?")
        }
        (101, 2) => {
            String::from("Target can't be polymorphed when doing this..")
        }
        (101, 10) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("You lost faction with {a}: {b} faction points.")
        }
        (101, 1) => {
            String::from("You must be in a different form to perform this action!")
        }
        (101, 12) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("You just decreased your faction standing with {a} to a whole new level! You are now a {b} them!")
        }
        (101, 88_659_692) => {
            let a = &arguments[0];
            format!("Welcome to Shadowlevel {a}.")
        }
        (101, 11) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Congratulations! You just improved your faction standing with {a} to a whole new degree! They now see you as {b} them!")
        }
        (101, 7) => {
            let a = &arguments[0];
            format!("You must have the {a} Perk.")
        }
        (101, 9) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("You gained faction with {a}: {b} faction points.")
        }
        (101, 28_798_749) => {
            String::from("Your target is grouped with someone at a level where they would normally not be allowed to attack you, due to level restrictions in PvP.  Would you still like to attack?")
        }
        (1003, 183_682_302) => {
            String::from("This profession is only available for players with the Shadowlands expansion pack!")
        }
        (1003, 174_528_734) => {
            String::from("Are you sure you want to quit character creation?")
        }
        (2007, 24677) => {
            String::from("You")
        }
        (2007, 1) => {
            String::from("Self")
        }
        (2007, 394_946) => {
            String::from("Your")
        }
        (2007, 94_932_420) => {
            String::from("Target")
        }
        (2007, 2) => {
            String::from("Team")
        }
        (2007, 8) => {
            String::from("Item")
        }
        (2007, 32) => {
            String::from("Same Side")
        }
        (2007, 4) => {
            String::from("Hostile")
        }
        (2007, 16) => {
            String::from("Monster")
        }
        (2008, 57) => {
            String::from("Operator_MonsterTemplate")
        }
        (2008, 10) => {
            String::from("Operator_TargetID")
        }
        (2008, 80) => {
            String::from("Operator_IsTowerCreateAllowed")
        }
        (2008, 91) => {
            String::from("Operator_HasRunningNano")
        }
        (2008, 74) => {
            String::from("Operator_TrickleDownLess")
        }
        (2008, 94) => {
            String::from("Operator_IsPerkLocked")
        }
        (2008, 99) => {
            String::from("Operator_False")
        }
        (2008, 100) => {
            String::from("Operator_OnCaster")
        }
        (2008, 62) => {
            String::from("Operator_IsLocationOk")
        }
        (2008, 4) => {
            String::from("Operator_And")
        }
        (2008, 18) => {
            String::from("Operator_OnTarget")
        }
        (2008, 36) => {
            String::from("Operator_HasNotFormula")
        }
        (2008, 46) => {
            String::from("Operator_IsAttacked")
        }
        (2008, 63) => {
            String::from("Operator_IsNotTooHighLevel")
        }
        (2008, 52) => {
            String::from("Operator_IsInNoFightingArea")
        }
        (2008, 3) => {
            String::from("Operator_Or")
        }
        (2008, 24) => {
            String::from("Operator_Unequal")
        }
        (2008, 11) => {
            String::from("Operator_TargetSignal")
        }
        (2008, 58) => {
            String::from("Operator_HasMaster")
        }
        (2008, 5) => {
            String::from("Operator_Time_Less")
        }
        (2008, 34) => {
            String::from("Operator_HasNotWieldedItem")
        }
        (2008, 98) => {
            String::from("Operator_True")
        }
        (2008, 88) => {
            String::from("Operator_UseLocation")
        }
        (2008, 33) => {
            String::from("Operator_HasWieldedItem")
        }
        (2008, 7) => {
            String::from("Operator_Item_Has")
        }
        (2008, 102) => {
            String::from("Operator_HasNotRunningNanoLine")
        }
        (2008, 67) => {
            String::from("Operator_NumberOfItems")
        }
        (2008, 39) => {
            String::from("Operator_IsInvalid")
        }
        (2008, 43) => {
            String::from("Operator_IsWithinWeaponRange")
        }
        (2008, 27) => {
            String::from("Operator_OnValidTarget")
        }
        (2008, 49) => {
            String::from("Operator_IsInDungeon")
        }
        (2008, 69) => {
            String::from("Operator_IsTeleporting")
        }
        (2008, 20) => {
            String::from("Operator_Signal")
        }
        (2008, 51) => {
            String::from("Operator_DistanceTo")
        }
        (2008, 38) => {
            String::from("Operator_IsValid")
        }
        (2008, 17) => {
            String::from("Operator_ItemAnim")
        }
        (2008, 45) => {
            String::from("Operator_IsFighting")
        }
        (2008, 42) => {
            String::from("Operator_Not")
        }
        (2008, 48) => {
            String::from("Operator_IsFoe")
        }
        (2008, 68) => {
            String::from("Operator_PrimaryTemplate")
        }
        (2008, 14) => {
            String::from("Operator_Secondary_Item")
        }
        (2008, 19) => {
            String::from("Operator_OnSelf")
        }
        (2008, 84) => {
            String::from("Operator_IsNpcOrNpcControlledPet")
        }
        (2008, 41) => {
            String::from("Operator_IsWithinVicinity")
        }
        (2008, 12) => {
            String::from("Operator_TargetStat")
        }
        (2008, 76) => {
            String::from("Operator_HasPetPendingNanoFormula")
        }
        (2008, 85) => {
            String::from("Operator_SameAsSelectedTarget")
        }
        (2008, 59) => {
            String::from("Operator_CanExecuteFormulaOnTarget")
        }
        (2008, 93) => {
            String::from("Operator_HasPerk")
        }
        (2008, 50) => {
            String::from("Operator_IsSameAs")
        }
        (2008, 22) => {
            String::from("Operator_BitAnd")
        }
        (2008, 40) => {
            String::from("Operator_IsAlive")
        }
        (2008, 65) => {
            String::from("Operator_KullNumberOf")
        }
        (2008, 23) => {
            String::from("Operator_BitOr")
        }
        (2008, 35) => {
            String::from("Operator_HasFormula")
        }
        (2008, 77) => {
            String::from("Operator_IsPet")
        }
        (2008, 47) => {
            String::from("Operator_IsAnyoneLooking")
        }
        (2008, 15) => {
            String::from("Operator_Area")
        }
        (2008, 53) => {
            String::from("Operator_Template_Compare")
        }
        (2008, 6) => {
            String::from("Operator_Time_Larger")
        }
        (2008, 44) => {
            String::from("Operator_IsNPC")
        }
        (2008, 61) => {
            String::from("Operator_IsUnderHeavyAttack")
        }
        (2008, 64) => {
            String::from("Operator_HasChangedRoomWhileFighting")
        }
        (2008, 66) => {
            String::from("Operator_TestNumPets")
        }
        (2008, 95) => {
            String::from("Operator_IsFactionReactionSet")
        }
        (2008, 54) => {
            String::from("Operator_Min_Max_Level_Compare")
        }
        (2008, 9) => {
            String::from("Operator_ID")
        }
        (2008, 81) => {
            String::from("Operator_InventorySlotIsFull")
        }
        (2008, 86) => {
            String::from("Operator_IsPlayerOrPlayerControlledPet")
        }
        (2008, 101) => {
            String::from("Operator_HasNotRunningNano")
        }
        (2008, 30) => {
            String::from("Operator_OnInvalidUser")
        }
        (2008, 71) => {
            String::from("Operator_ScanForStat")
        }
        (2008, 87) => {
            String::from("Operator_HasEnteredNonPvpZone")
        }
        (2008, 70) => {
            String::from("Operator_IsFlying")
        }
        (2008, 32) => {
            String::from("Operator_HasNotWornItem")
        }
        (2008, 90) => {
            String::from("Operator_IsOnDifferentPlayfield")
        }
        (2008, 97) => {
            String::from("Operator_IsPerkUnlocked")
        }
        (2008, 29) => {
            String::from("Operator_OnValidUser")
        }
        (2008, 25) => {
            String::from("Operator_Illegal")
        }
        (2008, 79) => {
            String::from("Operator_CanAttackChar")
        }
        (2008, 21) => {
            String::from("Operator_OnSecondaryItem")
        }
        (2008, 26) => {
            String::from("Operator_OnUser")
        }
        (2008, 31) => {
            String::from("Operator_HasWornItem")
        }
        (2008, 96) => {
            String::from("Operator_HasMoveToTarget")
        }
        (2008, 0) => {
            String::from("Operator_Equal")
        }
        (2008, 92) => {
            String::from("Operator_HasRunningNanoLine")
        }
        (2008, 1) => {
            String::from("Operator_Less")
        }
        (2008, 28) => {
            String::from("Operator_OnInvalidTarget")
        }
        (2008, 73) => {
            String::from("Operator_TrickleDownLarger")
        }
        (2008, 60) => {
            String::from("Operator_Area_TargetInVicinity")
        }
        (2008, 75) => {
            String::from("Operator_IsPetOverequipped")
        }
        (2008, 89) => {
            String::from("Operator_IsFalling")
        }
        (2008, 72) => {
            String::from("Operator_HasMeOnPetlist")
        }
        (2008, 13) => {
            String::from("Operator_Primary_Item")
        }
        (2008, 82) => {
            String::from("Operator_InventorySlotIsEmpty")
        }
        (2008, 37) => {
            String::from("Operator_OnGeneralBeholder")
        }
        (2008, 83) => {
            String::from("Operator_CanDisableDefenseShield")
        }
        (2008, 2) => {
            String::from("Operator_Larger")
        }
        (2008, 16) => {
            String::from("Operator_User")
        }
        (2008, 8) => {
            String::from("Operator_Item_HasNot")
        }
        (1001, 4) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("{a} removed the organization headquarters in {b}!  All personal belongings or houses in the city were instantly destroyed!")
        }
        (1001, 1) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("{a} turned the cloaking device in your city {b}.")
        }
        (1001, 3) => {
            let a = &arguments[0];
            format!("Your city in {a} has been targeted by hostile forces.")
        }
        (1001, 1_781_255) => {
            String::from("a disbanded organization")
        }
        (1001, 5) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("{a} has initiated removal of a {b} in {c}!  The house and all belongings in the house will be destroyed in 2 minutes.")
        }
        (1001, 2) => {
            String::from("Your radar station is picking up alien activity in the area surrounding your city.")
        }
        (1001, 174_310_933) => {
            String::from("Someone")
        }
        (1001, 6) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("{a} removed a {b} in {c}!  All benefits from this house and all belongings in the house were instantly destroyed!")
        }
        (1001, 254_018_580) => {
            let a = &arguments[0];
            format!("ICC planet-wide announcement: {a} has been awarded the highest honorary rank for outstanding dedication to the defence of Rubi-Ka.")
        }
        (1001, 7) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("{a} has initiated removal of the HQ in {b}!  The house and all belongings in the house will be destroyed in 2 minutes. This will also cause the other houses in the city to be deleted!")
        }
        (700, 179_922_163) => {
            String::from("strong3")
        }
        (700, 25_586_519) => {
            String::from("Skill Window")
        }
        (700, 144_888_439) => {
            String::from("Chat Config")
        }
        (700, 89_231_255) => {
            String::from("Friends Window")
        }
        (700, 45) => {
            String::new()
        }
        (700, 134_286_644) => {
            String::from("Always decline invitations to private groups")
        }
        (700, 220_670_083) => {
            String::from("Toggle options")
        }
        (700, 81_054_407) => {
            String::from("Info View")
        }
        (700, 75_072_286) => {
            String::from("Show button when receiving new IP/Perk/Technology")
        }
        (700, 103_207_380) => {
            String::from("Vehicle Select Window")
        }
        (700, 147_220_714) => {
            String::from("Ground Render Mode:")
        }
        (700, 76_207_778) => {
            String::from("Contains a list of all friends and chat channels available. You can hide and log chat channels or befriend people you meet here.")
        }
        (700, 136_817_395) => {
            String::from("Build Feedback")
        }
        (700, 164_343_831) => {
            String::from("Settings")
        }
        (700, 148_778_357) => {
            String::from("Right side visible")
        }
        (700, 357_001) => {
            String::from("Play")
        }
        (700, 237_816_517) => {
            String::from("Source Analyzer")
        }
        (700, 174_438_684) => {
            String::from("Controls")
        }
        (700, 255_665_285) => {
            String::from("Prev hostile")
        }
        (700, 166_870_003) => {
            String::from("Take screenshot")
        }
        (700, 117_351_445) => {
            String::from("Sneak/walk toggle")
        }
        (700, 142_222_899) => {
            String::from("Backpacks")
        }
        (700, 75_177_124) => {
            String::from("adjust")
        }
        (700, 152_389_589) => {
            String::from("Controls/Mouse")
        }
        (700, 5_147_588) => {
            String::from("greet")
        }
        (700, 232_278_028) => {
            String::from("Prev friendly")
        }
        (700, 160_186_452) => {
            String::from("Pet Report")
        }
        (700, 179_922_162) => {
            String::from("strong2")
        }
        (700, 157_399_299) => {
            String::from("Disable tips")
        }
        (700, 242_995_248) => {
            String::from("Behaviour/Experience")
        }
        (700, 133_975_715) => {
            String::from("Perk Actions")
        }
        (700, 190_209_267) => {
            String::from("View distance:")
        }
        (700, 214_485_137) => {
            String::from("Gives you a quick overview of your pets.")
        }
        (700, 79_981_251) => {
            String::from("Emotes")
        }
        (700, 67_805_198) => {
            String::from("Mission")
        }
        (700, 75_795_815) => {
            String::from("Inventory Window")
        }
        (700, 170_731_365) => {
            String::from("Chat Configuration")
        }
        (700, 46_390_787) => {
            String::from("Normal Actions")
        }
        (700, 144_409_692) => {
            String::from("3rd Trail")
        }
        (700, 90_676_948) => {
            String::from("NPC Chat")
        }
        (700, 145_586_963) => {
            String::from("Tutorial Tips")
        }
        (700, 255_746_469) => {
            String::from("Next hostile")
        }
        (700, 1472) => {
            String::from("Up")
        }
        (700, 115_725_585) => {
            String::from("Status information")
        }
        (700, 77_085_825) => {
            String::from("Camera")
        }
        (700, 14_097_413) => {
            String::from("This is where you distribute improvement points (IP) and where all your skills and abilities are listed.")
        }
        (700, 31_347_932) => {
            String::from("Open team window, or the team search window.")
        }
        (700, 241_390_579) => {
            String::from("Auto close windows")
        }
        (700, 64_971_746) => {
            String::from("Gender:")
        }
        (700, 5_393_340) => {
            String::from("kneel")
        }
        (700, 212_031_134) => {
            String::from("Show video on commercial billboards")
        }
        (700, 239_514_727) => {
            String::from("Show Left Wing")
        }
        (700, 5_925_812) => {
            String::from("Speed")
        }
        (700, 21_103_649) => {
            String::from("Show selected group in title bar")
        }
        (700, 363_380) => {
            String::from("read")
        }
        (700, 209_894_805) => {
            String::from("Camera Mode")
        }
        (700, 242_986_589) => {
            String::from("Toggle planetmap")
        }
        (700, 82_512_244) => {
            String::from("3rd Person Soft Locked View")
        }
        (700, 174_340_100) => {
            String::from("Commands")
        }
        (700, 257_820_820) => {
            String::from("Remove")
        }
        (700, 303_742) => {
            String::from("Clan")
        }
        (700, 90_822_183) => {
            String::from("Do not show this tip again")
        }
        (700, 50_363_567) => {
            String::from("Shop location:")
        }
        (700, 183_070_508) => {
            String::from("swroyal")
        }
        (700, 349_765) => {
            String::from("None")
        }
        (700, 154_707_508) => {
            String::from("Turn right")
        }
        (700, 161_448_524) => {
            String::from("<font color=yellow>Time left untill next attack:</font>")
        }
        (700, 123_755_381) => {
            String::from("pointlef")
        }
        (700, 165_041_749) => {
            String::from("Order Vehicle")
        }
        (700, 46_046_763) => {
            String::from("legshake")
        }
        (700, 88_639_353) => {
            String::from("Liquid animation")
        }
        (700, 113_891_412) => {
            String::from("Pet Wait")
        }
        (700, 144_216_281) => {
            String::from("Chat reply")
        }
        (700, 194_418_859) => {
            String::from("Ok")
        }
        (700, 193_085_429) => {
            String::from("Team Menu")
        }
        (700, 152_783_238) => {
            String::from("Sound On/Off")
        }
        (700, 254_394_900) => {
            String::from("Windows that should close when pressing ESC:")
        }
        (700, 131_525_495) => {
            String::from("Mission Window")
        }
        (700, 192_854_053) => {
            String::from("Autorun toggle")
        }
        (700, 6_244_482) => {
            String::from("XP Bar")
        }
        (700, 141_865_237) => {
            String::from("Run/walk toggle")
        }
        (700, 153_513_028) => {
            String::from("16/32 bit")
        }
        (700, 180_324_352) => {
            String::from("Open the help window.")
        }
        (700, 42_720_098) => {
            String::from("laugh-b")
        }
        (700, 65_812_549) => {
            String::from("Control center fade delay:")
        }
        (700, 102_130_174) => {
            String::from("Zoom in")
        }
        (700, 157_877_875) => {
            String::from("Yes")
        }
        (700, 106_151_492) => {
            String::from("Contains your current standing with all factions. Red means negative, green positive.")
        }
        (700, 236_777_091) => {
            String::from("<font color=yellow>Removed Items</font>")
        }
        (700, 76_279_876) => {
            String::from("Behind")
        }
        (700, 247_224_737) => {
            String::from("GUI/Chat Window")
        }
        (700, 86_504_771) => {
            String::from("Social Emotes")
        }
        (700, 176_741_625) => {
            String::from("Targeting")
        }
        (700, 74_236_910) => {
            String::from("Movement")
        }
        (700, 103_624_117) => {
            String::from("Cost to improve:")
        }
        (700, 176_926_963) => {
            String::from("<font color=green>Credits:</font>")
        }
        (700, 197_559_060) => {
            String::from("GUI/Raid")
        }
        (700, 108_725_338) => {
            String::from("Optimized")
        }
        (700, 229_432_946) => {
            String::from("Automatically reject mentor invites.")
        }
        (700, 244_761_645) => {
            String::from("Ground-shadows")
        }
        (700, 247_674_581) => {
            String::from("Help/Settings")
        }
        (700, 197_521_616) => {
            String::from("GUI/Help")
        }
        (700, 113_835_012) => {
            String::from("Pet Hunt")
        }
        (700, 113_837_215) => {
            String::from("Pet info")
        }
        (700, 38_421_697) => {
            String::from("Change your camera mode.")
        }
        (700, 116_713_015) => {
            String::from("Player Trade Window")
        }
        (700, 120_036_453) => {
            String::from("<font color=yellow>Placement</font>")
        }
        (700, 217_204_100) => {
            String::from("Strafe right")
        }
        (700, 21958) => {
            String::from("Off")
        }
        (700, 18_028_531) => {
            String::from("Special Actions")
        }
        (700, 106_887_061) => {
            String::from("Hide/Show shortcut bars")
        }
        (700, 145_418_899) => {
            String::from("Known friends")
        }
        (700, 257_960_388) => {
            String::from("Show Right Target Control")
        }
        (700, 66_471_345) => {
            String::from("Dump city advantages to chat")
        }
        (700, 115_639_876) => {
            String::from("Target your 3rd pet")
        }
        (700, 233_602_515) => {
            String::from("Misc Weapons")
        }
        (700, 95_404_947) => {
            String::from("thumbs")
        }
        (700, 219_239_495) => {
            String::from("Research Window")
        }
        (700, 247_227_683) => {
            String::from("GUI/NPC Dialog Window")
        }
        (700, 123_755_408) => {
            String::from("pointup")
        }
        (700, 143_372_861) => {
            String::from("Battle-music:")
        }
        (700, 83_978_095) => {
            String::from("Toggle inventory")
        }
        (700, 120_351_231) => {
            String::from("Switch mouse functionality")
        }
        (700, 137_056_661) => {
            String::from("Team Title")
        }
        (700, 143_561_945) => {
            String::from("Vs hard encounters only")
        }
        (700, 177_057_849) => {
            String::from("Pet targeting")
        }
        (700, 179_387_461) => {
            String::from("City Controller Charge")
        }
        (700, 243_002_128) => {
            String::from("Behaviour/PVP")
        }
        (700, 118_349_612) => {
            String::from("Cancel")
        }
        (700, 330_392) => {
            String::from("itch")
        }
        (700, 19_232_605) => {
            String::from("Target 4th team member")
        }
        (700, 171_359_930) => {
            String::from("Inv/info fade limit:")
        }
        (700, 4_699_134) => {
            String::from("Admin")
        }
        (700, 81_780_261) => {
            String::from("giggle")
        }
        (700, 237_456_836) => {
            String::from("Item Store")
        }
        (700, 57_222_937) => {
            String::from("Activates or deactivates the social equipment tab.")
        }
        (700, 242_996_893) => {
            String::from("Behaviour/Combat")
        }
        (700, 206_570_787) => {
            String::from("<font color=white> -- Unfinished, do not translate -- <p> -- more unfinsihed text that should not be translated.</font>")
        }
        (700, 197_515_477) => {
            String::from("Next chat line")
        }
        (700, 78_657_432) => {
            String::from("Dimach")
        }
        (700, 20_592_517) => {
            String::from("Helmet visible")
        }
        (700, 127_314_897) => {
            String::from("prostrate")
        }
        (700, 166_180_531) => {
            String::from("Shadows")
        }
        (700, 94_932_420) => {
            String::from("Target")
        }
        (700, 155_559_848) => {
            String::from("Toggle research")
        }
        (700, 103_869_428) => {
            String::from("Use/operate target")
        }
        (700, 21351) => {
            String::from("Low")
        }
        (700, 93_105_204) => {
            String::from("Result")
        }
        (700, 130_908_244) => {
            String::from("Strafe left")
        }
        (700, 205_964_856) => {
            String::from("Show Nano Target")
        }
        (700, 138_814_433) => {
            String::from("Contains a map of the area. Maps must be bought or ran as Nano-Programs for you to see an area.")
        }
        (700, 5_815_449) => {
            String::from("Query")
        }
        (700, 92_818_868) => {
            String::from("Ranged")
        }
        (700, 157_256_833) => {
            String::from("Show commercial billboards")
        }
        (700, 257_428_371) => {
            String::from("Shoulderpad options:")
        }
        (700, 170_867_722) => {
            String::from("Music volume:")
        }
        (700, 127_927_695) => {
            String::from("Inverted mouselook")
        }
        (700, 70_495_940) => {
            String::from("Texture resolution:")
        }
        (700, 210_681_316) => {
            String::from("Pet Guard")
        }
        (700, 358_537) => {
            String::from("pray")
        }
        (700, 100_277_806) => {
            String::from("Demolish Building")
        }
        (700, 146_473_507) => {
            String::from("Audio/Audio Settings")
        }
        (700, 207_177_044) => {
            String::from("3rd Person Locked View")
        }
        (700, 12577) => {
            String::from("<font color=green>+</font>")
        }
        (700, 165_400_215) => {
            String::from("Raid Window")
        }
        (700, 231_125_753) => {
            String::from("Opens the item store window, where you can purchase various items.")
        }
        (700, 266_584_127) => {
            String::from("<font color=yellow>Sign up to defend battle station in level range:</font>")
        }
        (700, 334_912) => {
            String::from("Jump")
        }
        (700, 326_740) => {
            String::from("Hunt")
        }
        (700, 75_839_698) => {
            String::from("Nano Bar")
        }
        (700, 142_220_649) => {
            String::from("backflip")
        }
        (700, 174_546_116) => {
            String::from("Pet Commands")
        }
        (700, 242_996_371) => {
            String::from("Behaviour/Inspect")
        }
        (700, 63_786_178) => {
            String::from("Contains all nano-programs you have uploaded. Drag-n-drop to the shortcut menu.")
        }
        (700, 355_507) => {
            String::from("Pets")
        }
        (700, 878) => {
            String::from("0")
        }
        (700, 322_608) => {
            String::from("Help")
        }
        (700, 5_896_842) => {
            String::from("<font color=yellow>Side:</font>")
        }
        (700, 89_451_210) => {
            String::from("Text fade delay:")
        }
        (700, 181_294_289) => {
            String::from("Pet Terminate")
        }
        (700, 55_267_653) => {
            String::from("Solo Title")
        }
        (700, 48_528_962) => {
            String::from("Show Left Bar")
        }
        (700, 166_212_407) => {
            String::from("Shallow")
        }
        (700, 109_932_805) => {
            String::from("Open chat configuration window.")
        }
        (700, 5_159_044) => {
            String::from("Guard")
        }
        (700, 63_010_653) => {
            String::from("Target 2nd team member")
        }
        (700, 131_209_523) => {
            String::from("GUI/Close By ESC")
        }
        (700, 194_691_098) => {
            String::from("Texture color depth:")
        }
        (700, 8_304_341) => {
            String::from("Next camera mode")
        }
        (700, 225_040_580) => {
            String::from("Forward")
        }
        (700, 138_053_170) => {
            String::from("Health Bar")
        }
        (700, 35_244_658) => {
            String::from("Toggle toolbar")
        }
        (700, 251_810_096) => {
            String::from("Your next upkeep payment is:")
        }
        (700, 160_087_586) => {
            String::from("<font color=yellow>Category</font>")
        }
        (700, 218_662_003) => {
            String::from("Show Nano Effects")
        }
        (700, 252_747_235) => {
            String::from("Attack Actions")
        }
        (700, 231_390_405) => {
            String::from("Gestures")
        }
        (700, 1_000_000) => {
            String::from("100000000")
        }
        (700, 247_227_459) => {
            String::from("GUI/Misc")
        }
        (700, 69_123_745) => {
            String::from("Reset fixed camera pos")
        }
        (700, 224_351_893) => {
            String::from("None")
        }
        (700, 394_129) => {
            String::from("ymca")
        }
        (700, 371_581) => {
            String::from("Team")
        }
        (700, 82_588_612) => {
            String::from("Helmet options:")
        }
        (700, 34_251_575) => {
            String::from("kisslow")
        }
        (700, 119_433_796) => {
            String::from("Maximum skill allowed:")
        }
        (700, 153_633_873) => {
            String::from("Initiate attack")
        }
        (700, 179_943_509) => {
            String::from("Legacy Title")
        }
        (700, 38_637_057) => {
            String::from("Abandon Terminal")
        }
        (700, 243_003_262) => {
            String::from("Behaviour/Mentor program")
        }
        (700, 20566) => {
            String::from("Inv")
        }
        (700, 206_155_708) => {
            String::from("Send tell")
        }
        (700, 116_406_664) => {
            String::from("Chat indicator (Flash head)")
        }
        (700, 211_735_108) => {
            String::from("Look at target")
        }
        (700, 219_827_786) => {
            String::from("Buffed Skill:")
        }
        (700, 112_882_002) => {
            String::from("Enable Control Center")
        }
        (700, 61) => {
            String::from("<font color=green>=</font>")
        }
        (700, 263_170_378) => {
            String::from("Max transparency level:")
        }
        (700, 76_348_884) => {
            String::from("Navigation")
        }
        (700, 126_844_280) => {
            String::from("Hello Kitty mech")
        }
        (700, 143_295_559) => {
            String::from("Perks Window")
        }
        (700, 247_026_179) => {
            String::from("<font color=green>Fixed keys</font>")
        }
        (700, 5_892_117) => {
            String::from("shake")
        }
        (700, 9_372_901) => {
            String::from("Result Analyzer")
        }
        (700, 194_302_691) => {
            String::from("<font color=yellow>Added Items</font>")
        }
        (700, 363_524) => {
            String::from("Quit")
        }
        (700, 48_571_423) => {
            String::from("NOTE: Changes to already open windows will not take<br>effect until the next time the window is opened")
        }
        (700, 200_759_038) => {
            String::from("Show help button")
        }
        (700, 244_042_430) => {
            String::from("Greeting")
        }
        (700, 153_276_023) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("Your character '{a}' is currently deactivated.\r\n\r\nIf you proceed to log in, this character will be marked as \"active\" and will then consume an available character slot.\r\n\r\nYou currently have {b} out of {c} available character slots.\r\n\r\nDo you wish to proceed logging in?")
        }
        (700, 183_851_093) => {
            String::from("Tower Build Menu")
        }
        (700, 44_640_818) => {
            String::from("Social Tab")
        }
        (700, 61_654_771) => {
            String::from("Show timer progress on icons")
        }
        (700, 193_576_036) => {
            String::from("Your upkeep account balance is:")
        }
        (700, 166_699_518) => {
            String::from("chicken")
        }
        (700, 199_125_587) => {
            String::from("Show Tracers")
        }
        (700, 237_259_797) => {
            String::from("It enables you to Train and untrain Perks. You get one new perk every ten level up to 200, and 1 for each level after.")
        }
        (700, 245_590_019) => {
            String::from("Fade End Distance:")
        }
        (700, 77_089_212) => {
            String::from("Cancel")
        }
        (700, 5_832_180) => {
            String::from("Right")
        }
        (700, 242_751_156) => {
            String::from("Allow double right pads")
        }
        (700, 94_079_128) => {
            String::from("Search")
        }
        (700, 346_191) => {
            String::from("Nano")
        }
        (700, 3_485_792) => {
            String::from("20-30")
        }
        (700, 103_118_685) => {
            String::from("Target 5th team member")
        }
        (700, 126_890_217) => {
            String::from("Abilities")
        }
        (700, 129_847_908) => {
            String::from("Behaviour/Chat")
        }
        (700, 191_887_715) => {
            String::from("Enable fixed F1-F6 buttons for pets")
        }
        (700, 163_515_564) => {
            String::from("Team channel")
        }
        (700, 190_441_399) => {
            String::from("Tradeskill Window")
        }
        (700, 37_920_300) => {
            String::from("Level:")
        }
        (700, 218_018_357) => {
            String::from("Enable social tab")
        }
        (700, 239_622_261) => {
            String::from("Show Left Menu")
        }
        (700, 80_502_307) => {
            String::from("Show window frame")
        }
        (700, 112_846_115) => {
            String::from("Show Buffs")
        }
        (700, 304_080) => {
            String::from("CP: 0")
        }
        (700, 163_960_615) => {
            String::from("Make Shop Org Exclusive")
        }
        (700, 6_655_914) => {
            String::from("Text fade time:")
        }
        (700, 171_862_616) => {
            String::from("Show spaceships")
        }
        (700, 207_356_345) => {
            String::from("This menu contains windows related to development of your character.")
        }
        (700, 96093) => {
            String::from("Target 3rd team member")
        }
        (700, 122_498_374) => {
            String::from("Sound FX On/Off")
        }
        (700, 129_897_640) => {
            String::from("Show Fight Target")
        }
        (700, 210_972_260) => {
            String::from("Turn left")
        }
        (700, 35_475_522) => {
            String::from("Equals the chat command '/pet follow'. Makes your pet (except towers) follow you. It will not attack anyone that attacks you. It will not return fire on itself.")
        }
        (700, 218_806_207) => {
            String::from("Visual/Ground")
        }
        (700, 93_759_653) => {
            String::from("Rotate")
        }
        (700, 12_151_379) => {
            String::from("Open the options panel.")
        }
        (700, 72_398_565) => {
            String::from("Prev camera mode")
        }
        (700, 143_374_980) => {
            String::from("Pet Behind")
        }
        (700, 84_464_771) => {
            String::from("Character view distance:")
        }
        (700, 46_999_571) => {
            String::from("Lock raid bar positions")
        }
        (700, 216_654_899) => {
            String::from("Your inventory is where you can see all the items you are carrying that have not yet been, or cannot be equipped.")
        }
        (700, 345_694) => {
            String::from("moon")
        }
        (700, 369_280) => {
            String::from("slap")
        }
        (700, 64_318_818) => {
            String::from("0:23:22")
        }
        (700, 196_039_580) => {
            String::from("Offscreen surface technology")
        }
        (700, 381_904) => {
            String::from("VP: 0")
        }
        (700, 231_412_514) => {
            String::from("New Character")
        }
        (700, 176_622_676) => {
            String::from("Switch layout")
        }
        (700, 247_605_061) => {
            String::from("You currently have an upkeep debt of:")
        }
        (700, 200_402_645) => {
            String::from("Prev chat line")
        }
        (700, 19332) => {
            String::from("eat")
        }
        (700, 13_053_188) => {
            String::from("Pick up target")
        }
        (700, 382_980) => {
            String::from("Wait")
        }
        (700, 89_559_932) => {
            String::from("Normal")
        }
        (700, 45_104_940) => {
            String::from("Control Panel")
        }
        (700, 129_721_171) => {
            String::from("Nano Controlling Units (NCU) contains all currently running nano-programs. Right click on the program to cancel it. Install a belt with memory to get more avaialable NCUs.")
        }
        (700, 65_802_296) => {
            String::from("Control center high fade:")
        }
        (700, 221_269_720) => {
            String::from("Flingshot")
        }
        (700, 94_503_731) => {
            String::from("Skills")
        }
        (700, 96_755_290) => {
            String::from("Fog mode:")
        }
        (700, 302_724) => {
            String::from("Chat")
        }
        (700, 19_517_557) => {
            String::from("Display PVP title")
        }
        (700, 134_462_424) => {
            String::from("Aimedshot")
        }
        (700, 140_930_451) => {
            String::from("The camera is locked in its position no matter where you go. Move and zoom it with numeric keys.")
        }
        (700, 130_565_664) => {
            String::from("Contains a map of the area seen from high above. Your position and the missions you may have 'uploaded' will show on the map.")
        }
        (700, 143_389_019) => {
            String::from("Pet Attack")
        }
        (700, 107_093_316) => {
            String::from("Equals the chat command '/pet attack'. Makes your pet stop fighting (if it is doing that) and instantly attacks your hostile nano (and fighting) target.")
        }
        (700, 220_081_237) => {
            String::from("Display organization name above head")
        }
        (700, 131_809_012) => {
            String::from("Mouse button hold delay:")
        }
        (700, 21376) => {
            String::from("Map")
        }
        (700, 4_863_637) => {
            String::from("Close")
        }
        (700, 359_445) => {
            String::from("puke")
        }
        (700, 4_860_802) => {
            String::from("Clear")
        }
        (700, 146_473_510) => {
            String::from("Audio/Voices")
        }
        (700, 122_448_284) => {
            String::from("Chat Config Panel")
        }
        (700, 93_079_605) => {
            String::from("Rename")
        }
        (700, 138_567_319) => {
            String::from("Shop Search Window")
        }
        (700, 11_022_126) => {
            String::from("italian")
        }
        (700, 318_508) => {
            String::from("Full")
        }
        (700, 102_954_089) => {
            String::from("Ground quality:")
        }
        (700, 174_492_350) => {
            String::from("Coordinates")
        }
        (700, 174_684_547) => {
            String::from("Corpses")
        }
        (700, 180_488_358) => {
            String::from("Video/Environment")
        }
        (700, 10_033_655) => {
            String::from("Warn when sending to unsubscribed groups")
        }
        (700, 197_253_981) => {
            String::from("Open nano programs window to show new nanos")
        }
        (700, 93_091_476) => {
            String::from("Report")
        }
        (700, 195_157_155) => {
            String::from("Visual/View Distance")
        }
        (700, 212_703_029) => {
            String::from("Leave queue")
        }
        (700, 60_210_182) => {
            String::from("Voices On/Off")
        }
        (700, 148_343_461) => {
            String::from("<font color=yellow>Max Price</font>")
        }
        (700, 3_565_828) => {
            String::from("32 bit")
        }
        (700, 54_776_153) => {
            String::from("Toggle skillsystem")
        }
        (700, 35_475_527) => {
            String::from("Equals the chat command '/pet terminate'. Instantly destroys your pet. Careful, no questions are asked!")
        }
        (700, 107_843_905) => {
            String::from("The camera is locked in one position, but it has a bit 'softening' on its movement. Move and zoom it with numeric keys.")
        }
        (700, 113_754_991) => {
            String::from("Shadowknowledge question")
        }
        (700, 145_977_481) => {
            String::from("<font color=yellow>Min Quality</font>")
        }
        (700, 185_249_799) => {
            String::from("distinguished")
        }
        (700, 200_738_708) => {
            String::from("Contains information about your team. Target a player and press the 'invite' button to team up. Accept invitations with 'Accept'.")
        }
        (700, 262_859_233) => {
            String::from("The following character is active on the server:")
        }
        (700, 3_889_987) => {
            String::from("Realistic clouds")
        }
        (700, 213_325_842) => {
            String::from("Auto attack monster")
        }
        (700, 222_562_139) => {
            String::from("Open the Tradeskill Kit.")
        }
        (700, 153_371_413) => {
            String::from("Name:")
        }
        (700, 368_480) => {
            String::from("<font color=yellow>Shop</font>")
        }
        (700, 300_473) => {
            String::from("Body")
        }
        (700, 75_078_772) => {
            String::from("Accept")
        }
        (700, 180_143_796) => {
            String::from("Open the stat window.")
        }
        (700, 195_249_491) => {
            String::from("Show Environment Effects")
        }
        (700, 68_154_613) => {
            String::from("Show Right Menu")
        }
        (700, 135_638_070) => {
            String::from("Music On/Off")
        }
        (700, 142_224_834) => {
            String::from("Backward")
        }
        (700, 218_361_379) => {
            String::from("Add new nanos to shortcut bar")
        }
        (700, 100_141_159) => {
            String::from("PlanetMap Window")
        }
        (700, 179_922_164) => {
            String::from("strong4")
        }
        (700, 67_308_242) => {
            String::from("military")
        }
        (700, 123_755_481) => {
            String::from("pointrig")
        }
        (700, 304_732) => {
            String::from("cool")
        }
        (700, 210_413_812) => {
            String::from("Fastattack")
        }
        (700, 71_245_417) => {
            String::from("Move camera direction by holding down right-mouse button.")
        }
        (700, 4_119_509) => {
            String::from("<font color=yellow>Available Vehicles</font>")
        }
        (700, 179_816_994) => {
            String::from("Visual/Textures")
        }
        (700, 43) => {
            String::from("+")
        }
        (700, 76_034_756) => {
            String::from("ballet")
        }
        (700, 75_921_897) => {
            String::from("apachi")
        }
        (700, 6_079_999) => {
            String::from("Inventory")
        }
        (700, 247_225_541) => {
            String::from("GUI/Equipment Window")
        }
        (700, 141_441_141) => {
            String::from("Disable Cloaking Device")
        }
        (700, 175_362_456) => {
            String::from("Mouse lag fix")
        }
        (700, 5_941_427) => {
            String::from("Stats")
        }
        (700, 21_655_119) => {
            String::from("Display my own replies in npc dialog")
        }
        (700, 42_711_566) => {
            String::from("Contains all worn weapons, armor and Implants.")
        }
        (700, 175_049_148) => {
            String::from("Bow special")
        }
        (700, 146_473_495) => {
            String::from("Audio/General")
        }
        (700, 99_541_550) => {
            String::from("Profession:")
        }
        (700, 222_004_483) => {
            String::from("Show Others Effects")
        }
        (700, 117_497_173) => {
            String::from("Mouse cursor context hints:")
        }
        (700, 152_194_247) => {
            String::from("Display notification window")
        }
        (700, 338_900) => {
            String::from("Left")
        }
        (700, 163_412_339) => {
            String::from("Text shadows:")
        }
        (700, 263_289_821) => {
            String::from("Invite to team")
        }
        (700, 237_974_052) => {
            String::from("Toggle raid")
        }
        (700, 265_929_107) => {
            String::from("<font color=yellow>Bought Items</font>")
        }
        (700, 35_475_521) => {
            String::from("Equals the chat command '/pet behind'. Makes your pet (except towers) stand behind you, aborting all attacks. It will not attack anyone that attacks you. It will not return fire on itself.")
        }
        (700, 31_376_901) => {
            String::from("Left side visible")
        }
        (700, 37_345_572) => {
            String::from("Breed:")
        }
        (700, 217_711_956) => {
            String::from("Vicinity")
        }
        (700, 30_127_342) => {
            String::from("1st/3rd person")
        }
        (700, 35_475_528) => {
            String::from("Equals the chat command '/pet wait'. Makes your pet stop fighting and stand in one place not following you. It will not attack anyone that attacks you. It will not return fire on itself.")
        }
        (700, 183_521_682) => {
            String::from("thinker")
        }
        (700, 167_264_290) => {
            String::from("Social tab options:")
        }
        (700, 5_463_550) => {
            String::from("Login")
        }
        (700, 106_183_290) => {
            String::from("Shop name:")
        }
        (700, 202_319_756) => {
            String::from("Rollup Panel")
        }
        (700, 207_423_774) => {
            String::from("Environment/Characters")
        }
        (700, 228_629_621) => {
            String::from("Simple")
        }
        (700, 155_663_454) => {
            String::from("Trailing 3rd Person View")
        }
        (700, 322_428) => {
            String::from("Heal")
        }
        (700, 93_074_804) => {
            String::from("Reload")
        }
        (700, 34_250_407) => {
            String::from("kisshigh")
        }
        (700, 5_855_785) => {
            String::from("rocky")
        }
        (700, 306_112) => {
            String::from("Deep")
        }
        (700, 4_833_316) => {
            String::from("Build")
        }
        (700, 81_146_727) => {
            String::from("Follow")
        }
        (700, 127_257_843) => {
            String::from("<font color=yellow>Profession:</font>")
        }
        (700, 146_968_238) => {
            String::from("Relaxing")
        }
        (700, 20_599_621) => {
            String::from("Sign up entire team")
        }
        (700, 209_367_070) => {
            String::from("Faction")
        }
        (700, 209_003_881) => {
            String::from("Ground full quality radius:")
        }
        (700, 217_760_307) => {
            String::from("Video/Video Settings")
        }
        (700, 43_918_104) => {
            String::from("Team search")
        }
        (700, 56_199_855) => {
            String::from("<font color=yellow>Location:</font>")
        }
        (700, 93_510_615) => {
            String::from("<font color=yellow>Transaction Log</font>")
        }
        (700, 349_775) => {
            String::from("nono")
        }
        (700, 67_857_329) => {
            String::from("Save fixed camera pos")
        }
        (700, 215_263_653) => {
            String::from("Use compressed textures")
        }
        (700, 105_621_024) => {
            String::from("Show confirm drop item dialog")
        }
        (700, 182_460_359) => {
            String::from("NCU Window")
        }
        (700, 133_003_046) => {
            String::from("Client Wildlife")
        }
        (700, 227_525_875) => {
            String::from("Friends")
        }
        (700, 186_976_650) => {
            String::from("Available Improvement Points:")
        }
        (700, 205_245_042) => {
            String::from("Alien XP Bar")
        }
        (700, 163_561_811) => {
            String::from("Save Changes")
        }
        (700, 115_639_396) => {
            String::from("Target your 1st pet")
        }
        (700, 123_755_217) => {
            String::from("pointba")
        }
        (700, 170_601_303) => {
            String::from("Nano & Aiding")
        }
        (700, 157_337_732) => {
            String::from("Lock equipment tabs")
        }
        (700, 84_267_173) => {
            String::from("Invite")
        }
        (700, 241_888_490) => {
            String::from("City Owner:")
        }
        (700, 4_888_227) => {
            String::from("cross")
        }
        (700, 87_475_413) => {
            String::from("lounge")
        }
        (700, 203_113_109) => {
            String::from("Duel Title")
        }
        (700, 805) => {
            String::from("0%")
        }
        (700, 3_451_140) => {
            String::from("16 bit")
        }
        (700, 14_362_615) => {
            String::from("Perk Window")
        }
        (700, 15_282_106) => {
            String::from("Master Volume:")
        }
        (700, 232_689_330) => {
            String::from("Auto Target Hostile Players")
        }
        (700, 2_449_379) => {
            String::from("Completed personal research projects")
        }
        (700, 35_475_526) => {
            String::from("Equals the chat command '/pet report'. Returns a report from your pet(s) about their current location and health.")
        }
        (700, 66_954_482) => {
            String::from("Auto attack player")
        }
        (700, 179_816_993) => {
            String::from("Visual/Misc")
        }
        (700, 266_770_382) => {
            String::from("Construct Buildings")
        }
        (700, 13_499_796) => {
            String::from("Disable inspect function")
        }
        (700, 35_702_000) => {
            String::from("Open planet map or local map.")
        }
        (700, 76_207_777) => {
            String::from("Contains a list of all available social actions.")
        }
        (700, 1_573_365) => {
            String::from("Current commission: 90%")
        }
        (700, 94_390_053) => {
            String::from("Simple")
        }
        (700, 4_835_748) => {
            String::from("Burst")
        }
        (700, 154_591_811) => {
            String::from("blowkiss")
        }
        (700, 210_012_884) => {
            String::from("Contains a list of all available pet commands.")
        }
        (700, 316_160) => {
            String::from("flip")
        }
        (700, 78_251_126) => {
            String::from("Show Muzzleflash")
        }
        (700, 15_799_114) => {
            String::from("Payment size:")
        }
        (700, 69_116_894) => {
            String::from("Scroll chat down")
        }
        (700, 359_472) => {
            String::from("pulp")
        }
        (700, 34_490_771) => {
            String::from("Transfer Credits")
        }
        (700, 35_405_658) => {
            String::from("Voices volume:")
        }
        (700, 5_896_631) => {
            String::from("shrug")
        }
        (700, 146_640_072) => {
            String::from("Refresh")
        }
        (700, 81_539_617) => {
            String::from("full")
        }
        (700, 220_312_921) => {
            String::from("Ground texture quality:")
        }
        (700, 4_917_663) => {
            String::from("disco")
        }
        (700, 30_182_771) => {
            String::from("Change Commission")
        }
        (700, 296_859) => {
            String::from("Back")
        }
        (700, 308_958) => {
            String::from("Down")
        }
        (700, 161_198_658) => {
            String::from("3rd Rubber")
        }
        (700, 162_040_520) => {
            String::from("scratch")
        }
        (700, 164_789_436) => {
            String::from("These are all the special attacks received from chosing perks.")
        }
        (700, 59_063_301) => {
            String::from("Map Menu")
        }
        (700, 178_621_034) => {
            String::from("Base Skill:")
        }
        (700, 212_954_661) => {
            String::from("Show social tab only")
        }
        (700, 68_350_997) => {
            String::from("The upkeep you will then be deducted is:")
        }
        (700, 5_653_706) => {
            String::from("Omni:")
        }
        (700, 70_113_153) => {
            String::from("Sidebar Opacity:")
        }
        (700, 230_514_867) => {
            String::from("Shop Windows")
        }
        (700, 328_911) => {
            String::from("Info")
        }
        (700, 133_755_575) => {
            String::from("Mission Selection Window")
        }
        (700, 67_151_146) => {
            String::from("<font color=#ffff33>Hear voices from:</font>")
        }
        (700, 4_834_005) => {
            String::from("bulge")
        }
        (700, 93_948_084) => {
            String::from("scared")
        }
        (700, 185_121_513) => {
            String::from("Directions")
        }
        (700, 86_189_811) => {
            String::from("Smooth Animations")
        }
        (700, 98_573_031) => {
            String::from("Map Window")
        }
        (700, 47_964_903) => {
            String::from("Team Window")
        }
        (700, 383_874) => {
            String::from("Wear")
        }
        (700, 100_745_728) => {
            String::from("Show selected group in input bar")
        }
        (700, 5_687_587) => {
            String::from("Perks")
        }
        (700, 81_335_881) => {
            String::from("If activated, no visuals from items in the normal wear tabs will be shown. Requires you to zone to take effect.")
        }
        (700, 185_740_386) => {
            String::from("Mini toolbar")
        }
        (700, 203_948_258) => {
            String::from("GUI/Control Center")
        }
        (700, 243_745_957) => {
            String::from("Character Name:")
        }
        (700, 5_622_817) => {
            String::from("off")
        }
        (700, 78_392_485) => {
            String::from("Delete")
        }
        (700, 135_734_436) => {
            String::from("<font color=yellow>Search Result</font>")
        }
        (700, 139_466_920) => {
            String::from("Lets you search for potential team members and set your own status as 'Looking for team'. If you find someone you want to team up with, you can either send them a message or invite them directly to your team.")
        }
        (700, 131_830_503) => {
            String::from("Wear Window")
        }
        (700, 113_550_212) => {
            String::from("Show Left Target Control")
        }
        (700, 172_720_596) => {
            String::from("Sneakattack")
        }
        (700, 204_398_211) => {
            String::from("<font color=yellow>Player Shops</font>")
        }
        (700, 21844) => {
            String::from("nod")
        }
        (700, 11_635_822) => {
            String::from("Location:")
        }
        (700, 82_126_180) => {
            String::from("Allow double left pads")
        }
        (700, 207_331_188) => {
            String::from("Character")
        }
        (700, 220_740_611) => {
            String::from("flamenco")
        }
        (700, 4_829_101) => {
            String::from("Find team")
        }
        (700, 178_596_005) => {
            String::from("Exit the game")
        }
        (700, 213_161_512) => {
            String::from("Toggle team search")
        }
        (700, 79_789_105) => {
            String::from("Wait for vertical sync")
        }
        (700, 150_656_080) => {
            String::from("Lets you create and organize raids.")
        }
        (700, 241_051_454) => {
            String::from("Store")
        }
        (700, 35_475_523) => {
            String::from("Equals the chat command '/pet guard'. Makes your pet follow you and instantly attack anyone that attacks you. It will move on to the next fighting target once it has finished with the first.")
        }
        (700, 5_489_333) => {
            String::from("Melee")
        }
        (700, 213_039_317) => {
            String::from("Sit/stand toggle")
        }
        (700, 72_604_035) => {
            String::from("Personal research:")
        }
        (700, 197_499_490) => {
            String::from("GUI/Character")
        }
        (700, 323_544) => {
            String::from("High")
        }
        (700, 218_706_033) => {
            String::from("Environment/Effects")
        }
        (700, 117_190_867) => {
            String::from("Global research:")
        }
        (700, 172_024_834) => {
            String::from("Show Right Bar")
        }
        (700, 81_462_071) => {
            String::from("Programs Window")
        }
        (700, 163_447_360) => {
            String::from("Auto show Tip of the Day")
        }
        (700, 247_229_886) => {
            String::from("GUI/Window")
        }
        (700, 80_525_356) => {
            String::from("Neutral")
        }
        (700, 200_268_963) => {
            String::from("Disable item description tooltip on shortcut bars")
        }
        (700, 148_276_647) => {
            String::from("Pet Follow")
        }
        (700, 245_416_247) => {
            String::from("Display invite dialog directly")
        }
        (700, 144_264_789) => {
            String::from("<font color=red>Cloaking Device</font>")
        }
        (700, 103_953_492) => {
            String::from("Cursor right")
        }
        (700, 4_665_473) => {
            String::from("simple")
        }
        (700, 4_819_164) => {
            String::from("Brawl")
        }
        (700, 120_081_661) => {
            String::from("PlanetMap")
        }
        (700, 42_720_019) => {
            String::from("laugh-s")
        }
        (700, 43_053_603) => {
            String::from("<font color=yellow>Sold Items</font>")
        }
        (700, 19_271_760) => {
            String::from("<font color=yellow>Vehicle Description</font>")
        }
        (700, 174_352_611) => {
            String::from("Compass")
        }
        (700, 152_386_765) => {
            String::from("Controls/Camera")
        }
        (700, 78_002_308) => {
            String::from("Combat")
        }
        (700, 180_487_990) => {
            String::from("Video/Effects")
        }
        (700, 78_619_363) => {
            String::from("Status:")
        }
        (700, 218_912_316) => {
            String::from("Guild channel")
        }
        (700, 226_815_598) => {
            String::from("Flash group invites in friends view")
        }
        (700, 247_225_396) => {
            String::from("GUI/Fading")
        }
        (700, 248_577_493) => {
            String::from("Target Analyzer")
        }
        (700, 259_167_246) => {
            String::from("Special Actions Menu. Hold-Down and drag to make a shortcut in the shortcut menu.")
        }
        (700, 4_738_713) => {
            String::from("angry")
        }
        (700, 93_863_077) => {
            String::from("salute")
        }
        (700, 145_434_691) => {
            String::from("Display names above head")
        }
        (700, 90_794_598) => {
            String::from("vs normal & hard")
        }
        (700, 209_370_132) => {
            String::from("Detailed")
        }
        (700, 55_208_673) => {
            String::from("Target self/last")
        }
        (700, 255_025_681) => {
            String::from("Use nobob camera")
        }
        (700, 370_436) => {
            String::from("spit")
        }
        (700, 94_364_608) => {
            String::from("Sign up")
        }
        (700, 99_849_971) => {
            String::from("Dialog boxes")
        }
        (700, 46_690_307) => {
            String::from("Personal research progress")
        }
        (700, 131_333_962) => {
            String::from("Choose Your Voice:")
        }
        (700, 247_726_585) => {
            String::from("Action when receiving invites to private groups:")
        }
        (700, 176_764_331) => {
            String::from("3rd Lock")
        }
        (700, 207_722_036) => {
            String::from("Show timer text on icons")
        }
        (700, 82_512_276) => {
            String::from("Tutorial tip enabled")
        }
        (700, 128_648_723) => {
            String::from("Actions")
        }
        (700, 94_816_405) => {
            String::from("Source")
        }
        (700, 235_189_939) => {
            String::from("Windows")
        }
        (700, 1371) => {
            String::from("Ok")
        }
        (700, 123_755_167) => {
            String::from("pointfor")
        }
        (700, 145_698_021) => {
            String::from("Athlete")
        }
        (700, 94_896_199) => {
            String::from("Spying")
        }
        (700, 119_895_901) => {
            String::from("Target 6th team member")
        }
        (700, 207_454_407) => {
            String::from("City Terminal Window")
        }
        (700, 27_585_123) => {
            String::from("Fade Start Distance:")
        }
        (700, 79_366_323) => {
            String::from("Completed global research projects")
        }
        (700, 218_211_572) => {
            String::from("Contains all current missions (quests) that you have. All new missions get waypoints on the map&nbsp;/&nbsp;compass. Right-click on the mission for more actions.")
        }
        (700, 69_413_476) => {
            String::from("Attack target")
        }
        (700, 142_026_883) => {
            String::from("applause")
        }
        (700, 383_173) => {
            String::from("wave")
        }
        (700, 176_463_879) => {
            String::from("Dancing")
        }
        (700, 214_858_723) => {
            String::from("Behaviour/Behaviour Settings")
        }
        (700, 255_457_924) => {
            String::from("Environment/Fog & Ground")
        }
        (700, 81_673_381) => {
            String::from("Enable Cloaking Device")
        }
        (700, 239_001_239) => {
            String::from("Control center low fade:")
        }
        (700, 208_355_027) => {
            String::from("The camera floats behind you like a kite. Zoom in out / move with numeric keys.")
        }
        (700, 21669) => {
            String::from("NCU")
        }
        (700, 135_126_234) => {
            String::from("<font color=green>Total Cost:</font>")
        }
        (700, 233_135_995) => {
            String::from("Doubleclick")
        }
        (700, 65_227_130) => {
            String::from("Owner organization:")
        }
        (700, 232_577_057) => {
            String::from("Total cost:")
        }
        (700, 262_859_234) => {
            String::from("Click \"Login\" to log in with this character. To choose another character; wait until this dialog box disappears and log in as you would normally.")
        }
        (700, 68_098_279) => {
            String::from("Show Right Wing")
        }
        (700, 179_974_149) => {
            String::from("Decline")
        }
        (700, 80_295_323) => {
            String::from("fblock")
        }
        (700, 256_592_730) => {
            String::from("<font color=#ffff33>Hits outside fight:</font>")
        }
        (700, 76_261_275) => {
            String::from("Attack")
        }
        (700, 161_807_668) => {
            String::from("Fade character when I zoom")
        }
        (700, 84_156_400) => {
            String::from("Scroll chat up")
        }
        (700, 174_294_852) => {
            String::from("Cursor left")
        }
        (700, 256_010_871) => {
            String::from("Team Search Window")
        }
        (700, 180_961_297) => {
            String::from("Terminate")
        }
        (700, 147_161_954) => {
            String::from("Suggested IP distribution")
        }
        (700, 5_125_508) => {
            String::from("gloat")
        }
        (700, 122_156_430) => {
            String::from("<font color=yellow>Your place in the queue:</font>")
        }
        (700, 180_975_955) => {
            String::from("surprised")
        }
        (700, 205_728_915) => {
            String::from("Simple clouds")
        }
        (700, 181_164_660) => {
            String::from("Tradeskill Kit")
        }
        (700, 306_324) => {
            String::from("curt")
        }
        (700, 57_026_196) => {
            String::from("lookout")
        }
        (700, 139_698_852) => {
            String::from("Camera keys 3rd person")
        }
        (700, 18791) => {
            String::from("bow")
        }
        (700, 182_596_310) => {
            String::from("Create ref of target")
        }
        (700, 204_507_263) => {
            String::from("Full auto")
        }
        (700, 139_912_483) => {
            String::from("Global research progress")
        }
        (700, 765) => {
            String::from(" ")
        }
        (700, 149_807_177) => {
            String::from("If this option is unchecked, you will not be able to doubleclick objects in the environment. You will need to target the object and use the keyboard shortcut for the action you want.")
        }
        (700, 5_943_482) => {
            String::from("<font color=yellow>Looking for team:</font>")
        }
        (700, 149_730_003) => {
            String::from("Perk Attacks")
        }
        (700, 106_154_544) => {
            String::from("<font color=yellow>Personal Description:</font>")
        }
        (700, 115_639_684) => {
            String::from("Target your 2nd pet")
        }
        (700, 185_217_109) => {
            String::from("Dislike")
        }
        (700, 145_959_017) => {
            String::from("<font color=yellow>Max Quality</font>")
        }
        (700, 4_952_139) => {
            String::from("drink")
        }
        (700, 81_745_893) => {
            String::from("Current Improvement:")
        }
        (700, 52_527_876) => {
            String::from("Vs normal and hard encounters")
        }
        (700, 88_229_907) => {
            String::from("Access to various windows.")
        }
        (700, 3_640_021) => {
            String::from("<font color=yellow>Min Price</font>")
        }
        (700, 13089) => {
            String::from("-")
        }
        (700, 94_813_080) => {
            String::from("speech")
        }
        (700, 142_055_041) => {
            String::from("Approval")
        }
        (700, 115_412_463) => {
            String::from("<font color=yellow>Item Location Info</font>")
        }
        (700, 178_634_002) => {
            String::from("Auto Target Hostile Monsters")
        }
        (700, 193_440_971) => {
            String::from("Tradeskill")
        }
        (700, 158_114_558) => {
            String::from("1st Person")
        }
        (700, 362_484) => {
            String::from("Raid")
        }
        (700, 35_475_525) => {
            String::from("Equals the chat command '/pet hunt'. Makes the pet roam around looking for a target to attack. When it has killed everything it can see (or after some time), it returns.")
        }
        (700, 119_286_588) => {
            String::from("Automatically reject duel requests")
        }
        (700, 106_962_876) => {
            String::from("Option Panel")
        }
        (700, 51_028_839) => {
            String::from("1st Person View")
        }
        (700, 113_830_700) => {
            String::from("Pet Heal")
        }
        (700, 180_980_756) => {
            String::from("surrender")
        }
        (700, 158_652_083) => {
            String::from("Show item descriptions in tooltips")
        }
        (700, 10_884_444) => {
            String::from("Open the Control Panel.")
        }
        (700, 77_515_443) => {
            String::from("Chests")
        }
        (700, 87_797_949) => {
            String::from("Medium")
        }
        (700, 218_757_706) => {
            String::from("fishsize")
        }
        (700, 179_922_161) => {
            String::from("strong1")
        }
        (700, 238_296_690) => {
            String::from("Trade & Repair")
        }
        (700, 262_601_543) => {
            String::from("Faction Window")
        }
        (700, 23_471_780) => {
            String::from("Zoom out")
        }
        (700, 35_475_524) => {
            String::from("Equals the chat command '/pet heal'. A special Meta-Phycesist heal-pet command, that makes the pet start a heal-cycle on your current nano-target.")
        }
        (700, 224_661_082) => {
            String::from("FX volume:")
        }
        (700, 213_797) => {
            String::from("100%")
        }
        (700, 232_491_820) => {
            String::from("Next friendly")
        }
        (700, 147_544_708) => {
            String::from("Autodetect")
        }
        (700, 346_165) => {
            String::from("<font color=yellow>Name</font>")
        }
        (10004, 158_376_515) => {
            String::from("Paid points: ")
        }
        (10004, 264_195_459) => {
            String::from("Veteran points: ")
        }
        (509, 9) => {
            String::from("Unique House")
        }
        (509, 189_712_468) => {
            String::from("Colonist cleared for passage!\r\n")
        }
        (509, 191_255_476) => {
            String::from("Exit Character Creation.")
        }
        (509, 164_305_064) => {
            String::from("Click to finish character creation. Once you click finish you can no longer edit your character's breed, appearance, profession or name!")
        }
        (509, 11_734_740) => {
            String::from("Difficulty to play: <FONT color=\"#ffff00\"> High </FONT>\r\nAn Agent&#8217;s life is spent in the shadows. Agents focus on concealment and subterfuge skills and one of their special abilities is going undercover. Doing so enables them to use nanotechnology normally only available to other professions. When it comes to combat, the Agent&#8217;s speciality is sniping opponents with high velocity rifles using unique nanotechnology to further increase the damage.")
        }
        (509, 107_121_557) => {
            String::from("Click to select Solitus Female.")
        }
        (509, 6) => {
            String::from("Cloaking Device")
        }
        (509, 155_299_122) => {
            String::from("Click to inspect the Keeper.")
        }
        (509, 252_555_380) => {
            String::from("Metaphysicist Selected.")
        }
        (509, 111_071_483) => {
            String::from("Choose a nickname.")
        }
        (509, 93_888_884) => {
            String::from("Adventurer Selected.")
        }
        (509, 171_771_173) => {
            String::from("Click the two arrows repeatedly to select a face.")
        }
        (509, 2_457_557) => {
            String::from("Select your character's appearance")
        }
        (509, 119_893_348) => {
            String::from("Fixer Selected.")
        }
        (509, 214_125_134) => {
            String::from("The server is down.")
        }
        (509, 31_587_506) => {
            String::from("Slender.")
        }
        (509, 161_383_857) => {
            String::from("Click this button to finish character creation and start in Rubi-Ka.\r\nYour character will be transferred to the ICC Shuttleport.\r\n\r\nOnce you click this button, you can no longer edit your character's breed, appearance, profession or name!\r\n\r\n(The Shadowlands professions Keeper and Shade cannot start in Rubi-Ka.)")
        }
        (509, 41_516_260) => {
            String::from("Opifex Male selected.")
        }
        (509, 105_053_140) => {
            String::from("by clicking on any of the fourteen profession buttons. Each profession has its advantages and disadvantages. A detailed description can be found when clicking the profession buttons. When you have made your choice click Next.")
        }
        (509, 1_622_994) => {
            String::from("The Morning Star Transfer Station")
        }
        (509, 260_210_404) => {
            String::from("Click to inspect the Agent.")
        }
        (509, 172_602_036) => {
            String::from("Nanomage Male selected.")
        }
        (509, 264_892_014) => {
            String::from("The Atrox is unique to Rubi-Ka, and was genetically engineered to be a manual laborer in the notum mines. The excessive manipulation led to the Atrox being a genderless breed. The Atrox is strong, hardy and resilient, and makes for a good soldier or worker.")
        }
        (509, 191_352_900) => {
            String::from("Next.")
        }
        (509, 47_929_166) => {
            String::from("The Solitus is the galaxy's most common breed - a direct natural descendant of the Homo Sapiens which evolved during Earth's long winter. The Solitus has no specific advantage, but is capable of adapting to almost any situation and environment, and has a proclivity for learning.")
        }
        (509, 207_923_652) => {
            String::from("Engineer Selected.")
        }
        (509, 5) => {
            String::from("Radar Station")
        }
        (509, 32_335_924) => {
            String::from("Keeper Selected.")
        }
        (509, 187_855_237) => {
            String::from("Click to select Opifex Female.")
        }
        (509, 11_350_309) => {
            String::from("Difficulty to play: <FONT color=\"#ffff00\"> High </FONT>\r\nThe Shade is a mix between a predator and a parasite. Dark and aggressive, the Shade utilizes nano-technology to suck the life and energy out of their prey, robbing them of the basic elements they need to subsist. The Shade stays out of harm's way by relying on concealment and good combat mobility. By using the element of surprise, the Shade can inflict large amounts of damage in a short amount of time.")
        }
        (509, 12_179_522) => {
            String::from("Difficulty to play: <FONT color=\"#ffff00\"> Medium </FONT>\r\nA Fixer specializes in getting people what they need when they need it.  By hacking into what is known as The Grid from anywhere in the world, Fixers can use it to transport themselves or their entire team in digital form around Rubi-Ka. The Fixers move fastest of all and special armors make them the hardest to hit as well. A Fixer's main combat strategy lies in limiting the maneuverability of opponents and they favour weapons from the sub-machinegun category.")
        }
        (509, 52_620_036) => {
            String::from("Suggest name.")
        }
        (509, 163_071_938) => {
            String::from("Difficulty to play: <FONT color=\"#ffff00\"> Medium / High </FONT>\r\nAn Engineer is a specialist in creating all sorts of machinery. Engineers really excel in constructing powerful battledroids and have access to unique nanotechnology to enhance and repair them. The Engineer's weapon skills are not that great, but the Engineer/robot-pet duo is quite formidable. All Engineers learn to create powerful protective shields and the best Engineers can hack into satellites, and use them to teleport any member of the team to the Engineer's location.")
        }
        (509, 103_286_066) => {
            String::from("Click to set your character's body type to Slender.")
        }
        (509, 196_674_932) => {
            String::from("When you are satisfied with your selection, click to proceed to appearance selection.")
        }
        (509, 234_583_117) => {
            String::from("Click to set your character's body type to Medium.")
        }
        (509, 190_301_160) => {
            String::from("Finish.")
        }
        (509, 122_709_390) => {
            String::from("Nickname is already in use.")
        }
        (509, 3) => {
            String::from("Grid")
        }
        (509, 770_493) => {
            String::from("Character problem. Contact support.")
        }
        (509, 128_396_253) => {
            String::from("Medium.")
        }
        (509, 140_082_788) => {
            String::from("Geosynchronous orbit around Rubi-Ka")
        }
        (509, 200_852_322) => {
            String::from("Difficulty to play: <FONT color=\"#ffff00\"> Medium </FONT>\r\nA Doctor is really a biotechnology specialist. The Doctors' prime skills focus mainly on healing and protecting but they also learn how to produce and administer powerful biotoxins that slow, weaken and wear down their opponents. Limited weapon skills can lead to a bumpy ride when going solo, but in a team the Doctor really shines. When chaos descends on the team in combat, its survival usually lies squarely on the Doctor's shoulders, so this profession is not for the faint-hearted.")
        }
        (509, 125_021_838) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Breed: {a}\r\nProfession: {b}")
        }
        (509, 0) => {
            String::from("Normal House")
        }
        (509, 72_444_052) => {
            String::from("Trader Selected.")
        }
        (509, 123_574_717) => {
            String::from("Medium.")
        }
        (509, 1000) => {
            String::from("undefined house type")
        }
        (509, 55_564_820) => {
            String::from("Digits can only be used at the end of the name and only after the 4th char.")
        }
        (509, 165_810_482) => {
            String::from("Click to inspect the Trader.")
        }
        (509, 195_917_652) => {
            String::from("Select head")
        }
        (509, 90_551_204) => {
            String::from("Account has illegal password.")
        }
        (509, 118_218_323) => {
            String::from("Checking credentials...\r\n")
        }
        (509, 169_342_932) => {
            String::from("Player has illegal password.")
        }
        (509, 262_139_760) => {
            String::from("Type in a name for your character in the text entry box. You can have the server generate a name for your character by clicking the Suggest Name Button. Note that your character's name needs to be unique. You will be notified if your chosen name has already been taken.")
        }
        (509, 51_780_996) => {
            String::from("Solitus Female selected.")
        }
        (509, 131_506_020) => {
            String::from("Difficulty to play: <FONT color=\"#ffff00\"> Low </FONT>\r\nWhen it comes to dishing out raw combat damage a Martial Artist outshines all other professions. Fighting unarmed, the Martial Artist's main strength lies in special attacks, knowing how to cripple opponents by attacking their weak spots. The Martial Artist is also a very proficient healer, truly surpassed by only the Doctor and the Adventurer.")
        }
        (509, 58_751_332) => {
            String::from("Difficulty to play: <FONT color=\"#ffff00\"> High </FONT>\r\nThe Bureaucrat brings order to chaos. Bureaucrats have very limited weapon skills but their vast knowledge of nanotechnology makes up for that. They use it to directly damage opponents and create robots that will fight for them. But most importantly, a Bureaucrat uses nanotechnology to boost his leadership skills and to control hostile beings, bending their minds and blurring their aims. In a team situation, a Bureaucrat is therefore a natural leader.")
        }
        (509, 246_788_914) => {
            String::from("Click to inspect the Engineer.")
        }
        (509, 196_649_904) => {
            String::from("Select your character's breed by clicking on the breed models. Each breed offers different advantages and disadvantages. Learn more by selecting a breed. When you have made your choice click Next.")
        }
        (509, 192_016_820) => {
            String::from("Click to go back to Breed selection.")
        }
        (509, 235_785_269) => {
            String::from("You must enter a name before you can proceed!")
        }
        (509, 111_591_508) => {
            String::from("Click to inspect the Martial Artist.")
        }
        (509, 93_730_238) => {
            String::from("Account already in use, character logged in.")
        }
        (509, 235_147_346) => {
            String::from("Click to inspect the Soldier.")
        }
        (509, 67_416_068) => {
            String::from("Player not found.")
        }
        (509, 156_926_699) => {
            String::from("Click to go back to Appearance selection.")
        }
        (509, 80_046_117) => {
            String::from("Click to select Nanomage Female.")
        }
        (509, 248_489_460) => {
            String::from("Account not paid.")
        }
        (509, 176_437_060) => {
            String::from("Soldier Selected.")
        }
        (509, 137_986_779) => {
            String::from("Click to go back to Profession selection.")
        }
        (509, 46_271_652) => {
            String::from("by selecting a face, body height, and body type. Your character's appearance does not influence the gameplay in any way. When you have made your choices, click Next.")
        }
        (509, 4) => {
            String::from("Guard House")
        }
        (509, 98_043_122) => {
            String::from("No server with correct RDB.")
        }
        (509, 191_315_100) => {
            String::from("Tall.")
        }
        (509, 76_728_164) => {
            String::from("Atrox selected.")
        }
        (509, 229_364_195) => {
            String::from("The name must contain more than 3 characters and less than 13.")
        }
        (509, 247_296_306) => {
            String::from("Click to inspect the Enforcer.")
        }
        (509, 1) => {
            String::from("HQ")
        }
        (509, 164_658_644) => {
            String::from("Please choose a Breed.")
        }
        (509, 163_481_026) => {
            String::from("Difficulty to play: <FONT color=\"#ffff00\"> Low </FONT>\r\nAn Enforcer specializes in close combat using raw power and naked rage to subdue opponents. Enforcers are physically better suited than all others to sustain damage and can learn some protective nanotechnology to further increase their chances of survival. Their brutality and in-your-face combat attitude normally make them the prime targets for any opponent. Enforcers utilize this for the good of the team and rely largely on others to heal them.")
        }
        (509, 108_184_756) => {
            String::from("Docking procedure completed")
        }
        (509, 342_389) => {
            String::from("Click to select Nanomage Male.")
        }
        (509, 225_957_460) => {
            String::from("Select height")
        }
        (509, 103_616_948) => {
            String::from("Nanomage Female selected.")
        }
        (509, 24_541_365) => {
            String::from("Boarding shuttle to Omni-1")
        }
        (509, 156_977_540) => {
            String::from("Click to proceed to name selection.")
        }
        (509, 181_436_052) => {
            String::from("Select build")
        }
        (509, 207_344_627) => {
            String::from("Click this button to finish character creation and start in the Shadowlands.\r\nYour character will be transferred to the Jobe research facilities.\r\n\r\nOnce you click this button, you can no longer edit your character's breed, appearance, profession or name!\r\n\r\n(You need the Shadowlands expansion pack to start in the Shadowlands.)")
        }
        (509, 240_934_557) => {
            String::from("Something is wrong with the server.")
        }
        (509, 260_745_330) => {
            String::from("Click to inspect the Fixer.")
        }
        (509, 24_135_060) => {
            String::from("Character not found.")
        }
        (509, 115_820_932) => {
            String::from("Opifex Female selected.")
        }
        (509, 183_833_282) => {
            String::from("Difficulty to play: <FONT color=\"#ffff00\"> High </FONT>\r\nThe Trader is the ultimate entrepreneur, getting more for less in every single transaction. Like others, they create and trade material goods. But in combat situations, they use unique nanotechnology which enables them to drain opponents of skills, energy and health, transfering those benefits to themselves or their allies. This frequently causes the most formidable opponents to wither to a cracked shell of their former self.")
        }
        (509, 185_671_390) => {
            String::from("Contact administration.")
        }
        (509, 63_701_108) => {
            String::from("Shade Selected.")
        }
        (509, 7) => {
            String::from("Mini Cloaking Device")
        }
        (509, 72_416_866) => {
            String::from("Difficulty to play: <FONT color=\"#ffff00\"> Low / Medium </FONT>\r\nAn Adventurer's soul is at home in the wild. Adventurers study the animals, learn their ways and gain some of their abilities along the way. Their weapon skills are well balanced with equal advancement opportunities in melee and ranged combat. They also become skilled at using nanotechnology to withstand damage or to create metaphysical cloaks that damage their aggressors. An Adventurer is an excellent healer, only truly rivaled by the Doctor.")
        }
        (509, 262_052_372) => {
            String::from("All your selections will be saved untill next time you enter.")
        }
        (509, 82_048_901) => {
            String::from("S E L E C T   A P P E A R A N C E")
        }
        (509, 108_956_596) => {
            String::from("Solitus Male selected.")
        }
        (509, 27_330_865) => {
            String::from("Continuing to DNA construction area")
        }
        (509, 35_129_054) => {
            String::from("The Nanomage is unique to Rubi-Ka, genetically engineered for the notum-rich environment, and thus unable to live anywhere else in the galaxy. The Nanomage has a strong advantage in using nanotechnology and metaphysics.")
        }
        (509, 94_046_782) => {
            String::from("The Opifex is genetically engineered to be a fast and agile breed, at the expense of its strength and stamina. The Opifex is versatile and has spread wide across the galaxy. The Opifex makes for a good agent, fixer or martial artist, where speed and precision is beneficial.")
        }
        (509, 108_018_980) => {
            String::from("Short.")
        }
        (509, 128_765_541) => {
            String::from("Click to select Opifex Male.")
        }
        (509, 152_027_794) => {
            String::from("Click to inspect the Doctor.")
        }
        (509, 224_353_244) => {
            String::from("Click to set your character's height to Tall.")
        }
        (509, 170_145_658) => {
            String::from("Nickname:")
        }
        (509, 56_161_934) => {
            String::from("Character already logged in.")
        }
        (509, 100_026_692) => {
            String::from("Click to set your character's height to Short.")
        }
        (509, 108_214_510) => {
            String::from("Difficulty to play: <FONT color=\"#ffff00\"> Medium </FONT>\r\nA Nano Technician is an expert user of aggressive nanotechnology. Nano Technicians are experts at using nanotechnology to deal explosive area-of-effect damage and the types of damage they can manage is incomparable. They are also capable of using other kinds of nanotechnology and can for example warp themselves between locations. Nano-Technicians must be devoted to nanotechnology skills and as a result, physical and weapon skills will suffer.")
        }
        (509, 177_652_275) => {
            String::from("Can't create more characters.")
        }
        (509, 260_793_573) => {
            String::from("E N T E R   N A M E")
        }
        (509, 13_618_949) => {
            String::from("Select face.")
        }
        (509, 163_999_213) => {
            String::from("Click to set your character's height to Medium.")
        }
        (509, 18_488_884) => {
            String::from("Nano Technician Selected.")
        }
        (509, 230_137_493) => {
            String::from("Click to select Solitus Male.")
        }
        (509, 234_353_508) => {
            String::from("Agent Selected.")
        }
        (509, 83_594_021) => {
            String::from("Please board the shuttle to Rubi-Ka, secure yourself and enjoy the ride!\r\nShortly you will find yourself in the Arrival Hall in the spaceport of Omni-1.")
        }
        (509, 107_443_049) => {
            String::from("Heavy.")
        }
        (509, 244_592_482) => {
            String::from("Difficulty to play: <FONT color=\"#ffff00\"> Low / Medium </FONT>\r\nA Soldier strives for excellence in armed combat. The skill advancement of Soldiers focuses entirely on assault and survival. They use unique nanotechnology to protect their bodies, enhance reflexes, weapon skills and their armor. Perhaps most importantly, Soldiers are able to create strong damage absorption shields around themselves which make them partly invulnerable and even reflect some of the damage back to the attacker.")
        }
        (509, 137_916_099) => {
            String::from("Difficulty to play: <FONT color=\"#ffff00\"> Medium / High </FONT>\r\nThe Meta-Physicists get their strength from the \"other side\". They can manifest their emotions in the material world and eventually control multiple materialized entities and use them in combat. The uniqueness of a Meta-Physicist lies in manipulating the underlying fabric of the world where nanotechnology operates, adjusting the nanotechnology skills of friends and foes alike. Their weapon skills are poor but they can use their powers to damage their opponents directly.")
        }
        (509, 261_898_517) => {
            String::from("Click to inspect the Shade.")
        }
        (509, 27_519_330) => {
            String::from("Click to inspect the Adventurer.")
        }
        (509, 183_066_372) => {
            String::from("Click to have the server suggest a name for your character. If the suggested name is not to your liking, click the button again. You can modify a suggested name by editing it in the text entry box.")
        }
        (509, 264_418_457) => {
            String::from("Click to set your character's body type to Heavy.")
        }
        (509, 112_774_212) => {
            String::from("S E L E C T   B R E E D")
        }
        (509, 52_480_948) => {
            String::from("Select your character's profession")
        }
        (509, 106_904_827) => {
            String::from("Go Back.")
        }
        (509, 177_133_309) => {
            String::from("The name can only contain alphabetic chars or digits. And no character after a digit.")
        }
        (509, 2) => {
            String::from("Market")
        }
        (509, 260_098_452) => {
            String::from("Doctor Selected.")
        }
        (509, 85_960_868) => {
            String::from("Bureaucrat Selected.")
        }
        (509, 8) => {
            String::from("Tax Refund Office")
        }
        (509, 7_564_900) => {
            String::from("Click to inspect the Bureaucrat.")
        }
        (509, 107_050_200) => {
            String::from("Click to select Atrox.")
        }
        (509, 108_549_524) => {
            String::from("Nickname is invalid or reserved.")
        }
        (509, 204_746_788) => {
            String::from("Martial Artist Selected.")
        }
        (509, 218_721_543) => {
            String::from("Click this button to finish character creation and start in Arete Landing. Your character will be transferred to Arete Landing. Once you click this button, you can no longer edit your character's breed, appearance, profession or name!")
        }
        (509, 213_881_643) => {
            String::from("Getting suggested nickname...\r\n")
        }
        (509, 24_169_982) => {
            String::from("Click to inspect the Nano Technician.")
        }
        (509, 42_695_918) => {
            String::from("Click to proceed to Profession selection.")
        }
        (509, 13_634_964) => {
            String::from("Account is locked.")
        }
        (509, 245_803_508) => {
            String::from("Enforcer Selected.")
        }
        (509, 207_007_426) => {
            String::from("Difficulty to play: <FONT color=\"#ffff00\"> Medium </FONT>\r\nThe Keeper is a fighter that radiates valour and heroism - a beacon of light and hope to the team. A formidable opponent who specialises in close combat, the Keeper is especially proficient wielding two-handed edged weapons. This profession's uniqueness lies in the ability to share life and diverse powers with nearby allies.")
        }
        (509, 116_636_116) => {
            String::from("Click to inspect the Metaphysicist.")
        }
        (509, 60_879_742) => {
            String::from("S E L E C T   P R O F E S S I O N")
        }
        (1000, 267_728_564) => {
            let a = &arguments[0];
            format!("You received {a} credits as surrender gift.")
        }
        (1000, 10_088_188) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("Are you sure you want to send {a} credits and {b} to {c}?")
        }
        (1000, 202_886_865) => {
            String::from("Social tab can not be activated in PVP areas.")
        }
        (1000, 265_505_899) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            format!("Unable to perform action {a} perk is locked, able in {b:02}:{c:02}:{d:02}")
        }
        (1000, 140_399_673) => {
            String::from("A massive artillery strike from the alien mothership has resulted in your death.")
        }
        (1000, 222_040_796) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            format!("Unable to perform action, {a} skill is locked, able in {b:02}:{c:02}:{d:02}")
        }
        (1000, 129_558_592) => {
            String::from("Congratulations on reaching level 10! You can continue playing for free in a whole new world of adventure, action and exploration by downloading the full version of Anarchy Online Classic. See www.anarchy-online.com for details and download links.")
        }
        (1000, 22_211_373) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Are you sure you want to send {a} credits and {b}?")
        }
        (1000, 157_877_875) => {
            String::from("Yes")
        }
        (1000, 210_536_067) => {
            String::from("Received team bonus:")
        }
        (1000, 126_449_605) => {
            String::from(" Active")
        }
        (1000, 73_139_891) => {
            let a = &arguments[0];
            format!("Are you sure you want to accept the item and pay {a} credits to the sender?")
        }
        (1000, 20_773_493) => {
            String::from("Are you sure you want to use this item?")
        }
        (1000, 27_342_852) => {
            String::from("You killed the mission target!..")
        }
        (1000, 86_321_429) => {
            String::from(" Disabled")
        }
        (1000, 250_391_324) => {
            String::from("Nanoprogram failed..unusual reason..")
        }
        (1000, 199_571_891) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            format!("<font color=CCInfoHeader>Time until suppression field changes to {a}%:</font> {b:02}:{c:02}:{d:02}")
        }
        (1000, 137_366_172) => {
            String::from("PVP is not allowed when social tab is active.")
        }
        (1000, 250_613_668) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("Are you sure you want to send {a} to {b} and request {c} credits in return? A commision of 5% will be charged by the mail system.")
        }
        (1000, 151_981_284) => {
            String::from("Hyperspace transport field is increasing in strength. Leave area to avoid potential permanent death.")
        }
        (1000, 3_359_156) => {
            String::from("You are now insured..")
        }
        (1000, 208_922_380) => {
            String::from("Are you sure you want to delete this message? This operation can not be reversed.")
        }
        (1000, 66_239_572) => {
            String::from("This controller has been destroyed, and does not work anymore.")
        }
        (1000, 257_836_195) => {
            let a = &arguments[0];
            format!("{a} appears...")
        }
        (1000, 77_097_141) => {
            let a = &arguments[0];
            format!("{a} became visible...")
        }
        (1000, 81_728_596) => {
            String::from("This controller has already been deactivated.")
        }
        (1000, 192_961_774) => {
            String::from("Social tab can not be activated in the battlestation.")
        }
        (1000, 71_585_108) => {
            String::from("Genetic insurance is voided by hyperspace travel. To avoid a potential breach of contract, and possible permanent death, genome will be recalled by your insurer.")
        }
        (1000, 743_316) => {
            let a = &arguments[0];
            let b = {
                if a.to_string() == "1" {
                    " credit was"
                } else {
                    " credits were"
                }
            };
            format!("{a} {b} deducted from your account.")
        }
        (1000, 2_158_542) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("Unable to perform action, able in {a:02}:{b:02}:{c:02}")
        }
        (1169, 100) => {
            String::from("Fledgling")
        }
        (1169, 2700) => {
            String::from("Hero")
        }
        (1169, 2000) => {
            String::from("Backer")
        }
        (1169, 800) => {
            String::from("Intermediate")
        }
        (1169, 1400) => {
            String::from("Qualified")
        }
        (1169, 0) => {
            String::from("None")
        }
        (1169, 1000) => {
            String::from("Fair")
        }
        (1169, 1800) => {
            String::from("Trustworthy")
        }
        (1169, 500) => {
            String::from("Newcomer")
        }
        (1169, 2100) => {
            String::from("Defender")
        }
        (1169, 400) => {
            String::from("Starter")
        }
        (1169, 1600) => {
            String::from("Suited")
        }
        (1169, 1900) => {
            String::from("Supporter")
        }
        (1169, 1300) => {
            String::from("Adept")
        }
        (1169, 2200) => {
            String::from("Challenger")
        }
        (1169, 2500) => {
            String::from("Medalist")
        }
        (1169, 1100) => {
            String::from("Able")
        }
        (1169, 200) => {
            String::from("Amateur")
        }
        (1169, 700) => {
            String::from("Common")
        }
        (1169, 300) => {
            String::from("Beginner")
        }
        (1169, 600) => {
            String::from("Student")
        }
        (1169, 2600) => {
            String::from("Champ")
        }
        (1169, 2800) => {
            String::from("Guardian")
        }
        (1169, 900) => {
            String::from("Mediocre")
        }
        (1169, 1200) => {
            String::from("Accomplished")
        }
        (1169, 3000) => {
            String::from("Vindicator")
        }
        (1169, 2300) => {
            String::from("Patron")
        }
        (1169, 2400) => {
            String::from("Protector")
        }
        (1169, 2900) => {
            String::from("Vanquisher")
        }
        (1169, 1700) => {
            String::from("Talented")
        }
        (1169, 1500) => {
            String::from("Competent")
        }
        (501, 67_156_085) => {
            let a = &arguments[0];
            format!("Do you want to join the organization \"{a}\"?")
        }
        (501, 220_373_365) => {
            let a = &arguments[0];
            let b = {
                if a.to_string() == "1" {
                    " credit. "
                } else {
                    " credits. "
                }
            };
            format!("The organization tax has been changed to {a} {b}  Do you wish to leave your organization?")
        }
        (501, 3_626_020) => {
            String::from("The organization name could not be changed, because the name has been changed too recently.  Organization names can only be changed once a week.")
        }
        (501, 132_464_904) => {
            String::from("The organization has no tax at this time.")
        }
        (501, 181_448_347) => {
            let a = &arguments[0];
            format!("{a} kicked from organization (alignment changed).")
        }
        (501, 182_257_540) => {
            String::from("You created an organization and became the leader of it.")
        }
        (501, 13_975_269) => {
            String::from("The organization name contained an illegal word or is a reserved name.")
        }
        (501, 35_204_276) => {
            let a = &arguments[0];
            format!("{a} changed alignment - organization disbanded.")
        }
        (501, 174_731_972) => {
            String::from("You can only create an organization of a full 6 member team.  All members must be of the same side.  No members can be part of any other organization. The leader can not be a free player.")
        }
        (501, 13_951_557) => {
            String::from("The name cannot start with a lowercase letter.")
        }
        (501, 192_568_104) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("WARNING: City upkeep ({a} credits) is due in {b} hours but the org bank only contains {c} credits. If the bank does not contain enough credits by the due date, your city will be demolished.")
        }
        (501, 250_057_874) => {
            String::from("No controller yet.")
        }
        (501, 167_950_692) => {
            String::from("The name is already used by another organization.")
        }
        (501, 155_922_318) => {
            let a = &arguments[0];
            format!("Are you sure you want to transfer organization leadership to {a}?")
        }
        (501, 244_098_632) => {
            let a = &arguments[0];
            let b = {
                if a.to_string() == "1" {
                    " credit "
                } else {
                    " credits"
                }
            };
            format!("You owe tax to the amount of {a} {b}.  Do you wish to pay the amount to your organization?\r\n(If not, your debt will have to be paid by the other members.)")
        }
        (501, 193_456_776) => {
            let a = &arguments[0];
            let b = {
                if a.to_string() == "1" {
                    " credit "
                } else {
                    " credits"
                }
            };
            format!("The organization has a tax of {a} {b}.")
        }
        (501, 13_952_082) => {
            String::from("The organization name contained an invalid character.  The character might not be allowed at all, or not in combination with the surrounding characters.")
        }
        (501, 89_300_600) => {
            String::from("The name had an invalid length.  Organization names must be between 4 and 40 characters long.")
        }
        (501, 204_233_972) => {
            String::from("Are you sure you want to disband your organization?\r\n\r\nNotice that the contents of the organization bank will be lost and any land control areas and cities controlled by the organization will be destroyed.")
        }
        (501, 9_700_511) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Controlled Land: {a} Controller: {b}")
        }
        (501, 24_190_273) => {
            String::from("The organization name must contain at least 3 alphabetical characters.")
        }
        (501, 115_075_092) => {
            String::from("Organization not found!")
        }
        (2001, 97) => {
            String::from("The Fire Armor Class defines how much damage you can mitigate against Fire based attacks.")
        }
        (2001, 144) => {
            String::from("Dimach is a skill used to equip weapons that have the Dimach special attack ability. The higher your skill in Dimach the more effective these special attacks will be. Note that Dimach is also enabled inherently when using no weapons (i.e. hand to hand combat) Dependencies: 80% Sense, 20% Psychic")
        }
        (2001, 157) => {
            String::from("Quantum forcefield technics is a tradeskill used to make quantum based components.  Dependencies: Intelligence 50%, Psychic 50%")
        }
        (2001, 91) => {
            String::from("The Melee and martial arts AC defines how much damage you can avoid from those types of attacks.")
        }
        (2001, 118) => {
            String::from("The melee initiative skill is used to attack quicker with melee weapons.  Dependencies 60% Sense, 20% Psychic, 10% Intelligence, 10% Agility")
        }
        (2001, 116) => {
            String::from("This skill allows you to use better Assault Rifles and increase your attack rating whilst using Assault Rifles.  Dependencies 40% Stamina, 30% Agility, 20% Sense 10% Strength.")
        }
        (2001, 152) => {
            String::from("Body development defines your health, or hit points.  It is exclusively dependant on Stamina.  Your character level, your profession, and your breed also influences your health.")
        }
        (2001, 117) => {
            String::from("This skill enables the driver to access water based vehicles.  Dependencies: 60% Sense, 20% Intelligence, 20% Agility.")
        }
        (2001, 137) => {
            String::from("Adventuring enables the character to access different backpacks and some special items. Dependencies: 20% Strength, 50% Agility, 30% Stamina")
        }
        (2001, 112) => {
            String::from("The Pistol Weapon Skill allows you to use Pistol Weapons and increases your attack rating when wielding Pistol Weapons.   Dependencies: 60% Agility, 40% Sense")
        }
        (2001, 96) => {
            String::from("The Posion Armor Class defines how much damage you can mitigate agiants poison based attacks.")
        }
        (2001, 126) => {
            String::from("Electrical engineering is a tradeskill mainly used in the production of electrical appliances and components.  Dependencies: Intelligence 50%, Agility 30%, Stamina 20%")
        }
        (2001, 17) => {
            String::from("Agility is one of your primary attributes. Your primary attributes affect your other skills - the higher your primary attribute the more benefit you will get in skills that use Agility as a dependency.\r\n")
        }
        (2001, 95) => {
            String::from("The Cold Armor Class defines how much damage you can mitigate against Cold based attacks.")
        }
        (2001, 151) => {
            String::from("Aimed shot is a skill used to equip weapons that have the Aimed Shot special attack ability. The higher your skill in Aimed Shot the more effective these special attacks will be. Dependency: 100% Sense")
        }
        (2001, 156) => {
            String::from("The run speed skill defines how fast your character can move. Dependencies: 40% Agility, 40% Stamina, 20% Strength")
        }
        (2001, 102) => {
            String::from("The One Handed (1h) Blunt Weapon Skill allows you to use 1hb Weapons and increases your attack rating when wielding 1hb Weapons.  Dependencies 50% Strength, 40% Stamina 10% Agility.")
        }
        (2001, 158) => {
            String::from("Weapon smithing is a tradeskill used primarily in the production of weapons and weapon upgrades.  Dependencies: Intelligence 50%, Strength 50%.")
        }
        (2001, 92) => {
            String::from("The Energy Armor Class defines how much damage you can mitigate against Energy based attacks.")
        }
        (2001, 119) => {
            String::from("The ranged weapon initiative skill allows you to attack quicker with any ranged weapon.  Dependencies: 60% Sense, 20% Psychic, 10% Intelligence 10% Agility.")
        }
        (2001, 108) => {
            String::from("The Thrown Weapons Skill allows you to use thrown weapons and increases your attack rating when wielding thrown weapons. Dependencies 60% Agility, 20% Strength 20% Sense.")
        }
        (2001, 133) => {
            String::from("The ranged energy weapon skill allows you to use Ranged Energy Weapons and increases your attack rating when wielding Ranged Energy Weapons. Dependencies: 40% Psychic, 40% Sense, 20% Intelligence")
        }
        (2001, 138) => {
            String::from("Swim speed effects how fast you can swim.  Dependencies 20% Agility, 60% Stamina  20% Strength.")
        }
        (2001, 167) => {
            String::from("The Full Auto skill is a skill used to equip weapons that have the Full Auto special attack ability.  The higher your skill in Full Auto, the more effective these special attacks will be.  Dependencies: 60% Strength, 40% Stamina.")
        }
        (2001, 109) => {
            String::from("The Grenade Skill allows you to use grenade based weapons and increases your attack rating when wielding grenade based weapons. Dependencies: 40%Agility, 40% Sense, 20% Intelligence")
        }
        (2001, 130) => {
            String::from("Matter creation is used to make matter appear from energy, the basic functioning of the nano bots.  Dependencies: Intelligence 80%, Stamina 20%.")
        }
        (2001, 123) => {
            String::from("First aid allows you to use health and nano point stim injectors that can be bought from medical vendors  Dependencies: 40% Sense, 30% Intelligence, 30% Agility")
        }
        (2001, 153) => {
            String::from("The duck skill is the one used to determine if you are hit by certain types of weapons, most commonly shotguns.  Dependencies: 50% Agility, 30% Sense, 20% Intelligence.")
        }
        (2001, 154) => {
            String::from("The Dodge Ranged skill is the one used to determine if you are hit by certain types of weapons or attacks, in this case most common ranged weapons.  Dependencies 50% Agility, 30% Sense, 20% Intelligence")
        }
        (2001, 155) => {
            String::from("The evade close combat attack skill is the one used to determine if you are hit by certain types of weapons or attacks, in this case Melee and Martial arts attacks.  Dependencies: 50% Agility, 30% Sense, 20% Intelligence.")
        }
        (2001, 150) => {
            let a = &arguments[0];
            format!("The fling shot is a skill used to equip weapons that have the fling shot special attack ability. The higher your skill in fling shot the more effective these special attacks will be.   It is 100{a}ependent on Agility.")
        }
        (2001, 122) => {
            String::from("Sensory improvement is a skill used for learning and casting Nano formulas that use Sensory improvement as a requirement. If a nano formula is offensive and carries a skill check that is based off Sensory improvement then the higher this skill the higher the chance will be of the attack landing on your target.  Dependencies: Intelligence 80%, Strength 20%.")
        }
        (2001, 127) => {
            String::from("Matter Metamorphosis is a skill used for learning and casting Nano formulas that use Matter Metamorphosis as a requirement. If a nano formula is offensive and carries a skill check that is based off Matter Metamorphosis then the higher this skill the higher the chance will be of the attack landing on your target.   Dependencies: Intelligence 80%, Psychic 20%")
        }
        (2001, 18) => {
            String::from("Stamina is one of your primary attributes. Your primary attributes affect your other skills - the higher your primary attribute the more benefit you will get in skills that use Stamina as a dependency.")
        }
        (2001, 143) => {
            String::from("Riposte is used to equip some Martial Art special attacks and some special devices. Dependencies: 50% Agility, 50% Sense")
        }
        (2001, 159) => {
            String::from("Pharmaceutical tech is a tradeskill primarily used to make medicines and stims.  Dependencies: Intelligence 80%, Agility 20%")
        }
        (2001, 101) => {
            String::from("The multiple wield melee weapons skill allows you to equip a weapon with this requirement in your off hand.  Dependencies Agility 60%, Strength 30% and Stamina 10%.")
        }
        (2001, 103) => {
            String::from("The One Handed (1h) Edged Weapon Skill allows you to use 1he Weapons and increases your attack rating when wielding 1he Weapons. Dependencies: 30% Strength, 30% Stamina, 40% Agility")
        }
        (2001, 104) => {
            String::from("The Energy Melee Weapon skill allows you to use energy base melee weapons and increases your attack rating when wielding energy melee weapons. Dependencies:50% Stamina, 50% Intelligence")
        }
        (2001, 140) => {
            String::from("This skill is used to determine what type of map implants you can use, and how much information you get from them.  Map implants can be bought from map vendor terminals and offer such upgrades as being able to see monsters on the mini-map.\r\n\r\nDependencies: Sense 50%, Intelligence 40%, Psychic 10%")
        }
        (2001, 136) => {
            String::from("Perception is the core skill used in detecting things, be it concealed people, hidden objects or traps.  Dependent on Sense 70% and Intelligence 30%.")
        }
        (2001, 100) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("Martial Arts is the main unarmed combat skill.  Increasing this allows you to not only hit more often, but to do extra damage per hit.  It's 50{a}ependent on Agility, 20{b}n Strength and 30{c}n Psychic.")
        }
        (2001, 168) => {
            String::from("Your Nano Formula Resist skill decides how well you resist hostile Nano Programs. Raising this skill will help you to occasionally resist nano based attacks. Dependencies:  Intelligence 20%, Psychic 80%.")
        }
        (2001, 124) => {
            String::from("Treatment allows you to use health and nano point treatment kits that can be bought from medical vendors. It is also used to determine what level of implants you can equip. Dependencies: 50% Intelligence, 30% Agility and 20% Sense")
        }
        (2001, 114) => {
            String::from("The Sub Machine Gun skill allows you to use SMG Weapons and increases your attack rating when wielding SMG Weapons. Dependencies: 30% Strength, 30% Agility, 30% Stamina, 10% Sense")
        }
        (2001, 141) => {
            String::from("Tutoring is a skill that allows you to use the tutoring devices that can be found on Rubi-Ka. These devices can be used to temporarily transfer tradeskill knowledge to another person.  Dependencies: 70% Intelligence, 20% Sense, 10% Psychic")
        }
        (2001, 105) => {
            String::from("The Two handed Edged weapon skill allows you to use 2he Weapons and increases your attack rating when wielding 2he Weapons. Dependencies: 60% Strength, 40% Stamina")
        }
        (2001, 148) => {
            String::from("Burst is a skill used to equip weapons that have the Burst special attack ability. The higher your skill in Burst the more effective these special attacks will be.  Defined by: Agility 50%, Strength 30%, Stamina 20%")
        }
        (2001, 115) => {
            String::from("This skill allows you to use better Shotguns and increase your attack rating whilst using Shotguns.  Dependencies: 60% Agility 40% Strength.")
        }
        (2001, 162) => {
            String::from("Psychology is used as a skill check in the charm line of nano formulas. It is also used in some tradeskill processes. Dependencies: Intelligence 50%, Sense 50%")
        }
        (2001, 134) => {
            String::from("The multiple wield ranged weapons skill allows you to equip a weapon with this requirement in your off hand  Dependencies: 60% Agility, 40% Intelligence")
        }
        (2001, 135) => {
            String::from("Trap disarmament is of course used to disarm traps.  It is also used in conjunction with perception to detect traps.  Dependencies: Sense 60%, Intelligence 20% and Agility 30%.")
        }
        (2001, 121) => {
            String::from("The bow special attack skill is a skill used to equip weapons that have the Bow special attack ability. The higher your skill in bow special attack the more effective these special attacks will be. Dependencies:  50% Agility, 40% Sense and 10% Strength.")
        }
        (2001, 125) => {
            String::from("Mechanical engineering is the tradeskill focusing on motors and other mechanical objects.  Dependencies: Intelligence 50%, Agility 50%")
        }
        (2001, 111) => {
            String::from("This skill allows you to use better Bows and increase your attack rating whilst using Bows. Dependencies: 40% Agility, 40% Sense 20% Strength.")
        }
        (2001, 106) => {
            String::from("The Piercing Weapons skill allows you to use piercing weapons and increases your attack rating when wielding piercing weapons.  Dependencies: 50% Agility, 30% Stamina, 20% Strength")
        }
        (2001, 107) => {
            String::from("The Two Handed (2h) Blunt Weapon Skill allows you to use 2hb Weapons and increases your attack rating when wielding 2hb Weapons. Dependencies: 50%Strength, 50% Stamina")
        }
        (2001, 147) => {
            String::from("Fast attack is a skill used to equip weapons that have the Fast Attack special attack ability. The higher your skill in fast Attack the more effective these special attacks will be. Dependencies: 60% Agility, 40% Sense")
        }
        (2001, 90) => {
            String::from("All Armor Class skills are not true skills in any sense of the word.  You can't normally increase them by any other means than wearing armor, their listings here are only for information purposes.")
        }
        (2001, 146) => {
            String::from("Sneak attack is a skill used to equip weapons that have the Sneak attack special attack ability. The higher your skill in sneak attack the more effective these special attacks will be. Dependedncies: 80% Sense, 20% Psychic")
        }
        (2001, 113) => {
            String::from("The Rifle Skill allows you to use rifles weapons and increases your attack rating when wielding rifles. Dependencies: 60% Agility, 40% Sense")
        }
        (2001, 129) => {
            String::from("The nano skill psychological modifications is a skill used for learning and casting Nano formulas that use psychological modifications as a requirement. If a nano formula is offensive and carries a skill check that is based off psychological modifications then the higher this skill the higher the chance will be of the attack landing on your target.  Dependencies: Intelligence 80%, Sense 20%")
        }
        (2001, 160) => {
            String::from("Nano programming is a tradeskill used primarily in the production of Implants and Nano programs.  Dependencies: Intelligence 100%")
        }
        (2001, 163) => {
            String::from("Chemistry is a tradeskill primarily used in the production of chemicals and other components.  Dependencies: Stamina 50%, Intelligence 50%")
        }
        (2001, 139) => {
            String::from("This skill enables the driver to access airborne vehicles. Dependencies: 60% Sense, 20% Intelligence, 20% Agility")
        }
        (2001, 166) => {
            String::from("A skill which defines what type of ground vehicle you can use.  Dependencies: Sense 60%, Intelligence 20%, Agility 20%")
        }
        (2001, 16) => {
            String::from("Strength is one of your primary attributes. Your primary attributes affect your other skills - the higher your primary attribute the more benefit you will get in skills that use Strength as a dependency.")
        }
        (2001, 93) => {
            String::from("The Chemical damage armor class shows you how much protection you have against Chemical based attacks.")
        }
        (2001, 165) => {
            String::from("Breaking and entering is a skill used to open locked doors and other locked devices.  Dependencies: Agility 40%, Sense 30% and Psychic 30%.")
        }
        (2001, 132) => {
            String::from("This skill defines, along with profession, character level and breed, how much Nano Energy you have.  Dependencies: 70% Psychic, 10% Sense, 10% Intelligence, 10% Stamina")
        }
        (2001, 164) => {
            String::from("Concealment is a skill used to hide from observation.  Dependencies: Sense 70% and Agility 30%.")
        }
        (2001, 21) => {
            String::from("Psychic is one of your primary attributes. Your primary attributes affect your other skills - the higher your primary attribute the more benefit you will get in skills that use Psychic as a dependency.")
        }
        (2001, 145) => {
            String::from("Deflect is a requirement for equipping certain weapons and combat based items. Deflect increases the chance for a glancing blow.  Depenedencies: 50% Strength, 20% Agility, 30% Sense")
        }
        (2001, 131) => {
            String::from("Time and space is the nano skill used to run nano programs affecting time and space.  Dependencies: Intelligence 80%, Agility 20%.")
        }
        (2001, 110) => {
            String::from("The Heavy Weapons Skill allows you to use Heavy Weapons and increases your attack rating when wielding Heavy Weapons.  Dependencies: 60% Agility, 40% Strength.")
        }
        (2001, 120) => {
            String::from("The physical prowess initiative skill is used to increase the speed of your martial art attacks.  Dependencies 60% Sense, 20% Psychic, 10% Intelligence, 10% Agility")
        }
        (2001, 94) => {
            String::from("The Radiation Armor Class defines how much damage you can mitigate against Radiation based attacks.")
        }
        (2001, 20) => {
            String::from("Sense is one of your primary attributes. Your primary attributes affect your other skills - the higher your primary attribute the more benefit you will get in skills that use Sense as a dependency.")
        }
        (2001, 19) => {
            String::from("Intelligence is one of your primary attributes. Your primary attributes affect your other skills - the higher your primary attribute the more benefit you will get in skills that use Intelligence as a dependency.")
        }
        (2001, 142) => {
            String::from("Brawling is a skill used to equip weapons that have the Brawl special attack ability. The higher your skill in Brawl the more effective these special attacks will be. Note that Brawl is also enabled inherently when using no weapons (i.e. hand to hand combat)  Dependencies: 60% Strength, 40% Stamina")
        }
        (2001, 128) => {
            String::from("Biological metamorphosis is a skill used for learning and casting Nano formulas that use Biological metamorphosis as a requirement. If a nano formula is offensive and carries a skill check that is based off Biological metamorphosis then the higher this skill the higher the chance will be of the attack landing on your target.  Dependencies: Intelligence 80%, Psychic 20%")
        }
        (2001, 149) => {
            String::from("The nano program initiative skill is used to more quickly execute nano programs.  Dependencies: 60% Sense, 40% Agility")
        }
        (2001, 161) => {
            String::from("Computer literacy is the skill that allows you to use better computer equipment like belts and NCU units. It also determines which entrances and exits you can use in the grid   Dependencies: Intelligence 100%.")
        }
        (1060, 801) => {
            String::from("Clerk")
        }
        (1060, 1407) => {
            String::from("Savior")
        }
        (1060, 902) => {
            String::from("Worker")
        }
        (1060, 1504) => {
            String::from("Specter")
        }
        (1060, 301) => {
            String::from("Techie")
        }
        (1060, 1102) => {
            String::from("Apprentice")
        }
        (1060, 1305) => {
            String::from("Ancient")
        }
        (1060, 302) => {
            String::from("Mechanic")
        }
        (1060, 1307) => {
            String::from("Savior")
        }
        (1060, 901) => {
            String::from("Docker")
        }
        (1060, 201) => {
            String::from("Initiate")
        }
        (1060, 907) => {
            String::from("Godfather")
        }
        (1060, 1103) => {
            String::from("Initiate")
        }
        (1060, 1207) => {
            String::from("Deity")
        }
        (1060, 1501) => {
            String::from("Haunt")
        }
        (1060, 705) => {
            String::from("CEO")
        }
        (1060, 403) => {
            String::from("Embezzler")
        }
        (1060, 407) => {
            String::from("Kingpin")
        }
        (1060, 607) => {
            String::from("Vanguard")
        }
        (1060, 702) => {
            String::from("Salesman")
        }
        (1060, 707) => {
            String::from("Mogul")
        }
        (1060, 1401) => {
            String::from("Escort")
        }
        (1060, 1503) => {
            String::from("Shadow")
        }
        (1060, 503) => {
            String::from("Agent")
        }
        (1060, 905) => {
            String::from("Made Man")
        }
        (1060, 103) => {
            String::from("Soldier")
        }
        (1060, 502) => {
            String::from("Spy")
        }
        (1060, 506) => {
            String::from("Grandmaster Assassin")
        }
        (1060, 206) => {
            String::from("Legend")
        }
        (1060, 101) => {
            String::from("Militiaman")
        }
        (1060, 602) => {
            String::from("Hunter")
        }
        (1060, 803) => {
            String::from("Secretary")
        }
        (1060, 805) => {
            String::from("Minister")
        }
        (1060, 104) => {
            String::from("Lieutenant")
        }
        (1060, 307) => {
            String::from("Supreme Creator")
        }
        (1060, 1005) => {
            String::from("Master Surgeon")
        }
        (1060, 1006) => {
            String::from("Chief of Staff Surgery")
        }
        (1060, 1105) => {
            String::from("Techno-Mage")
        }
        (1060, 1203) => {
            String::from("Priest")
        }
        (1060, 205) => {
            String::from("Master 10th Dan")
        }
        (1060, 402) => {
            String::from("Thug")
        }
        (1060, 606) => {
            String::from("Ranger")
        }
        (1060, 106) => {
            String::from("General")
        }
        (1060, 703) => {
            String::from("Merchant")
        }
        (1060, 1101) => {
            String::from("Novice")
        }
        (1060, 1506) => {
            String::from("Spirit")
        }
        (1060, 1007) => {
            String::from("Eternalist")
        }
        (1060, 305) => {
            String::from("Master Engineer")
        }
        (1060, 507) => {
            String::from("Finalizer")
        }
        (1060, 303) => {
            String::from("Engineer")
        }
        (1060, 1202) => {
            String::from("Devotee")
        }
        (1060, 706) => {
            String::from("Chairman and CEO")
        }
        (1060, 1505) => {
            String::from("Vampire")
        }
        (1060, 1201) => {
            String::from("Disciple")
        }
        (1060, 107) => {
            String::from("Field Marshal")
        }
        (1060, 203) => {
            String::from("Monk")
        }
        (1060, 603) => {
            String::from("Preserver")
        }
        (1060, 903) => {
            String::from("Foreman")
        }
        (1060, 501) => {
            String::from("Informer")
        }
        (1060, 1507) => {
            String::from("Lich")
        }
        (1060, 1304) => {
            String::from("Old")
        }
        (1060, 1003) => {
            String::from("Doctor")
        }
        (1060, 807) => {
            String::from("Dictator")
        }
        (1060, 102) => {
            String::from("Mercenary")
        }
        (1060, 304) => {
            String::from("Chief Engineer")
        }
        (1060, 406) => {
            String::from("The Man")
        }
        (1060, 1004) => {
            String::from("Surgeon")
        }
        (1060, 105) => {
            String::from("Colonel")
        }
        (1060, 202) => {
            String::from("Apprentice")
        }
        (1060, 505) => {
            String::from("Master Assassin")
        }
        (1060, 306) => {
            String::from("Nobel Prize Engineer")
        }
        (1060, 605) => {
            String::from("Wanderer")
        }
        (1060, 1107) => {
            String::from("Techno Arch-Wizard")
        }
        (1060, 906) => {
            String::from("Don")
        }
        (1060, 1001) => {
            String::from("Nurse")
        }
        (1060, 1301) => {
            String::from("Young")
        }
        (1060, 1303) => {
            String::from("Adult")
        }
        (1060, 804) => {
            String::from("Manager")
        }
        (1060, 1306) => {
            String::from("Archaic")
        }
        (1060, 404) => {
            String::from("Fixer")
        }
        (1060, 604) => {
            String::from("Adventurer")
        }
        (1060, 704) => {
            String::from("Board Member")
        }
        (1060, 504) => {
            String::from("Assassin")
        }
        (1060, 1104) => {
            String::from("Sorcerer")
        }
        (1060, 1106) => {
            String::from("Techno-Wizard")
        }
        (1060, 1205) => {
            String::from("Spiritual Leader")
        }
        (1060, 1206) => {
            String::from("Saint")
        }
        (1060, 405) => {
            String::from("Doer")
        }
        (1060, 1402) => {
            String::from("Sentinel")
        }
        (1060, 806) => {
            String::from("President")
        }
        (1060, 1405) => {
            String::from("Defender")
        }
        (1060, 1302) => {
            String::from("Adolescent")
        }
        (1060, 601) => {
            String::from("Scout")
        }
        (1060, 701) => {
            String::from("Courier")
        }
        (1060, 1406) => {
            String::from("Keeper")
        }
        (1060, 1502) => {
            String::from("Phantom")
        }
        (1060, 1002) => {
            String::from("Medic")
        }
        (1060, 401) => {
            String::from("Lifter")
        }
        (1060, 1204) => {
            String::from("Archdeacon")
        }
        (1060, 1403) => {
            String::from("Warden")
        }
        (1060, 802) => {
            String::from("Bureaucrat")
        }
        (1060, 204) => {
            String::from("Master 5th Dan")
        }
        (1060, 904) => {
            String::from("Union Man")
        }
        (1060, 207) => {
            String::from("Guru")
        }
        (1060, 1404) => {
            String::from("Vigilante")
        }
        (2016, 686_007) => {
            String::from("Specialist Commerce Access")
        }
        (2016, 686_015) => {
            String::from("Certificate of Travel")
        }
        (2016, 179_026_692) => {
            String::from("The bug report could not be sent.  Reason: Unknown.")
        }
        (2016, 3) => {
            String::from("Exotic")
        }
        (2016, 6_399_012) => {
            String::from("<font color=CCInfoHeader>You are about to report this bug:</font><br>\r\n<font color=CCInfoText>")
        }
        (2016, 4) => {
            String::from("Quest")
        }
        (2016, 686_000) => {
            String::from("Returned Credit Card")
        }
        (2016, 686_001) => {
            String::from("Stolen Credits")
        }
        (2016, 16_883_060) => {
            String::from("The bug report was not sent.")
        }
        (2016, 1) => {
            String::from("Trash")
        }
        (2016, 257_013) => {
            String::from("Uncle Bazzit's Alien Library")
        }
        (2016, 40_655_268) => {
            String::from("The bug report has been sent; thank you for reporting it.")
        }
        (2016, 2) => {
            String::from("Normal")
        }
        (2016, 5) => {
            String::from("Social")
        }
        (2016, 686_012) => {
            String::from("The Shiny Sword")
        }
        (2016, 0) => {
            String::from("Unknown")
        }
        (2016, 65016) => {
            String::from("Alien Technology Mastery")
        }
        (502, 243_081_980) => {
            let a = &arguments[0];
            format!("The {a} skill can not be reset.")
        }
        (502, 45_549_043) => {
            String::from("You are only allowed to reset abilities and skills.")
        }
        (502, 127_195_861) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = {
                if c.to_string() == "1" {
                    " second. "
                } else {
                    " seconds."
                }
            };
            format!("- Removing lock on skill {a} ({b}), which had a remainder of {c} {d}")
        }
        (502, 202_459_220) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = {
                if c.to_string() == "1" {
                    " second. "
                } else {
                    " seconds."
                }
            };
            format!("- The skill {a} ({b}) is locked for {c} {d}")
        }
        (502, 89_070_509) => {
            let a = &arguments[0];
            format!("Do you really want to reset the skill {a}?")
        }
        (502, 109_178_077) => {
            String::from("You have the option of a full IP reset, are you sure you want to use it? \r\n\r\nAll abilities and skills will be set to their minimum values. Your IP will restored to the default amount for your level.")
        }
        (502, 126_196_928) => {
            String::from("You lack the IP needed to increase further in this skill.")
        }
        (502, 96_620_988) => {
            String::from("Reset all skills")
        }
        (502, 68_499_036) => {
            String::from("You must unequip all items and implants before you are allowed to reset any skills or abilities.")
        }
        (2002, 668) => {
            String::from("Battlestation Side")
        }
        (2002, 18) => {
            String::from("Stamina")
        }
        (2002, 214) => {
            String::from("Nano Points")
        }
        (2002, 517) => {
            String::from("SpecialAttackShield")
        }
        (2002, 149) => {
            String::from("Nano Execution Init")
        }
        (2002, 230) => {
            String::from("ShieldRadiationAC")
        }
        (2002, 97) => {
            String::from("Fire Armor-Class")
        }
        (2002, 122) => {
            String::from("Sensory Improvement and Modification")
        }
        (2002, 234) => {
            String::from("ShieldPoisonAC")
        }
        (2002, 251) => {
            String::from("Number of pets")
        }
        (2002, 675) => {
            String::from("PVP Duel Deaths")
        }
        (2002, 516) => {
            String::from("AttackShield")
        }
        (2002, 68) => {
            String::from("Veteran points")
        }
        (2002, 126) => {
            String::from("Electrical Engineering")
        }
        (2002, 163) => {
            String::from("Chemistry")
        }
        (2002, 410) => {
            String::from("Number of fighting opponents")
        }
        (2002, 682) => {
            String::from("PVP Solo Score")
        }
        (2002, 388) => {
            String::from("Tower Type")
        }
        (2002, 661) => {
            String::from("Damage To Nano Multiplier")
        }
        (2002, 455) => {
            String::from("Monster Type")
        }
        (2002, 108) => {
            String::from("Knife or Sharp Object Throwing")
        }
        (2002, 158) => {
            String::from("Weapon Smithing")
        }
        (2002, 156) => {
            String::from("Run Speed")
        }
        (2002, 120) => {
            String::from("Physical Prowess and Martial Arts Initiative")
        }
        (2002, 123) => {
            String::from("First Aid")
        }
        (2002, 531) => {
            String::from("Expansion playfield")
        }
        (2002, 215) => {
            String::from("GM capabilities")
        }
        (2002, 560) => {
            String::from("Faction with Omni-Armed Forces")
        }
        (2002, 232) => {
            String::from("ShieldNanoAC")
        }
        (2002, 568) => {
            String::from("Faction with The Assertive Operators")
        }
        (2002, 593) => {
            String::from("Regain XP Percentage")
        }
        (2002, 315) => {
            String::from("Added to Nano Damage")
        }
        (2002, 51) => {
            String::from("Aggdef")
        }
        (2002, 100) => {
            String::from("Martial Arts")
        }
        (2002, 1) => {
            String::from("Life")
        }
        (2002, 129) => {
            String::from("Psychological Modifications")
        }
        (2002, 125) => {
            String::from("Mechanical Engineering")
        }
        (2002, 146) => {
            String::from("Sneak Attack")
        }
        (2002, 152) => {
            String::from("Body Development")
        }
        (2002, 662) => {
            String::from("Use Mech")
        }
        (2002, 169) => {
            String::from("Alien defender title")
        }
        (2002, 565) => {
            String::from("Faction with Clan Vanguards")
        }
        (2002, 117) => {
            String::from("Vehicle Navigation, Water")
        }
        (2002, 105) => {
            String::from("2 Handed Edged Weapons")
        }
        (2002, 281) => {
            String::from("Added to Chemical Damage")
        }
        (2002, 689) => {
            String::from("Heal Reactivity")
        }
        (2002, 489) => {
            String::from("Backstab")
        }
        (2002, 672) => {
            String::from("Paid Points")
        }
        (2002, 681) => {
            String::from("PVP Ranked Team Deaths")
        }
        (2002, 339) => {
            String::from("Damage override type")
        }
        (2002, 155) => {
            String::from("Evade Close Combat and Martial Art Attacks")
        }
        (2002, 3) => {
            String::from("AttackSpeed")
        }
        (2002, 141) => {
            String::from("Tutoring")
        }
        (2002, 142) => {
            String::from("Brawling")
        }
        (2002, 118) => {
            String::from("Melee Weapons Initiative")
        }
        (2002, 124) => {
            String::from("Treatment")
        }
        (2002, 318) => {
            String::from("Percentage Additional Nano Executing Cost")
        }
        (2002, 231) => {
            String::from("ShieldColdAC")
        }
        (2002, 380) => {
            String::from("RangeIncreaserWeapon")
        }
        (2002, 659) => {
            String::from("Damage To Nano")
        }
        (2002, 679) => {
            String::from("PVP Ranked Solo Deaths")
        }
        (2002, 680) => {
            String::from("PVP Ranked Team Kills")
        }
        (2002, 381) => {
            String::from("RangeIncreaserNF")
        }
        (2002, 279) => {
            String::from("Added to Melee Damage")
        }
        (2002, 226) => {
            String::from("ShieldProjectileAC")
        }
        (2002, 19) => {
            String::from("Intelligence")
        }
        (2002, 133) => {
            String::from("Ranged Energy Weapons")
        }
        (2002, 94) => {
            String::from("Radiation Armor-Class")
        }
        (2002, 227) => {
            String::from("ShieldMeleeAC")
        }
        (2002, 104) => {
            String::from("Melee Energy Weapons")
        }
        (2002, 167) => {
            String::from("Full Auto")
        }
        (2002, 221) => {
            String::from("Max Nano")
        }
        (2002, 319) => {
            String::from("Percentage Additional Experience")
        }
        (2002, 213) => {
            String::from("Team Side")
        }
        (2002, 92) => {
            String::from("Energy Attack Armor-Class")
        }
        (2002, 159) => {
            String::from("Pharmacological Technology")
        }
        (2002, 164) => {
            String::from("Concealment")
        }
        (2002, 470) => {
            String::from("MapOptions")
        }
        (2002, 116) => {
            String::from("Assault Rifle")
        }
        (2002, 157) => {
            String::from("Quantum Force field Technology")
        }
        (2002, 165) => {
            String::from("Breaking and Entering")
        }
        (2002, 515) => {
            String::from("Tower Instance")
        }
        (2002, 132) => {
            String::from("Nano Energy Pool")
        }
        (2002, 563) => {
            String::from("Faction with Gaia")
        }
        (2002, 579) => {
            String::from("Shadowbreed")
        }
        (2002, 110) => {
            String::from("Operate Heavy Weapons")
        }
        (2002, 674) => {
            String::from("PVP Duel Kills")
        }
        (2002, 112) => {
            String::from("Pistol")
        }
        (2002, 537) => {
            String::from("Direct Nano Damage Vulnerability")
        }
        (2002, 101) => {
            String::from("Multiple Melee Weapons")
        }
        (2002, 148) => {
            String::from("Burst")
        }
        (2002, 145) => {
            String::from("Deflect")
        }
        (2002, 383) => {
            String::from("Decreased Nano-Interrupt Modifier %")
        }
        (2002, 137) => {
            String::from("Outdoor Adventuring")
        }
        (2002, 153) => {
            String::from("Duck Explosions and Thrown Objects")
        }
        (2002, 382) => {
            String::from("SkillLockModifier")
        }
        (2002, 561) => {
            String::from("Faction with The Sentinels")
        }
        (2002, 535) => {
            String::from("Healing Efficiency")
        }
        (2002, 228) => {
            String::from("ShieldEnergyAC")
        }
        (2002, 154) => {
            String::from("Dodge Ranged Attacks")
        }
        (2002, 90) => {
            String::from("Impact and Projectile Weapon Armor-Class")
        }
        (2002, 106) => {
            String::from("Piercing Weapons")
        }
        (2002, 143) => {
            String::from("Riposte")
        }
        (2002, 111) => {
            String::from("Bow")
        }
        (2002, 151) => {
            String::from("Aimed Shot")
        }
        (2002, 20) => {
            String::from("Sense")
        }
        (2002, 161) => {
            String::from("Computer Literacy")
        }
        (2002, 233) => {
            String::from("ShieldFireAC")
        }
        (2002, 311) => {
            String::from("Added to Cold Damage")
        }
        (2002, 316) => {
            String::from("Added to Fire Damage")
        }
        (2002, 391) => {
            String::from("Critical Decrease")
        }
        (2002, 58) => {
            String::from("Played")
        }
        (2002, 584) => {
            String::from("Associated access card")
        }
        (2002, 109) => {
            String::from("Grenade or Lumping Throwing")
        }
        (2002, 27) => {
            String::from("Health")
        }
        (2002, 564) => {
            String::from("Faction with Omni-Trans")
        }
        (2002, 684) => {
            String::from("PVP Duel Score")
        }
        (2002, 229) => {
            String::from("ShieldChemicalAC")
        }
        (2002, 138) => {
            String::from("Swimming")
        }
        (2002, 669) => {
            String::from("Victory points")
        }
        (2002, 317) => {
            String::from("Added to Poison Damage")
        }
        (2002, 160) => {
            String::from("Nano-Bot Programming")
        }
        (2002, 582) => {
            String::from("Allowed apartments")
        }
        (2002, 282) => {
            String::from("Added to Radiation Damage")
        }
        (2002, 369) => {
            String::from("VisualSex")
        }
        (2002, 102) => {
            String::from("1 Hand Blunt Weapons")
        }
        (2002, 131) => {
            String::from("Time and Space Alteration")
        }
        (2002, 21) => {
            String::from("Psychic")
        }
        (2002, 562) => {
            String::from("Faction with Omni-Med")
        }
        (2002, 201) => {
            String::from("Aggressiveness")
        }
        (2002, 127) => {
            String::from("Matter Metamorphosis")
        }
        (2002, 457) => {
            String::from("hatelist size")
        }
        (2002, 569) => {
            String::from("Faction with The Unredeemed")
        }
        (2002, 16) => {
            String::from("Strength")
        }
        (2002, 75) => {
            String::from("Awarded Omni-Tek Tokens")
        }
        (2002, 536) => {
            String::from("Direct Nano Damage Efficiency")
        }
        (2002, 280) => {
            String::from("Added to Energy Damage")
        }
        (2002, 571) => {
            String::from("Faction with The Benign Conservers")
        }
        (2002, 38) => {
            String::from("Combat Range")
        }
        (2002, 114) => {
            String::from("Machine Guns and Sub Machine Guns")
        }
        (2002, 566) => {
            String::from("Faction with Guardian of Shadow")
        }
        (2002, 119) => {
            String::from("Ranged Weapons Initiative")
        }
        (2002, 33) => {
            String::from("Side")
        }
        (2002, 115) => {
            String::from("Shotgun")
        }
        (2002, 22) => {
            String::from("AMS")
        }
        (2002, 91) => {
            String::from("Melee Attacks and Martial Art Armor-Class")
        }
        (2002, 95) => {
            String::from("Cold Armor-Class")
        }
        (2002, 407) => {
            String::from("Nano points")
        }
        (2002, 525) => {
            String::from("Percentage of remaining health")
        }
        (2002, 93) => {
            String::from("Chemical Armor-Class")
        }
        (2002, 570) => {
            String::from("Faction with The Devoted")
        }
        (2002, 113) => {
            String::from("Rifle and Sniper-Rifle")
        }
        (2002, 140) => {
            String::from("Map Navigation")
        }
        (2002, 379) => {
            String::from("CriticalIncrease")
        }
        (2002, 586) => {
            String::from("MapArea")
        }
        (2002, 673) => {
            String::from("Visual Flags")
        }
        (2002, 162) => {
            String::from("Psychology")
        }
        (2002, 17) => {
            String::from("Agility")
        }
        (2002, 62) => {
            String::from("Awarded Clan Tokens")
        }
        (2002, 247) => {
            String::from("Temporary Skill Reduction")
        }
        (2002, 278) => {
            String::from("Added to Projectile Damage")
        }
        (2002, 677) => {
            String::from("PVP Profession Duel Deaths")
        }
        (2002, 676) => {
            String::from("PVP Profession Duel Kills")
        }
        (2002, 683) => {
            String::from("PVP Team Score")
        }
        (2002, 150) => {
            String::from("Fling Shot")
        }
        (2002, 144) => {
            String::from("Dimach (Soul Attack)")
        }
        (2002, 274) => {
            String::from("Equipped weapons")
        }
        (2002, 572) => {
            String::from("Faction with The Redeemed")
        }
        (2002, 61) => {
            String::from("Credits")
        }
        (2002, 147) => {
            String::from("Fast Attack")
        }
        (2002, 199) => {
            String::from("Reset points")
        }
        (2002, 121) => {
            String::from("Bow Special Attack")
        }
        (2002, 96) => {
            String::from("Disease and Poison Armor-Class")
        }
        (2002, 390) => {
            String::from("Lowres Mesh")
        }
        (2002, 107) => {
            String::from("2 Handed Blunt Weapons")
        }
        (2002, 587) => {
            String::from("Number of team members")
        }
        (2002, 54) => {
            String::from("Level")
        }
        (2002, 134) => {
            String::from("Multiple Ranged Weapons")
        }
        (2002, 135) => {
            String::from("Trap Disarmament")
        }
        (2002, 180) => {
            String::from("Used NCU")
        }
        (2002, 130) => {
            String::from("Matter Creations")
        }
        (2002, 128) => {
            String::from("Biological Metamorphosis")
        }
        (2002, 206) => {
            String::from("Reflect Melee AC")
        }
        (2002, 139) => {
            String::from("Vehicle Navigation, Airborne")
        }
        (2002, 166) => {
            String::from("Vehicle Navigation, Ground")
        }
        (2002, 360) => {
            String::from("Scale")
        }
        (2002, 37) => {
            String::from("Title level")
        }
        (2002, 389) => {
            String::from("Expansion pack")
        }
        (2002, 567) => {
            String::from("Faction with Followers")
        }
        (2002, 103) => {
            String::from("1 Hand Edged Weapons")
        }
        (2002, 168) => {
            String::from("Nano Resistance")
        }
        (2002, 368) => {
            String::from("VisualProfession")
        }
        (2002, 583) => {
            String::from("Owned apartments")
        }
        (2002, 276) => {
            String::from("Added to all Offensive Rolls")
        }
        (2002, 136) => {
            String::from("Perception and Spotting")
        }
        (2002, 678) => {
            String::from("PVP Ranked Solo Kills")
        }
        (2002, 277) => {
            String::from("Added to all Defencive Rolls")
        }
        (2002, 367) => {
            String::from("VisualBreed")
        }
        (10001, 147_096_568) => {
            let a = &arguments[0];
            format!("Rename {a}")
        }
        (10001, 177_634_599) => {
            String::from("Clone Window")
        }
        (10001, 135_286_355) => {
            String::from("Muted channels")
        }
        (10001, 49_866_466) => {
            String::from("Clear")
        }
        (10001, 198_175_874) => {
            let a = &arguments[0];
            format!("Ignoring character \"{a}\".")
        }
        (10001, 243_121_396) => {
            String::from("Always Behind")
        }
        (10001, 167_612_764) => {
            String::from("Normal")
        }
        (10001, 100_359_747) => {
            String::from("Timeout messages")
        }
        (10001, 259_053_652) => {
            String::from("You are not authorized to perform that command. (Your request has been logged)")
        }
        (10001, 65_057_168) => {
            String::from("You left private group: ")
        }
        (10001, 67_213_864) => {
            let a = &arguments[0];
            format!("Unable to find script {a} .")
        }
        (10001, 120_758_547) => {
            String::from("Autosubscribe new channels")
        }
        (10001, 155_259_844) => {
            String::from("Hide channels subscribed to other windows")
        }
        (10001, 108_064_280) => {
            String::from("Refresh")
        }
        (10001, 170_253_061) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("The group name is ambiguous. The following groups matched:\r\n{a}\r\nYou can \"auto-complete\" the group name by typing <font color=yellow>'{b} {c}'</font> and then pressing <TAB>.")
        }
        (10001, 101_825_639) => {
            String::from("Show message log")
        }
        (10001, 227_544_104) => {
            let a = &arguments[0];
            format!("Befriend {a}")
        }
        (10001, 23_033_694) => {
            String::from("Sending petition.")
        }
        (10001, 146_099_907) => {
            let a = &arguments[0];
            format!("Are you sure you want to remove user <font color=yellow>{a}</font> from your friend list?")
        }
        (10001, 182_632_675) => {
            String::from("Borderless")
        }
        (10001, 242_509_767) => {
            String::from("Delete Window")
        }
        (10001, 231_562_867) => {
            String::from("Chat Windows")
        }
        (10001, 200_490_068) => {
            let a = &arguments[0];
            format!("To [{a}]: ")
        }
        (10001, 255_526_215) => {
            String::from("Follow")
        }
        (10001, 253_408_182) => {
            String::from("AFK off.")
        }
        (10001, 210_084_400) => {
            String::from("Active")
        }
        (10001, 9_389_587) => {
            String::from("No selectable items")
        }
        (10001, 177_336_403) => {
            String::from("Combat Messages")
        }
        (10001, 248_362_547) => {
            String::from("Private Channels")
        }
        (10001, 153_560_515) => {
            String::from("A window with that name already exists")
        }
        (10001, 133_278_526) => {
            String::from("AFK on. All tell messages will be replied with afk.")
        }
        (10001, 94_336_004) => {
            String::from("You joined private group: ")
        }
        (10001, 5_107_500) => {
            String::from("Talk to Channel")
        }
        (10001, 77_817_000) => {
            let a = &arguments[0];
            format!("Clone {a}")
        }
        (10001, 62_446_603) => {
            String::from("Mouse Input Disabled. You can now click through this chat window. Use ALT + Mouseclick to override this setting.")
        }
        (10001, 112_579_191) => {
            String::from("Show chat window")
        }
        (10001, 22_857_075) => {
            String::from("Offline Friends")
        }
        (10001, 5_434_819) => {
            String::from("Error: to few arguments")
        }
        (10001, 5_472_280) => {
            let a = &arguments[0];
            format!("Mail {a}")
        }
        (10001, 152_445_938) => {
            let a = &arguments[0];
            format!("Unignoring character \"{a}\".")
        }
        (10001, 53_984_709) => {
            let a = &arguments[0];
            format!("Log [{a}]")
        }
        (10001, 86_674_023) => {
            String::from("Show log window")
        }
        (10001, 217_719_283) => {
            String::from("Log messages")
        }
        (10001, 7_942_212) => {
            let a = &arguments[0];
            format!("No chat command or script named \"{a}\" available.")
        }
        (10001, 93_079_237) => {
            String::from("Remove")
        }
        (10001, 199_227_008) => {
            String::from(" joined the group.")
        }
        (10001, 177_712_999) => {
            String::from("Enter name of new window")
        }
        (10001, 20_841_710) => {
            let a = &arguments[0];
            format!("AFK is currently on. Current AFK message is '{a}'.")
        }
        (10001, 196_784_165) => {
            String::from("Mode")
        }
        (10001, 57_069_749) => {
            String::from(" declined the invitation.")
        }
        (10001, 27_838_876) => {
            String::from("Warning: Message sent to group not subscribed to this window (goto [F10->GUI/Chat] to disable this warning).")
        }
        (10001, 22_607_145) => {
            String::from("The user is currently AFK.")
        }
        (10001, 85_740_804) => {
            String::from("Sending name change request.")
        }
        (10001, 116_436_341) => {
            let a = &arguments[0];
            format!("Failed to send private message to user \"{a}\".")
        }
        (10001, 94_334_131) => {
            String::from(" shouts: ")
        }
        (10001, 182_685_246) => {
            String::from("Private Group Invitation")
        }
        (10001, 257_164_119) => {
            String::from("Rename Window")
        }
        (10001, 204_386_119) => {
            let a = &arguments[0];
            format!("Are you sure you want to delete window '{a}'?")
        }
        (10001, 111_666_535) => {
            let a = &arguments[0];
            format!("{a} will go to this window.")
        }
        (10001, 110_410_901) => {
            String::from("Style")
        }
        (10001, 124_307_572) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            let e = {
                if c.to_string() == "1" {
                    "hour "
                } else {
                    " hours"
                }
            };
            let f = {
                if d.to_string() == "1" {
                    "minute "
                } else {
                    " minutes"
                }
            };
            format!("{a} is AFK (Away from keyboard) since {b} {e} and {c} {f} ago.\r\n{d}\r\n")
        }
        (10001, 140_283_232) => {
            String::from("Inactive")
        }
        (10001, 185_475_708) => {
            String::from("Visual")
        }
        (10001, 6_520_779) => {
            String::from("No target selected and no nick specified.")
        }
        (10001, 158_338_720) => {
            let a = &arguments[0];
            format!("Disband {a}")
        }
        (10001, 262_314_229) => {
            String::from("Autohide inputbar")
        }
        (10001, 266_692_840) => {
            let a = &arguments[0];
            format!("Ignore {a}")
        }
        (10001, 120_626_503) => {
            String::from("Script name is WAY too long. 180 chars max")
        }
        (10001, 56_682_932) => {
            String::from("The window you are closing contains your unsubscribed channels. Your unsubscribed channels are the chat channels you dont have set up to go to any other window. By closing this window you will only receive messages from the channels you are subscribed to and have set up in a window. You can assign the unsubscribed channels to another window by pressing the menu button and click \"Display unsubscribed Channels\"")
        }
        (10001, 206_293_347) => {
            String::from("Log Messages")
        }
        (10001, 113_408_663) => {
            String::from("Show Log Window")
        }
        (10001, 5_449_559) => {
            String::from("Show log window")
        }
        (10001, 184_272_035) => {
            String::from("Chat Channels")
        }
        (10001, 112_348_791) => {
            let a = &arguments[0];
            format!("You were invited to a private chat group by {a}.")
        }
        (10001, 109_983_634) => {
            String::from("Ignore")
        }
        (10001, 145_132_217) => {
            String::from("Transparency")
        }
        (10001, 216_014_759) => {
            String::from("New Window")
        }
        (10001, 135_941_524) => {
            String::from("Disable text input")
        }
        (10001, 187_644_212) => {
            let a = &arguments[0];
            format!("Chat group invitation from {a} was auto declined.")
        }
        (10001, 120_922_995) => {
            String::from("Recent Messages")
        }
        (10001, 134_508_747) => {
            String::from("is afk")
        }
        (10001, 257_166_759) => {
            String::from("Remove Window")
        }
        (10001, 201_023_520) => {
            String::from("Always On Top")
        }
        (10001, 125_600_404) => {
            String::from("Warning, you are about to close your only active window! If you do this, you will not have any chat windows, are you sure you want to do this?")
        }
        (10001, 192_570_169) => {
            let a = &arguments[0];
            format!("Group '{a}' is read-only.")
        }
        (10001, 118_983_027) => {
            String::from("No Selectable Groups")
        }
        (10001, 6_097_656) => {
            let a = &arguments[0];
            format!("Invite {a}")
        }
        (10001, 14_462_052) => {
            let a = &arguments[0];
            format!("{a} has been removed from this window.")
        }
        (10001, 100_121_299) => {
            String::from("Public Channels")
        }
        (10001, 151_444_333) => {
            String::from("You are not currently in a team.")
        }
        (10001, 18_300_484) => {
            let a = &arguments[0];
            format!("You were invited to a private chat group by {a}\r\nPlease select which chat window(s) you want to assign the private chat to and click [Yes] to join.")
        }
        (10001, 195_613_152) => {
            let a = &arguments[0];
            format!("Failed to send message to group '{a}'.")
        }
        (10001, 126_740_259) => {
            String::from("To many arguments.")
        }
        (10001, 201_246_484) => {
            String::from("Deactivate On Send")
        }
        (10001, 65_031_894) => {
            String::from("AFK is currently off.")
        }
        (10001, 33_773_188) => {
            String::from("It is not possible to delete the default window. Close it if you don't want to use it anymore.")
        }
        (10001, 127_920_856) => {
            String::from("Enable clickthrough")
        }
        (10001, 125_143_529) => {
            String::from("All matching groups are read-only.")
        }
        (10001, 42_198_691) => {
            String::from("You can only select groups that are subscribed to this window.")
        }
        (10001, 102_130_094) => {
            String::from("Chat Configuration")
        }
        (10001, 70_969_062) => {
            String::from("Can't ignore yourself.")
        }
        (10001, 227_102_743) => {
            String::from("Warning:")
        }
        (10001, 78_020_182) => {
            let a = &arguments[0];
            format!("Copy of {a}")
        }
        (10001, 227_468_400) => {
            String::from(" left the group.")
        }
        (10001, 264_029_846) => {
            String::from("Can't invite yourself.")
        }
        (10001, 28_169_253) => {
            String::from("Enter Window Name")
        }
        (10001, 170_677_084) => {
            String::from("Normal")
        }
        (10001, 266_679_528) => {
            let a = &arguments[0];
            format!("Unignore {a}")
        }
        (10001, 93_079_605) => {
            String::from("Rename")
        }
        (10001, 134_413_881) => {
            String::from("Type a custom AFK message or wait for\r\nthe timeout to use the default message.\r\n")
        }
        (10001, 63_369_171) => {
            String::from("Show timestamps")
        }
        (10001, 144_888_439) => {
            String::from("Chat Config")
        }
        (10001, 113_417_188) => {
            String::from("Open Chat")
        }
        (10001, 252_340_547) => {
            String::from(" whispers: ")
        }
        (10001, 188_082_334) => {
            String::from("You have been automatically set AFK.")
        }
        (10001, 4_706_587) => {
            String::from("is back")
        }
        (10001, 199_016_563) => {
            String::from("Online Friends")
        }
        (10001, 46_072_146) => {
            let a = &arguments[0];
            format!("Unknown user \"{a}\".")
        }
        (10001, 199_184_607) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Changed AFK message from '{a}' to '{b}'")
        }
        (10001, 118_080_020) => {
            String::from("You can re-open chat windows by pressing Actions - Chat Config.")
        }
        (10001, 148_967_715) => {
            String::from("Borderless")
        }
        (10001, 180_538_088) => {
            let a = &arguments[0];
            format!("Delete {a}")
        }
        (10001, 235_181_075) => {
            String::from("Can only ignore characters.")
        }
        (10001, 156_821_813) => {
            String::from("None of the matching groups are selectable in this window.")
        }
        (10001, 214_106_624) => {
            let a = &arguments[0];
            format!("Leave {a}")
        }
        (10001, 77_249_784) => {
            let a = &arguments[0];
            format!("Are you sure you want to ignore further messages from user <font color=yellow>{a}</font>?")
        }
        (10001, 206_155_708) => {
            String::from("Send Tell")
        }
        (10001, 225_898_222) => {
            String::from("You are not a member of an organization.")
        }
        (10001, 9_197_447) => {
            String::from("Default Window")
        }
        (10001, 71_296_869) => {
            String::from("Subscribe Channels")
        }
        (10001, 102_962_323) => {
            String::from("Show Options")
        }
        (10000, 67_416_958) => {
            String::from("No previous profession.")
        }
        (10000, 169_150_309) => {
            String::from("Team Score")
        }
        (10000, 265_347_003) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Are you sure you want to train the {a} perk? You can only remove a perk you have chosen once every {b} hours.")
        }
        (10000, 59_010_663) => {
            String::from("Follow")
        }
        (10000, 54_384_795) => {
            String::from("Attack")
        }
        (10000, 5_998_501) => {
            String::from("Trade")
        }
        (10000, 263_030_447) => {
            String::from("Move to Default Tab")
        }
        (10000, 26_717_043) => {
            String::from("Available Perk Points: ")
        }
        (10000, 160_754_954) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("{a}<br>Quality level {b}<br>Used in {c} different processes")
        }
        (10000, 262_816_943) => {
            String::from("Hide")
        }
        (10000, 225_003_075) => {
            String::from("Favorites")
        }
        (10000, 55_823_111) => {
            String::from("..Anarchy Online is loading..")
        }
        (10000, 77_379_209) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("The result will be a quality level {a} {b}")
        }
        (10000, 94_097_293) => {
            String::from("Unlock Item")
        }
        (10000, 183_970_850) => {
            String::from("3rd Person Rubber")
        }
        (10000, 265_118_132) => {
            String::from("You have already built a market house in this city.")
        }
        (10000, 11_092_117) => {
            String::from("List Mode")
        }
        (10000, 155_868_060) => {
            String::from("Please choose your destination:\r\n")
        }
        (10000, 260_979_949) => {
            String::from("Look at Item")
        }
        (10000, 100_133_899) => {
            String::from("You need to train the previous perk in the line first.")
        }
        (10000, 236_378_921) => {
            String::from("Psi")
        }
        (10000, 64_996_768) => {
            String::from("Center the view on the shop marker.")
        }
        (10000, 346_165) => {
            String::from("Name")
        }
        (10000, 117_988_351) => {
            let a = &arguments[0];
            format!("Activate this toggle button and click on one of your trained perks to untrain it. Only one perk can be untrained every {a} hours.")
        }
        (10000, 47_571_949) => {
            String::from("You are no longer in a team.")
        }
        (10000, 77_497_557) => {
            String::from("Change")
        }
        (10000, 371_581) => {
            String::from("Team")
        }
        (10000, 172_849_634) => {
            let a = &arguments[0];
            format!("Are you sure you want to delete '{a}'?")
        }
        (10000, 11_536_197) => {
            String::from("Atk Range")
        }
        (10000, 47_274_052) => {
            let a = &arguments[0];
            format!("Can't use key <font color=aqua>'{a}'</font> since it is fixed to other functionality.")
        }
        (10000, 172_734_357) => {
            String::from("You have already built a tax office in this city.")
        }
        (10000, 198_833_245) => {
            let a = &arguments[0];
            format!("{a} left your team.")
        }
        (10000, 230_869_204) => {
            String::from("Zoom out")
        }
        (10000, 25_947_445) => {
            String::from("Are you sure you want to quit the game?")
        }
        (10000, 251_878_668) => {
            String::from("Heal")
        }
        (10000, 119_930_224) => {
            String::from("To reactivate the help button floater go to the preference panel (F10) and enable it under the &quot;Misc&quot; preference page.")
        }
        (10000, 210_237_567) => {
            String::from("Nano program")
        }
        (10000, 157_877_875) => {
            String::from("Yes")
        }
        (10000, 216_396_245) => {
            String::from("NanoLine")
        }
        (10000, 200_373_652) => {
            String::from("You are no longer invited to the team.")
        }
        (10000, 4_833_316) => {
            String::from("Build")
        }
        (10000, 17_881_012) => {
            String::from("Untrain Perk timeout")
        }
        (10000, 1_154_356) => {
            String::from("Reject")
        }
        (10000, 126_212_659) => {
            String::from("Are you sure you want to reset all key bindings to their original value?")
        }
        (10000, 103_085_669) => {
            String::from("Duel Score")
        }
        (10000, 166_505_699) => {
            String::from("Used Perk Points: ")
        }
        (10000, 50_593_137) => {
            String::from("Mission is not in this area. You will be informed when entering correct area.")
        }
        (10000, 242_790_799) => {
            String::from("No waypoint info available in mission data.")
        }
        (10000, 79_630_697) => {
            String::from("Easy")
        }
        (10000, 246_238_652) => {
            String::from("3rd Person Trail")
        }
        (10000, 191_385_459) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("{a}/{b} slots available")
        }
        (10000, 107_711_047) => {
            String::from("Autofade Window")
        }
        (10000, 328_911) => {
            String::from("Info")
        }
        (10000, 264_391_647) => {
            String::from("Request NPC info")
        }
        (10000, 157_188_915) => {
            String::from("Programs")
        }
        (10000, 51_646_228) => {
            String::from("Uncharted territory.  Shop not found.")
        }
        (10000, 138_752_643) => {
            String::from("Unlock All Items")
        }
        (10000, 121_422_094) => {
            String::from("Look at Mission")
        }
        (10000, 326_238) => {
            String::from("Icon")
        }
        (10000, 79_218_627) => {
            String::from("Physical")
        }
        (10000, 82_598_203) => {
            String::from("3rd Person Lock")
        }
        (10000, 82_230_653) => {
            let a = &arguments[0];
            format!("{a} joined your team.")
        }
        (10000, 3_332_485) => {
            String::from("You have already built a satellite uplink in this city.")
        }
        (10000, 1_031_975) => {
            String::from("Are you sure you want to Leave the organization?")
        }
        (10000, 99_965_405) => {
            String::from("Look at Item")
        }
        (10000, 190_849_663) => {
            String::from("Nano")
        }
        (10000, 209_495_280) => {
            String::from("Only houses can be dropped on the building map.")
        }
        (10000, 225_772_949) => {
            String::from("Solo Score")
        }
        (10000, 55_395_285) => {
            String::from("Alien Experience")
        }
        (10000, 226_032_482) => {
            let a = &arguments[0];
            format!("Make {a} leader")
        }
        (10000, 220_762_773) => {
            String::from("You have already built a grid house in this city.")
        }
        (10000, 114_185_427) => {
            String::from("No alien perk points available")
        }
        (10000, 137_583_324) => {
            String::from("City Terminal")
        }
        (10000, 102_563_717) => {
            String::from("Maybe this item is used as a Source instead?")
        }
        (10000, 175_102_436) => {
            String::from("Combat")
        }
        (10000, 54_018_976) => {
            String::from("Labels On Top")
        }
        (10000, 259_664_398) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("Prev profession: {a}, {b}, lvl {c}")
        }
        (10000, 143_176_083) => {
            String::from("Quit To Windows")
        }
        (10000, 147_680_743) => {
            String::from("Cloth")
        }
        (10000, 112_921_353) => {
            String::from("Enter hotkey")
        }
        (10000, 167_961_112) => {
            let a = &arguments[0];
            format!("Team side is {a}.")
        }
        (10000, 41_241_860) => {
            String::from("Reset To Default")
        }
        (10000, 170_054_814) => {
            let a = &arguments[0];
            format!("Press the key you want assigned to <font color=aqua>'{a}'</font>")
        }
        (10000, 210_221_709) => {
            String::from("Item")
        }
        (10000, 246_280_723) => {
            String::from("You are unable to train a new alien perk without an alien perk point.  You can gain new points by receiving alien experience.")
        }
        (10000, 159_921_171) => {
            String::from("Click to zoom out or double click right button directly on map.")
        }
        (10000, 60_006_901) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("Key <font color=aqua>'{a}'</font> is already assigned to <font color=aqua>'{b}'</font>. Do you want to perform the change and reset <font color=aqua>'{c}'</font> to <font color=aqua>'NONE'</font>?")
        }
        (10000, 206_303_972) => {
            String::from("Bad")
        }
        (10000, 36_327_090) => {
            String::from("Failed to create configuration directory.")
        }
        (10000, 55_920_483) => {
            String::from("<font color=CCInfoHeader>Please select the planet map to use.</font><br><br><font color=CCInfoHeadline>When in Shadowlands:</font><br>")
        }
        (10000, 134_301_329) => {
            String::from("Buy Land Area")
        }
        (10000, 261_253_103) => {
            String::from("Do you really want to delete this nano?")
        }
        (10000, 222_811_381) => {
            String::from("List mode")
        }
        (10000, 5_421_253) => {
            String::from("Leave")
        }
        (10000, 113_857_267) => {
            String::from("No perk points available")
        }
        (10000, 225_329_134) => {
            String::from("Shop Commission")
        }
        (10000, 151_171_587) => {
            String::from("Perk Lines")
        }
        (10000, 79_755_620) => {
            String::from("Mystical")
        }
        (10000, 121_792_165) => {
            String::from("Create Reference")
        }
        (10000, 126_108_565) => {
            String::from("Name")
        }
        (10000, 120_699_445) => {
            String::from("Delete")
        }
        (10000, 150_182_726) => {
            String::from("Can't send a join team request to yourself.")
        }
        (10000, 153_195_006) => {
            String::from("First Person")
        }
        (10000, 346_191) => {
            String::from("Nano")
        }
        (10000, 193_692_387) => {
            String::from("Click to zoom in or double click left button directly on map.")
        }
        (10000, 64_930_882) => {
            String::from("Center view on your character.")
        }
        (10000, 89_261_993) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("NCU ({a}/{b})")
        }
        (10000, 256_032_238) => {
            String::from("Head on")
        }
        (10000, 93_542_798) => {
            String::from("Accept Mission")
        }
        (10000, 5_020_002) => {
            String::from("Error:")
        }
        (10000, 146_737_635) => {
            String::from("Build Houses")
        }
        (10000, 383_874) => {
            String::from("Wear")
        }
        (10000, 66_747_571) => {
            String::from("You are unable to train a new Perk without a point. You gain one point every 10th level, and 1 for every Shadowlevel")
        }
        (10000, 355_507) => {
            String::from("Pets")
        }
        (10000, 114_966_606) => {
            String::from("Pointer position")
        }
        (10000, 93_079_605) => {
            String::from("Rename")
        }
        (10000, 239_071_396) => {
            String::from("The waypoint is shown as a white dot on the map and a yellow indicator on the compass.")
        }
        (10000, 253_071_360) => {
            String::from("Shop")
        }
        (10000, 4_863_637) => {
            String::from("Close")
        }
        (10000, 260_993_567) => {
            String::from("Look at Nano")
        }
        (10000, 212_790_383) => {
            String::from("Delete Nano")
        }
        (10000, 7_927_827) => {
            String::from("Clickable Links")
        }
        (10000, 202_118_786) => {
            String::from("Order")
        }
        (10000, 75_634_499) => {
            String::from("Radius")
        }
        (10000, 128_361_390) => {
            String::from("Center view on mission marker.")
        }
        (10000, 135_798_905) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Executing Nano Program: {a} on item {b}.")
        }
        (10000, 204_307_732) => {
            String::from("Uncharted territory.  Mission not found.")
        }
        (10000, 11_230_876) => {
            String::from("Commands the pet to heal your selected target")
        }
        (10000, 129_627_167) => {
            String::from("Displays how long it will take before you can untrain the next perk.")
        }
        (10000, 129_971_979) => {
            String::from("Untrain Perk")
        }
        (10000, 143_099_062) => {
            String::from("You have already built a cloaking device in this city.")
        }
        (10000, 194_418_911) => {
            String::from("No")
        }
        (10000, 203_067_290) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            format!("The {a} must be at least level {b} to combine with the {c} level {d}.")
        }
        (10000, 139_791_968) => {
            String::from("Reset Ownership")
        }
        (10000, 210_471_925) => {
            String::from("Experience")
        }
        (10000, 14_362_615) => {
            String::from("Perk Window")
        }
        (10000, 6_291_457) => {
            String::from("You do not meet the criteria required to learn this perk.")
        }
        (10000, 23_966_852) => {
            String::from("Prot")
        }
        (10000, 129_697_525) => {
            String::from("New Mail")
        }
        (10000, 368_480) => {
            String::from("Shop")
        }
        (10000, 89_435_005) => {
            String::from("No team")
        }
        (10000, 99_463_045) => {
            String::from("Character already in a team. Can't invite.")
        }
        (10000, 194_418_859) => {
            String::from("OK")
        }
        (10000, 206_323_989) => {
            String::from("You have already built this building in this city.")
        }
        (10000, 262_354_565) => {
            String::from("Activate skill reset mode")
        }
        (10000, 33_175_379) => {
            String::from("Profession Perks")
        }
        (10000, 237_124_853) => {
            String::from("You have already built a headquarter in this city.")
        }
        (10000, 118_349_612) => {
            String::from("Cancel")
        }
        (10000, 119_958_499) => {
            String::from("No destinations available.")
        }
        (10000, 13_620_244) => {
            String::from("Open the land control build menu")
        }
        (10000, 77_293_788) => {
            String::from("Requirements:\r\n")
        }
        (10000, 78_396_167) => {
            String::from("Disable/enable window fading")
        }
        (10000, 10_943_915) => {
            String::from("<div><font color==CCInfoText>Note: This was just a simulate check.</font></div>")
        }
        (10000, 203_939_637) => {
            String::from("Item Price")
        }
        (10000, 86_859_851) => {
            String::from("You already know this perk.")
        }
        (10000, 153_518_008) => {
            let a = &arguments[0];
            format!("<font color=CCInfoText>Do you really want to delete the nano <font color=CCInfoHeadline>{a}</font>?</font>")
        }
        (10000, 79_744_894) => {
            String::from("Open")
        }
        (10000, 84_202_590) => {
            String::from("Info on ")
        }
        (10000, 11_166_692) => {
            String::from("Commands the pet to stop its current action, and wait for a new command")
        }
        (10000, 51_644_164) => {
            String::from("Press the key combination you want to bind to this action.")
        }
        (10000, 165_682_599) => {
            String::from("Pet Window")
        }
        (10000, 204_536_802) => {
            String::from("Open the land control war menu")
        }
        (10000, 99_708_235) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Are you sure you want to remove the {a} perk? It will be {b} hours before you can remove another!")
        }
        (10000, 158_452_193) => {
            String::from("<font color=CCInfoHeader>Please select the planet map to use.</font><br><br><font color=CCInfoHeadline>When on Rubi-Ka:</font><br>")
        }
        (10000, 56_702_133) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Team level: {a}  Team side: {b}")
        }
        (10000, 5_372_431) => {
            String::from("Kick ")
        }
        (10000, 147_902_437) => {
            String::from("Auto arrange")
        }
        (10000, 156_586_547) => {
            String::from("Update Team Members")
        }
        (10000, 218_897_477) => {
            String::from("Controller Charge")
        }
        (10000, 257_769_737) => {
            String::from("Atk Delay")
        }
        (10000, 191_622_832) => {
            String::from("You have new mail. Visit a mail terminal to read it.")
        }
        (10000, 181_452_167) => {
            String::from("Remaing")
        }
        (10000, 144_910_531) => {
            String::from("Reset All Hotkeys")
        }
        (10000, 36_718_324) => {
            String::from("Failed to load the GUI definition!")
        }
        (10000, 178_113_221) => {
            String::from("Hide Frame")
        }
        (10000, 114_286_795) => {
            String::from("Remove Perk")
        }
        (10000, 344_211) => {
            String::from("Misc")
        }
        (10000, 176_385_516) => {
            String::from("Modifies:\r\n")
        }
        (10000, 31_206_174) => {
            String::from("Zoom in")
        }
        (10000, 203_245_065) => {
            String::from("Rcg Delay")
        }
        (10000, 135_600_211) => {
            String::from("Request Missions")
        }
        (10000, 103_740_371) => {
            String::from("You must have at least 2 free slots in your inventory when selecting a mission.")
        }
        (10000, 211_692_462) => {
            String::from("You can not untrain a perk that you dont know!")
        }
        (10000, 216_568_099) => {
            String::from("Available Alien Perk Points: ")
        }
        (10000, 212_772_509) => {
            String::from("Delete Item")
        }
        (10000, 14_542_144) => {
            String::from("Show land control areas on the map")
        }
        (10000, 110_790_135) => {
            String::from("Close View")
        }
        (10000, 91_403_440) => {
            String::from("Upload to map")
        }
        (10000, 139_141_187) => {
            String::from("Tools")
        }
        (10000, 170_994_517) => {
            let a = &arguments[0];
            format!("{a} is too high, and will prevent some of the other members of your team in getting xp, are you sure you want to invite him?")
        }
        (10000, 231_672_504) => {
            String::from("The name entered does not match the name of your character.")
        }
        (10000, 240_963_975) => {
            String::from("Info about this window")
        }
        (10000, 188_630_843) => {
            String::from("Commands the pet to attack your selected target")
        }
        (10000, 66_331_364) => {
            String::from("Nano Cost")
        }
        (10000, 237_446_638) => {
            String::from("You must first select the character you want to join your team.")
        }
        (10000, 1_952_557) => {
            String::from("Joining team.")
        }
        (10000, 225_654_005) => {
            String::from("The current playfield type does not allow a market to be built.")
        }
        (10000, 228_142_896) => {
            String::from("There is limited space in your inbox; if it becomes full, you will be unable to receive mail messages until you free up space by deleting old messages.")
        }
        (10000, 5_587_401) => {
            String::from("<font color=red>NONE</font>")
        }
        (10000, 125_358_590) => {
            String::from("Quit To Login")
        }
        (10000, 241_788_702) => {
            String::from("Reset Position")
        }
        (10000, 256_273_934) => {
            String::from("Hidden")
        }
        (10000, 257_495_084) => {
            String::from("Select All")
        }
        (10000, 126_090_750) => {
            String::from("Icon")
        }
        (10000, 110_085_117) => {
            let a = &arguments[0];
            format!("Want to join {a}'s team?")
        }
        (10000, 114_743_685) => {
            String::from("Space")
        }
        (10000, 241_948_484) => {
            String::from("You can only remove a Perk that is the highest of its line!")
        }
        (10000, 86_087_694) => {
            String::from("Request NPC description")
        }
        (10000, 2_716_565) => {
            String::from("Build Menu")
        }
        (10000, 98_273_118) => {
            String::from("Weapon")
        }
        (10000, 11_599_347) => {
            String::from("Demolish Houses")
        }
        (10000, 5_591_426) => {
            String::from("Upkeep Controller")
        }
        (10000, 3_860_335) => {
            String::from("Join Team?")
        }
        (10000, 153_268_952) => {
            let a = &arguments[0];
            format!("<font color=CCInfoText>Do you really want to delete the item <font color=CCInfoHeadline>{a}</font>?</font>")
        }
        (10000, 177_634_807) => {
            String::from("Close Window")
        }
        (10000, 44_189_342) => {
            String::from("Invalid Commission")
        }
        (10000, 235_038_437) => {
            String::from("Delete Reference")
        }
        (10000, 261_213_383) => {
            String::from("Remain")
        }
        (10000, 88_211_117) => {
            String::from("You were kicked from the team.")
        }
        (10000, 191_296_532) => {
            String::from("Key already used")
        }
        (10000, 6_770_569) => {
            String::from("Hostile Only")
        }
        (10000, 66_421_390) => {
            String::from("Do you really want to delete this mission?")
        }
        (10000, 146_447_444) => {
            String::from("Recruit")
        }
        (10000, 57_483_443) => {
            String::from("Used Alien Perk Points: ")
        }
        (10000, 267_537_961) => {
            String::from("Would you also like to reset the list of viewed tips?")
        }
        (10000, 35_463_380) => {
            String::from("Not connected")
        }
        (10000, 8_507_780) => {
            let a = &arguments[0];
            format!("Would you like to team up with {a}? To do so you must first say no to the other request and then make the person ask you again.")
        }
        (10000, 79_662_484) => {
            String::from("Good")
        }
        (10000, 126_141_893) => {
            String::from("Type")
        }
        (10000, 193_057_735) => {
            String::from("Commands the pet to stop its current action, and follow you")
        }
        (10000, 3_982_420) => {
            String::from("Uncharted territory.  Player not found.")
        }
        (10000, 171_050_437) => {
            let a = &arguments[0];
            format!("{a} is too low to get any xp with your current team, are you sure you want to invite him?")
        }
        (10000, 214_304_457) => {
            String::from("Inventory")
        }
        (10000, 108_725_763) => {
            String::from("Options")
        }
        (10000, 251_814_516) => {
            String::from("Wait")
        }
        (10000, 47_335_251) => {
            String::from("Transfer Credits")
        }
        (10000, 253_005_794) => {
            String::from("Character")
        }
        (10000, 103_731_486) => {
            String::from("You are the wrong profession to combine these two items.")
        }
        (10000, 128_381_694) => {
            String::from("Enter new shop commission.")
        }
        (10000, 59_295_557) => {
            String::from("Paste")
        }
        (10000, 30_802_355) => {
            String::from("Implant")
        }
        (10000, 127_299_845) => {
            String::from("Auto arrange")
        }
        (10000, 141_273_460) => {
            String::from("Waypoint uploaded to map and compass.")
        }
        (10000, 102_400_536) => {
            let a = &arguments[0];
            format!("Team level is {a}.")
        }
        (10000, 179_298_440) => {
            let a = &arguments[0];
            format!("Would you like to team up with {a}? To do so you must first leave your current team and make the person ask you again.")
        }
        (10000, 199_898_590) => {
            String::from("Show Hidden Items")
        }
        (10000, 180_901_566) => {
            String::from("Delete Mission")
        }
        (10000, 200_792_437) => {
            let a = &arguments[0];
            format!("{a} declined to join your team.")
        }
        (10000, 104_569_627) => {
            String::from("Attack")
        }
        (10000, 134_543_732) => {
            String::from("There is no available character slot to activate this character.")
        }
        (10000, 168_551_360) => {
            String::from("City Map")
        }
        (10000, 1_571_651) => {
            String::from("Credits")
        }
        (10000, 20_720_116) => {
            String::from("Reload")
        }
        (10000, 57_184_399) => {
            let a = &arguments[0];
            format!("Congratulations, you have reached level 200 and will no longer gain Experience.\r\n\r\nThe only way to advance further is by gaining Shadow Knowledge and the amount of Shadow Knowledge you get is determined by your faction standing with {a}.")
        }
        (10000, 355_467) => {
            String::from("Perk")
        }
        (10000, 1_635_214) => {
            let a = &arguments[0];
            format!("Current Commission: {a}%")
        }
        (10000, 80_381_347) => {
            String::from("General Perks")
        }
        (10000, 94_375_821) => {
            String::from("Lock Item")
        }
        (10000, 126_135_189) => {
            String::from("Time")
        }
        (10000, 41_648_252) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            format!("You need at least <font color=red>{a}</font> skill in {b} to combine these two items.<br><br>Your skill in {c} is {d}")
        }
        (10000, 239_043_241) => {
            String::from("Copy")
        }
        (10000, 251_981_772) => {
            String::from("Medical")
        }
        (10000, 23_615_340) => {
            String::from("Special")
        }
        (10000, 14_519_703) => {
            String::from("Are you sure you want to permanently delete the chat log?")
        }
        (10000, 170_647_748) => {
            String::from("Chat command: ")
        }
        (10000, 260_921_592) => {
            String::from("Health")
        }
        (10000, 268_230_760) => {
            let a = &arguments[0];
            format!("You do not have enough credits for generating missions, you need at least {a}.")
        }
        (10000, 86_977_476) => {
            let a = &arguments[0];
            format!("Do you want to drop item <font color=yellow>{a}</font> to the ground?")
        }
        (10000, 200_506_403) => {
            String::from("Chaos")
        }
        (10000, 180_099_877) => {
            String::from("This will hide the game and switch to Windows. Continue?")
        }
        (10000, 212_410_675) => {
            String::from("Are you sure you want to disable all tips?")
        }
        (10000, 79_642_948) => {
            String::from("Hard")
        }
        (10000, 47_316_708) => {
            String::from("NCU Cost")
        }
        (10000, 163_887_216) => {
            String::from("XP")
        }
        (10000, 211_871_299) => {
            String::from("Give items")
        }
        (10000, 239_033_077) => {
            String::from("<font color=#dddd44>Warning!</font><br><font color=#66aa66>Terminating your character will transfer any experience points gained since last time you <a href=\"showfile://Death and insurance.txt\">saved</a> to your experience pool. Continue?</font>")
        }
        (10000, 82_543_528) => {
            String::from("Health")
        }
        (10000, 10_000_821) => {
            String::from("Alien Experience")
        }
        (10000, 22_653_524) => {
            String::from("Mission")
        }
        (10000, 72_071_186) => {
            String::from("War")
        }
        (10000, 131_083_717) => {
            String::from("Experience")
        }
        (10000, 3_622_952) => {
            String::from("Stealth")
        }
        (10000, 206_709_315) => {
            String::from("Group Perks")
        }
        (10000, 132_254_436) => {
            String::from("Accept")
        }
        (10002, 140_816_276) => {
            String::from("Here is where you can destroy houses. \r\n\r\nNOTE: If you destroy your Organisation Headquarters, all your other houses will be destroyed. Your city plot will be lost if you leave it empty for 24 hours or more.\r\n\r\nTo remove a house, target the house you want to remove and press the \"Demolish Building\" button to remove the house. Once you have pressed this button it will take 2 minutes until the house is completely removed from the area. You will lose the Advantages this house gives as soon as the houses is removed.\r\n\r\nNOTE: There is no way to undo once you demolish a house. The house will be lost (as in deleted, and NOT put back in your inventory) and you will need to obtain new houses in order to build on the area.")
        }
        (10002, 29_711_188) => {
            String::from("You do not have access to this city terminal.")
        }
        (10002, 188_286_316) => {
            String::from("Click to Purchase land area!")
        }
        (10002, 228_244_148) => {
            String::from("The terminal is already in use. Access denied.")
        }
        (10002, 151_488_084) => {
            String::from("This page allows you to build houses. The houses are color coded and the colors means the following.\r\n\r\nYellow house - Existing house\r\nBlue house - House being destroyed\r\nGreen house - Not yet built house on legal location\r\nRed house - Not yet build house on illegal location\r\n\r\nOn houses that have a door on them you are able to see a dot on the shape. This dot marks the entrance. Try to rotate the house some times before building so you are sure that the door is facing the right way. There is no way to undo building a house once its built.")
        }
        (10002, 207_173_998) => {
            String::from("Put money in Upkeep Account")
        }
        (10002, 220_109_620) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Welcome to the City Controller!\r\n\r\nThis city district is for sale.\r\nBuying this district will cost you #1:{a} credits.\r\nThe upkeep is #2:{b} credits per month.\r\n\r\nThe City Controller is the heart of every city district. By using this controller, you are able to purchase land, build houses, and lots more. This controller uses raw Notum for its power supply, and it also has an energy pool attached to it. Certain houses have attached advantages that will benefit all members of your organization. If the energy pool drops below a certain point, the advantages will become inactive.  The controller will need to be recharged with the proper items for those advantages to reactivate.\r\n\r\nIf you choose to buy this city district, you will have 24 hours to build an organizational headquarter. If you have not built an organizational headquarter in the city district within this time frame, you will lose the rights to the land, and you will not be able to claim the city district again until an additional 48 hours have passed.\r\n\r\nWhen you accept the agreement to buy this area, the credits for the purchase will be withdrawn from your character's individual credit balance. Each organization can only own one city district.\r\n\r\nIf the \"Click here to purchase land area\" is greyed out, check the following criteria:\r\n\r\nAre you in an organization?\r\nDo you have enough credits on your character to purchase the area?\r\nDoes your organization own another city district elsewhere?\r\nDoes this instance belong to your organization?")
        }
        (10002, 223_708_388) => {
            String::from("This page contains information about upkeep payment.\r\n\r\nTo the right there are three sections you need to know about.\r\n\r\n\"Your next upkeep payment is\"\r\nThis gives you the date the server will attempt to withdraw money from your organization bank. If you have not got any money in your account at this time your city will be demolished. The area will then be set up for sale again, and anyone can purchase it.\r\n\r\n\"Your current monthly upkeep is\"\r\nThis tells you how much upkeep you are to pay for the area each month at the date mentioned in \"Your next upkeep payment is\".\r\n")
        }
        (10002, 234_912_297) => {
            let a = &arguments[0];
            format!("This house can only be built by {a}'s.")
        }
        (10002, 203_061_820) => {
            String::from("Cancel")
        }
        (200, 259_575_204) => {
            String::from("Timed logout aborted.")
        }
        (200, 256_914_611) => {
            let a = &arguments[0];
            format!("Timed logout started. This will take {a} seconds.")
        }
        (200, 179_936_499) => {
            String::from("Success.")
        }
        (200, 10_607_989) => {
            let a = &arguments[0];
            format!("Reconnecting you to '{a}'.")
        }
        (200, 75_723_156) => {
            String::from("Pressing alt+f4 once more within 3 seconds will lead to fast quit of game. Your character will remain in game world for some minutes.")
        }
        (200, 28_597_332) => {
            String::from("Pressing alt+f4 or typing /quit once more will lead to fast quit of game. Your character will remain in game world for some minutes.")
        }
        (200, 26_967_759) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Client build version: {a} Server: {b}")
        }
        (200, 43_325_156) => {
            String::from("Failure: it is crowded.")
        }
        (200, 92_750_798) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Trying to log you in to '{a}'. {b}")
        }
        (200, 183_695_501) => {
            String::from("ERROR: Client and server versions mismatched!")
        }
        (600, 131_506_020) => {
            String::from("Difficulty to play: <FONT color=\"#ffff00\"> Low </FONT>\r\nWhen it comes to dishing out raw combat damage a Martial Artist outshines all other professions. Fighting unarmed, the Martial Artist's main strength lies in special attacks, knowing how to cripple opponents by attacking their weak spots. The Martial Artist is also a very proficient healer, truly surpassed by only the Doctor and the Adventurer.")
        }
        (600, 264_892_014) => {
            String::from("The Atrox is unique to Rubi-Ka, and was genetically engineered to be a manual laborer in the notum mines. The excessive manipulation led to the Atrox being a genderless breed. The Atrox is strong, hardy and resilient, and makes for a good soldier or worker.")
        }
        (600, 27_330_865) => {
            String::from("Continuing to DNA construction area")
        }
        (600, 234_353_508) => {
            String::from("Agent Selected.")
        }
        (600, 52_480_948) => {
            String::from("Select your character's profession")
        }
        (600, 191_352_900) => {
            String::from("Next.")
        }
        (600, 177_133_309) => {
            String::from("The name can only contain alphabetic chars or digits. And no character after a digit.")
        }
        (600, 200_852_322) => {
            String::from("Difficulty to play: <FONT color=\"#ffff00\"> Medium </FONT>\r\nA Doctor is really a biotechnology specialist. The Doctors' prime skills focus mainly on healing and protecting but they also learn how to produce and administer powerful biotoxins that slow, weaken and wear down their opponents. Limited weapon skills can lead to a bumpy ride when going solo, but in a team the Doctor really shines. When chaos descends on the team in combat, its survival usually lies squarely on the Doctor's shoulders, so this profession is not for the faint-hearted.")
        }
        (600, 191_315_100) => {
            String::from("Tall.")
        }
        (600, 98_043_122) => {
            String::from("No server with correct RDB.")
        }
        (600, 55_564_820) => {
            String::from("Digits can only be used at the end of the name and only after the 4th char.")
        }
        (600, 214_125_134) => {
            String::from("The server is down.")
        }
        (600, 262_139_760) => {
            String::from("Type in a name for your character in the text entry box. You can have the server generate a name for your character by clicking the Suggest Name Button. Note that your character's name needs to be unique. You will be notified if your chosen name has already been taken.")
        }
        (600, 235_785_269) => {
            String::from("You must enter a name before you can proceed!")
        }
        (600, 103_616_948) => {
            String::from("Nanomage Female selected.")
        }
        (600, 195_917_652) => {
            String::from("Select head")
        }
        (600, 125_021_838) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Breed: {a}\r\nProfession: {b}")
        }
        (600, 105_053_140) => {
            String::from("by clicking on any of the fourteen profession buttons. Each profession has its advantages and disadvantages. A detailed description can be found when clicking the profession buttons. When you have made your choice click Next.")
        }
        (600, 137_986_779) => {
            String::from("Click to go back to Profession selection.")
        }
        (600, 163_481_026) => {
            String::from("Difficulty to play: <FONT color=\"#ffff00\"> Low </FONT>\r\nAn Enforcer specializes in close combat using raw power and naked rage to subdue opponents. Enforcers are physically better suited than all others to sustain damage and can learn some protective nanotechnology to further increase their chances of survival. Their brutality and in-your-face combat attitude normally make them the prime targets for any opponent. Enforcers utilize this for the good of the team and rely largely on others to heal them.")
        }
        (600, 204_746_788) => {
            String::from("Martial Artist Selected.")
        }
        (600, 140_082_788) => {
            String::from("Geosynchronous orbit around Rubi-Ka")
        }
        (600, 189_712_468) => {
            String::from("Colonist cleared for passage!\r\n")
        }
        (600, 76_728_164) => {
            String::from("Atrox selected.")
        }
        (600, 100_026_692) => {
            String::from("Click to set your character's height to Short.")
        }
        (600, 115_820_932) => {
            String::from("Opifex Female selected.")
        }
        (600, 51_780_996) => {
            String::from("Solitus Female selected.")
        }
        (600, 116_636_116) => {
            String::from("Click to inspect the Metaphysicist.")
        }
        (600, 152_027_794) => {
            String::from("Click to inspect the Doctor.")
        }
        (600, 111_071_483) => {
            String::from("Choose a nickname.")
        }
        (600, 155_299_122) => {
            String::from("Click to inspect the Keeper.")
        }
        (600, 770_493) => {
            String::from("Character problem. Contact support.")
        }
        (600, 60_879_742) => {
            String::from("S E L E C T   P R O F E S S I O N")
        }
        (600, 172_602_036) => {
            String::from("Nanomage Male selected.")
        }
        (600, 163_071_938) => {
            String::from("Difficulty to play: <FONT color=\"#ffff00\"> Medium / High </FONT>\r\nAn Engineer is a specialist in creating all sorts of machinery. Engineers really excel in constructing powerful battledroids and have access to unique nanotechnology to enhance and repair them. The Engineer's weapon skills are not that great, but the Engineer/robot-pet duo is quite formidable. All Engineers learn to create powerful protective shields and the best Engineers can hack into satellites, and use them to teleport any member of the team to the Engineer's location.")
        }
        (600, 246_788_914) => {
            String::from("Click to inspect the Engineer.")
        }
        (600, 123_574_717) => {
            String::from("Medium.")
        }
        (600, 247_296_306) => {
            String::from("Click to inspect the Enforcer.")
        }
        (600, 264_418_457) => {
            String::from("Click to set your character's body type to Heavy.")
        }
        (600, 52_620_036) => {
            String::from("Suggest name.")
        }
        (600, 112_774_212) => {
            String::from("S E L E C T   B R E E D")
        }
        (600, 7_564_900) => {
            String::from("Click to inspect the Bureaucrat.")
        }
        (600, 191_255_476) => {
            String::from("Exit Character Creation.")
        }
        (600, 156_926_699) => {
            String::from("Click to go back to Appearance selection.")
        }
        (600, 224_353_244) => {
            String::from("Click to set your character's height to Tall.")
        }
        (600, 108_549_524) => {
            String::from("Nickname is invalid or reserved.")
        }
        (600, 128_765_541) => {
            String::from("Click to select Opifex Male.")
        }
        (600, 176_437_060) => {
            String::from("Soldier Selected.")
        }
        (600, 93_888_884) => {
            String::from("Adventurer Selected.")
        }
        (600, 94_046_782) => {
            String::from("The Opifex is genetically engineered to be a fast and agile breed, at the expense of its strength and stamina. The Opifex is versatile and has spread wide across the galaxy. The Opifex makes for a good agent, fixer or martial artist, where speed and precision is beneficial.")
        }
        (600, 164_305_064) => {
            String::from("Click to finish character creation. Once you click finish you can no longer edit your character's breed, appearance, profession or name!")
        }
        (600, 107_050_200) => {
            String::from("Click to select Atrox.")
        }
        (600, 161_383_857) => {
            String::from("Click this button to finish character creation and start in Rubi-Ka.\r\nYour character will be transferred to the ICC Shuttleport.\r\n\r\nOnce you click this button, you can no longer edit your character's breed, appearance, profession or name!\r\n\r\n(The Shadowlands professions Keeper and Shade cannot start in Rubi-Ka.)")
        }
        (600, 164_658_644) => {
            String::from("Please choose a Breed.")
        }
        (600, 187_855_237) => {
            String::from("Click to select Opifex Female.")
        }
        (600, 31_587_506) => {
            String::from("Slender.")
        }
        (600, 196_674_932) => {
            String::from("When you are satisfied with your selection, click to proceed to appearance selection.")
        }
        (600, 244_592_482) => {
            String::from("Difficulty to play: <FONT color=\"#ffff00\"> Low / Medium </FONT>\r\nA Soldier strives for excellence in armed combat. The skill advancement of Soldiers focuses entirely on assault and survival. They use unique nanotechnology to protect their bodies, enhance reflexes, weapon skills and their armor. Perhaps most importantly, Soldiers are able to create strong damage absorption shields around themselves which make them partly invulnerable and even reflect some of the damage back to the attacker.")
        }
        (600, 342_389) => {
            String::from("Click to select Nanomage Male.")
        }
        (600, 24_169_982) => {
            String::from("Click to inspect the Nano Technician.")
        }
        (600, 11_350_309) => {
            String::from("Difficulty to play: <FONT color=\"#ffff00\"> High </FONT>\r\nThe Shade is a mix between a predator and a parasite. Dark and aggressive, the Shade utilizes nano-technology to suck the life and energy out of their prey, robbing them of the basic elements they need to subsist. The Shade stays out of harm's way by relying on concealment and good combat mobility. By using the element of surprise, the Shade can inflict large amounts of damage in a short amount of time.")
        }
        (600, 229_364_195) => {
            String::from("The name must contain more than 3 characters and less than 13.")
        }
        (600, 230_137_493) => {
            String::from("Click to select Solitus Male.")
        }
        (600, 35_129_054) => {
            String::from("The Nanomage is unique to Rubi-Ka, genetically engineered for the notum-rich environment, and thus unable to live anywhere else in the galaxy. The Nanomage has a strong advantage in using nanotechnology and metaphysics.")
        }
        (600, 83_594_021) => {
            String::from("Please board the shuttle to Rubi-Ka, secure yourself and enjoy the ride!\r\nShortly you will find yourself in the Arrival Hall in the spaceport of Omni-1.")
        }
        (600, 163_999_213) => {
            String::from("Click to set your character's height to Medium.")
        }
        (600, 185_671_390) => {
            String::from("Contact administration.")
        }
        (600, 103_286_066) => {
            String::from("Click to set your character's body type to Slender.")
        }
        (600, 128_396_253) => {
            String::from("Medium.")
        }
        (600, 107_443_049) => {
            String::from("Heavy.")
        }
        (600, 190_301_160) => {
            String::from("Finish.")
        }
        (600, 106_904_827) => {
            String::from("Go Back.")
        }
        (600, 248_489_460) => {
            String::from("Account not paid.")
        }
        (600, 218_721_543) => {
            String::from("Click this button to finish character creation and start in Arete Landing. Your character will be transferred to Arete Landing. Once you click this button, you can no longer edit your character's breed, appearance, profession or name!")
        }
        (600, 108_214_510) => {
            String::from("Difficulty to play: <FONT color=\"#ffff00\"> Medium </FONT>\r\nA Nano Technician is an expert user of aggressive nanotechnology. Nano Technicians are experts at using nanotechnology to deal explosive area-of-effect damage and the types of damage they can manage is incomparable. They are also capable of using other kinds of nanotechnology and can for example warp themselves between locations. Nano-Technicians must be devoted to nanotechnology skills and as a result, physical and weapon skills will suffer.")
        }
        (600, 108_956_596) => {
            String::from("Solitus Male selected.")
        }
        (600, 196_649_904) => {
            String::from("Select your character's breed by clicking on the breed models. Each breed offers different advantages and disadvantages. Learn more by selecting a breed. When you have made your choice click Next.")
        }
        (600, 93_730_238) => {
            String::from("Account already in use, character logged in.")
        }
        (600, 177_652_275) => {
            String::from("Can't create more characters.")
        }
        (600, 207_923_652) => {
            String::from("Engineer Selected.")
        }
        (600, 67_416_068) => {
            String::from("Player not found.")
        }
        (600, 13_634_964) => {
            String::from("Account is locked.")
        }
        (600, 118_218_323) => {
            String::from("Checking credentials...\r\n")
        }
        (600, 207_344_627) => {
            String::from("Click this button to finish character creation and start in the Shadowlands.\r\nYour character will be transferred to the Jobe research facilities.\r\n\r\nOnce you click this button, you can no longer edit your character's breed, appearance, profession or name!\r\n\r\n(You need the Shadowlands expansion pack to start in the Shadowlands.)")
        }
        (600, 41_516_260) => {
            String::from("Opifex Male selected.")
        }
        (600, 245_803_508) => {
            String::from("Enforcer Selected.")
        }
        (600, 46_271_652) => {
            String::from("by selecting a face, body height, and body type. Your character's appearance does not influence the gameplay in any way. When you have made your choices, click Next.")
        }
        (600, 260_745_330) => {
            String::from("Click to inspect the Fixer.")
        }
        (600, 260_793_573) => {
            String::from("E N T E R   N A M E")
        }
        (600, 260_210_404) => {
            String::from("Click to inspect the Agent.")
        }
        (600, 90_551_204) => {
            String::from("Account has illegal password.")
        }
        (600, 108_018_980) => {
            String::from("Short.")
        }
        (600, 261_898_517) => {
            String::from("Click to inspect the Shade.")
        }
        (600, 63_701_108) => {
            String::from("Shade Selected.")
        }
        (600, 170_145_658) => {
            String::from("Nickname:")
        }
        (600, 42_695_918) => {
            String::from("Click to proceed to Profession selection.")
        }
        (600, 119_893_348) => {
            String::from("Fixer Selected.")
        }
        (600, 137_916_099) => {
            String::from("Difficulty to play: <FONT color=\"#ffff00\"> Medium / High </FONT>\r\nThe Meta-Physicists get their strength from the \"other side\". They can manifest their emotions in the material world and eventually control multiple materialized entities and use them in combat. The uniqueness of a Meta-Physicist lies in manipulating the underlying fabric of the world where nanotechnology operates, adjusting the nanotechnology skills of friends and foes alike. Their weapon skills are poor but they can use their powers to damage their opponents directly.")
        }
        (600, 225_957_460) => {
            String::from("Select height")
        }
        (600, 72_416_866) => {
            String::from("Difficulty to play: <FONT color=\"#ffff00\"> Low / Medium </FONT>\r\nAn Adventurer's soul is at home in the wild. Adventurers study the animals, learn their ways and gain some of their abilities along the way. Their weapon skills are well balanced with equal advancement opportunities in melee and ranged combat. They also become skilled at using nanotechnology to withstand damage or to create metaphysical cloaks that damage their aggressors. An Adventurer is an excellent healer, only truly rivaled by the Doctor.")
        }
        (600, 262_052_372) => {
            String::from("All your selections will be saved untill next time you enter.")
        }
        (600, 108_184_756) => {
            String::from("Docking procedure completed")
        }
        (600, 80_046_117) => {
            String::from("Click to select Nanomage Female.")
        }
        (600, 156_977_540) => {
            String::from("Click to proceed to name selection.")
        }
        (600, 169_342_932) => {
            String::from("Player has illegal password.")
        }
        (600, 24_135_060) => {
            String::from("Character not found.")
        }
        (600, 213_881_643) => {
            String::from("Getting suggested nickname...\r\n")
        }
        (600, 85_960_868) => {
            String::from("Bureaucrat Selected.")
        }
        (600, 24_541_365) => {
            String::from("Boarding shuttle to Omni-1")
        }
        (600, 122_709_390) => {
            String::from("Nickname is already in use.")
        }
        (600, 11_734_740) => {
            String::from("Difficulty to play: <FONT color=\"#ffff00\"> High </FONT>\r\nAn Agent&#8217;s life is spent in the shadows. Agents focus on concealment and subterfuge skills and one of their special abilities is going undercover. Doing so enables them to use nanotechnology normally only available to other professions. When it comes to combat, the Agent&#8217;s speciality is sniping opponents with high velocity rifles using unique nanotechnology to further increase the damage.")
        }
        (600, 107_121_557) => {
            String::from("Click to select Solitus Female.")
        }
        (600, 12_179_522) => {
            String::from("Difficulty to play: <FONT color=\"#ffff00\"> Medium </FONT>\r\nA Fixer specializes in getting people what they need when they need it.  By hacking into what is known as The Grid from anywhere in the world, Fixers can use it to transport themselves or their entire team in digital form around Rubi-Ka. The Fixers move fastest of all and special armors make them the hardest to hit as well. A Fixer's main combat strategy lies in limiting the maneuverability of opponents and they favour weapons from the sub-machinegun category.")
        }
        (600, 56_161_934) => {
            String::from("Character already logged in.")
        }
        (600, 111_591_508) => {
            String::from("Click to inspect the Martial Artist.")
        }
        (600, 27_519_330) => {
            String::from("Click to inspect the Adventurer.")
        }
        (600, 72_444_052) => {
            String::from("Trader Selected.")
        }
        (600, 32_335_924) => {
            String::from("Keeper Selected.")
        }
        (600, 1_622_994) => {
            String::from("The Morning Star Transfer Station")
        }
        (600, 13_618_949) => {
            String::from("Select face.")
        }
        (600, 183_833_282) => {
            String::from("Difficulty to play: <FONT color=\"#ffff00\"> High </FONT>\r\nThe Trader is the ultimate entrepreneur, getting more for less in every single transaction. Like others, they create and trade material goods. But in combat situations, they use unique nanotechnology which enables them to drain opponents of skills, energy and health, transfering those benefits to themselves or their allies. This frequently causes the most formidable opponents to wither to a cracked shell of their former self.")
        }
        (600, 207_007_426) => {
            String::from("Difficulty to play: <FONT color=\"#ffff00\"> Medium </FONT>\r\nThe Keeper is a fighter that radiates valour and heroism - a beacon of light and hope to the team. A formidable opponent who specialises in close combat, the Keeper is especially proficient wielding two-handed edged weapons. This profession's uniqueness lies in the ability to share life and diverse powers with nearby allies.")
        }
        (600, 82_048_901) => {
            String::from("S E L E C T   A P P E A R A N C E")
        }
        (600, 58_751_332) => {
            String::from("Difficulty to play: <FONT color=\"#ffff00\"> High </FONT>\r\nThe Bureaucrat brings order to chaos. Bureaucrats have very limited weapon skills but their vast knowledge of nanotechnology makes up for that. They use it to directly damage opponents and create robots that will fight for them. But most importantly, a Bureaucrat uses nanotechnology to boost his leadership skills and to control hostile beings, bending their minds and blurring their aims. In a team situation, a Bureaucrat is therefore a natural leader.")
        }
        (600, 234_583_117) => {
            String::from("Click to set your character's body type to Medium.")
        }
        (600, 165_810_482) => {
            String::from("Click to inspect the Trader.")
        }
        (600, 240_934_557) => {
            String::from("Something is wrong with the server.")
        }
        (600, 171_771_173) => {
            String::from("Click the two arrows repeatedly to select a face.")
        }
        (600, 183_066_372) => {
            String::from("Click to have the server suggest a name for your character. If the suggested name is not to your liking, click the button again. You can modify a suggested name by editing it in the text entry box.")
        }
        (600, 252_555_380) => {
            String::from("Metaphysicist Selected.")
        }
        (600, 260_098_452) => {
            String::from("Doctor Selected.")
        }
        (600, 192_016_820) => {
            String::from("Click to go back to Breed selection.")
        }
        (600, 47_929_166) => {
            String::from("The Solitus is the galaxy's most common breed - a direct natural descendant of the Homo Sapiens which evolved during Earth's long winter. The Solitus has no specific advantage, but is capable of adapting to almost any situation and environment, and has a proclivity for learning.")
        }
        (600, 181_436_052) => {
            String::from("Select build")
        }
        (600, 235_147_346) => {
            String::from("Click to inspect the Soldier.")
        }
        (600, 18_488_884) => {
            String::from("Nano Technician Selected.")
        }
        (600, 2_457_557) => {
            String::from("Select your character's appearance")
        }
        (110, 103_385_539) => {
            String::from("No more charges.")
        }
        (110, 163_682_084) => {
            String::from("Do you want to rent this shop terminal?")
        }
        (110, 160_127_261) => {
            let a = &arguments[0];
            format!("You must not have unique item: {a}")
        }
        (110, 182_784_660) => {
            String::from("Use of the market is limited to paying customers.")
        }
        (110, 16_156_542) => {
            String::from("This action requires that you have the second specialization completed!")
        }
        (110, 199_909_321) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("{a} attacked by {b}!")
        }
        (110, 245_161_956) => {
            String::from("Target item is not owned by the char and does not have the stationary flags set!")
        }
        (110, 32_292_532) => {
            String::from("PvP disgrace period has ended.")
        }
        (110, 110_108_692) => {
            String::from("Teleport failed. The destination playfield has reached its player limit.")
        }
        (110, 138_249_668) => {
            String::from("Invalid petcommand.")
        }
        (110, 250_182_048) => {
            String::from("As a result of your death, you just lost some of your Alien Experience.")
        }
        (110, 217_574_658) => {
            String::from("No position and no receiver.")
        }
        (110, 105_185_965) => {
            String::from("You're not allowed to use this item.")
        }
        (110, 209_495_735) => {
            String::from("Temporary items can't be moved into a bag!")
        }
        (110, 238_130_271) => {
            String::from("You've already got this nano program.")
        }
        (110, 12_337_566) => {
            String::from("Please wait until previous action has finished.")
        }
        (110, 69_819_878) => {
            String::from("Can't assist yourself.")
        }
        (110, 94_564_995) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Your pet {a} was damaged by a toxic substance for {b} points of damage.")
        }
        (110, 159_651_462) => {
            String::from("Item must be applied on self.")
        }
        (110, 18_897_751) => {
            String::from("The mine was sprung! You failed disarming the mine!")
        }
        (110, 59_575_492) => {
            let a = &arguments[0];
            format!("Nanoprogram {a} terminated...")
        }
        (110, 172_710_793) => {
            String::from("You can assemble these two items, with excellent quality!")
        }
        (110, 7_144_452) => {
            let a = &arguments[0];
            format!("This XP was added to the pool of unsaved experience points that you get back through gaining new experience points. Your current pool of unsaved experience is {a}.")
        }
        (110, 85_787_460) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("You were drained for {a} points of nano energy by {b}.")
        }
        (110, 184_106_293) => {
            String::from("You are not allowed to teleport players to this playfield.")
        }
        (110, 246_170_789) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("Player {a} hit you for {b} points of {c} damage.")
        }
        (110, 258_871_949) => {
            String::from("You have already got this unique item")
        }
        (110, 260_086_693) => {
            String::from("You can't drop this item here!")
        }
        (110, 245_003_004) => {
            String::from("You cannot attack this player since you are too far away in level.")
        }
        (110, 13_065_464) => {
            String::from("invalid length")
        }
        (110, 265_632_434) => {
            String::from("You need to be an organization member to perform this action!")
        }
        (110, 120_348_612) => {
            String::from("Item dropped on ground.")
        }
        (110, 149_001_680) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            let e = &arguments[4];
            let f = &arguments[5];
            format!("{a} owns the shop \"{b}\" in the market at ({c}, {d}) in \"{e}\" ({f}).")
        }
        (110, 134_065_072) => {
            String::from("You already own a shop; you can't own more than one shop at a time!")
        }
        (110, 186_514_322) => {
            String::from("It's not possible to reverse engineer this item.")
        }
        (110, 146_515_795) => {
            String::from("You can't trade more items in one go.")
        }
        (110, 20_453_397) => {
            let a = &arguments[0];
            format!("{a} is open. Please close it first.")
        }
        (110, 240_058_395) => {
            String::from("You have gained a new Perk!")
        }
        (110, 98_652_424) => {
            let a = &arguments[0];
            format!("Leadership was given to you by {a}.")
        }
        (110, 264_925_177) => {
            String::from("Wrong mission key: the mission key contains no authorization to open this entrance!\r\n")
        }
        (110, 205_739_797) => {
            let a = &arguments[0];
            format!("Your trade partner already has this unique item: {a}")
        }
        (110, 88_185_376) => {
            let a = &arguments[0];
            format!("{a} of your XP were allocated to global research.\r\n")
        }
        (110, 126_200_468) => {
            let a = &arguments[0];
            format!("Right now you have spent {a} IP in Map Navigation. You will not get these back.")
        }
        (110, 210_536_934) => {
            let a = &arguments[0];
            format!("Entering '{a}'")
        }
        (110, 160_969_117) => {
            let a = &arguments[0];
            format!("You must have unique item: {a}")
        }
        (110, 204_982_716) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            format!("{a} hit {b} for {c} points of {d} damage.")
        }
        (110, 122_930_573) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("\"{a}\" at level {b} with price {c}.")
        }
        (110, 243_874_121) => {
            String::from("The item was not accepted as a key for this door.")
        }
        (110, 20_103_316) => {
            String::from("You need to select a target for this action!")
        }
        (110, 229_754_349) => {
            String::from("The nickname you have specified is not valid:")
        }
        (110, 115_813_076) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            let e = &arguments[4];
            let f = &arguments[5];
            let g = &arguments[6];
            let h = &arguments[7];
            format!("{a}: Health: {b}%  Nano: {c}%  NCU: {d}/{e}    Timeleft: {f} s    Position: {g},{h}\r\n")
        }
        (110, 12_594_478) => {
            let a = &arguments[0];
            format!("Your team has received a new mission. The keeper of the mission is {a}.")
        }
        (110, 38_272_366) => {
            String::from("You need to have a 1 hand edged weapon equipped in your right hand.")
        }
        (110, 83_397_292) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Invalid use of control codes at position {a} in '{b}'")
        }
        (110, 226_935_915) => {
            let a = &arguments[0];
            format!("Must have perk: {a}")
        }
        (110, 54_640_832) => {
            String::from("This is a player shop!")
        }
        (110, 223_638_590) => {
            String::from("You need to have a melee weapon equipped in your right hand.")
        }
        (110, 253_484_164) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("\"{a}\" is of a too low quality level. It must be at least at quality {b}.")
        }
        (110, 117_739_867) => {
            String::from("Poison roll sucessful..")
        }
        (110, 3_480_567) => {
            String::from("Crowd limiting was enforced. You were removed from the crowd.")
        }
        (110, 743_316) => {
            let a = &arguments[0];
            let b = {
                if a.to_string() == "1" {
                    " credit was "
                } else {
                    " credits were"
                }
            };
            format!("{a} {b} deducted from your account.")
        }
        (110, 167_634_562) => {
            String::from("This action can only be done by the leader of the organization owning this shop!")
        }
        (110, 67_192_350) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Unknown error at position {a} in '{b}'")
        }
        (110, 629_344) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("You recouped {a} of your previously lost experience - remaining to be recouped: {b}.")
        }
        (110, 221_009_476) => {
            String::from("Monster-spawn to inventory is ILLEGAL.")
        }
        (110, 154_478_725) => {
            let a = &arguments[0];
            format!("Target breed must be {a}!")
        }
        (110, 111_805_842) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("Something hit {a} for {b} points of {c} damage.")
        }
        (110, 38_318_091) => {
            String::from("Unique item returned to bank.")
        }
        (110, 183_929_219) => {
            String::from("Spirits can only be used by Shades!")
        }
        (110, 225_651_460) => {
            String::from("You are awarded a token for your team's heroic effort.")
        }
        (110, 89_746_517) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("{a} hit you for {b} points of {c} damage.")
        }
        (110, 38_761_861) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("invalid use of uppercase/lowercase at position {a} in '{b}'")
        }
        (110, 70_076_351) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("{a} cannot run the nano: {b}!")
        }
        (110, 109_059_972) => {
            String::from("No entrance door identity found.")
        }
        (110, 120_893_516) => {
            let a = &arguments[0];
            format!("{a} rejected your challenge.")
        }
        (110, 182_595_589) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("Time until decay: {a}:{b}:{c} (owner inactivity)")
        }
        (110, 50_403_583) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            let e = &arguments[4];
            format!("Player {a} profession {b} has {c} in full AMS for {d} with multiplier {e}.")
        }
        (110, 150_897_955) => {
            String::from("You must be in the Shadowlands!")
        }
        (110, 143_905_433) => {
            String::from("Wrong item: the item is not a mission key!\r\n")
        }
        (110, 57_671_460) => {
            let a = &arguments[0];
            format!("{a} credits were deducted from your account.")
        }
        (110, 101_800_949) => {
            String::from("You won a new PvP title!")
        }
        (110, 165_170_163) => {
            String::from("You can not use this on a vehicle.")
        }
        (110, 210_178_164) => {
            String::from("The recipient was not found. Please make sure you entered the recipient name correctly.")
        }
        (110, 238_534_821) => {
            let a = &arguments[0];
            format!("{a} skill available.")
        }
        (110, 121_819_877) => {
            let a = &arguments[0];
            format!("You can't afford to save. At your level you need {a} credits to save.")
        }
        (110, 193_866_428) => {
            String::from("You can only heal yourself or your pet in a duel.")
        }
        (110, 179_934_581) => {
            let a = &arguments[0];
            format!("You need at least {a} free inventory slots! Please remove an item and try again.")
        }
        (110, 100_703_038) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = {
                if a.to_string() == "1" {
                    " member was "
                } else {
                    " members were"
                }
            };
            format!("{a} team {c} unable to receive the team mission, because a member of the team has too high a level to be able to share experience! ( {b} )\r\n")
        }
        (110, 179_318_652) => {
            String::from("PvP not allowed since you are neutral. Can only attack if attacked first.")
        }
        (110, 88_689_636) => {
            String::from("Target is dead. Trade aborted.")
        }
        (110, 198_893_196) => {
            String::from("The target team is full.")
        }
        (110, 48_324_276) => {
            String::from("Must have specific active mission")
        }
        (110, 24_370_003) => {
            String::from("Mindcontrol only work on NPCs")
        }
        (110, 136_744_723) => {
            String::from("No hidden objects found.")
        }
        (110, 91945) => {
            String::from("Your voucher was not accepted by the system. This may be a temporary malfunction. Please try again later.")
        }
        (110, 180_695_854) => {
            String::from("You must be indoors to perform this action.")
        }
        (110, 215_288_929) => {
            String::from("Changing area. Please wait.")
        }
        (110, 37_445_127) => {
            String::from("Reloading...")
        }
        (110, 75_012_101) => {
            String::from("You lost your PvP title!")
        }
        (110, 108_623_444) => {
            String::from("Raid created.")
        }
        (110, 132_570_597) => {
            String::from("You are not allowed to execute this heal due to the difference in suppression gas levels between you and your target.")
        }
        (110, 163_421_198) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("{a} casts nanoprogram '{b}' on {c}.")
        }
        (110, 229_062_263) => {
            String::from("You have a prize waiting for you. This prize is for one of your characters only, you should log in with the character you want to acquire the prize for. Make sure you have an empty slot in your inventory then type `/command claim` in the chat window.")
        }
        (110, 195_424_253) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("You can not afford this item. {a} extra {b} is needed.")
        }
        (110, 6_774_439) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("{a} must recently have {b} running!")
        }
        (110, 96_965_431) => {
            String::from("You can't execute nanoprograms while swimming!")
        }
        (110, 191_983_332) => {
            let a = &arguments[0];
            format!("(GM feedback) This savebooth has an invalid e_ResurrectDest ({a})!")
        }
        (110, 6_355_396) => {
            String::from("You failed the requirements to train this perk!")
        }
        (110, 162_426_029) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Price set on {a} to {b}!")
        }
        (110, 217_059_156) => {
            String::from("You cannot unlearn a Perk that is a requirement for another Perk.")
        }
        (110, 32_855_904) => {
            let a = &arguments[0];
            format!("This is your shop (max {a} items)!")
        }
        (110, 116_194_815) => {
            let a = &arguments[0];
            format!("Shop commission set to {a}%!")
        }
        (110, 95_906_044) => {
            String::from("Your opponent accepted your challenge.")
        }
        (110, 173_819_685) => {
            String::from("Target is already in a trade.")
        }
        (110, 197_782_805) => {
            let a = &arguments[0];
            format!("{a} available.")
        }
        (110, 89_766_772) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("{a}'s execution of {b} got interrupted by {c}..")
        }
        (110, 147_956_516) => {
            String::from("The maximum amount of items in your apartment has been reached! This item will be lost if left here.")
        }
        (110, 232_281_461) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("You absorbed {a} points of {b} damage.")
        }
        (110, 205_332_636) => {
            String::from("Other player's inventory is full, can't complete trade!")
        }
        (110, 179_047_940) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            let e = &arguments[4];
            let f = &arguments[5];
            format!("{a} can now loot <a href=\"itemref://{b}/{c}/{d}\">{e}</a> from {f}.")
        }
        (110, 79_653_355) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Target {a} your {b}!")
        }
        (110, 191_797_221) => {
            String::from("You have made a challenge to a pet fight.")
        }
        (110, 237_847_795) => {
            String::from("You must accept all attachments before you can delete the message.")
        }
        (110, 64_419_060) => {
            String::from("Mission expired.")
        }
        (110, 135_453_684) => {
            String::from("Nano program aborted.")
        }
        (110, 183_520_067) => {
            let a = &arguments[0];
            format!("The shop contains {a} items.")
        }
        (110, 48_456_878) => {
            String::from("You need to have a bow weapon equipped.")
        }
        (110, 236_786_603) => {
            let a = &arguments[0];
            format!("Your attack shield blocked the attack! ({a}) left.")
        }
        (110, 153_860_147) => {
            String::from("You can not send nodrop items to the market.")
        }
        (110, 151_684_081) => {
            String::from("Laser targets can only be placed in landcontrol areas.")
        }
        (110, 33_183_701) => {
            String::from("GM kicked you from your organization.")
        }
        (110, 112_872_740) => {
            String::from("Your target can not be an opponent for a pet duel.")
        }
        (110, 227_415_294) => {
            String::from("You need to have a 2 hand edged weapon equipped.")
        }
        (110, 241_952_942) => {
            String::from("This action requires that you have the third specialization completed!")
        }
        (110, 68_235_396) => {
            String::from("Stuck-resolve command not allowed while in combat.")
        }
        (110, 80_377_220) => {
            String::from("The old Rubi-Ka market system has been disabled. Please visit a market terminal to access the new system.")
        }
        (110, 138_866_588) => {
            String::from("Sending mail is limited to paying customers.")
        }
        (110, 208_896_900) => {
            let a = &arguments[0];
            format!("{a} tried to hide but failed.")
        }
        (110, 198_764_389) => {
            String::from("Can't use all ammo at once.")
        }
        (110, 255_324_078) => {
            String::from("Can't afford to change profession!")
        }
        (110, 8_996_836) => {
            String::from("This item can't be lifted.")
        }
        (110, 2_485_460) => {
            String::from("Nano program execution error. You fumbled.")
        }
        (110, 1_622_884) => {
            let a = &arguments[0];
            format!("This shop is owned by: {a}")
        }
        (110, 75_171_540) => {
            String::from("Target has surrendered.")
        }
        (110, 84_058_663) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            let e = &arguments[4];
            let f = &arguments[5];
            let g = &arguments[6];
            let h = &arguments[7];
            format!("{a}: Health: {b}%  Nano: {c}%  NCU: {d}/{e}    Position: {f},{g}    Fighting: {h}\r\n")
        }
        (110, 132_904_324) => {
            String::from("No PvP grace period since you already are in a fight.")
        }
        (110, 249_817_907) => {
            String::from("You can loot these remains.")
        }
        (110, 176_044_766) => {
            String::from("You got a new mission.")
        }
        (110, 32_194_660) => {
            String::from("This shop has been unlocked!")
        }
        (110, 182_416_562) => {
            String::from("You gain Shadowknowledge from killing monsters and performing tasks aligned with the two Unredeemed / Redeemed factions. If you kill an Unredeemed monster, you need Redeemed faction and vice versa. The higher the faction, the greater the Shadowknowledge.\" ")
        }
        (110, 107_353_316) => {
            let a = &arguments[0];
            format!("{a} cannot be summoned because it is in a fight.")
        }
        (110, 2_091_948) => {
            String::from("You gained a new Shadowlevel!")
        }
        (110, 106_185_396) => {
            let a = &arguments[0];
            format!("You tried to hit {a}, but missed!")
        }
        (110, 218_510_731) => {
            let a = &arguments[0];
            format!("Your Full IP Reset has finished.\r\n\r\nYou now have {a} IP.")
        }
        (110, 225_252_741) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("{a} absorbed {b} points of {c} damage.")
        }
        (110, 109_640_532) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("You cannot process the \"{a}\" as it needs to have exactly {b} charges.")
        }
        (110, 75_682_676) => {
            String::from("NoDrop item can't be traded or sold.")
        }
        (110, 162_154_068) => {
            String::from("This item contains a unique item you've already got.")
        }
        (110, 629_451) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("You recouped {a} of your previously lost Shadowknowledge - remaining to be recouped: {b}.")
        }
        (110, 197_583_108) => {
            String::from("Authorization accepted, please wait!")
        }
        (110, 112_959_924) => {
            String::from("ducked")
        }
        (110, 138_284_289) => {
            String::from("Duels are not allowed in PVP areas.")
        }
        (110, 242_934_654) => {
            String::from("You need to be in an area owned by your organization to perform this action!")
        }
        (110, 145_393_213) => {
            String::from("You are not allowed to execute hostile nanoprogram on this target.")
        }
        (110, 92_364_323) => {
            String::from("You need at least 2 free inventory slots for mission items.")
        }
        (110, 8_208_531) => {
            String::from("You have full access to this city terminal.")
        }
        (110, 106_156_137) => {
            String::from("Item must be applied on a friendly target.")
        }
        (110, 4_105_896) => {
            String::from("You must have the Legacy of the Xan booster!")
        }
        (110, 142_728_293) => {
            String::from("You need at least one free inventory slot! Please remove an item and try again.")
        }
        (110, 707) => {
            let a = &arguments[0];
            format!("{a}")
        }
        (110, 249_863_897) => {
            String::from("Both items must be in your inventory window.")
        }
        (110, 197_599_390) => {
            String::from("You must be in a battle station to perform this action.")
        }
        (110, 175_335_076) => {
            String::from("You are awarded a token for your heroic effort.")
        }
        (110, 154_991_563) => {
            let a = &arguments[0];
            format!("Your special attack shield blocked the attack! ({a}) left.")
        }
        (110, 133_836_485) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("You hit {a} for {b} points of damage.")
        }
        (110, 197_584_373) => {
            String::from("This voucher is not valid for this city district. Please consult the item description.")
        }
        (110, 154_786_440) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = {
                if b.to_string() == "1" {
                    "credit "
                } else {
                    " credits"
                }
            };
            format!("Received reward: {a} SK, {b} {c}.")
        }
        (110, 83_111_204) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Invalid use of digits at position {a} in '{b}'")
        }
        (110, 171_196_092) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            let e = &arguments[4];
            format!("\"{a}\" is of a too low quality level. With \"{b}\" at quality of {c} , the \"{d}\" must be at least at quality {e}.")
        }
        (110, 139_545_492) => {
            String::from("PvP not allowed in this district.")
        }
        (110, 80_264_137) => {
            String::from("The container must be empty before it can be sold!")
        }
        (110, 75_889_919) => {
            let a = &arguments[0];
            format!("You need at least {a} remaining nano energy to execute this program.")
        }
        (110, 245_219_682) => {
            String::from("Team loot order must be set to 'leader' to perform this action.")
        }
        (110, 56_715_957) => {
            String::from("You can't promote someone to your own level in organization or higher!")
        }
        (110, 87_280_017) => {
            String::from("You must have the Shadowlands Expansion Pack!")
        }
        (110, 143_937_877) => {
            String::from("Must not be in combat mode.")
        }
        (110, 184_840_837) => {
            String::from("You will not be able to gain ShadowKnowledge nor obtain ShadowLevels (levels above 200)! You can change this through the option->misc view.")
        }
        (110, 266_975_780) => {
            String::from("Chest contains a Unique item, and can't be traded.")
        }
        (110, 268_234_035) => {
            String::from("Duels are not allowed between flagged characters.")
        }
        (110, 55_195_541) => {
            let a = &arguments[0];
            format!("Target side must be {a}!")
        }
        (110, 102_376_403) => {
            String::from("Searching for hidden objects.")
        }
        (110, 147_700_404) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("{a} {b} is required to be at least {c}!")
        }
        (110, 167_845_155) => {
            String::from("You can not send container items through the mail system.")
        }
        (110, 56_855_412) => {
            String::from("The door is now locked.")
        }
        (110, 121_950_320) => {
            String::from("A too high level player in your team prevents you from receiving any experience.")
        }
        (110, 41_219_730) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("{a}'s damage shield hit {b} for {c} points of damage.")
        }
        (110, 37_045_237) => {
            String::from("You can't save here!")
        }
        (110, 4_567_740) => {
            String::from("Arming trap successful.")
        }
        (110, 187_163_108) => {
            String::from("Mission chance of token reward not upped due to your superior level.")
        }
        (110, 193_870_018) => {
            String::from("Only team leader can change loot order!")
        }
        (110, 7_311_748) => {
            let a = &arguments[0];
            format!("Your nano execution got interrupted by {a}..")
        }
        (110, 103_392_196) => {
            String::from("Fight!")
        }
        (110, 38_085_045) => {
            String::from("Special attack not possible. The target must be fighting someone else.")
        }
        (110, 187_774_013) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("{a} bought {b} for {c} credits.")
        }
        (110, 31_082_299) => {
            String::from("Mission items (including mission items in bags) can not be placed in the bank.")
        }
        (110, 154_558_667) => {
            String::from("Wait for your previous special attack to complete.")
        }
        (110, 265_391_758) => {
            String::from("Tradepartner is unable to carry more than 1 billion credits. Trade aborted.")
        }
        (110, 265_839_031) => {
            String::from("Text and/or subject is too long. Please shorten your message.")
        }
        (110, 72_118_335) => {
            String::from("Stuck-resolve command not allowed while one or more pet is affected by hostile nano programs.")
        }
        (110, 185_346_162) => {
            String::from("Player is already a team member.")
        }
        (110, 265_067_572) => {
            String::from("You have completed the third specialization!")
        }
        (110, 63_713_223) => {
            String::from("Target must be fighting.")
        }
        (110, 40_181_148) => {
            String::from("You can't split as your inventory is full!")
        }
        (110, 2_562_484) => {
            String::from("You must be in a raid team.")
        }
        (110, 76_616_117) => {
            let a = &arguments[0];
            format!("The shop balance is {a} credits.")
        }
        (110, 96_085_107) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Your summon target was bound at position {a},{b}. You need to move closer to that position before you are allowed to summon him.")
        }
        (110, 137_471_989) => {
            String::from("Wrong ammotype.")
        }
        (110, 235_430_520) => {
            String::from("The target must have the Legacy of the Xan booster!")
        }
        (110, 55_963_445) => {
            String::from("You did too little damage on the target to get any XP for this kill!")
        }
        (110, 185_674_821) => {
            String::from("Your trade partner already has this unique item.")
        }
        (110, 174_549_876) => {
            String::from("Your summon target has not been bound while in this team.")
        }
        (110, 118_314_187) => {
            let a = &arguments[0];
            let b = {
                if a.to_string() == "1" {
                    " second. "
                } else {
                    " seconds."
                }
            };
            format!("You will be open to attack in {a} {b}")
        }
        (110, 34_043_559) => {
            String::from("You can't execute nanoprograms while falling!")
        }
        (110, 107_290_227) => {
            let a = &arguments[0];
            let b = {
                if a.to_string() == "1" {
                    " bullet... "
                } else {
                    " bullets..."
                }
            };
            format!("You hit with {a} {b}")
        }
        (110, 156_906_702) => {
            String::from("You need to have a energy weapon equipped in your right hand.")
        }
        (110, 56_838_396) => {
            let a = &arguments[0];
            format!("{a} is not a valid opponent for pet duel.")
        }
        (110, 68_481_987) => {
            String::from("You do not have enough credits to complete this transaction.")
        }
        (110, 212_855_861) => {
            String::from("You can't move.")
        }
        (110, 115_443_319) => {
            String::from("You're opponent withdrew from the pet duel.")
        }
        (110, 123_319_228) => {
            String::from("Target is engaged in a duel.")
        }
        (110, 106_107_150) => {
            String::from("You need to have a shotgun weapon equipped.")
        }
        (110, 104_440_254) => {
            String::from("You need to have a heavy weapon equipped.")
        }
        (110, 197_910_644) => {
            String::from("Your target can't be attacked.")
        }
        (110, 114_868_695) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Playshifting failed: The server was unable to start the building. ( {a}, {b} )")
        }
        (110, 171_187_118) => {
            String::from("You are already running this action!")
        }
        (110, 216_079_284) => {
            String::from("This mine is armed.  You might be able to disarm it using the proper tools!")
        }
        (110, 254_181_591) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("{a} must not have a {b} running!")
        }
        (110, 257_117_687) => {
            String::from("You can't perform social animations while swimming!")
        }
        (110, 87_819_175) => {
            String::from("You must be standing to perform this action.")
        }
        (110, 60_292_788) => {
            String::from("You have lost the duel!")
        }
        (110, 251_038_293) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            format!("Left hand weapon {a} : {b} : {c} : {d}")
        }
        (110, 100_495_847) => {
            String::from("You can't do this while fighting!")
        }
        (110, 165_894_343) => {
            String::from("You can't take a fixture from a building owned by someone.")
        }
        (110, 53_106_771) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("{a} was attacked with nanobots for {b} points of {c} damage.")
        }
        (110, 180_930_712) => {
            let a = &arguments[0];
            format!("Entering '{a}'.")
        }
        (110, 247_632_055) => {
            String::from("You failed to hide as you are currently fighting.")
        }
        (110, 210_473_883) => {
            String::from("Your inventory is full. An item has been put into your bank.")
        }
        (110, 229_754_212) => {
            String::from("You need to be standing on the ground in order to forage!")
        }
        (110, 56_542_450) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("{a} was unaffected by your {b}.")
        }
        (110, 97_227_364) => {
            String::from("Items must be unequipped before they can be deleted!")
        }
        (110, 204_261_150) => {
            String::from("You need the Alien Invasion expansion pack to access this bonus.")
        }
        (110, 248_158_311) => {
            String::from("Target must be sitting on ground in order to perform this action.")
        }
        (110, 174_062_400) => {
            String::from("No suppression field at all. This is mayhem.")
        }
        (110, 262_909_045) => {
            String::from("Target is unable to receive the effect!")
        }
        (110, 227_119_501) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("You countered {a}'s attempt to run {b} within your NCU.")
        }
        (110, 202_439_743) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            let e = {
                if d.to_string() == "1" {
                    " lock. "
                } else {
                    " locks."
                }
            };
            format!("{a} ({b}:{c}) has {d} skill {e}")
        }
        (110, 100_643_845) => {
            String::from("Suppression field at 75%. Wildlife may attack you.")
        }
        (110, 266_407_541) => {
            String::from("Stuck-resolve command executing. New /stuck command will be available in 3 minutes.")
        }
        (110, 46_620_156) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("You hit {a} for {b} points of {c} damage.")
        }
        (110, 8_554_146) => {
            String::from("You have accepted your targets offer of surrender, and are thus not allowed to attack it again.")
        }
        (110, 6_793_475) => {
            let a = &arguments[0];
            let b = {
                if a.to_string() == "1" {
                    "credit "
                } else {
                    " credits"
                }
            };
            format!("You received {a} {b} from the corpse.")
        }
        (110, 43_985_445) => {
            let a = &arguments[0];
            format!("You have already got this unique item: {a}")
        }
        (110, 76_013_323) => {
            String::from("This action requires that you are wielding Izgimmer-modified Cyberdeck!")
        }
        (110, 264_498_178) => {
            String::from("Player character")
        }
        (110, 8_161_459) => {
            let a = &arguments[0];
            format!("You were damaged by a toxic substance for {a} points of damage.")
        }
        (110, 56_163_172) => {
            String::from("/team loot")
        }
        (110, 198_887_917) => {
            String::from("You can't split this item.")
        }
        (110, 261_570_713) => {
            let a = &arguments[0];
            format!("Maximum increase in ability is {a} per level!")
        }
        (110, 96_829_143) => {
            String::from("Can't add fixture to building.")
        }
        (110, 43_710_079) => {
            String::from("Item can't be used as ammo.")
        }
        (110, 104_633_444) => {
            String::from("Your vote has been registered!")
        }
        (110, 6_523_623) => {
            String::from("Neither you nor target can be fighting!")
        }
        (110, 158_271_107) => {
            String::from("You must be able to use weapons!")
        }
        (110, 3_642_644) => {
            String::from("You can not send an item that is being equipped.")
        }
        (110, 68_917_625) => {
            String::from("You can assemble these two items, but they will be of poor quality.")
        }
        (110, 16_191_029) => {
            let a = &arguments[0];
            format!("{a} contain items. Remove the items first.")
        }
        (110, 55_017_541) => {
            String::from("Stuck-resolve command not allowed for players that cannot move.")
        }
        (110, 149_657_844) => {
            String::from("You can't attack this target.")
        }
        (110, 99_318_165) => {
            String::from("You cannot drop a unique item on the ground.")
        }
        (110, 147_670_715) => {
            let a = &arguments[0];
            format!("You parried the attack from {a}!")
        }
        (110, 266_056_324) => {
            String::from("No room in your inventory. Item dropped on ground.")
        }
        (110, 22_751_484) => {
            let a = &arguments[0];
            format!("You increased your nanopool with {a} points.")
        }
        (110, 114_804_363) => {
            String::from("Items can't be used directly from the bank!")
        }
        (110, 24_180_037) => {
            String::from("You can only vote once!")
        }
        (110, 263_631_788) => {
            String::from("/team loot [all/leader/alpha]")
        }
        (110, 213_884_439) => {
            String::from("You stop bluffing")
        }
        (110, 23_698_185) => {
            let a = &arguments[0];
            format!("Attacked by {a}!")
        }
        (110, 215_187_916) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Need additional skill for combine: {a} {b}")
        }
        (110, 93_068_324) => {
            String::from("Glancing hit.")
        }
        (110, 129_169_355) => {
            String::from("You will not gain Shadowknowledge until you have made your final choice of side!")
        }
        (110, 68_111_125) => {
            String::from("Cash and C.O.D. must be positive values.")
        }
        (110, 124_673_902) => {
            String::from("You need to have a pistol weapon equipped in your right hand.")
        }
        (110, 238_526_887) => {
            String::from("Nanobots are recharging, please wait.")
        }
        (110, 69_391_868) => {
            String::from("Can only stack items of the same level and type.")
        }
        (110, 151_378_752) => {
            String::from("Do you wish to reset ownership of this shop?")
        }
        (110, 228_010_023) => {
            String::from("Somebody is already looting these remains.")
        }
        (110, 254_767_059) => {
            String::from("You can't loot NoDrop items!")
        }
        (110, 129_216_898) => {
            String::from("You have no service tower.")
        }
        (110, 61_283_733) => {
            String::from("You died of reflect shield damage!")
        }
        (110, 101_968_183) => {
            String::from("NCU error: Better nano program already running.")
        }
        (110, 155_854_693) => {
            let a = &arguments[0];
            format!("A player hit you for {a} points of damage.")
        }
        (110, 192_984_696) => {
            String::from("You have reached your max for this title.")
        }
        (110, 265_762_094) => {
            String::from("Target is in a district with higher suppression. Using rules of that district.")
        }
        (110, 230_773_561) => {
            String::from("You must empty your reclaim before you can save! Type \"/reclaim\" to access your items.\r\n")
        }
        (110, 226_232_628) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("{a} IP have been returned to you for the reset of {b}!")
        }
        (110, 180_281_076) => {
            String::from("The duel challenge was retracted.")
        }
        (110, 190_685_838) => {
            String::from("You feel ok again. All residues from being resurrected are gone.")
        }
        (110, 144_211_586) => {
            String::from("Stationary item too far away.")
        }
        (110, 39_731_172) => {
            String::from("Equipment tabs are locked.")
        }
        (110, 150_782_277) => {
            let a = &arguments[0];
            format!("Target sex must be {a}!")
        }
        (110, 96_374_740) => {
            String::from("The minimum amount of credits allowed is 4000.")
        }
        (110, 179_548_020) => {
            String::from("Chest contains a NoDrop item, and can't be dropped.")
        }
        (110, 226_507_011) => {
            String::from("You must not be able to use weapons!")
        }
        (110, 34_558_004) => {
            String::from("Repair roll failed.")
        }
        (110, 80_908_221) => {
            String::from("You unlocked the item.")
        }
        (110, 87_316_052) => {
            String::from("No combination found!")
        }
        (110, 139_675_988) => {
            let a = &arguments[0];
            format!("Player {a} went link dead.")
        }
        (110, 58_096_372) => {
            String::from("Vehicles are prohibited in this area.\r\n")
        }
        (110, 2_537_906) => {
            String::from("Unable to fit item in container.")
        }
        (110, 46_677_479) => {
            String::from("You can't reset skills while equipping items!")
        }
        (110, 226_758_292) => {
            String::from("You can not send an item that is being equipped.")
        }
        (110, 52_237_334) => {
            String::from("Cant find last concrete playfield location for receiver.")
        }
        (110, 85_898_988) => {
            String::from("You can not order a vehicle while dueling.")
        }
        (110, 96_676_609) => {
            String::from("You don't meet the criteria for joining this organization!")
        }
        (110, 63_630_372) => {
            let a = &arguments[0];
            format!("You have been detected by {a}!")
        }
        (110, 71_761_708) => {
            String::from("Your opponent rejected your challenge.")
        }
        (110, 60_420_421) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            let e = &arguments[4];
            let f = &arguments[5];
            format!("{a} looted <a href=\"itemref://{b}/{c}/{d}\">{e}</a> from {f}.")
        }
        (110, 103_442_423) => {
            String::from("You can only bluff while walking!")
        }
        (110, 130_263_283) => {
            String::from("Your breed can't have more professions!")
        }
        (110, 53_232_989) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            let e = &arguments[4];
            let f = &arguments[5];
            format!("The server has randomly assigned loot rights of <a href=\"itemref://{a}/{b}/{c}\">{d}</a> from {e} to {f}")
        }
        (110, 238_610_964) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("You were hit for {a} points of damage by {b}'s reflect shield.")
        }
        (110, 98_781_741) => {
            String::from("This is a NoDrop item - it can't be dropped.")
        }
        (110, 120_316_341) => {
            let a = &arguments[0];
            format!("{a} has challenged you to a duel.")
        }
        (110, 231_194_948) => {
            String::from("You have completed the first specialization!")
        }
        (110, 89_028_179) => {
            String::from("Shades can't use implants!")
        }
        (110, 154_183_844) => {
            String::from("You caused trivial drops to be removed from the kill.")
        }
        (110, 89_732_308) => {
            String::from("Your summon failed because the area is currently too crowded.")
        }
        (110, 227_657_197) => {
            String::from("Nickname problem:")
        }
        (110, 85_312_665) => {
            String::from("The message you are trying to preform this operation on is busy. Please try again in a moment.")
        }
        (110, 140_211_165) => {
            String::from("Duels are not allowed between players in teams.")
        }
        (110, 36_693_582) => {
            String::from("You need to have a rifle weapon equipped.")
        }
        (110, 40_687_125) => {
            let a = &arguments[0];
            format!("You need at least {a} in Multiple Melee to wield these two weapons.")
        }
        (110, 191_940_452) => {
            String::from("You're not allowed to loot these remains!")
        }
        (110, 265_992_564) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            let e = &arguments[4];
            format!("{a} has played {b} days {c} hours {d} minutes {e} seconds")
        }
        (110, 205_676_242) => {
            String::from("You need to be a team leader to perform this action!")
        }
        (110, 241_016_932) => {
            String::from("It was not possible to add this item to the contract!")
        }
        (110, 189_398_684) => {
            String::from("Repair roll successful.")
        }
        (110, 72_805_276) => {
            String::from("You can't reset this skill! You need to have spent some IP on the skill first.")
        }
        (110, 100_644_085) => {
            String::from("Suppression field at 25%. You are in the political zone.")
        }
        (110, 138_728_324) => {
            String::from("You have not been challenged to a pet duel.")
        }
        (110, 210_762_644) => {
            String::from("You or your allies must be in combat.")
        }
        (110, 218_986_573) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("You were attacked with nanobots from {a} for {b} points of {c} damage.")
        }
        (110, 250_504_978) => {
            String::from("You must carry the container in order to operate on the items inside it!")
        }
        (110, 30_581_968) => {
            let a = &arguments[0];
            format!("You gained {a} new Alien Experience Points.")
        }
        (110, 213_562_708) => {
            String::from("Your character was saved (GM message only).")
        }
        (110, 227_798_683) => {
            String::from("This action requires that you are wielding your profession's Nanodeck!")
        }
        (110, 68_153_662) => {
            String::from("Have fun :-)")
        }
        (110, 69_853_428) => {
            let a = &arguments[0];
            let b = {
                if a.to_string() == "1" {
                    " second. "
                } else {
                    " seconds."
                }
            };
            format!("Suppression gas will take effect in {a} {b}")
        }
        (110, 83_559_874) => {
            String::from("This action requires that you are wearing Grid Armor!")
        }
        (110, 3_692_693) => {
            String::from("You are already in a trade.")
        }
        (110, 160_782_964) => {
            let a = &arguments[0];
            format!("\"/org leave\" command not completed. You remain a member of the organization \"{a}\".")
        }
        (110, 164_980_407) => {
            String::from("CHAIN DEATH SYSTEM MESSAGE: Anarchy Online has a special \"chain death\" avoidance system.  If your character dies very quickly more than twice, the system assumes that you have been chain killed.  You will now be placed in a safe location.")
        }
        (110, 258_789_332) => {
            String::from("You were unable to attack the target!")
        }
        (110, 64_971_876) => {
            String::from("The door is now unlocked.")
        }
        (110, 37_112_101) => {
            String::from("Your inventory is full. Can't complete trade!")
        }
        (110, 116_451_874) => {
            String::from("You need to select a team member first!")
        }
        (110, 101_527_576) => {
            let a = &arguments[0];
            format!("If this item is combined with a: {a}")
        }
        (110, 226_192_526) => {
            String::from("You need to have a 2 hand blunt weapon equipped in your right hand.")
        }
        (110, 231_650_788) => {
            String::from("No target to assist.")
        }
        (110, 248_735_703) => {
            String::from("Your inventory and bank is full. An item is put where you are standing.")
        }
        (110, 37_657_666) => {
            String::from("Target must be player character.")
        }
        (110, 5_179_231) => {
            let a = &arguments[0];
            format!("If you deduct these two from the total normal IP mentioned above, you will receive {a} additional IP points.")
        }
        (110, 8_802_756) => {
            let a = &arguments[0];
            format!("{a} vanished.")
        }
        (110, 61_896_768) => {
            String::from("You succeded disarming the trap!.")
        }
        (110, 46_513_671) => {
            String::from("It is not possible to assemble those two items. Maybe the order was wrong?")
        }
        (110, 58_683_275) => {
            String::from("Sneak Attack!")
        }
        (110, 49_654_165) => {
            String::from("You are unable to sneak while controlling a vehicle.")
        }
        (110, 53_289_259) => {
            String::from("This computer deck slot is not available.")
        }
        (110, 204_934_244) => {
            String::from("Unable to find target item?")
        }
        (110, 109_086_469) => {
            let a = &arguments[0];
            format!("You were hit for {a} points of damage.")
        }
        (110, 261_532_677) => {
            String::from("You can't attack while in this state.")
        }
        (110, 253_351_854) => {
            String::from("This shopping booth has been locked down and can not be used until it has been unlocked or reset. Please try another booth.")
        }
        (110, 12_306_981) => {
            String::from("You died of fall damage!")
        }
        (110, 51_912_388) => {
            String::from("Critical hit!")
        }
        (110, 247_124_532) => {
            let a = &arguments[0];
            format!("You successfully bound {a} teammember(s) to this location.")
        }
        (110, 263_200_964) => {
            String::from("You are not allowed to attack this target.")
        }
        (110, 221_741_460) => {
            String::from("Fear activated.")
        }
        (110, 6_039_988) => {
            String::from("Only the leader of the organization owning this market can set the rent on the shop!")
        }
        (110, 19_661_557) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Someone absorbed {a} points of {b} damage.")
        }
        (110, 134_742_360) => {
            String::from("Dimach!")
        }
        (110, 207_248_100) => {
            String::from("You can't log out while in a fight.")
        }
        (110, 266_514_407) => {
            String::from("Your opponent has proposed a draw. Type /duel draw to accept.")
        }
        (110, 24_828_611) => {
            String::from("Fear has no impact on players.")
        }
        (110, 229_499_317) => {
            let a = &arguments[0];
            format!("You were challenged by {a} to a pet fight.\r\nWrite \"/petduel accept\" to accept the challenge.\r\nOr write \"/petduel reject\" to reject.")
        }
        (110, 190_747_645) => {
            let a = &arguments[0];
            format!("Unable to create mission: {a} was not in the team when the mission was created.")
        }
        (110, 212_805_180) => {
            String::from("You just wasted an item.")
        }
        (110, 70_098_404) => {
            let a = &arguments[0];
            format!("Character stored. {a} Shadowknowledge saved.")
        }
        (110, 134_429_625) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("You need following skill to reverse engineer: {a} {b}")
        }
        (110, 46_155_422) => {
            String::from("Executing programs here is forbidden.")
        }
        (110, 120_303_419) => {
            String::from("You cannot trade items directly from your bank.")
        }
        (110, 179_854_371) => {
            String::from("Shop contains no entries.")
        }
        (110, 263_188_032) => {
            String::from("The owner of the shop is accessing the shop. Please wait.")
        }
        (110, 102_894_949) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("You were attacked with nanobots for {a} points of {b} damage.")
        }
        (110, 262_030_676) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = {
                if a.to_string() == "1" {
                    " item "
                } else {
                    " items"
                }
            };
            let d = {
                if a.to_string() == "1" {
                    " credit "
                } else {
                    " credits"
                }
            };
            format!("(Found {a} {c} and {b} {d} in the shop.)")
        }
        (110, 97_926_018) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Your reflect shield hit {a} for {b} points of damage.")
        }
        (110, 186_942_868) => {
            String::from("You can't fight at the moment!")
        }
        (110, 23_247_575) => {
            String::from("mission building")
        }
        (110, 183_823_060) => {
            String::from("Recipient name must be a minimum of 3 characters, and subject must a minimum of 5 characters.")
        }
        (110, 253_883_112) => {
            let a = &arguments[0];
            format!("Received mission reward: Cash {a}")
        }
        (110, 86_550_761) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("Item: {a} level: {b} price: {c}")
        }
        (110, 41_932_069) => {
            String::from("Target is outside range.")
        }
        (110, 42_865_918) => {
            String::from("You need to have a 1 hand edged weapon equipped.")
        }
        (110, 245_559_163) => {
            let a = &arguments[0];
            let b = {
                if a.to_string() == "1" {
                    " credit "
                } else {
                    " credits"
                }
            };
            format!("You have received {a} {b} from your organization bank.")
        }
        (110, 222_748_692) => {
            String::from("Your summon target has not been bound on this playfield.")
        }
        (110, 219_247_493) => {
            String::from("data error playfieldtype=0?..")
        }
        (110, 128_345_015) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("{a} must have a {b} running!")
        }
        (110, 113_877_645) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Item at: {a} {b}.")
        }
        (110, 62_593_236) => {
            String::from("Warning! Trap detected.")
        }
        (110, 132_067_749) => {
            String::from("Unable to get the source item, at all!")
        }
        (110, 56_094_733) => {
            String::from("Target already contains this unique item.")
        }
        (110, 178_879_861) => {
            String::from("It is not possible to combine these two items in that order.")
        }
        (110, 182_414_030) => {
            String::from("Target can't be performing an item action while receiving this buff!")
        }
        (110, 98_743_044) => {
            String::from("Deck items must be unequipped before they can be deleted!")
        }
        (110, 185_579_892) => {
            String::from("Maps options/map already uploaded.")
        }
        (110, 204_742_267) => {
            String::from("Mail attachments must be in your main inventory to be sent.")
        }
        (110, 154_030_303) => {
            String::from("Stuck-resolve command not allowed while affected by hostile nano programs.")
        }
        (110, 163_549_107) => {
            String::from("Nanobots are busy. Aborting previous nano program.")
        }
        (110, 181_398_620) => {
            String::from("Bank is full.")
        }
        (110, 56_341_172) => {
            String::from("There is nothing you could loot inside.")
        }
        (110, 222_821_742) => {
            String::from("You need to have a 2 hand edged weapon equipped in your right hand.")
        }
        (110, 231_617_463) => {
            String::from("CharNameState hit default. BUG")
        }
        (110, 6_690_270) => {
            String::from("You no longer meet the criteria for beeing a member of this organization!")
        }
        (110, 113_022_661) => {
            String::from("Spawn to other playfield is activated.")
        }
        (110, 26_209_284) => {
            String::from("Duel challenge accepted!")
        }
        (110, 116_350_327) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("{a} must not have {b} running!")
        }
        (110, 241_356_222) => {
            String::from("You need to have a pistol weapon equipped.")
        }
        (110, 113_321_884) => {
            let a = &arguments[0];
            format!("{a} accepted your challenge.")
        }
        (110, 46_864_357) => {
            String::from("Already in use by someone else. Please wait your turn.")
        }
        (110, 251_706_116) => {
            String::from("Target")
        }
        (110, 143_452_319) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("{a} can run the nano: {b}!")
        }
        (110, 22_016_194) => {
            String::from("This action requires that you are not wearing Grid Armor!")
        }
        (110, 162_192_643) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            let e = &arguments[4];
            let f = &arguments[5];
            let g = &arguments[6];
            format!("{a}: Health: {b}%  Nano: {c}%  NCU: {d}/{e}    Position: {f},{g}\r\n")
        }
        (110, 17_845_059) => {
            String::from("You need the Shadowlands expansion pack to access Perks.")
        }
        (110, 171_807_383) => {
            String::from("You cannot unlearn a Perk you do not know!")
        }
        (110, 205_510_916) => {
            String::from("Only organization leaders can remove items from the contract.")
        }
        (110, 242_222_983) => {
            String::from("Wearing is disabled when you have items in your overflow!")
        }
        (110, 214_344_940) => {
            String::from("You have assigned XP to global research, but do not currently have a research goal.")
        }
        (110, 4_338_460) => {
            String::from("Inventory full. Please make room in your inventory first!")
        }
        (110, 72_928_841) => {
            String::from("You can not send temporary items to the market.")
        }
        (110, 15_198_999) => {
            String::from("Your target is in the same organization as you")
        }
        (110, 101_851_753) => {
            String::from("Treasure-spawn to inventory is ILLEGAL.")
        }
        (110, 113_966_673) => {
            let a = &arguments[0];
            format!("You were healed for {a} points of nano delta.")
        }
        (110, 50_179_253) => {
            String::from("Target is outside range for trade. If you want to trade with the target, move closer!")
        }
        (110, 66_434_430) => {
            let a = &arguments[0];
            format!("Team-mission chance of token reward upped to {a}% due to the team's heroic effort.")
        }
        (110, 7_658_105) => {
            let a = &arguments[0];
            format!("This effect can only be utilitized by {a}.")
        }
        (110, 28_954_583) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Your nano program {a} has stopped running on {b}...")
        }
        (110, 254_707_413) => {
            String::from("No skill on this combination!")
        }
        (110, 147_716_638) => {
            String::from("You need to have a grenade weapon equipped.")
        }
        (110, 165_237_059) => {
            String::from("Do you wish to transfer all credits from the shop?")
        }
        (110, 133_176_132) => {
            String::from("ShopNet: No entries found!")
        }
        (110, 36_900_067) => {
            let a = &arguments[0];
            format!("You were healed for {a} points.")
        }
        (110, 205_979_571) => {
            String::from("Available team commands:")
        }
        (110, 216_734_035) => {
            let a = &arguments[0];
            format!("Item deleted at pos {a}.")
        }
        (110, 127_970_254) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("{a} {b} is not less than {c}!")
        }
        (110, 105_140_873) => {
            String::from("No room in your inventory.")
        }
        (110, 46_118_322) => {
            String::from("The quality of the reverse engineered items are poor.")
        }
        (110, 144_934_005) => {
            String::from("You're not authorized to use this voucher. Ensure you have the city terminal interface open and are permitted to purchase the city district.")
        }
        (110, 88_088_781) => {
            String::from("You need a selected target for this program!")
        }
        (110, 88_285_237) => {
            String::from("You only gain Shadowknowledge from unsided actions or opposing side kills!")
        }
        (110, 26_366_276) => {
            String::from("Reloading completed")
        }
        (110, 169_479_550) => {
            String::from("You need at least 2 free slots to receive mission reward. Mission completion is pending.")
        }
        (110, 80_879_008) => {
            String::from("Shop for Rent")
        }
        (110, 160_942_668) => {
            String::from("Nano execution aborted. Target already has a higher level program running.")
        }
        (110, 130_032_452) => {
            String::from("Target is already dead.")
        }
        (110, 76_809_564) => {
            let a = &arguments[0];
            format!("New Level: {a}!")
        }
        (110, 228_084_612) => {
            String::from("Can't carry that.")
        }
        (110, 185_398_382) => {
            String::from("You need to have a bow weapon equipped in your right hand.")
        }
        (110, 20_247_365) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("{a} {b} base value is required to be at least {c}!")
        }
        (110, 220_022_439) => {
            String::from("You can not do this while flying.")
        }
        (110, 147_918_301) => {
            String::from("You need to be in a team to perform this action!")
        }
        (110, 97_364_923) => {
            let a = &arguments[0];
            format!("{a} parried your attack!")
        }
        (110, 54_881_700) => {
            String::from("Your deck must be empty before it can be unequipped!")
        }
        (110, 193_454_836) => {
            String::from("Your opponent does not have a duel pet spawned.")
        }
        (110, 1_675_284) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("ShopNet: Too many entries found ({a}). {b} entries displayed:")
        }
        (110, 28_885_080) => {
            let a = &arguments[0];
            format!("(GM feedback) This savebooth has an invalid cash ({a})!")
        }
        (110, 90_052_988) => {
            String::from("Your inventory is full!")
        }
        (110, 132_892_244) => {
            String::from("Invalid tower command.")
        }
        (110, 101_485_589) => {
            let a = &arguments[0];
            format!("You have completed your research on \"{a}\".")
        }
        (110, 176_657_492) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("You healed {a} for {b} points of health.")
        }
        (110, 138_115_844) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Something hit {a} for {b} points of damage by reflect shield.")
        }
        (110, 150_668_261) => {
            String::from("Wearing items is forbidden here.")
        }
        (110, 184_471_956) => {
            String::from("Unable to execute nano program. You can't execute this nano on the target.")
        }
        (110, 208_051_539) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("You have gained {a} new Reset Points. Your total Reset Point count is: {b}")
        }
        (110, 41_643_150) => {
            String::from("You need to have a 1 hand blunt weapon equipped in your right hand.")
        }
        (110, 176_711_820) => {
            let a = &arguments[0];
            format!("You have challenged {a} to a pet fight.")
        }
        (110, 29_631_970) => {
            String::from("Item-spawn to inventory failed cause we did not find receiver.")
        }
        (110, 8_605_508) => {
            String::from("Target is not in line of sight!")
        }
        (110, 13_964_516) => {
            String::from("Aimed Shot!")
        }
        (110, 20_556_461) => {
            String::from("You must be standing up to execute a nano program.")
        }
        (110, 186_105_556) => {
            String::from("You can't break hostile nanoprograms on yourself!")
        }
        (110, 40_747_948) => {
            String::from("You cannot attack this tower since you are too far away in level to the tower.")
        }
        (110, 108_871_108) => {
            String::from("You've received an item as mission reward!")
        }
        (110, 223_146_229) => {
            String::from("Delayed spawn is activated.")
        }
        (110, 214_890_029) => {
            String::from("Wait for current nano program execution to finish.")
        }
        (110, 247_700_080) => {
            let a = &arguments[0];
            format!("You received {a} xp.")
        }
        (110, 185_395_998) => {
            String::from("You need to have a SMG weapon equipped in your right hand.")
        }
        (110, 186_610_068) => {
            String::from("Daze activated.")
        }
        (110, 93_806_252) => {
            String::from("Nano program failed. Too low level.")
        }
        (110, 186_191_623) => {
            String::from("A trap was sprung! You failed disarming the trap!.")
        }
        (110, 212_310_624) => {
            String::from("The shop was altered by the owner. Trade aborted.\r\n")
        }
        (110, 54_455_863) => {
            String::from("The target must be in your organization!")
        }
        (110, 31_879_876) => {
            String::from("Warning - Complimentary insurance buffer has been terminated. From now on, all experience gained since last insurance payment will be lost upon death. Please, for the good of your future, remember to insure yourself regularly.")
        }
        (110, 164_914_276) => {
            String::from("Nickname is not accepted!")
        }
        (110, 98_660_718) => {
            String::from("Item can't be dropped at that location!")
        }
        (110, 19_550_189) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            let e = &arguments[4];
            format!("{a} was attacked with {b} from {c} for {d} points of {e} damage.")
        }
        (110, 125_843_573) => {
            String::from("This item requires a hostile target to be applied on.")
        }
        (110, 14_986_599) => {
            String::from("Already reloading. Please wait.")
        }
        (110, 88_098_005) => {
            String::from("You can not use this while controlling a vehicle.")
        }
        (110, 225_834_830) => {
            String::from("Player is already in an organization.")
        }
        (110, 47_654_467) => {
            String::from("You were successfully bound to this location.")
        }
        (110, 22_330_617) => {
            String::from("Items must reside in the same inventory.")
        }
        (110, 263_916_277) => {
            String::from("The two items are the same item!")
        }
        (110, 7_184_644) => {
            let a = &arguments[0];
            format!("This Shadowknowledge was added to the pool of unsaved experience points that you get back through gaining new experience points. Your current pool of unsaved experience is {a}.")
        }
        (110, 154_098_633) => {
            String::from("You can not send temporary items through the mail system.")
        }
        (110, 264_534_194) => {
            String::from("You need to be an organization leader to perform this action!")
        }
        (110, 18_548_276) => {
            String::from("PvP grace period has ended.")
        }
        (110, 202_746_628) => {
            String::from("Team member must be present!")
        }
        (110, 267_524_032) => {
            String::from("This is now an open shop! (open for all)")
        }
        (110, 124_989_336) => {
            String::from("Absolute ability maximum for your breed has been reached!")
        }
        (110, 233_275_556) => {
            String::from("This item can't be traded!")
        }
        (110, 220_669_556) => {
            String::from("Combine failed!")
        }
        (110, 172_693_509) => {
            let a = &arguments[0];
            format!("You have gained a new Alien Title: {a}!")
        }
        (110, 146_762_345) => {
            String::from("You can assemble these two items with ok quality results.")
        }
        (110, 53_932_440) => {
            let a = &arguments[0];
            format!("Spawns performed: {a}")
        }
        (110, 7_708_509) => {
            String::from("You're not a member of the looting team!")
        }
        (110, 227_312_414) => {
            String::from("You need to have a 2 hand blunt weapon equipped.")
        }
        (110, 234_603_812) => {
            let a = &arguments[0];
            format!("This terminal can only be rented by members of {a}!")
        }
        (110, 31_032_772) => {
            String::from("The target is not accepting duel challenges.")
        }
        (110, 138_451_742) => {
            String::from("You need the Alien Invasion expansion pack to access this perk.\r\n")
        }
        (110, 62_070_433) => {
            String::from("This item can only be used in a landcontrol area of your organization.")
        }
        (110, 51_152_356) => {
            String::from("Your inspect request was rejected.")
        }
        (110, 142_285_296) => {
            String::from("You must sit in order to use /lounge or /sleep.")
        }
        (110, 108_584_146) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("You got healed by {a} for {b} points of health.")
        }
        (110, 212_592_180) => {
            String::from("All items have been reclaimed!")
        }
        (110, 25_405_660) => {
            String::from("You have assigned XP to research, but do not currently have a research goal.")
        }
        (110, 243_180_281) => {
            String::from("Move item to inventory before using it.")
        }
        (110, 112_527_513) => {
            String::from("This item can only reside in your main inventory!")
        }
        (110, 130_706_571) => {
            String::from("Temporary items can't be moved into the bank!")
        }
        (110, 65_351_820) => {
            String::from("Unable to execute nano program. You must stand still.")
        }
        (110, 138_036_463) => {
            let a = &arguments[0];
            format!("You drained {a} points of nano from the target.")
        }
        (110, 95_202_091) => {
            String::from("Items being sent to the market must be in your main inventory.")
        }
        (110, 78_100_660) => {
            String::from("All items must be unequipped before performing a skill reset!")
        }
        (110, 9_823_011) => {
            String::from("You can't wear stacked items.")
        }
        (110, 183_230_340) => {
            let a = &arguments[0];
            format!("Nickname is accepted as '{a}'")
        }
        (110, 39_591_797) => {
            String::from("You can not heal this target because the level difference is too large.")
        }
        (110, 214_489_629) => {
            String::from("No room for more Perks!")
        }
        (110, 220_064_249) => {
            String::from("This item can't be used directly. Use it on another item to boobytrap it.")
        }
        (110, 160_396_958) => {
            String::from("You need to have an unarmed combat weapon equipped.")
        }
        (110, 47_797_998) => {
            String::from("The target must have the Alien Invasion expansion pack!")
        }
        (110, 236_876_551) => {
            String::from("Cannot follow selected target.")
        }
        (110, 140_239_672) => {
            String::from("Duels are not allowed between players in vehicles.")
        }
        (110, 240_455_300) => {
            String::from("Please wait.")
        }
        (110, 92_604_412) => {
            String::from("Swapping items between wear inventories and social tab is not allowed.")
        }
        (110, 157_974_308) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Received mission reward: XP {a} Cash {b}")
        }
        (110, 26_345_748) => {
            String::from("Do you wish to permanently commit to your side? (You can no longer change side after reaching level 201!)")
        }
        (110, 163_173_060) => {
            let a = &arguments[0];
            format!("{a} is inspecting your equipment.")
        }
        (110, 220_207_812) => {
            String::from("Fling Shot!")
        }
        (110, 99_077_476) => {
            String::from("The specified mail message was not found.")
        }
        (110, 178_397_477) => {
            String::from("Can't attack. Target is still in PvP grace period.")
        }
        (110, 253_228_628) => {
            String::from("Draw proposed. Waiting for reply.")
        }
        (110, 88_620_286) => {
            String::from("The target must have the Lost Eden expansion pack!")
        }
        (110, 59_157_820) => {
            String::from("Item must be applied on self while dueling.")
        }
        (110, 114_064_052) => {
            String::from("evaded")
        }
        (110, 158_781_765) => {
            String::from("Not allowed to execute friendly nanoprogram on anyone on the opposite side of your team in the conflict.")
        }
        (110, 39_240_084) => {
            String::from("You cannot attack the tower -- the defense shield is enabled.  Disable it by using a defense shield disabler item near the tower.  (You don't have to do this if your organization has a controller tower.)")
        }
        (110, 230_632_366) => {
            String::from("You need to have a rifle weapon equipped in your right hand.")
        }
        (110, 87_026_963) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("{a} can loot these remains. (Loot order: {b})")
        }
        (110, 239_029_653) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("{a} must have enough free nano controlling units (NCU) to have {b} running!")
        }
        (110, 200_449_475) => {
            String::from("Please note that there might be some very minor rounding errors, but they should not amount to more than 5-10 IP total. Please also note that some people have more IP than their normal total IP, because of special items yielding additional IP.")
        }
        (110, 160_619_144) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = {
                if b.to_string() == "1" {
                    "credit "
                } else {
                    " credits"
                }
            };
            format!("Received reward: {a} XP, {b} {c}.")
        }
        (110, 94_618_563) => {
            String::from("To delete a bag, you must remove all contents.")
        }
        (110, 92_583_404) => {
            let a = &arguments[0];
            format!("{a} is full.")
        }
        (110, 162_097_758) => {
            String::from("You need to have a heavy weapon equipped in your right hand.")
        }
        (110, 81_344_862) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Playshifting failed: The server was unable to start the mission building. ( {a}, {b} )")
        }
        (110, 120_692_516) => {
            let a = &arguments[0];
            format!("For you to receive the full reward from this quest, you can not be grouped with players above level {a} while solving the quest or when handing in the quest result.")
        }
        (110, 204_307_477) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("You have to kill {a} more {b} to fulfill one of your missions!")
        }
        (110, 47_585_139) => {
            String::from("You are unable to gain a new level with tradeskills. You need to get the last experience points in some other way.")
        }
        (110, 175_101_176) => {
            let a = &arguments[0];
            format!("You cannot use letters ( {a} ) in this command.")
        }
        (110, 83_514_020) => {
            String::from("Stuck-resolve command not allowed while one or more pet is in combat.")
        }
        (110, 112_228_263) => {
            String::from("Base abilities too low to increase skill more.")
        }
        (110, 59_137_061) => {
            let a = &arguments[0];
            format!("Target side must not be {a}!")
        }
        (110, 113_786_558) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("{a} was born on {b}")
        }
        (110, 222_924_355) => {
            let a = &arguments[0];
            format!("{a} contain items. Remove the items first.")
        }
        (110, 126_160_643) => {
            String::from("You must not be in the Shadowlands in order to use this!")
        }
        (110, 59_824_862) => {
            String::from("Wait for current program activation to finish.")
        }
        (110, 239_322_891) => {
            String::from("Fast attack!")
        }
        (110, 112_569_780) => {
            String::from("dodged")
        }
        (110, 242_204_274) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            format!("Weapon {a} : {b} : {c} : {d}")
        }
        (110, 260_013_981) => {
            String::from("You locked the item.")
        }
        (110, 55_820_024) => {
            String::from("Target can not be in a vehicle.")
        }
        (110, 208_928_804) => {
            String::from("NoDrop item can't be added to a container on ground!")
        }
        (110, 106_497_456) => {
            let a = &arguments[0];
            format!("All in all, you now have {a} IP points.")
        }
        (110, 145_329_275) => {
            String::from("The quality of the new item is ok.")
        }
        (110, 229_248_035) => {
            String::from("This can only be performed in the Shadowlands!")
        }
        (110, 165_509_237) => {
            String::from("The target is outside special attack range!")
        }
        (110, 80_040_421) => {
            String::from("Nickname 'undefined' failure!")
        }
        (110, 23_121_860) => {
            String::from("Team member went link dead.")
        }
        (110, 73_584_804) => {
            String::from("Disarming the trap failed!")
        }
        (110, 149_035_951) => {
            String::from("/org vote <voteentry id> | info")
        }
        (110, 206_073_989) => {
            String::from("You died of weapon damage!")
        }
        (110, 73_185_573) => {
            String::from("The team did too little damage to get any xp from this kill!")
        }
        (110, 167_200_732) => {
            String::from("No Shadowknowledge is learned! You are not in the Shadowlands!")
        }
        (110, 38_275_621) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("{a} took {b} points of fall damage.")
        }
        (110, 98_398_109) => {
            let a = &arguments[0];
            format!("Executing Nano Program: {a}.")
        }
        (110, 111_501_557) => {
            String::from("Only one vote at a time!")
        }
        (110, 190_680_229) => {
            String::from("You must be in a land control area within your PvP range to perform this action!")
        }
        (110, 112_975_933) => {
            String::from("You are not a member of a team!")
        }
        (110, 58_906_678) => {
            String::from("Must target yourself")
        }
        (110, 176_528_068) => {
            String::from("Item successfully sent to market.")
        }
        (110, 60_207_397) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            format!("{a} was attacked with {b} for {c} points of {d} damage.")
        }
        (110, 154_451_060) => {
            String::from("GM notice: You were unaffected by areaformula!")
        }
        (110, 133_271_472) => {
            let a = &arguments[0];
            format!("This is your private shop (max {a} items)!")
        }
        (110, 264_430_233) => {
            String::from("Resurrection shock fills your body. Stats temporarily reduced.")
        }
        (110, 31_160_628) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            format!("Player {a} profession {b} has {c} in full AMS for {d} with no multiplier.")
        }
        (110, 130_511_913) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("You were unaffected by {a} from {b}.")
        }
        (110, 236_671_044) => {
            String::from("This Perk doesn't exist!")
        }
        (110, 253_549_742) => {
            String::from("Fighting prevents mission checking from being performed. Try to repeat your actions after you finish the fight!")
        }
        (110, 14_793_904) => {
            String::from("This shop can now only be used by characters on this account!")
        }
        (110, 227_193_107) => {
            let a = &arguments[0];
            format!("You gained {a} points of Shadowknowledge as a side bonus.")
        }
        (110, 147_974_487) => {
            String::from("You start sneaking.")
        }
        (110, 264_429_801) => {
            String::from("Your bodily remains have been made available.")
        }
        (110, 14_539_294) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("{a} {b} base value is not less than {c}!")
        }
        (110, 262_436_567) => {
            String::from("You are too high level to forage in this area!")
        }
        (110, 107_998_517) => {
            String::from("Items can't be used directly from the corpse!")
        }
        (110, 131_173_557) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("You gained {a} {b}.")
        }
        (110, 174_062_405) => {
            String::from("Suppression field at 5%. PvP with everyone outside your organization...")
        }
        (110, 53_816_121) => {
            String::from("Entering unclaimed city.")
        }
        (110, 144_839_764) => {
            String::from("Warning - The immediate transfer of personal items to your new body after death has now ceased, and all items will remain with your corpse for 1 minute before being transfered to the item reclaim terminal.")
        }
        (110, 154_793_861) => {
            let a = &arguments[0];
            format!("Someone's reflect shield hit you for {a} points of damage.")
        }
        (110, 13_197_412) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("You were hit for {a} points of damage by {b}'s damage shield.")
        }
        (110, 42_166_683) => {
            String::from("You can't pick this lock!")
        }
        (110, 101_123_118) => {
            String::from("You must have the Alien Invasion expansion!")
        }
        (110, 152_712_420) => {
            String::from("You cannot attack your pet.")
        }
        (110, 117_401_824) => {
            String::from("/org startvote &lt;text&gt; &lt;duration&gt; &lt;voteentries&gt;")
        }
        (110, 235_707_822) => {
            String::from("You need to have an assault rifle weapon equipped in your right hand.")
        }
        (110, 247_249_156) => {
            String::from("You can not do this with a vehicle equipped.")
        }
        (110, 64_711_726) => {
            String::from("This action requires that you have the fourth specialization completed!")
        }
        (110, 65_148_729) => {
            String::from("You are not allowed to execute a friendly nano program on this target here! PvP level restrictions apply on healing/friendly nano programs in Land Control areas.")
        }
        (110, 16_638_438) => {
            String::from("This is a team private playfield.  You are not a member of the team, and will be deported shortly.")
        }
        (110, 139_784_973) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            format!("{a} was attacked with nanobots from {b} for {c} points of {d} damage.")
        }
        (110, 56_144_451) => {
            String::from("Not allowed to attack team-members.")
        }
        (110, 235_094_081) => {
            String::from("Entering new area.")
        }
        (110, 263_676_236) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            format!("It is theoretically possible to combine \"{a}\" with \"{b}\". But you need at least {c} in {d}.")
        }
        (110, 30_588_315) => {
            String::from("The quality of the reverse engineered items are ok.")
        }
        (110, 79_405_833) => {
            String::from("You need the Alien Invasion expansion to use this city terminal.")
        }
        (110, 25_560_336) => {
            String::from("Chest contains a NoDrop item, and can't be traded or sold.")
        }
        (110, 268_034_741) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("The shop \"{a}\" owned by {b} contains the following items:")
        }
        (110, 191_090_750) => {
            let a = &arguments[0];
            format!("Duel starts in {a} seconds.")
        }
        (110, 67_890_820) => {
            String::from("You have no pet.")
        }
        (110, 116_643_813) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("{a} must not have enough free nano controlling units (NCU) to have {b} running!")
        }
        (110, 99_047_202) => {
            String::from("Social armor cannot be worn with other clothes.")
        }
        (110, 182_363_601) => {
            String::from("The target must have the Shadowlands Expansion Pack!")
        }
        (110, 187_295_829) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            format!("Right hand weapon {a} : {b} : {c} : {d}")
        }
        (110, 237_612_305) => {
            String::from("To return the mail, it must either be cash on delivery, or you must already have the unique item. In addition, the mail must not have been returned before.")
        }
        (110, 72_541_812) => {
            let a = &arguments[0];
            format!("ShopNet: {a} entries found:")
        }
        (110, 238_324_800) => {
            let a = &arguments[0];
            format!("{a} doesn't own a shop!")
        }
        (110, 154_478_020) => {
            String::from("You can't use this item now!")
        }
        (110, 213_281_328) => {
            let a = &arguments[0];
            format!("As a result of your death, you just lost some of your Alien Experience ({a}).")
        }
        (110, 252_906_644) => {
            let a = &arguments[0];
            format!("{a} saw through your bluff!")
        }
        (110, 100_862_949) => {
            String::from("You died of shield damage!")
        }
        (110, 202_263_127) => {
            String::from("You must be sitting in order to log out.")
        }
        (110, 194_843_333) => {
            let a = &arguments[0];
            format!("DumpDamage for {a} done.")
        }
        (110, 211_856_692) => {
            String::from("resisted")
        }
        (110, 143_564_917) => {
            String::from("Item spawn to inventory done.")
        }
        (110, 175_658_468) => {
            String::from("You have no line of sight to the target!")
        }
        (110, 116_606_052) => {
            String::from("Starting attack failed.")
        }
        (110, 184_786_807) => {
            String::from("You can't do this while you are falling!")
        }
        (110, 209_219_581) => {
            String::from("Target must not be in a team in order to perform this action.")
        }
        (110, 137_573_278) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("{a} forces your NCU to run {b}...")
        }
        (110, 108_119_101) => {
            String::from("You can't use this nano program at the moment.")
        }
        (110, 160_614_526) => {
            String::from("This action requires that you have the first specialization completed!")
        }
        (110, 168_142_573) => {
            String::from("Can't trade this type of item from container!")
        }
        (110, 55_082_631) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Teleport: {a} : {b}")
        }
        (110, 74_020_435) => {
            String::from("No items in reclaim booth. Try later.")
        }
        (110, 267_025_765) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("You were attacked with {a} for {b} points of {c} damage.")
        }
        (110, 1_491_292) => {
            String::from("PvP not allowed since your team is neutral. Can only attack if attacked first.")
        }
        (110, 57_225_976) => {
            let a = &arguments[0];
            format!("Following {a} ...")
        }
        (110, 158_910_633) => {
            String::from("You received a mission key.")
        }
        (110, 52_404_348) => {
            String::from("Target identity for position relative spawn is NULL.")
        }
        (110, 27_107_026) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("{a}'s reflect shield hit {b} for {c} points of damage.")
        }
        (110, 237_636_556) => {
            String::from("You do not have enough credits to pay for the item in this message.")
        }
        (110, 25_272_340) => {
            String::from("Your attack was blocked by an attack shield!")
        }
        (110, 248_416_728) => {
            let a = &arguments[0];
            format!("Entering city controlled by: {a}")
        }
        (110, 102_719_811) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            format!("You combined \"{a}\" with \"{b}\" and the result is a quality level {c} \"{d}\".")
        }
        (110, 183_539_902) => {
            String::from("You need to have a piercing weapon equipped.")
        }
        (110, 263_625_671) => {
            let a = &arguments[0];
            format!("Team looting set to: {a}")
        }
        (110, 168_362_354) => {
            String::from("Player is already an organization member.")
        }
        (110, 265_781_900) => {
            String::from("Lockpicking successful.")
        }
        (110, 48_462_814) => {
            String::from("You need to have a SMG weapon equipped.")
        }
        (110, 161_382_174) => {
            String::from("Items with a limited timespan can't be sold.")
        }
        (110, 230_434_588) => {
            let a = &arguments[0];
            format!("{a} is already engaged in a pet duel.")
        }
        (110, 231_829_077) => {
            String::from("There are no forageable items in this area!")
        }
        (110, 90_195_273) => {
            String::from("Target slot is not empty.")
        }
        (110, 200_322_372) => {
            String::from("You need to have a duel pet spawned before you can challenge anybody.")
        }
        (110, 257_936_869) => {
            String::from("The molecular structure of this creature prohibits the use of this technology.")
        }
        (110, 259_138_580) => {
            String::from("Combat is not possible in this district.")
        }
        (110, 159_227_364) => {
            let a = &arguments[0];
            format!("Character stored. {a} XP saved.")
        }
        (110, 18_049_691) => {
            String::from("You're unable to attack.")
        }
        (110, 76_015_762) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("Teleport: {a} : {b} : {c}")
        }
        (110, 87_719_948) => {
            String::from("Someone in your team is of too high level for you to request a team mission. You can not share experience with that person...\r\n")
        }
        (110, 193_606_820) => {
            let a = &arguments[0];
            format!("Team member {a} went link dead.")
        }
        (110, 41_593_765) => {
            String::from("The item is not owned by you. Move it into inventory first!")
        }
        (110, 152_477_303) => {
            let a = &arguments[0];
            format!("Attacking {a}...")
        }
        (110, 163_792_286) => {
            String::from("Surface not found, no spawn to surface.")
        }
        (110, 23_166_740) => {
            String::from("This shop has been locked!")
        }
        (110, 135_035_262) => {
            String::from("You will not be rewarded for this mission due to a too high level character in the team.")
        }
        (110, 6_112_365) => {
            String::from("You can't attack this item.")
        }
        (110, 77_592_676) => {
            String::from("You or your allies can not be in combat.")
        }
        (110, 238_998_585) => {
            String::from("Stationary source item too far away!")
        }
        (110, 178_697_301) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("{a} executes {b} within your NCU...")
        }
        (110, 73_097_448) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            format!("You hit {a} with {b} for {c} points of {d} damage.")
        }
        (110, 226_958_237) => {
            String::from("Your target countered the nano program.")
        }
        (110, 26_457_698) => {
            String::from("Use the Def-Agg slider in the Stats view to change between defensive and aggressive.")
        }
        (110, 14_897_763) => {
            String::from("You found no items here!")
        }
        (110, 2_825_822) => {
            String::from("You need to have a long range weapon equipped.")
        }
        (110, 139_890_823) => {
            String::from("You can only place one laser marker per playfield.")
        }
        (110, 209_930_948) => {
            String::from("Burst!")
        }
        (110, 70_232_243) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("You hit {a} with nanobots for {b} points of {c} damage.")
        }
        (110, 143_845_444) => {
            String::from("Your inventory overflowed! Get the items before leaving the area or they will be deleted.")
        }
        (110, 232_197_924) => {
            String::from("Target dynel for position relative spawn is not found.")
        }
        (110, 128_073_058) => {
            String::from("The quality of the new item is poor.")
        }
        (110, 84_220_094) => {
            let a = &arguments[0];
            format!("You do not have enough cash to pay for the organizational upkeep (which is currently {a} credits). You have 30 minutes to acquire the cash, after which you will be suspended if you still cannot pay.")
        }
        (110, 120_958_178) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Your damage shield hit {a} for {b} points of damage.")
        }
        (110, 131_809_364) => {
            String::from("Warning - Complimentary insurance buffer has been reduced. From now on, you will experience resurrection shock following lifeforce transfer to your new body.")
        }
        (110, 205_237_300) => {
            String::from("Target resisted.")
        }
        (110, 27_841_188) => {
            String::from("This mine has already been disarmed.")
        }
        (110, 61_167_522) => {
            String::from("The target must be a controller!")
        }
        (110, 85_243_829) => {
            String::from("Unequipping items is forbidden here.")
        }
        (110, 23_943_115) => {
            String::from("This action requires that you are wielding Basic Cyberdeck!")
        }
        (110, 15_895_888) => {
            String::from("Invalid /org command. Type /org help  in chat to view available commands.")
        }
        (110, 5_087_712) => {
            let a = &arguments[0];
            format!("{a} of your XP were allocated to your personal research.\r\n")
        }
        (110, 191_748_729) => {
            String::from("Stationary target item too far away!")
        }
        (110, 120_670_466) => {
            String::from("Congratulations! You have received a new special attack: Backstab!")
        }
        (110, 196_108_469) => {
            String::from("Player can no longer afford the trade. Trade aborted.")
        }
        (110, 196_159_176) => {
            let a = &arguments[0];
            format!("Must not be in raid with {a}")
        }
        (110, 266_398_756) => {
            String::from("Mission accomplished.")
        }
        (110, 244_601_803) => {
            String::from("Special attack not possible. You must be behind the target.")
        }
        (110, 146_409_321) => {
            String::from("The reverse engineered items are of excellent quality!")
        }
        (110, 171_743_285) => {
            String::from("You are not allowed to enter markets of the opposing side. Evicting.")
        }
        (110, 145_432_389) => {
            let a = &arguments[0];
            format!("{a} cannot be summoned because it is immobile.")
        }
        (110, 68_812_244) => {
            String::from("Hash did not resolve to any childs.")
        }
        (110, 186_059_979) => {
            String::from("ok")
        }
        (110, 65_399_588) => {
            String::from("Temporary items can't be placed in containers!")
        }
        (110, 107_202_452) => {
            String::from("You can't be polymorphed when crawling!")
        }
        (110, 191_760_312) => {
            String::from("Your target didn't lose enough to grant any reward.")
        }
        (110, 73_728_428) => {
            String::from("Your inventory and bank is full. Items are put where you are standing.")
        }
        (110, 268_380_125) => {
            String::from("Can't pick up item. You don't own it.")
        }
        (110, 226_580_676) => {
            String::from("No items found.")
        }
        (110, 192_837_750) => {
            String::from("You can not send mail to yourself.")
        }
        (110, 110_386_572) => {
            String::from("You won the pet duel.")
        }
        (110, 263_479_890) => {
            String::from("Backstab!")
        }
        (110, 62_741_893) => {
            String::from("Unable to create new mission: you have exceeded the limit.")
        }
        (110, 51_106_092) => {
            String::from("You lost the pet duel.")
        }
        (110, 60_190_871) => {
            String::from("Duel ended in a draw.")
        }
        (110, 91_089_881) => {
            let a = &arguments[0];
            format!("This shop is rented by: {a}")
        }
        (110, 161_390_523) => {
            let a = &arguments[0];
            format!("You lost {a} points of Shadowknowledge.")
        }
        (110, 80_707_678) => {
            String::from("Wait for current nano program execution to finish.")
        }
        (110, 103_703_510) => {
            String::from("GM notice: You were unaffected by areaformula!")
        }
        (110, 142_769_077) => {
            String::from("You must be outdoors to perform this action.")
        }
        (110, 150_303_335) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            let e = &arguments[4];
            let f = &arguments[5];
            let g = &arguments[6];
            let h = &arguments[7];
            let i = &arguments[8];
            format!("{a}: Health: {b}%  Nano: {c}%  NCU: {d}/{e}    Timeleft: {f} s    Position: {g},{h}    Fighting: {i}\r\n")
        }
        (110, 19_093_644) => {
            String::from("Illegal spawn level!")
        }
        (110, 197_201_148) => {
            String::from("You can't increase a skill that is temporarily changed!")
        }
        (110, 252_883_556) => {
            let a = &arguments[0];
            format!("{a} tried to hit you, but missed!")
        }
        (110, 259_112_197) => {
            let a = &arguments[0];
            format!("{a} has requested to be your mentor. If you accept, a web browser will be opened to the account management page where you can confirm the request.")
        }
        (110, 5_701_128) => {
            String::from("No corpse in reclaim booth.")
        }
        (110, 245_994_094) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("It is theoretically possible to combine \"{a}\" with \"{b}\". But you are the wrong profession")
        }
        (110, 142_566_068) => {
            String::from("You need a Reset Point in order to reset a skill!")
        }
        (110, 211_132_293) => {
            let a = &arguments[0];
            format!("You are not allowed to put the item <font color=\"yellow\">{a}</font> there, it is not a fixture.<BR>\r\nDo you want to <font color=\"red\">delete</font> the item instead?")
        }
        (110, 176_388_169) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("{a}: damage {b}\r\n")
        }
        (110, 118_928_084) => {
            String::from("Looter is not present.")
        }
        (110, 32_889_214) => {
            String::from("The items contained in this mail message have already been taken.")
        }
        (110, 60_034_452) => {
            String::from("All team members can loot these remains.")
        }
        (110, 26_946_931) => {
            String::from("You can't execute nano programs on items.")
        }
        (110, 81_580_153) => {
            String::from("Your organization already owns a city.")
        }
        (110, 241_691_685) => {
            String::from("Towers can only be attacked when the gaslevel is below 75%")
        }
        (110, 259_666_400) => {
            String::from("Sell items to the shop that you want other players to be able to buy!")
        }
        (110, 102_343_175) => {
            String::from("You can't attack while swimming.")
        }
        (110, 153_864_357) => {
            String::from("You must unequip this item to trade it.")
        }
        (110, 188_287_179) => {
            String::from("You can not order a vehicle while affected by resurrection shock.")
        }
        (110, 38_761_362) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("invalid character at position {a} in '{b}'")
        }
        (110, 239_281_287) => {
            String::from("Target playfield is dying.")
        }
        (110, 55_819_585) => {
            let a = &arguments[0];
            format!("You have left the duel area. {a} seconds until automatic forfeit.")
        }
        (110, 111_398_311) => {
            String::from("Nano execution aborted because either you or your target is teleporting.")
        }
        (110, 249_763_572) => {
            String::from("Nickname is not accepted. Nickname matches entry in list of illegal and/or reserved names.")
        }
        (110, 93_885_538) => {
            String::from("You're unable to wear this item.")
        }
        (110, 209_914_812) => {
            String::from("Brawl!")
        }
        (110, 5_680_702) => {
            String::from("You need to have a energy weapon equipped.")
        }
        (110, 133_264_628) => {
            String::from("Unable to get the target item, at all!")
        }
        (110, 173_694_303) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("You increased nano on {a} for {b} points.")
        }
        (110, 174_567_932) => {
            String::from("Poison roll failed..")
        }
        (110, 61_955_389) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("You got nano from {a} for {b} points.")
        }
        (110, 87_846_176) => {
            String::from("This shop is only open for members of the organization owning the market!")
        }
        (110, 191_732_363) => {
            String::from("You lacked the skill to perform the action.")
        }
        (110, 90_448_100) => {
            String::from("Target is not your pet.")
        }
        (110, 88_395_896) => {
            let a = &arguments[0];
            format!("You are now a member of the organization \"{a}\".")
        }
        (110, 164_102_084) => {
            String::from("You must be behind the target!")
        }
        (110, 108_633_128) => {
            let a = &arguments[0];
            format!("You were deducted {a} credits organization tax.")
        }
        (110, 137_609_118) => {
            String::from("Invalid arguments for quest spawning.")
        }
        (110, 183_542_539) => {
            let a = &arguments[0];
            format!("{a} blocked your attack, and performs an attack of opportunity!")
        }
        (110, 169_515_335) => {
            String::from("You end sneaking.")
        }
        (110, 150_961_061) => {
            String::from("Executing programs is currently unavailable.")
        }
        (110, 20_226_388) => {
            String::from("This weapon doesn't work against this target.")
        }
        (110, 197_138_709) => {
            let a = &arguments[0];
            format!("Unable to create new mission: {a} has exceeded the limit.")
        }
        (110, 76_370_149) => {
            let a = &arguments[0];
            format!("The /stuck command will be available in {a} seconds.")
        }
        (110, 199_555_093) => {
            let a = &arguments[0];
            format!("Someone's damage shield hit you for {a} points of damage.")
        }
        (110, 155_585_597) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Your team and your targets team share members from the same organization: {a} and {b}")
        }
        (110, 151_543_093) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            let e = &arguments[4];
            let f = &arguments[5];
            format!("{a} deleted <a href=\"itemref://{b}/{c}/{d}\">{e}</a> from {f}.")
        }
        (110, 12_339_173) => {
            let a = &arguments[0];
            format!("You were hit for {a} points of damage.")
        }
        (110, 138_964_008) => {
            let a = &arguments[0];
            format!("Must not be in organization with {a}")
        }
        (110, 145_444_926) => {
            String::from("You must have the Lost Eden expansion!")
        }
        (110, 211_829_310) => {
            String::from("You can't dual wield this weapon with the already equipped weapon.")
        }
        (110, 160_989_348) => {
            String::from("You have completed the second specialization!")
        }
        (110, 109_935_669) => {
            String::from("You died of liquid damage!")
        }
        (110, 240_786_501) => {
            String::from("It is no longer possible to change this option!")
        }
        (110, 22_015_973) => {
            String::from("You died of nano program damage!")
        }
        (110, 66_653_597) => {
            String::from("Already using an item.")
        }
        (110, 220_179_189) => {
            String::from("Target does not have enough nano controlling units (NCU) left.")
        }
        (110, 195_042_293) => {
            String::from("Attack not allowed since you are on the same side in the conflict.")
        }
        (110, 59_395_603) => {
            let a = &arguments[0];
            format!("One of your team members ({a}) is in the same organization as your target")
        }
        (110, 116_246_126) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("Locked down by Org leader: time until reset {a:02}:{b:02}:{c:02}")
        }
        (110, 96_347_739) => {
            String::from("You are currently unable to use perk special actions!")
        }
        (110, 131_926_324) => {
            String::from("You can't open a chest while you're in a fight.")
        }
        (110, 252_552_387) => {
            String::from("You have now received extra Increase Points that you can distribute on your skills. Use the Skill Window for this.")
        }
        (110, 264_526_156) => {
            String::from("Chest full.")
        }
        (110, 16_419_587) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("{a} tries to attack you with {b}, but misses!")
        }
        (110, 69_616_863) => {
            String::from("Full Auto!")
        }
        (110, 47_860_421) => {
            String::from("You are now locked to this instance.")
        }
        (110, 171_954_057) => {
            let a = &arguments[0];
            format!("This effect can't be utilitized by {a}!")
        }
        (110, 29_209_012) => {
            let a = &arguments[0];
            format!("You have challenged {a} to a duel. Waiting for opponent to reply.")
        }
        (110, 57_976_771) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("{a} must be {b}")
        }
        (110, 61_425_492) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Something hit {a} for {b} points of damage by damage shield.")
        }
        (110, 122_300_136) => {
            let a = &arguments[0];
            format!("Must not be in team with {a}")
        }
        (110, 50_086_020) => {
            String::from("It is locked.")
        }
        (110, 160_480_308) => {
            String::from("Inspect function option is disabled.")
        }
        (110, 62_455_941) => {
            String::from("You succeeded disarming the mine!")
        }
        (110, 77_312_781) => {
            String::from("You must carry the item.")
        }
        (110, 97_089_187) => {
            let a = &arguments[0];
            format!("You received a new mission with {a}% added richness (QL) to the treasures.")
        }
        (110, 136_704_116) => {
            String::from("Target is not in a fight.")
        }
        (110, 90_809_749) => {
            String::from("Special attack not possible. The target is aware of your presence.")
        }
        (110, 64_653_764) => {
            String::from("Stuck-resolve command not allowed while standing.")
        }
        (110, 188_845_972) => {
            String::from("Target playfield not found.")
        }
        (110, 252_109_012) => {
            String::from("Mindcontrol activated.")
        }
        (110, 268_399_849) => {
            String::from("Your mission has failed. Target was not killed properly!")
        }
        (110, 65_806_413) => {
            String::from("The target must be in your team!")
        }
        (110, 242_457_140) => {
            String::from("Organization form changed!")
        }
        (110, 172_120_388) => {
            String::from("Must be in grace period.")
        }
        (110, 145_504_786) => {
            let a = &arguments[0];
            format!("Current team loot order is: {a}")
        }
        (110, 173_450_925) => {
            String::from("You cannot attack this target, because you are in a mixed omni/clan team.")
        }
        (110, 108_070_804) => {
            let a = &arguments[0];
            format!("You need at least {a} in Multiple Ranged to wield these two weapons.")
        }
        (110, 24_883_171) => {
            String::from("You can increase the necessary skills from the skill view (Button below with SKL on it). Later you can use nanoprograms, implants and other items to enhance your character.")
        }
        (110, 226_754_835) => {
            let a = &arguments[0];
            format!("{a} xp was gained as a side bonus!")
        }
        (110, 236_262_381) => {
            let a = &arguments[0];
            format!("Leadership was given to you by GM {a}.")
        }
        (110, 240_791_923) => {
            let a = &arguments[0];
            format!("{a} already has an open city terminal. You only have limited access to this city terminal.")
        }
        (110, 56_347_732) => {
            String::from("Your mail will not be sent due to unacceptable content. Please revise your text and/or subject.")
        }
        (110, 96_106_701) => {
            String::from("Players cannot join a team while either they or the team are in a fight.")
        }
        (110, 246_239_454) => {
            String::from("Trying to spawn unsupported type (OBType).")
        }
        (110, 148_488_884) => {
            let a = &arguments[0];
            format!("The current commission is: {a}%")
        }
        (110, 176_204_491) => {
            let a = &arguments[0];
            format!("You need to increase your faction standing with {a} to gain Shadow Knowledge.")
        }
        (110, 147_913_734) => {
            String::from("Position given, but playfield is unknown.")
        }
        (110, 265_449_739) => {
            let a = &arguments[0];
            format!("You gained {a} points of Shadowknowledge.")
        }
        (110, 207_774_771) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("You try to attack {a} with {b}, but you miss!")
        }
        (110, 85_240_131) => {
            String::from("While crawling, you may only fight with ranged weapons!")
        }
        (110, 197_777_451) => {
            String::from("This action requires that you are wielding Augmented Cyberdeck!")
        }
        (110, 172_745_013) => {
            String::from("This city district is already occupied.")
        }
        (110, 211_473_316) => {
            String::from("You can't delete an item that is being equipped!")
        }
        (110, 154_268_609) => {
            String::from("The target must have the Notum Wars Booster Pack!")
        }
        (110, 167_480_990) => {
            String::from("You need to have a long range weapon equipped in your right hand.")
        }
        (110, 79_979_934) => {
            let a = &arguments[0];
            format!("Mission chance of token reward upped to {a}% due to your heroic effort.")
        }
        (110, 102_432_958) => {
            String::from("You need to have a shotgun weapon equipped in your right hand.")
        }
        (110, 151_203_608) => {
            let a = &arguments[0];
            format!("Must be in team with {a}")
        }
        (110, 266_000_116) => {
            String::from("You must be in front of the target!")
        }
        (110, 107_900_350) => {
            String::from("You need to have a piercing weapon equipped in your right hand.")
        }
        (110, 25_614_836) => {
            String::from("This item requires a fighting-target to be applied on.")
        }
        (110, 62_658_469) => {
            String::from("You already have this unique item")
        }
        (110, 172_796_489) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("You need following skill to combine: {a} {b}")
        }
        (110, 32_054_429) => {
            String::from("Target out of range for nano program.")
        }
        (110, 74_103_140) => {
            String::from("Your special attack was blocked by a special attack shield!")
        }
        (110, 74_548_413) => {
            String::from("You don't own that item.")
        }
        (110, 9_040_707) => {
            String::from("You have been selected as a member of the boarding team.  You have 5 minutes to enter the spaceship.  You can do this by walking into the teleport beam.")
        }
        (110, 133_891_201) => {
            let a = &arguments[0];
            format!("You were healed for {a} points of heal delta.")
        }
        (110, 14_895_299) => {
            String::from("Unable to find a valid position.")
        }
        (110, 155_227_557) => {
            String::from("You will now be able to gain ShadowKnowledge and obtain ShadowLevels (levels above 200)!")
        }
        (110, 190_455_252) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Playshifting failed: The server was unable to start the playfield. ( {a}, {b} )")
        }
        (110, 124_550_313) => {
            String::from("Nano program executed successfully.")
        }
        (110, 30_590_100) => {
            String::from("Source item is not owned by the char and does not have the stationary flags set.")
        }
        (110, 88_056_277) => {
            String::from("You died of self termination!")
        }
        (110, 196_521_796) => {
            String::from("Your mentor invite has been rejected.")
        }
        (110, 206_104_233) => {
            String::from("Not enough nano energy to execute nanoprogram.")
        }
        (110, 176_103_600) => {
            let a = &arguments[0];
            format!("Received mission reward: XP {a}")
        }
        (110, 247_899_273) => {
            String::from("Unable to enter mission building: are you using an old mission key?\r\n")
        }
        (110, 7_676_619) => {
            String::from("Your inventory is full. Items are put into your bank.")
        }
        (110, 42_022_099) => {
            String::from("Accessing implants requires technical supervision.")
        }
        (110, 176_365_658) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("{a}:{b}: damage {c}")
        }
        (110, 121_390_268) => {
            String::from("The attached credits added to your total would exceed the maximum amount of credits allowed.")
        }
        (110, 129_293_400) => {
            let a = &arguments[0];
            format!("Must be in organization with {a}")
        }
        (110, 202_437_824) => {
            String::from("Skill lock commands:\r\nlist   - list the skill locks on a character\r\nremove - remove a specific skill lock from a character\r\nclear  - remove all skill locks from a character\r\n")
        }
        (110, 145_834_868) => {
            String::from("Wrong target selected.")
        }
        (110, 211_111_331) => {
            String::from("You can wield no weapons before using this item.")
        }
        (110, 196_887_139) => {
            String::from("Use a surgery clinic to remove implants!")
        }
        (110, 34_898_229) => {
            String::from("Please wait for item to deactivate before attempting to wear another.")
        }
        (110, 221_860_521) => {
            String::from("Your remains have not been reconstructed yet.")
        }
        (110, 63_376_425) => {
            String::from("Access the mission entrance by dragging a valid mission key onto it, or just walk through it!\r\n")
        }
        (110, 75_628_489) => {
            String::from("You are not allowed to execute friendly nanoprogram on enemy.")
        }
        (110, 169_766_805) => {
            let a = &arguments[0];
            format!("{a} kicked you from your organization.")
        }
        (110, 216_503_149) => {
            String::from("Target must be in a team in order to perform this action.")
        }
        (110, 41_106_052) => {
            String::from("Team-mission chance of token reward not upped due to too high level character in team.")
        }
        (110, 236_710_069) => {
            String::from("Adding fixture to the building was not allowed. The building has to have capacity and you must own it.")
        }
        (110, 3_278_652) => {
            let a = &arguments[0];
            format!("Maximum increase in skill is {a} per level!")
        }
        (110, 54_186_628) => {
            let a = &arguments[0];
            format!("{a} is already fully loaded.")
        }
        (110, 129_266_804) => {
            String::from("You have completed the fourth specialization!")
        }
        (110, 53_281_856) => {
            String::from("Trade contains a NoDrop item and can't be completed.")
        }
        (110, 21_049_539) => {
            String::from("Temporary items can't be used in a tradeskill process!")
        }
        (110, 126_565_325) => {
            let a = &arguments[0];
            format!("Broken item at: {a}.")
        }
        (110, 20_213_689) => {
            String::from("The new item is of excellent quality!")
        }
        (110, 22_901_477) => {
            String::from("Special attack is unavailable.")
        }
        (110, 193_947_284) => {
            String::from("Area change not initiated on server. You may retreat or try crossing again.")
        }
        (110, 231_552_041) => {
            String::from("Your target is already engaged in a pet duel.")
        }
        (110, 74_985_272) => {
            let a = &arguments[0];
            format!("Must be in raid with {a}")
        }
        (110, 27_674_548) => {
            String::from("Lockpicking failed.")
        }
        (110, 83_322_987) => {
            let a = &arguments[0];
            format!("You block the attack from {a}, and perform an attack of opportunity!")
        }
        (110, 71_102_660) => {
            String::from("Your duel challenge was refused.")
        }
        (110, 90_418_825) => {
            String::from("Nanoprogram did not activate. Not enough nanoenergy.")
        }
        (110, 13_206_071) => {
            String::from("You can't execute nanoprograms while crawling!")
        }
        (110, 122_823_936) => {
            String::from("Shop commands:\r\nstatus - give some status information about the target's shop\r\nitems - list the contents of the target's shop")
        }
        (110, 81_298_628) => {
            String::from("Trade cancelled.")
        }
        (110, 190_344_405) => {
            String::from("Must be in land control area level range.")
        }
        (110, 201_501_911) => {
            String::from("building")
        }
        (110, 153_980_007) => {
            String::from("You are too low level to forage in this area!")
        }
        (110, 35_961_822) => {
            String::from("You need to have a melee weapon equipped.")
        }
        (110, 134_616_974) => {
            String::from("You need to have a grenade weapon equipped in your right hand.")
        }
        (110, 163_393_380) => {
            let a = &arguments[0];
            format!("Your items will be reclaimed in {a} seconds, and made available in a reclaim booth near your resurrection site.")
        }
        (110, 230_129_486) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("Org missing payment lockdown: time until reset {a}:{b}:{c}")
        }
        (110, 161_390_352) => {
            let a = &arguments[0];
            format!("You lost {a} xp.")
        }
        (110, 151_666_194) => {
            String::from("You're unable to unequip this item.")
        }
        (110, 84_337_565) => {
            String::from("You found an item!")
        }
        (110, 239_125_981) => {
            String::from("Cash on delivery mail must contain an item.")
        }
        (110, 16_807_491) => {
            String::from("The selected target needs to be an NPC.")
        }
        (110, 268_207_179) => {
            String::from("No room in your inventory. Item sent to bank.")
        }
        (110, 202_983_733) => {
            String::from("Unable to find source item?")
        }
        (110, 129_537_138) => {
            String::from("Social armor requires that no other armor is worn.")
        }
        (110, 158_238_750) => {
            String::from("Veto position failed, no spawn done.")
        }
        (110, 239_701_196) => {
            String::from("You can only cast friendly nanos on yourself in a duel.")
        }
        (110, 91_885_729) => {
            String::from("You must have the Notum Wars Booster Pack!")
        }
        (110, 56_215_635) => {
            String::from("You can not send container items to the market.")
        }
        (110, 98_001_989) => {
            String::from("Stuck-resolve mode entered; expect resolution in 30 seconds.")
        }
        (110, 79_090_035) => {
            let a = &arguments[0];
            format!("One of your targets team members ({a}) is in the same organization as you")
        }
        (110, 248_706_260) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Unique template {a} cleared on {b}.")
        }
        (110, 143_629_720) => {
            String::from("You can't reload the weapon with this ammunition type.")
        }
        (110, 235_433_772) => {
            String::from("All your team members are of too low level for you to request a team mission. You can not share experience with them...")
        }
        (110, 213_590_151) => {
            String::from("Target is surrendering to another person.")
        }
        (110, 60_243_143) => {
            String::from("You start bluffing")
        }
        (110, 97_040_692) => {
            String::from("No entrance door dynel found.")
        }
        (110, 155_245_326) => {
            String::from("You need to have an unarmed combat weapon equipped in your right hand.")
        }
        (110, 193_173_787) => {
            String::from("This action requires that you are wielding Jobe-chipped Cyberdeck!")
        }
        (110, 215_635_013) => {
            String::from("Credits must be a positive value.")
        }
        (110, 117_444_414) => {
            String::from("You need to have an assault rifle weapon equipped.")
        }
        (110, 65_166_910) => {
            String::from("You are not in an organization.")
        }
        (110, 102_153_836) => {
            let a = &arguments[0];
            format!("Congratulations! You have now reached the level of {a}!")
        }
        (110, 111_941_116) => {
            String::from("You are already engaged in a duel.")
        }
        (110, 127_063_686) => {
            String::from("You can't attack yourself.")
        }
        (110, 122_255_150) => {
            String::from("You cannot attack this target, because you are in a mixed Red/Blue team.")
        }
        (110, 54_882_997) => {
            let a = &arguments[0];
            format!("You have been challenged to a fight from {a}. Do you accept?")
        }
        (110, 248_967_972) => {
            String::from("Unable to execute nano program. Target not found.")
        }
        (110, 62_734_319) => {
            let a = &arguments[0];
            format!("No ammo was found for {a}!")
        }
        (110, 141_178_878) => {
            String::from("You're unable to perform this action; please check the requirements of the item.")
        }
        (110, 177_744_001) => {
            String::from("You are entering an overcrowded area. You will be moved back. Stop now. Please wait or find another destination.")
        }
        (110, 172_594_057) => {
            String::from("You are too far away; please move closer!")
        }
        (110, 213_582_914) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            format!("{a} hit {b} for {c} points of {d} damage.")
        }
        (110, 180_544_964) => {
            String::from("Mission spawn performed.")
        }
        (110, 268_127_840) => {
            String::from("Suppression field at 100%. No fighting possible.")
        }
        (110, 117_740_361) => {
            String::from("You need a free entry in your inventory for your access card before you can enter!")
        }
        (110, 9_596_784) => {
            let a = &arguments[0];
            format!("{a}'s Shop")
        }
        (110, 168_308_756) => {
            let a = &arguments[0];
            format!("You did not have enough cash to pay for the organizational upkeep (which is currently {a} credits). This amount is deducted from your account every 2 hours of played time. Your organization membership is now suspended for 30 minutes. At the end of this period, the system will again try to deduct the upkeep amount. If you are able to pay then, your organization membership will be reinstated. If not, another 30 minute suspension is applied.")
        }
        (110, 236_131_847) => {
            String::from("Move item to inventory before dropping it on ground.")
        }
        (110, 245_829_571) => {
            String::from("You can not send nodrop items through the mail system.")
        }
        (110, 114_257_650) => {
            String::from("Your")
        }
        (110, 145_701_333) => {
            String::from("Must be in combat mode.")
        }
        (110, 199_506_192) => {
            let a = &arguments[0];
            format!("You have gained enough Alien XP to get to another alien level but your level must be: {a} to obtain it.")
        }
        (110, 87_953_780) => {
            String::from("You have reached the trade limit for this shop!")
        }
        (110, 5_925_493) => {
            String::from("You're not allowed to invite other players.")
        }
        (110, 182_802_785) => {
            String::from("Nano program failed. Already executing nanoprogram.")
        }
        (110, 239_303_476) => {
            String::from("Arming trap failed.")
        }
        (110, 254_360_748) => {
            String::from("Unable to send mail. The recipient's mailbox is full.")
        }
        (110, 138_343_248) => {
            String::from("This is now a closed shop, only open for members of your organization!")
        }
        (110, 13_262_176) => {
            String::from("This is a private shop!")
        }
        (110, 213_725_380) => {
            let a = &arguments[0];
            format!("The /stuck command will be resolved in {a} seconds.")
        }
        (110, 20_542_174) => {
            String::from("You have won the duel!")
        }
        (110, 16_891_246) => {
            String::from("Player is not in your organization.")
        }
        (110, 166_718_360) => {
            let a = &arguments[0];
            format!("You drained {a} points of health from the target.")
        }
        (110, 175_022_562) => {
            String::from("NCU error: This nano program can't automatically replace other program.")
        }
        (110, 183_853_698) => {
            String::from("You are now renting this shop terminal!")
        }
        (110, 128_107_796) => {
            String::from("Target can't be in a fight.")
        }
        (110, 192_016_077) => {
            String::from("You need a target for this item!")
        }
        (110, 128_711_717) => {
            let a = &arguments[0];
            format!("You already have this unique item: {a}")
        }
        (110, 108_021_749) => {
            let a = &arguments[0];
            format!("You took {a} points of fall damage.")
        }
        (110, 42_763_038) => {
            String::from("You need to have a 1 hand blunt weapon equipped.")
        }
        (110, 87_926_248) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("(GM feedback) didn't find booth {a}:{b}!")
        }
        (110, 19_815_477) => {
            String::from("Action is not available.")
        }
        (110, 111_838_350) => {
            String::from("You must not have any items equipped.")
        }
        (2003, 21) => {
            String::from("Psychic")
        }
        (2003, 477) => {
            String::from("MaxReflectedEnergyDmg")
        }
        (2003, 247) => {
            String::from("TemporarySkillReduction")
        }
        (2003, 95) => {
            String::from("Cold AC")
        }
        (2003, 243) => {
            String::from("AbsorbColdAC")
        }
        (2003, 536) => {
            String::from("Direct Nano Damage Efficiency")
        }
        (2003, 568) => {
            String::from("Faction with The Assertive Operators")
        }
        (2003, 584) => {
            String::from("ApartmentAccessCard")
        }
        (2003, 48) => {
            String::from("Organization rank")
        }
        (2003, 97) => {
            String::from("Fire AC")
        }
        (2003, 3) => {
            String::from("AttackSpeed")
        }
        (2003, 587) => {
            String::from("Number of team members")
        }
        (2003, 208) => {
            String::from("ReflectChemicalAC")
        }
        (2003, 105) => {
            String::from("2h Edged")
        }
        (2003, 661) => {
            String::from("DamageToNanoMultiplier")
        }
        (2003, 475) => {
            String::from("MaxReflectedProjectileDmg")
        }
        (2003, 8) => {
            String::from("TimeExist")
        }
        (2003, 124) => {
            String::from("Treatment")
        }
        (2003, 45) => {
            String::from("Free deck slot")
        }
        (2003, 231) => {
            String::from("ShieldColdAC")
        }
        (2003, 129) => {
            String::from("Psycho Modi")
        }
        (2003, 214) => {
            String::from("Nano Points")
        }
        (2003, 245) => {
            String::from("AbsorbPoisonAC")
        }
        (2003, 246) => {
            String::from("AbsorbNanoAC")
        }
        (2003, 515) => {
            String::from("TowerInstance")
        }
        (2003, 128) => {
            String::from("Bio Metamor")
        }
        (2003, 454) => {
            String::from("Proximity Range")
        }
        (2003, 680) => {
            String::from("PVPRankedTeamKills")
        }
        (2003, 684) => {
            String::from("PVPDuelScore")
        }
        (2003, 104) => {
            String::from("Melee Ener.")
        }
        (2003, 154) => {
            String::from("Dodge-Rng")
        }
        (2003, 489) => {
            String::from("Backstab")
        }
        (2003, 675) => {
            String::from("PVPDuelDeaths")
        }
        (2003, 367) => {
            String::from("VisualBreed")
        }
        (2003, 151) => {
            String::from("Aimed Shot")
        }
        (2003, 516) => {
            String::from("AttackShield")
        }
        (2003, 561) => {
            String::from("Faction with The Sentinels")
        }
        (2003, 565) => {
            String::from("Faction with Clan Vanguards")
        }
        (2003, 153) => {
            String::from("Duck-Exp")
        }
        (2003, 221) => {
            String::from("Max Nano")
        }
        (2003, 482) => {
            String::from("MaxReflectedFireDmg")
        }
        (2003, 120) => {
            String::from("Physic. Init")
        }
        (2003, 130) => {
            String::from("Matter Crea")
        }
        (2003, 96) => {
            String::from("Disease AC")
        }
        (2003, 138) => {
            String::from("Swimming")
        }
        (2003, 390) => {
            String::from("LowresMesh")
        }
        (2003, 368) => {
            String::from("VisualProfession")
        }
        (2003, 146) => {
            String::from("Sneak Atck")
        }
        (2003, 685) => {
            String::from("Mission Bits 14")
        }
        (2003, 117) => {
            String::from("Vehicle Water")
        }
        (2003, 479) => {
            String::from("MaxReflectedRadiationDmg")
        }
        (2003, 535) => {
            String::from("Healing Efficiency")
        }
        (2003, 230) => {
            String::from("ShieldRadiationAC")
        }
        (2003, 280) => {
            String::from("Add. Energy Dam.")
        }
        (2003, 20) => {
            String::from("Sense")
        }
        (2003, 360) => {
            String::from("Scale")
        }
        (2003, 126) => {
            String::from("Elec. Engi")
        }
        (2003, 681) => {
            String::from("PVPRankedTeamDeaths")
        }
        (2003, 407) => {
            String::from("Nano points")
        }
        (2003, 236) => {
            String::from("InsurancePercentage")
        }
        (2003, 562) => {
            String::from("Faction with Omni-Med")
        }
        (2003, 140) => {
            String::from("Map Navig.")
        }
        (2003, 37) => {
            String::from("TitleLevel")
        }
        (2003, 389) => {
            String::from("Expansions")
        }
        (2003, 110) => {
            String::from("Heavy Weapons")
        }
        (2003, 481) => {
            String::from("MaxReflectedNanoDmg")
        }
        (2003, 17) => {
            String::from("Agility")
        }
        (2003, 382) => {
            String::from("SkillLockModifier")
        }
        (2003, 381) => {
            String::from("RangeInc. NF")
        }
        (2003, 137) => {
            String::from("Adventuring")
        }
        (2003, 135) => {
            String::from("Trap Disarm.")
        }
        (2003, 517) => {
            String::from("SpecialAttackShield")
        }
        (2003, 168) => {
            String::from("Nano Resist")
        }
        (2003, 672) => {
            String::from("PaidPoints")
        }
        (2003, 111) => {
            String::from("Bow")
        }
        (2003, 339) => {
            String::from("Damage override type")
        }
        (2003, 19) => {
            String::from("Intelligence")
        }
        (2003, 282) => {
            String::from("Add. Rad. Dam.")
        }
        (2003, 480) => {
            String::from("MaxReflectedColdDmg")
        }
        (2003, 115) => {
            String::from("Shotgun")
        }
        (2003, 149) => {
            String::from("NanoC. Init.")
        }
        (2003, 659) => {
            String::from("DamageToNano")
        }
        (2003, 316) => {
            String::from("Add. Fire Dam.")
        }
        (2003, 226) => {
            String::from("ShieldProjectileAC")
        }
        (2003, 136) => {
            String::from("Perception")
        }
        (2003, 572) => {
            String::from("Faction with The Redeemed")
        }
        (2003, 181) => {
            String::from("Max NCU")
        }
        (2003, 388) => {
            String::from("TowerType")
        }
        (2003, 118) => {
            String::from("Melee. Init.")
        }
        (2003, 35) => {
            String::from("Damage to Pet")
        }
        (2003, 113) => {
            String::from("Rifle")
        }
        (2003, 100) => {
            String::from("Martial Arts")
        }
        (2003, 91) => {
            String::from("Melee/ma AC")
        }
        (2003, 143) => {
            String::from("Riposte")
        }
        (2003, 317) => {
            String::from("Add. Poison Dam.")
        }
        (2003, 563) => {
            String::from("Faction with Gaia")
        }
        (2003, 132) => {
            String::from("Nano Pool")
        }
        (2003, 564) => {
            String::from("Faction with Omni-Trans")
        }
        (2003, 22) => {
            String::from("Attack rating")
        }
        (2003, 60) => {
            String::from("Profession")
        }
        (2003, 122) => {
            String::from("Sensory Impr")
        }
        (2003, 570) => {
            String::from("Faction with The Devoted")
        }
        (2003, 274) => {
            String::from("Equipped Weapon Type")
        }
        (2003, 107) => {
            String::from("2h Blunt")
        }
        (2003, 142) => {
            String::from("Brawling")
        }
        (2003, 164) => {
            String::from("Concealment")
        }
        (2003, 228) => {
            String::from("ShieldEnergyAC")
        }
        (2003, 465) => {
            String::from("Cry For Help Range")
        }
        (2003, 589) => {
            String::from("Playfield")
        }
        (2003, 134) => {
            String::from("Multi Ranged")
        }
        (2003, 242) => {
            String::from("AbsorbRadiationAC")
        }
        (2003, 343) => {
            String::from("HealDelta")
        }
        (2003, 364) => {
            String::from("NanoDelta")
        }
        (2003, 116) => {
            String::from("Assault Rif")
        }
        (2003, 478) => {
            String::from("MaxReflectedChemicalDmg")
        }
        (2003, 470) => {
            String::from("MapOptions")
        }
        (2003, 201) => {
            String::from("Aggressiveness")
        }
        (2003, 219) => {
            String::from("ReflectFireAC")
        }
        (2003, 683) => {
            String::from("PVPTeamScore")
        }
        (2003, 39) => {
            String::from("Damage To Pet Damage Multiplier")
        }
        (2003, 686) => {
            String::from("Mission Bits 15")
        }
        (2003, 162) => {
            String::from("Psychology")
        }
        (2003, 244) => {
            String::from("AbsorbFireAC")
        }
        (2003, 571) => {
            String::from("Faction with The Benign Conservers")
        }
        (2003, 233) => {
            String::from("ShieldFireAC")
        }
        (2003, 303) => {
            String::from("Organization upkeep")
        }
        (2003, 391) => {
            String::from("Critical Decrease")
        }
        (2003, 159) => {
            String::from("Pharma Tech")
        }
        (2003, 139) => {
            String::from("Vehicle Air")
        }
        (2003, 167) => {
            String::from("Full Auto")
        }
        (2003, 169) => {
            String::from("Alien Defender Title")
        }
        (2003, 379) => {
            String::from("CriticalIncrease")
        }
        (2003, 51) => {
            String::from("Aggdef")
        }
        (2003, 131) => {
            String::from("Time&Space")
        }
        (2003, 155) => {
            String::from("Evade-ClsC")
        }
        (2003, 218) => {
            String::from("ReflectNanoAC")
        }
        (2003, 225) => {
            String::from("ReflectPoisonAC")
        }
        (2003, 662) => {
            String::from("Use Mech")
        }
        (2003, 278) => {
            String::from("Add. Proj. Dam.")
        }
        (2003, 152) => {
            String::from("Body Dev.")
        }
        (2003, 239) => {
            String::from("AbsorbMeleeAC")
        }
        (2003, 586) => {
            String::from("MapArea")
        }
        (2003, 112) => {
            String::from("Pistol")
        }
        (2003, 205) => {
            String::from("ReflectProjectileAC")
        }
        (2003, 567) => {
            String::from("Faction with Followers")
        }
        (2003, 27) => {
            String::from("Health")
        }
        (2003, 114) => {
            String::from("MG / SMG")
        }
        (2003, 560) => {
            String::from("Faction with Omni-Armed Forces")
        }
        (2003, 106) => {
            String::from("Piercing")
        }
        (2003, 234) => {
            String::from("ShieldPoisonAC")
        }
        (2003, 279) => {
            String::from("Add. Melee Dam.")
        }
        (2003, 319) => {
            String::from("% Add. Xp")
        }
        (2003, 0) => {
            String::from("Flags")
        }
        (2003, 127) => {
            String::from("Matt.Metam")
        }
        (2003, 161) => {
            String::from("Comp. Liter")
        }
        (2003, 10) => {
            String::from("Prof.Title Lvl")
        }
        (2003, 281) => {
            String::from("Add. Chem. Dam.")
        }
        (2003, 141) => {
            String::from("Tutoring")
        }
        (2003, 58) => {
            String::from("Played")
        }
        (2003, 369) => {
            String::from("VisualSex")
        }
        (2003, 145) => {
            String::from("Deflect")
        }
        (2003, 315) => {
            String::from("Add. Nano Dam.")
        }
        (2003, 94) => {
            String::from("Radiation AC")
        }
        (2003, 579) => {
            String::from("Shadowbreed")
        }
        (2003, 702) => {
            String::from("Item Template ID")
        }
        (2003, 241) => {
            String::from("AbsorbChemicalAC")
        }
        (2003, 311) => {
            String::from("Add. Cold Dam.")
        }
        (2003, 75) => {
            String::from("Awarded Omni-Tek Tokens")
        }
        (2003, 302) => {
            String::from("Organization cost")
        }
        (2003, 238) => {
            String::from("AbsorbProjectileAC")
        }
        (2003, 62) => {
            String::from("Awarded Clan Tokens")
        }
        (2003, 147) => {
            String::from("Fast Attack")
        }
        (2003, 119) => {
            String::from("Ranged. Init.")
        }
        (2003, 125) => {
            String::from("Mech. Engi")
        }
        (2003, 251) => {
            String::from("Number of pets")
        }
        (2003, 383) => {
            String::from("Decreased Nano-Interrupt Modifier %")
        }
        (2003, 525) => {
            String::from("% Health")
        }
        (2003, 101) => {
            String::from("Mult. Melee")
        }
        (2003, 217) => {
            String::from("ReflectColdAC")
        }
        (2003, 213) => {
            String::from("Team Side")
        }
        (2003, 678) => {
            String::from("PVPRankedSoloKills")
        }
        (2003, 679) => {
            String::from("PVPRankedSoloDeaths")
        }
        (2003, 229) => {
            String::from("ShieldChemicalAC")
        }
        (2003, 483) => {
            String::from("MaxReflectedPoisonDmg")
        }
        (2003, 54) => {
            String::from("Level")
        }
        (2003, 342) => {
            String::from("HealInterval")
        }
        (2003, 566) => {
            String::from("Faction with Guardian of Shadow")
        }
        (2003, 674) => {
            String::from("PVPDuelKills")
        }
        (2003, 144) => {
            String::from("Dimach")
        }
        (2003, 363) => {
            String::from("NanoInterval")
        }
        (2003, 93) => {
            String::from("Chemical AC")
        }
        (2003, 380) => {
            String::from("RangeInc. Weapon")
        }
        (2003, 582) => {
            String::from("ApartmentsAllowed")
        }
        (2003, 102) => {
            String::from("1h Blunt")
        }
        (2003, 103) => {
            String::from("1h Edged")
        }
        (2003, 676) => {
            String::from("PVPProfessionDuelKills")
        }
        (2003, 682) => {
            String::from("PVPSoloScore")
        }
        (2003, 33) => {
            String::from("Side")
        }
        (2003, 668) => {
            String::from("Battlestation Side")
        }
        (2003, 150) => {
            String::from("Fling Shot")
        }
        (2003, 569) => {
            String::from("Faction with The Unredeemed")
        }
        (2003, 277) => {
            String::from("Add All Def.")
        }
        (2003, 157) => {
            String::from("Quantum FT")
        }
        (2003, 121) => {
            String::from("Bow Spc Att")
        }
        (2003, 660) => {
            String::from("AccountFlags")
        }
        (2003, 232) => {
            String::from("ShieldNanoAC")
        }
        (2003, 133) => {
            String::from("Ranged Ener")
        }
        (2003, 199) => {
            String::from("Reset points")
        }
        (2003, 108) => {
            String::from("Sharp Obj")
        }
        (2003, 61) => {
            String::from("Credit")
        }
        (2003, 688) => {
            String::from("Rarity")
        }
        (2003, 53) => {
            String::from("IP")
        }
        (2003, 156) => {
            String::from("Run Speed")
        }
        (2003, 123) => {
            String::from("First Aid")
        }
        (2003, 273) => {
            String::from("Secondary Item Template ID")
        }
        (2003, 158) => {
            String::from("Weapon Smt")
        }
        (2003, 165) => {
            String::from("Break&Entry")
        }
        (2003, 438) => {
            String::from("Playfield type")
        }
        (2003, 593) => {
            String::from("Regain XP Percentage")
        }
        (2003, 276) => {
            String::from("Add All Off.")
        }
        (2003, 318) => {
            String::from("% Add. Nano Cost")
        }
        (2003, 257) => {
            String::from("Mission Bits 2")
        }
        (2003, 689) => {
            String::from("Heal Reactivity")
        }
        (2003, 476) => {
            String::from("MaxReflectedMeleeDmg")
        }
        (2003, 109) => {
            String::from("Grenade")
        }
        (2003, 166) => {
            String::from("Vehicle Ground")
        }
        (2003, 207) => {
            String::from("ReflectEnergyAC")
        }
        (2003, 240) => {
            String::from("AbsorbEnergyAC")
        }
        (2003, 92) => {
            String::from("Energy AC")
        }
        (2003, 1) => {
            String::from("Max Health")
        }
        (2003, 163) => {
            String::from("Chemistry")
        }
        (2003, 216) => {
            String::from("ReflectRadiationAC")
        }
        (2003, 677) => {
            String::from("PVPProfessionDuelDeaths")
        }
        (2003, 90) => {
            String::from("Imp/Proj AC")
        }
        (2003, 673) => {
            String::from("VisualFlags")
        }
        (2003, 215) => {
            String::from("GM Level")
        }
        (2003, 227) => {
            String::from("ShieldMeleeAC")
        }
        (2003, 52) => {
            String::from("XP")
        }
        (2003, 180) => {
            String::from("Used NCU")
        }
        (2003, 206) => {
            String::from("ReflectMeleeAC")
        }
        (2003, 160) => {
            String::from("Nano Progra")
        }
        (2003, 18) => {
            String::from("Stamina")
        }
        (2003, 455) => {
            String::from("Monster Type")
        }
        (2003, 431) => {
            String::from("Selected target")
        }
        (2003, 148) => {
            String::from("Burst")
        }
        (2003, 537) => {
            String::from("Direct Nano Damage Vulnerability")
        }
        (2003, 38) => {
            String::from("CombatRange")
        }
        (2003, 16) => {
            String::from("Strength")
        }
        (2003, 583) => {
            String::from("ApartmentsOwned")
        }
        (2010, 206) => {
            String::from("Programmer")
        }
        (2010, 405) => {
            String::from("Leader")
        }
        (2010, 453) => {
            String::from("Cutthroat")
        }
        (2010, 503) => {
            String::from("Intern")
        }
        (2010, 753) => {
            String::from("Silencer")
        }
        (2010, 208) => {
            String::from("Analyst")
        }
        (2010, 551) => {
            String::from("Theorist")
        }
        (2010, 610) => {
            String::from("Harbinger")
        }
        (2010, 758) => {
            String::from("Veiled")
        }
        (2010, 559) => {
            String::from("Nanobinder")
        }
        (2010, 2010) => {
            String::from("Champion")
        }
        (2010, 409) => {
            String::from("Director")
        }
        (2010, 750) => {
            String::from("None")
        }
        (2010, 100) => {
            String::from("None")
        }
        (2010, 309) => {
            String::from("Outrider")
        }
        (2010, 454) => {
            String::from("Obliterator")
        }
        (2010, 606) => {
            String::from("Visionary")
        }
        (2010, 1009) => {
            String::from("Honored")
        }
        (2010, 609) => {
            String::from("Summoner")
        }
        (2010, 159) => {
            String::from("Deconstructor")
        }
        (2010, 452) => {
            String::from("Brute")
        }
        (2010, 2003) => {
            String::from("Rookie")
        }
        (2010, 300) => {
            String::from("None")
        }
        (2010, 407) => {
            String::from("Administrator")
        }
        (2010, 457) => {
            String::from("Crusher")
        }
        (2010, 200) => {
            String::from("None")
        }
        (2010, 151) => {
            String::from("Vandal")
        }
        (2010, 560) => {
            String::from("Arch-Magus")
        }
        (2010, 103) => {
            String::from("Okuiri")
        }
        (2010, 350) => {
            String::from("None")
        }
        (2010, 459) => {
            String::from("Eradicator")
        }
        (2010, 650) => {
            String::from("None")
        }
        (2010, 56) => {
            String::from("Drill Sergeant")
        }
        (2010, 55) => {
            String::from("Sergeant")
        }
        (2010, 607) => {
            String::from("Psion")
        }
        (2010, 303) => {
            String::from("Poacher")
        }
        (2010, 707) => {
            String::from("Crusader")
        }
        (2010, 709) => {
            String::from("Partisan")
        }
        (2010, 752) => {
            String::from("Butcher")
        }
        (2010, 153) => {
            String::from("Defacer")
        }
        (2010, 1010) => {
            String::from("Hero")
        }
        (2010, 2008) => {
            String::from("Experienced")
        }
        (2010, 259) => {
            String::from("Eraser")
        }
        (2010, 158_737_026) => {
            String::from("Grand Master")
        }
        (2010, 1003) => {
            String::from("Captain")
        }
        (2010, 53) => {
            String::from("Lance-Corporal")
        }
        (2010, 2005) => {
            String::from("Novice")
        }
        (2010, 60) => {
            String::from("Lieutenant General")
        }
        (2010, 105) => {
            String::from("Menkyo")
        }
        (2010, 210) => {
            String::from("Guru")
        }
        (2010, 310) => {
            String::from("Desperado")
        }
        (2010, 308) => {
            String::from("Pioneer")
        }
        (2010, 402) => {
            String::from("Assistant")
        }
        (2010, 460) => {
            String::from("Destroyer")
        }
        (2010, 553) => {
            String::from("Illusionist")
        }
        (2010, 657) => {
            String::from("None")
        }
        (2010, 705) => {
            String::from("Banneret")
        }
        (2010, 706) => {
            String::from("Conqueror")
        }
        (2010, 659) => {
            String::from("None")
        }
        (2010, 202) => {
            String::from("Scripter")
        }
        (2010, 756) => {
            String::from("Obfuscator")
        }
        (2010, 1006) => {
            String::from("Commander")
        }
        (2010, 1008) => {
            String::from("Distinguished")
        }
        (2010, 506) => {
            String::from("M.D.")
        }
        (2010, 359) => {
            String::from("Magnate")
        }
        (2010, 558) => {
            String::from("Thaumaturge")
        }
        (2010, 156) => {
            String::from("Saboteur")
        }
        (2010, 2004) => {
            String::from("Apprentice")
        }
        (2010, 760) => {
            String::from("Dominus Umbra")
        }
        (2010, 109) => {
            String::from("Kengo")
        }
        (2010, 404) => {
            String::from("Manager")
        }
        (2010, 59) => {
            String::from("Brigadier")
        }
        (2010, 305) => {
            String::from("Vagabond")
        }
        (2010, 552) => {
            String::from("Technician")
        }
        (2010, 1004) => {
            String::from("Tactician")
        }
        (2010, 52) => {
            String::from("Gunner")
        }
        (2010, 602) => {
            String::from("Mentalist")
        }
        (2010, 160) => {
            String::from("Annhilator")
        }
        (2010, 2001) => {
            String::from("Student")
        }
        (2010, 500) => {
            String::from("None")
        }
        (2010, 406) => {
            String::from("Chief")
        }
        (2010, 257) => {
            String::from("Executioner")
        }
        (2010, 252) => {
            String::from("Marksman")
        }
        (2010, 660) => {
            String::from("teh lulz")
        }
        (2010, 101) => {
            String::from("Ashigaru")
        }
        (2010, 107) => {
            String::from("Dai-Shihan")
        }
        (2010, 2002) => {
            String::from("Freshman")
        }
        (2010, 2009) => {
            String::from("Expert")
        }
        (2010, 356) => {
            String::from("Dealer")
        }
        (2010, 554) => {
            String::from("Catalyst")
        }
        (2010, 702) => {
            String::from("Squire")
        }
        (2010, 703) => {
            String::from("Chevalier")
        }
        (2010, 556) => {
            String::from("Elementalist")
        }
        (2010, 201) => {
            String::from("Phreaker")
        }
        (2010, 2006) => {
            String::from("Neophyte")
        }
        (2010, 51) => {
            String::from("Private")
        }
        (2010, 358) => {
            String::from("Distributor")
        }
        (2010, 205) => {
            String::from("Shifter")
        }
        (2010, 258) => {
            String::from("Liquidator")
        }
        (2010, 157) => {
            String::from("Pyrotechnician")
        }
        (2010, 450) => {
            String::from("None")
        }
        (2010, 104) => {
            String::from("Mokuroku")
        }
        (2010, 353) => {
            String::from("Retailer")
        }
        (2010, 658) => {
            String::from("None")
        }
        (2010, 150) => {
            String::from("None")
        }
        (2010, 507) => {
            String::from("Physician")
        }
        (2010, 456) => {
            String::from("Slaughterer")
        }
        (2010, 102) => {
            String::from("Mudansha")
        }
        (2010, 302) => {
            String::from("Drifter")
        }
        (2010, 555) => {
            String::from("Accelerant")
        }
        (2010, 557) => {
            String::from("Kineticist")
        }
        (2010, 605) => {
            String::from("Diviner")
        }
        (2010, 708) => {
            String::from("Hallowed")
        }
        (2010, 254) => {
            String::from("Sniper")
        }
        (2010, 58) => {
            String::from("Major")
        }
        (2010, 354) => {
            String::from("Entrepreneur")
        }
        (2010, 351) => {
            String::from("Hustler")
        }
        (2010, 1007) => {
            String::from("General")
        }
        (2010, 301) => {
            String::from("Tracker")
        }
        (2010, 1000) => {
            String::from("None")
        }
        (2010, 455) => {
            String::from("Slayer")
        }
        (2010, 306) => {
            String::from("Chaser")
        }
        (2010, 355) => {
            String::from("Marketer")
        }
        (2010, 2000) => {
            String::from("None")
        }
        (2010, 600) => {
            String::from("None")
        }
        (2010, 508) => {
            String::from("Toxicologist")
        }
        (2010, 604) => {
            String::from("Medium")
        }
        (2010, 652) => {
            String::from("None")
        }
        (2010, 1002) => {
            String::from("Suppressor")
        }
        (2010, 255) => {
            String::from("Killer")
        }
        (2010, 403) => {
            String::from("Supervisor")
        }
        (2010, 203) => {
            String::from("Rigger")
        }
        (2010, 110) => {
            String::from("Kensei")
        }
        (2010, 501) => {
            String::from("Chiropractor")
        }
        (2010, 701) => {
            String::from("Ward")
        }
        (2010, 2007) => {
            String::from("Veteran")
        }
        (2010, 209) => {
            String::from("Security Expert")
        }
        (2010, 154) => {
            String::from("Breaker")
        }
        (2010, 504) => {
            String::from("Aide")
        }
        (2010, 601) => {
            String::from("Clairvoyant")
        }
        (2010, 158) => {
            String::from("Demolitionist")
        }
        (2010, 108) => {
            String::from("Shugyosha")
        }
        (2010, 155) => {
            String::from("Sapper")
        }
        (2010, 253) => {
            String::from("Hunter")
        }
        (2010, 704) => {
            String::from("Knight")
        }
        (2010, 751) => {
            String::from("Stalker")
        }
        (2010, 250) => {
            String::from("None")
        }
        (2010, 550) => {
            String::from("None")
        }
        (2010, 408) => {
            String::from("Executive")
        }
        (2010, 755) => {
            String::from("Mirage")
        }
        (2010, 410) => {
            String::from("Chancellor")
        }
        (2010, 152) => {
            String::from("Wrecker")
        }
        (2010, 700) => {
            String::from("None")
        }
        (2010, 304) => {
            String::from("Traveler")
        }
        (2010, 1005) => {
            String::from("Leader")
        }
        (2010, 352) => {
            String::from("Peddler")
        }
        (2010, 106) => {
            String::from("Kaiden")
        }
        (2010, 50) => {
            String::from("None")
        }
        (2010, 651) => {
            String::from("None")
        }
        (2010, 710) => {
            String::from("Templar")
        }
        (2010, 757) => {
            String::from("Hazed")
        }
        (2010, 251) => {
            String::from("Prowler")
        }
        (2010, 759) => {
            String::from("Eidolon")
        }
        (2010, 451) => {
            String::from("Mook")
        }
        (2010, 207) => {
            String::from("Operator")
        }
        (2010, 357) => {
            String::from("Wholesaler")
        }
        (2010, 505) => {
            String::from("Resident")
        }
        (2010, 655) => {
            String::from("None")
        }
        (2010, 656) => {
            String::from("None")
        }
        (2010, 1001) => {
            String::from("Scout")
        }
        (2010, 57) => {
            String::from("Grenadier")
        }
        (2010, 260) => {
            String::from("Professional")
        }
        (2010, 502) => {
            String::from("Pharmacist")
        }
        (2010, 256) => {
            String::from("Eliminator")
        }
        (2010, 603) => {
            String::from("Psychic")
        }
        (2010, 654) => {
            String::from("None")
        }
        (2010, 754) => {
            String::from("Defiler")
        }
        (2010, 400) => {
            String::from("None")
        }
        (2010, 401) => {
            String::from("Receptionist")
        }
        (2010, 204) => {
            String::from("Hacker")
        }
        (2010, 360) => {
            String::from("Tycoon")
        }
        (2010, 510) => {
            String::from("Archiater")
        }
        (2010, 509) => {
            String::from("Virologist")
        }
        (2010, 54) => {
            String::from("Corporal")
        }
        (2010, 608) => {
            String::from("Spiritualist")
        }
        (2010, 653) => {
            String::from("None")
        }
        (2010, 307) => {
            String::from("Explorer")
        }
        (2010, 458) => {
            String::from("Pulverizer")
        }
        (2012, 183_860_754) => {
            String::from("Shortcut Bars")
        }
        (2012, 75_149_406) => {
            String::from("Actions")
        }
        (2012, 94_932_420) => {
            String::from("Targets")
        }
        (2012, 114_048_788) => {
            String::from("Movements")
        }
        (2012, 371_956) => {
            String::from("Text")
        }
        (2012, 77_085_825) => {
            String::from("Cameras")
        }
        (2012, 98_585_447) => {
            String::from("Windows")
        }
        (2013, 562) => {
            String::from("Omni-Med")
        }
        (2013, 9) => {
            String::from("Piercing")
        }
        (2013, 566) => {
            String::from("Guardian of Shadow")
        }
        (2013, 15) => {
            String::from("Grenade")
        }
        (2013, 569) => {
            String::from("The Unredeemed")
        }
        (2013, 4) => {
            String::from("SMG")
        }
        (2013, 0) => {
            String::from("Unarmed")
        }
        (2013, 16) => {
            String::from("Heavy")
        }
        (2013, 567) => {
            String::from("Followers")
        }
        (2013, 570) => {
            String::from("The Devoted")
        }
        (2013, 12) => {
            String::from("Rifle")
        }
        (2013, 3) => {
            String::from("Bow")
        }
        (2013, 564) => {
            String::from("Omni-Trans")
        }
        (2013, 1) => {
            String::from("Melee")
        }
        (2013, 11) => {
            String::from("Assault Rifle")
        }
        (2013, 2) => {
            String::from("Long Range")
        }
        (2013, 571) => {
            String::from("The Benign Conservers")
        }
        (2013, 8) => {
            String::from("2 Handed Blunt")
        }
        (2013, 565) => {
            String::from("Clan Vanguards")
        }
        (2013, 572) => {
            String::from("The Redeemed")
        }
        (2013, 13) => {
            String::from("Shotgun")
        }
        (2013, 5) => {
            String::from("1 Handed Edged")
        }
        (2013, 6) => {
            String::from("1 Handed Blunt")
        }
        (2013, 14) => {
            String::from("Energy")
        }
        (2013, 568) => {
            String::from("The Assertive Operators")
        }
        (2013, 561) => {
            String::from("The Sentinels")
        }
        (2013, 7) => {
            String::from("2 Handed Edged")
        }
        (2013, 563) => {
            String::from("Gaia")
        }
        (2013, 560) => {
            String::from("Omni-Armed Forces")
        }
        (2013, 10) => {
            String::from("Pistol")
        }
        (2014, 566) => {
            String::from("Guardian of Shadow")
        }
        (2014, 567) => {
            String::from("Followers")
        }
        (2014, 568) => {
            String::from("The Assertive Operators")
        }
        (2014, 571) => {
            String::from("The Benign Conservers")
        }
        (2014, 572) => {
            String::from("The Redeemed")
        }
        (2014, 563) => {
            String::from("Gaia")
        }
        (2014, 561) => {
            String::from("The Sentinels")
        }
        (2014, 564) => {
            String::from("Omni-Trans")
        }
        (2014, 560) => {
            String::from("Omni-Armed Forces")
        }
        (2014, 562) => {
            String::from("Omni-Med")
        }
        (2014, 565) => {
            String::from("Clan Vanguards")
        }
        (2014, 570) => {
            String::from("The Devoted")
        }
        (2014, 569) => {
            String::from("The Unredeemed")
        }
        (20000, 81_038_248) => {
            String::from("the receivers inbox is full")
        }
        (20000, 201_379_664) => {
            String::from("Could not find any help for that command.")
        }
        (20000, 172_363_154) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Offline message from {a} ({b})")
        }
        (20000, 54_583_877) => {
            let a = &arguments[0];
            format!("Could not send message to offline player: {a}")
        }
        (20000, 93_798_454) => {
            String::from("You can not add/remove yourself.")
        }
        (20000, 264_895_893) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            format!("Player {a} (id: {b}) is currently offline\r\nLast known level: {c}, and playfield: {d}")
        }
        (20000, 51_031_966) => {
            String::from("You have no active petition with the GM or Advisor you are trying to contact.\r\nTo get in touch with an Advisor, please file a petition using /petition <message>.\r\nBefore you do, please check known issues and fixes at http://community.anarchy-online.com/")
        }
        (20000, 166_867_336) => {
            String::from("You need to add a player name after \"/cc addbuddy\" and \"/cc rembuddy\".")
        }
        (20000, 223_936_404) => {
            let a = &arguments[0];
            format!("{a} has been removed from your buddy-list.")
        }
        (20000, 247_680_077) => {
            String::from("Available /cc commands:\r\n/cc help [command]\r\n/cc gag ...\r\n/cc ungag ...\r\n/cc pinfo <player>\r\n/cc gmdeaf\r\n/cc addbuddy <player>\r\n/cc rembuddy <player>\r\n/cc rembuddy ?\r\n/cc motd ...")
        }
        (20000, 544_567_129) => {
            String::new()
        }
        (20000, 158_601_204) => {
            let a = &arguments[0];
            format!("{a} is offline, message has been buffered.")
        }
        (20000, 55_531_266) => {
            String::from("Available /cc commands:\r\n/cc help [command]\r\n/cc addbuddy <player>\r\n/cc rembuddy <player>\r\n/cc rembuddy ?")
        }
        (20000, 175_017_231) => {
            let a = &arguments[0];
            format!("Motd has been set to \"{a}\".")
        }
        (20000, 230_313_758) => {
            let a = &arguments[0];
            let b = {
                if a.to_string() == "1" {
                    "minute "
                } else {
                    " minutes "
                }
            };
            format!("Current gag will time out in {a} {b}.")
        }
        (20000, 206_649_861) => {
            let a = &arguments[0];
            format!("No player with name {a} was found.")
        }
        (20000, 18_838_393) => {
            let a = &arguments[0];
            let b = {
                if a.to_string() == "1" {
                    "buddy "
                } else {
                    " buddies "
                }
            };
            format!("Removing {a} {b}.")
        }
        (20000, 32_005_140) => {
            let a = &arguments[0];
            format!("{a} has been added to your buddy-list.")
        }
        (20000, 100_199_604) => {
            let a = &arguments[0];
            format!("Client {a} does not exist/is not online at the moment.")
        }
        (20000, 197_338_868) => {
            let a = &arguments[0];
            format!("You have reached the maximum number of friends ({a}) you can have in the buddy list.")
        }
        (20000, 61_503_945) => {
            String::from("Your ability to talk out loud has been revoked temporarily with a GM gag.")
        }
        (20000, 201_146_790) => {
            String::from("The gmdeaf command is used to enable disable a special \"deaf\" mode for gm's.\r\nWhen a GM has the gmdeaf mode enabled, ha can only receive /tell messages\r\nfrom other gm's or from people in his buddy-list. To ease the pain of\r\nadding/removing players from the buddy-list, GM's can use the /cc addbuddy\r\nand /cc rembuddy commands.\r\n/cc gmdeaf - toggles the gmdeaf flags, it's default off.\r\n")
        }
        (20000, 170_904_871) => {
            String::from("Your client is sending a high frequency of chat messages.\r\nThe last message you sent has been deleted by flood protection.\r\nPlease check your scripts if they include chat messages and remove any loops that might have cause this to happen.'")
        }
        (20000, 238_833_796) => {
            let a = &arguments[0];
            format!("Could not find the player with id {a} in the chatserver database.")
        }
        (20000, 226_911_017) => {
            String::from("This comand is used to add other players to your friendslist.\r\n/cc addbuddy <player> - will add \"player\" to the friendslist.")
        }
        (20000, 1_952_522_356) => {
            String::new()
        }
        (20000, 61_614_736) => {
            String::from("Your ability to talk in private groups has been revoked temporarily with a GM gag.")
        }
        (20000, 134_870_373) => {
            String::from("Your ability to send private messages has been revoked temporarily with a GM gag.")
        }
        (20000, 246_267_543) => {
            String::from("The /cc [un]gag commands are used to gag players.\r\n/cc gag <player> - Completely gags the player, he will still be able to hear, but he can't say anything.\r\n/cc ungag <player> - Removes all gags from the player.\r\n/cc gag <player> vicinity - The player will not be able to send vicinity chat.\r\n/cc ungag <player> vicinity - Removes the vicinity gag.\r\n/cc gag <player> groups - The player will not be able to send to any king of group chat.\r\n/cc ungag <player> groups - Removes the groups gag.\r\n/cc gag <player> tell - The player will not be able to use the /tell command.\r\n/cc ungag <player> tell - Removes the tell gag.\r\n/cc gag <player> timeout <minutes> - All the players gags will timeout after x minutes.\r\n/cc gag <player> timeout 0 - Disables the timeout.\r\n/cc gag <player> list - Shows what gags a player have.\r\n/cc gag list - Shows a list of all gagged players.")
        }
        (20000, 74_098_291) => {
            let a = &arguments[0];
            format!("Currect motd: \"{a}\".")
        }
        (20000, 258_810_596) => {
            String::from("Motd has been cleared.")
        }
        (20000, 83_258_804) => {
            let a = &arguments[0];
            format!("Illegal argument to help: {a}")
        }
        (20000, 182_196_292) => {
            String::from("/cc motd is used to set and clear the message players see when they log in.\r\n\"/cc motd\" - will show the current motd message.\r\n\"/cc motd clear\" - will reset the message back to default.\r\n\"/cc motd <text>\" - will set the message to \"text\".\r\n")
        }
        (20000, 56_089_684) => {
            let a = &arguments[0];
            format!("{a} was not in your buddy-list.")
        }
        (20000, 207_061_892) => {
            let a = &arguments[0];
            format!("{a} is already in your buddy-list.")
        }
        (20000, 265_276_023) => {
            String::from("the message is too big to fit in the inbox")
        }
        (20000, 218_303_104) => {
            String::from("Your ability to talk in public groups has been revoked temporarily with a GM gag.")
        }
        (20000, 219_570_393) => {
            String::from("This comand can be used to remove players from you friendslist.\r\n/cc rembuddy <player> - will remove \"player\" from your list.\r\n/cc rembuddy ? - will remove all unknown friends (those listed under the question-mark in the client)\r\n")
        }
        (10010, 197_338_868) => {
            let a = &arguments[0];
            format!("You have reached the maximum number of friends ({a}) you can have in the buddy list.")
        }
        (10010, 265_276_023) => {
            String::from("the message is too big to fit in the inbox")
        }
        (10010, 55_531_266) => {
            String::from("Available /cc commands:\r\n/cc help [command]\r\n/cc addbuddy <player>\r\n/cc rembuddy <player>\r\n/cc rembuddy ?")
        }
        (10010, 230_313_758) => {
            let a = &arguments[0];
            let b = {
                if a.to_string() == "1" {
                    "minute "
                } else {
                    " minutes "
                }
            };
            format!("Current gag will time out in {a} {b}.")
        }
        (10010, 2) => {
            String::from("Melee Weapons")
        }
        (10010, 8) => {
            String::from("Combat & Healing")
        }
        (10010, 175_017_231) => {
            let a = &arguments[0];
            format!("Motd has been set to \"{a}\".")
        }
        (10010, 238_833_796) => {
            let a = &arguments[0];
            format!("Could not find the player with id {a} in the chatserver database.")
        }
        (10010, 81_038_248) => {
            String::from("the receivers inbox is full")
        }
        (10010, 100_199_604) => {
            let a = &arguments[0];
            format!("Client {a} does not exist/is not online at the moment.")
        }
        (10010, 172_363_154) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Offline message from {a} ({b})")
        }
        (10010, 247_680_077) => {
            String::from("Available /cc commands:\r\n/cc help [command]\r\n/cc gag ...\r\n/cc ungag ...\r\n/cc pinfo <player>\r\n/cc gmdeaf\r\n/cc addbuddy <player>\r\n/cc rembuddy <player>\r\n/cc rembuddy ?\r\n/cc motd ...")
        }
        (10010, 18_838_393) => {
            let a = &arguments[0];
            let b = {
                if a.to_string() == "1" {
                    "buddy "
                } else {
                    " buddies "
                }
            };
            format!("Removing {a} {b}.")
        }
        (10010, 4) => {
            String::from("Ranged Weapons")
        }
        (10010, 83_258_804) => {
            let a = &arguments[0];
            format!("Illegal argument to help: {a}")
        }
        (10010, 134_870_373) => {
            String::from("Your ability to send private messages has been revoked temporarily with a GM gag.")
        }
        (10010, 3) => {
            String::from("Melee Specials")
        }
        (10010, 201_146_790) => {
            String::from("The gmdeaf command is used to enable disable a special \"deaf\" mode for gm's.\r\nWhen a GM has the gmdeaf mode enabled, ha can only receive /tell messages\r\nfrom other gm's or from people in his buddy-list. To ease the pain of\r\nadding/removing players from the buddy-list, GM's can use the /cc addbuddy\r\nand /cc rembuddy commands.\r\n/cc gmdeaf - toggles the gmdeaf flags, it's default off.\r\n")
        }
        (10010, 223_936_404) => {
            let a = &arguments[0];
            format!("{a} has been removed from your buddy-list.")
        }
        (10010, 51_031_966) => {
            String::from("You have no active petition with the GM or Advisor you are trying to contact.\r\nTo get in touch with an Advisor, please file a petition using /petition <message>.\r\nBefore you do, please check known issues and fixes at http://community.anarchy-online.com/")
        }
        (10010, 56_089_684) => {
            let a = &arguments[0];
            format!("{a} was not in your buddy-list.")
        }
        (10010, 258_810_596) => {
            String::from("Motd has been cleared.")
        }
        (10010, 206_649_861) => {
            let a = &arguments[0];
            format!("No player with name {a} was found.")
        }
        (10010, 264_895_893) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            format!("Player {a} (id: {b}) is currently offline\r\nLast known level: {c}, and playfield: {d}")
        }
        (10010, 201_379_664) => {
            String::from("Could not find any help for that command.")
        }
        (10010, 9) => {
            String::from("Trade & Repair")
        }
        (10010, 544_567_129) => {
            String::new()
        }
        (10010, 1_952_522_356) => {
            String::new()
        }
        (10010, 54_583_877) => {
            let a = &arguments[0];
            format!("Could not send message to offline player: {a}")
        }
        (10010, 1) => {
            String::from("Body & Defense")
        }
        (10010, 32_005_140) => {
            let a = &arguments[0];
            format!("{a} has been added to your buddy-list.")
        }
        (10010, 166_867_336) => {
            String::from("You need to add a player name after \"/cc addbuddy\" and \"/cc rembuddy\".")
        }
        (10010, 246_267_543) => {
            String::from("The /cc [un]gag commands are used to gag players.\r\n/cc gag <player> - Completely gags the player, he will still be able to hear, but he can't say anything.\r\n/cc ungag <player> - Removes all gags from the player.\r\n/cc gag <player> vicinity - The player will not be able to send vicinity chat.\r\n/cc ungag <player> vicinity - Removes the vicinity gag.\r\n/cc gag <player> groups - The player will not be able to send to any king of group chat.\r\n/cc ungag <player> groups - Removes the groups gag.\r\n/cc gag <player> tell - The player will not be able to use the /tell command.\r\n/cc ungag <player> tell - Removes the tell gag.\r\n/cc gag <player> timeout <minutes> - All the players gags will timeout after x minutes.\r\n/cc gag <player> timeout 0 - Disables the timeout.\r\n/cc gag <player> list - Shows what gags a player have.\r\n/cc gag list - Shows a list of all gagged players.")
        }
        (10010, 182_196_292) => {
            String::from("/cc motd is used to set and clear the message players see when they log in.\r\n\"/cc motd\" - will show the current motd message.\r\n\"/cc motd clear\" - will reset the message back to default.\r\n\"/cc motd <text>\" - will set the message to \"text\".\r\n")
        }
        (10010, 5) => {
            String::from("Ranged Specials")
        }
        (10010, 0) => {
            String::from("Abilities")
        }
        (10010, 158_601_204) => {
            let a = &arguments[0];
            format!("{a} is offline, message has been buffered.")
        }
        (10010, 218_303_104) => {
            String::from("Your ability to talk in public groups has been revoked temporarily with a GM gag.")
        }
        (10010, 61_614_736) => {
            String::from("Your ability to talk in private groups has been revoked temporarily with a GM gag.")
        }
        (10010, 7) => {
            String::from("Exploring")
        }
        (10010, 6) => {
            String::from("Nanos & Casting")
        }
        (10010, 170_904_871) => {
            String::from("Your client is sending a high frequency of chat messages.\r\nThe last message you sent has been deleted by flood protection.\r\nPlease check your scripts if they include chat messages and remove any loops that might have cause this to happen.'")
        }
        (10010, 61_503_945) => {
            String::from("Your ability to talk out loud has been revoked temporarily with a GM gag.")
        }
        (10010, 74_098_291) => {
            let a = &arguments[0];
            format!("Currect motd: \"{a}\".")
        }
        (10010, 93_798_454) => {
            String::from("You can not add/remove yourself.")
        }
        (10010, 207_061_892) => {
            let a = &arguments[0];
            format!("{a} is already in your buddy-list.")
        }
        (10010, 219_570_393) => {
            String::from("This comand can be used to remove players from you friendslist.\r\n/cc rembuddy <player> - will remove \"player\" from your list.\r\n/cc rembuddy ? - will remove all unknown friends (those listed under the question-mark in the client)\r\n")
        }
        (10010, 226_911_017) => {
            String::from("This comand is used to add other players to your friendslist.\r\n/cc addbuddy <player> - will add \"player\" to the friendslist.")
        }
        (10010, 9999) => {
            String::from("Disabled / Legacy")
        }
        (4_294_967_295, 1_952_522_356) => {
            String::new()
        }
        (4_294_967_295, 544_567_129) => {
            String::new()
        }
        (2017, 257_013) => {
            String::from("Uncle Bazzit's Alien Library")
        }
        (2017, 686_000) => {
            String::from("Returned Credit Card")
        }
        (2017, 40_655_268) => {
            String::from("The bug report has been sent; thank you for reporting it.")
        }
        (2017, 686_015) => {
            String::from("Certificate of Travel")
        }
        (2017, 686_012) => {
            String::from("The Shiny Sword")
        }
        (2017, 6_399_012) => {
            String::from("<font color=CCInfoHeader>You are about to report this bug:</font><br>\r\n<font color=CCInfoText>")
        }
        (2017, 686_007) => {
            String::from("Specialist Commerce Access")
        }
        (2017, 65016) => {
            String::from("Alien Technology Mastery")
        }
        (2017, 686_001) => {
            String::from("Stolen Credits")
        }
        (2017, 16_883_060) => {
            String::from("The bug report was not sent.")
        }
        (2017, 179_026_692) => {
            String::from("The bug report could not be sent.  Reason: Unknown.")
        }
        (100, 567) => {
            let a = &arguments[0];
            format!("You increased your nanopool with {a} points.")
        }
        (100, 623) => {
            String::from("You must be able to use weapons!")
        }
        (100, 646) => {
            String::from("Withdraw cash from the shop:")
        }
        (100, 649) => {
            String::from(" /shop add <amount>")
        }
        (100, 702) => {
            String::from("Unable to find target item?")
        }
        (100, 704) => {
            String::from("No skill on this combination!")
        }
        (100, 711) => {
            String::from("This action requires that you are not wearing Grid Armor!")
        }
        (100, 70) => {
            String::from("Target is not in line of sight!")
        }
        (100, 191) => {
            String::from("Reloading completed")
        }
        (100, 358) => {
            String::from("You must be standing up to execute a nano program.")
        }
        (100, 28) => {
            String::from("Players cannot join a team while either they or the team are in a fight.")
        }
        (100, 18) => {
            String::from("You've received an item as mission reward!")
        }
        (100, 448) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Something hit {a} for {b} points of damage by reflect shield.")
        }
        (100, 489) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            format!("You combined \"{a}\" with \"{b}\" and the result is a quality level {c} \"{d}\".")
        }
        (100, 571) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Your team and your targets team share members from the same organization: {a} and {b}")
        }
        (100, 342) => {
            String::from("All team members can loot these remains.")
        }
        (100, 39) => {
            String::from("You need to select a target for this action!")
        }
        (100, 328) => {
            String::from("/team loot [all/leader/alpha]")
        }
        (100, 400) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            let e = &arguments[4];
            format!("{a} has played {b} days {c} hours {d} minutes {e} seconds")
        }
        (100, 310) => {
            String::from("Fear activated.")
        }
        (100, 293) => {
            String::from("Your attack was blocked by an attack shield!")
        }
        (100, 43) => {
            String::from("Your mission has failed. Target was not killed properly!")
        }
        (100, 469) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Your summon target was bound at position {a},{b}. You need to move closer to that position before you are allowed to summon him.")
        }
        (100, 8) => {
            String::from("Target does not have enough nano controlling units (NCU) left.")
        }
        (100, 288) => {
            String::from("Your inventory is full. An item has been put into your bank.")
        }
        (100, 381) => {
            String::from("You cannot attack your pet.")
        }
        (100, 94) => {
            String::from("You need to be an organization member to perform this action!")
        }
        (100, 154) => {
            String::from("NoDrop item can't be traded or sold.")
        }
        (100, 651) => {
            String::from(" /shop price <pos> <price>")
        }
        (100, 100) => {
            String::from("Your target is in the same organization as you")
        }
        (100, 550) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("{a} {b} is not less than {c}!")
        }
        (100, 23) => {
            String::from("No room in your inventory. Item sent to bank.")
        }
        (100, 695) => {
            String::from("Stationary source item too far away!")
        }
        (100, 474) => {
            let a = &arguments[0];
            format!("Character stored. {a} Shadowknowledge saved.")
        }
        (100, 490) => {
            String::from("It is not possible to combine these two items in that order.")
        }
        (100, 343) => {
            String::from("You can loot these remains.")
        }
        (100, 433) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("Player {a} hit you for {b} points of {c} damage.")
        }
        (100, 595) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Your nano program {a} has stopped running on {b}...")
        }
        (100, 242) => {
            String::from("Your inventory overflowed! Get the items before leaving the area or they will be deleted.")
        }
        (100, 547) => {
            let a = &arguments[0];
            format!("This effect can't be utilitized by {a}!")
        }
        (100, 5) => {
            String::from("No items in reclaim booth. Try later.")
        }
        (100, 158) => {
            String::from("Items must reside in the same inventory.")
        }
        (100, 520) => {
            let a = &arguments[0];
            format!("{a} xp was gained as a side bonus!")
        }
        (100, 460) => {
            String::from("Team member must be present!")
        }
        (100, 573) => {
            let a = &arguments[0];
            format!("One of your targets team members ({a}) is in the same organization as you")
        }
        (100, 223) => {
            String::from("You can't save here!")
        }
        (100, 211_769_543) => {
            String::from("Flying")
        }
        (100, 5_290_846) => {
            String::from("Looking for team: ON")
        }
        (100, 111) => {
            String::from("No hidden objects found.")
        }
        (100, 357) => {
            String::from("You can't execute nanoprograms while falling!")
        }
        (100, 311) => {
            String::from("The selected target needs to be an NPC.")
        }
        (100, 255) => {
            String::from("Lockpicking failed.")
        }
        (100, 169) => {
            String::from("This is a NoDrop item - it can't be dropped.")
        }
        (100, 285) => {
            String::from("Unique item returned to bank.")
        }
        (100, 313) => {
            String::from("Item can't be used as ammo.")
        }
        (100, 504) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("You were unaffected by {a} from {b}.")
        }
        (100, 95) => {
            String::from("You need to be an organization leader to perform this action!")
        }
        (100, 555) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("{a} {b} base value is not less than {c}!")
        }
        (100, 560) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("{a} must not have {b} running!")
        }
        (100, 145_730_516) => {
            let a = &arguments[0];
            format!("Your Organization cannot initiate a bombing raid at the moment. Please wait at least {a} seconds.")
        }
        (100, 488) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("\"{a}\" is of a too low quality level. It must be at least at quality {b}.")
        }
        (100, 549) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("{a} {b} is required to be at least {c}!")
        }
        (100, 88) => {
            String::from("You cannot unlearn a Perk you do not know!")
        }
        (100, 84_653_510) => {
            String::from("Looking for team: OFF")
        }
        (100, 14) => {
            String::from("You no longer meet the criteria for beeing a member of this organization!")
        }
        (100, 599) => {
            let a = &arguments[0];
            format!("Your nano execution got interrupted by {a}..")
        }
        (100, 674) => {
            String::from("This action can only be done by the leader of the organization owning this shop!")
        }
        (100, 256) => {
            String::from("Arming trap successful.")
        }
        (100, 398) => {
            let a = &arguments[0];
            format!("Team member {a} went link dead.")
        }
        (100, 278) => {
            String::from("Maps options/map already uploaded.")
        }
        (100, 506) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("You have gained {a} new Reset Points. Your total Reset Point count is: {b}")
        }
        (100, 276) => {
            String::from("This action requires that you are wielding Augmented Cyberdeck!")
        }
        (100, 10) => {
            String::from("Adding fixture to the building was not allowed. The building has to have capacity and you must own it.")
        }
        (100, 271) => {
            String::from("This action requires that you have the second specialization completed!")
        }
        (100, 378) => {
            String::from("Attack not allowed since you are on the same side in the conflict.")
        }
        (100, 65) => {
            String::from("Warning - The immediate transfer of personal items to your new body after death has now ceased, and all items will remain with your corpse for 1 minute before being transfered to the item reclaim terminal.")
        }
        (100, 336) => {
            String::from("Invalid tower command.")
        }
        (100, 298) => {
            String::from("A too high level player in your team prevents you from receiving any experience.")
        }
        (100, 407) => {
            let a = &arguments[0];
            format!("{a} available.")
        }
        (100, 168) => {
            String::from("Item can't be dropped at that location!")
        }
        (100, 408) => {
            let a = &arguments[0];
            format!("The /stuck command will be resolved in {a} seconds.")
        }
        (100, 600) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("{a}'s execution of {b} got interrupted by {c}..")
        }
        (100, 596) => {
            let a = &arguments[0];
            format!("Nanoprogram {a} terminated...")
        }
        (100, 601) => {
            let a = &arguments[0];
            format!("Team looting set to: {a}")
        }
        (100, 653) => {
            String::from(" /shop private")
        }
        (100, 132) => {
            String::from("Your target can't be attacked.")
        }
        (100, 34) => {
            String::from("Fighting prevents mission checking from being performed. Try to repeat your actions after you finish the fight!")
        }
        (100, 424) => {
            let a = &arguments[0];
            format!("You were healed for {a} points.")
        }
        (100, 294) => {
            String::from("Your special attack was blocked by a special attack shield!")
        }
        (100, 195) => {
            String::from("You're unable to attack.")
        }
        (100, 430) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("{a} was attacked with nanobots for {b} points of {c} damage.")
        }
        (100, 274) => {
            String::from("This action requires that you are wielding Izgimmer-modified Cyberdeck!")
        }
        (100, 137) => {
            String::from("You found no items here!")
        }
        (100, 712) => {
            String::from("You already own a shop; you can't own more than one shop at a time!")
        }
        (100, 56) => {
            let a = &arguments[0];
            format!("No ammo was found for {a}!")
        }
        (100, 221) => {
            String::from("Must not be in combat mode.")
        }
        (100, 516) => {
            let a = &arguments[0];
            format!("You need at least {a} free inventory slots! Please remove an item and try again.")
        }
        (100, 54) => {
            String::from("Lockpicking successful.")
        }
        (100, 260) => {
            String::from("Target is unable to receive the effect!")
        }
        (100, 2) => {
            String::from("Your bodily remains have been made available.")
        }
        (100, 150) => {
            String::from("Target is outside range for trade. If you want to trade with the target, move closer!")
        }
        (100, 302) => {
            String::from("Nanobots are recharging, please wait.")
        }
        (100, 49) => {
            String::from("You just wasted an item.")
        }
        (100, 126) => {
            String::from("You start bluffing")
        }
        (100, 355) => {
            String::from("You can't execute nanoprograms while swimming!")
        }
        (100, 420) => {
            String::from("Crowd limiting was enforced. You were removed from the crowd.")
        }
        (100, 440) => {
            let a = &arguments[0];
            format!("Someone's damage shield hit you for {a} points of damage.")
        }
        (100, 305) => {
            String::from("Mindcontrol activated.")
        }
        (100, 471) => {
            let a = &arguments[0];
            format!("You can't afford to save. At your level you need {a} credits to save.")
        }
        (100, 578) => {
            let a = &arguments[0];
            format!("Team-mission chance of token reward upped to {a}% due to the team's heroic effort.")
        }
        (100, 103) => {
            String::from("Team-mission chance of token reward not upped due to too high level character in team.")
        }
        (100, 174) => {
            String::from("Items can't be used directly from the bank!")
        }
        (100, 270) => {
            String::from("This action requires that you have the third specialization completed!")
        }
        (100, 453) => {
            let a = &arguments[0];
            format!("You tried to hit {a}, but missed!")
        }
        (100, 245) => {
            String::from("It is locked.")
        }
        (100, 207) => {
            String::from("Please wait until previous action has finished.")
        }
        (100, 177) => {
            String::from("You can't attack while in this state.")
        }
        (100, 86) => {
            String::from("You cannot unlearn a Perk that is a requirement for another Perk.")
        }
        (100, 480) => {
            let a = &arguments[0];
            format!("Your special attack shield blocked the attack! ({a}) left.")
        }
        (100, 190) => {
            String::from("Use the Def-Agg slider in the Stats view to change between defensive and aggressive.")
        }
        (100, 359) => {
            String::from("You can't execute nano programs on items.")
        }
        (100, 439) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("You were hit for {a} points of damage by {b}'s damage shield.")
        }
        (100, 501) => {
            String::from("The reverse engineered items are of excellent quality!")
        }
        (100, 668) => {
            String::from(" /shop open")
        }
        (100, 213) => {
            String::from("Target must be sitting on ground in order to perform this action.")
        }
        (100, 376) => {
            String::from("It was not possible to add this item to the contract!")
        }
        (100, 139) => {
            String::from("You need to be standing on the ground in order to forage!")
        }
        (100, 673) => {
            String::from("This shop has been locked!")
        }
        (100, 676) => {
            String::from("The shop must be empty of items and cash!")
        }
        (100, 142) => {
            String::from("Item must be applied on self.")
        }
        (100, 442) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("{a}'s damage shield hit {b} for {c} points of damage.")
        }
        (100, 284) => {
            String::from("Area change not initiated on server. You may retreat or try crossing again.")
        }
        (100, 462) => {
            String::from("This can only be performed in the Shadowlands!")
        }
        (100, 259) => {
            String::from("No more charges.")
        }
        (100, 493) => {
            String::from("You can assemble these two items, with excellent quality!")
        }
        (100, 108) => {
            String::from("Target is dead. Trade aborted.")
        }
        (100, 38) => {
            String::from("You can't use this nano program at the moment.")
        }
        (100, 419) => {
            let a = &arguments[0];
            let b = {
                if a.to_string() == "1" {
                    "credit "
                } else {
                    " credits"
                }
            };
            format!("You received {a} {b} from the corpse.")
        }
        (100, 531) => {
            String::from("You need to have a bow weapon equipped.")
        }
        (100, 240) => {
            String::from("You can't dual wield this weapon with the already equipped weapon.")
        }
        (100, 693) => {
            let a = &arguments[0];
            format!("Broken item at: {a}.")
        }
        (100, 427) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("You were attacked with nanobots for {a} points of {b} damage.")
        }
        (100, 409) => {
            let a = &arguments[0];
            format!("The /stuck command will be available in {a} seconds.")
        }
        (100, 133) => {
            String::from("Temporary items can't be placed in containers!")
        }
        (100, 110) => {
            String::from("Warning! Trap detected.")
        }
        (100, 85) => {
            String::from("You will not gain Shadowknowledge until you have made your final choice of side!")
        }
        (100, 7) => {
            String::from("Wait for your previous special attack to complete.")
        }
        (100, 9) => {
            String::from("Nano program failed. Already executing nanoprogram.")
        }
        (100, 244) => {
            String::from("You locked the item.")
        }
        (100, 352) => {
            String::from("Executing programs is currently unavailable.")
        }
        (100, 62) => {
            String::from("You have reached the trade limit for this shop!")
        }
        (100, 497) => {
            String::from("The new item is of excellent quality!")
        }
        (100, 4) => {
            String::from("Your remains have not been reconstructed yet.")
        }
        (100, 66) => {
            String::from("Warning - Complimentary insurance buffer has been reduced. From now on, you will experience resurrection shock following lifeforce transfer to your new body.")
        }
        (100, 521) => {
            let a = &arguments[0];
            format!("You gained {a} points of Shadowknowledge.")
        }
        (100, 632) => {
            let a = &arguments[0];
            format!("ShopNet: {a} entries found:")
        }
        (100, 648) => {
            String::from("Add cash to the shop:")
        }
        (100, 297) => {
            String::from("You are awarded a token for your heroic effort.")
        }
        (100, 624) => {
            String::from("You must not be able to use weapons!")
        }
        (100, 356) => {
            String::from("You can't execute nanoprograms while crawling!")
        }
        (100, 346) => {
            String::from("The door is now unlocked.")
        }
        (100, 502) => {
            let a = &arguments[0];
            format!("Player {a} went link dead.")
        }
        (100, 370) => {
            String::from("Not allowed to execute friendly nanoprogram on anyone on the opposite side of your team in the conflict.")
        }
        (100, 375) => {
            String::from("Nano program failed. Too low level.")
        }
        (100, 657) => {
            let a = &arguments[0];
            format!("Executing Nano Program: {a}.")
        }
        (100, 208) => {
            String::from("Starting attack failed.")
        }
        (100, 421) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("You hit {a} for {b} points of damage.")
        }
        (100, 323) => {
            String::from("This mine has already been disarmed.")
        }
        (100, 609) => {
            String::from("You have completed the third specialization!")
        }
        (100, 716) => {
            let a = &arguments[0];
            format!("{a} doesn't own a shop!")
        }
        (100, 316) => {
            String::from("Shop contains no entries.")
        }
        (100, 389) => {
            String::from("Can't afford to change profession!")
        }
        (100, 451) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Someone absorbed {a} points of {b} damage.")
        }
        (100, 638) => {
            String::from("This is a player shop!")
        }
        (100, 267) => {
            String::from("You must have the Shadowlands Expansion Pack!")
        }
        (100, 314) => {
            String::from("Wrong ammotype.")
        }
        (100, 32) => {
            String::from("Can't pick up item. You don't own it.")
        }
        (100, 321) => {
            String::from("The mine was sprung! You failed disarming the mine!")
        }
        (100, 6) => {
            String::from("Special attack not possible. The target is aware of your presence.")
        }
        (100, 367) => {
            String::from("Your target countered the nano program.")
        }
        (100, 371) => {
            String::from("Nano program aborted.")
        }
        (100, 393) => {
            let a = &arguments[0];
            format!("Congratulations! You have now reached the level of {a}!")
        }
        (100, 423) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("{a} hit you for {b} points of {c} damage.")
        }
        (100, 656) => {
            String::from("Tradepartner is unable to carry more than 1 billion credits. Trade aborted.")
        }
        (100, 684) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("Org missing payment lockdown: time until reset {a}:{b}:{c}")
        }
        (100, 226_935_915) => {
            String::from("Must have perk: ")
        }
        (100, 350) => {
            String::from("Mission accomplished.")
        }
        (100, 652) => {
            String::from("Private shop (only available to characters on your account):")
        }
        (100, 639) => {
            let a = &arguments[0];
            format!("This is your private shop (max {a} items)!")
        }
        (100, 580) => {
            String::from("Burst!")
        }
        (100, 149) => {
            String::from("You've already got this nano program.")
        }
        (100, 500) => {
            String::from("The quality of the reverse engineered items are ok.")
        }
        (100, 217) => {
            String::from("Target must be fighting.")
        }
        (100, 388) => {
            String::from("Towers can only be attacked when the gaslevel is below 75%")
        }
        (100, 492) => {
            String::from("You can assemble these two items with ok quality results.")
        }
        (100, 63) => {
            String::from("You failed to hide as you are currently fighting.")
        }
        (100, 123) => {
            String::from("Already using an item.")
        }
        (100, 156) => {
            String::from("Items with a limited timespan can't be sold.")
        }
        (100, 720) => {
            String::from("Shop commands:\r\nstatus - give some status information about the target's shop\r\nitems - list the contents of the target's shop")
        }
        (100, 435) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("Something hit {a} for {b} points of {c} damage.")
        }
        (100, 700) => {
            String::from("Unable to get the target item, at all!")
        }
        (100, 340) => {
            String::from("You end sneaking.")
        }
        (100, 633) => {
            String::from("You must select a player shop first!")
        }
        (100, 426) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("You were attacked with {a} for {b} points of {c} damage.")
        }
        (100, 404) => {
            let a = &arguments[0];
            format!("{a} vanished.")
        }
        (100, 449) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("You absorbed {a} points of {b} damage.")
        }
        (100, 708) => {
            String::from("Deck items must be unequipped before they can be deleted!")
        }
        (100, 645) => {
            String::from(" /shop status")
        }
        (100, 444) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("You were hit for {a} points of damage by {b}'s reflect shield.")
        }
        (100, 52) => {
            String::from("Target can't be performing an item action while receiving this buff!")
        }
        (100, 542) => {
            let a = &arguments[0];
            format!("This effect can only be utilitized by {a}.")
        }
        (100, 82) => {
            String::from("You gained a new Shadowlevel!")
        }
        (100, 121) => {
            String::from("Unequipping items is forbidden here.")
        }
        (100, 303) => {
            String::from("You must sit in order to use /lounge.")
        }
        (100, 611) => {
            String::from("Do you want to rent this shop terminal?")
        }
        (100, 253) => {
            String::from("Repair roll successful.")
        }
        (100, 494) => {
            String::from("It is not possible to assemble those two items. Maybe the order was wrong?")
        }
        (100, 272) => {
            String::from("This action requires that you have the first specialization completed!")
        }
        (100, 721) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("The shop \"{a}\" owned by {b} contains the following items:")
        }
        (100, 118) => {
            String::from("Social armor cannot be worn with other clothes.")
        }
        (100, 366) => {
            String::from("You countered the nano program.")
        }
        (100, 117) => {
            String::from("Stuck-resolve command executing. New /stuck command will be available in 3 minutes.")
        }
        (100, 362) => {
            String::from("NCU error: Better nano program already running.")
        }
        (100, 296) => {
            String::from("You are awarded a token for your team's heroic effort.")
        }
        (100, 482) => {
            let a = &arguments[0];
            format!("You drained {a} points of health from the target.")
        }
        (100, 20) => {
            String::from("Already in use by someone else. Please wait your turn.")
        }
        (100, 161) => {
            String::from("You can't split as your inventory is full!")
        }
        (100, 198) => {
            String::from("Target has surrendered.")
        }
        (100, 391) => {
            String::from("You have now received extra Increase Points that you can distribute on your skills. Use the Skill Window for this.")
        }
        (100, 277) => {
            String::from("This action requires that you are wielding Basic Cyberdeck!")
        }
        (100, 484) => {
            let a = &arguments[0];
            format!("{a} contain items. Remove the items first.")
        }
        (100, 598) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("{a} executes {b} within your NCU...")
        }
        (100, 577) => {
            let a = &arguments[0];
            format!("Mission chance of token reward upped to {a}% due to your heroic effort.")
        }
        (100, 585) => {
            String::from("Dimach!")
        }
        (100, 614) => {
            let a = &arguments[0];
            format!("For you to receive the full reward from this quest, you can not be grouped with players above level {a} while solving the quest or when handing in the quest result.")
        }
        (100, 678) => {
            String::from("Shop for Rent")
        }
        (100, 475) => {
            let a = &arguments[0];
            format!("{a} cannot be summoned because it is in a fight.")
        }
        (100, 89) => {
            String::from("This Perk doesn't exist!")
        }
        (100, 114) => {
            String::from("Stuck-resolve command not allowed while one or more pet is in combat.")
        }
        (100, 129) => {
            String::from("Stuck-resolve command not allowed while standing.")
        }
        (100, 689) => {
            String::from("As a result of your death, you just lost some of your Alien Experience.")
        }
        (100, 713) => {
            let a = &arguments[0];
            format!("If this item is combined with a: {a}")
        }
        (100, 717) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            let e = &arguments[4];
            let f = &arguments[5];
            format!("{a} owns the shop \"{b}\" in the market at ({c}, {d}) in \"{e}\" ({f}).")
        }
        (100, 55) => {
            String::from("Please wait.")
        }
        (100, 431) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            format!("{a} hit {b} for {c} points of {d} damage.")
        }
        (100, 53) => {
            String::from("You can't break hostile nanoprograms on yourself!")
        }
        (100, 361) => {
            String::from("The molecular structure of this creature prohibits the use of this technology.")
        }
        (100, 459) => {
            String::from("You need to select a team member first!")
        }
        (100, 173) => {
            String::from("You don't own that item.")
        }
        (100, 660) => {
            String::from("This shopping booth has been locked down and can not be used until it has been unlocked or reset. Please try another booth.")
        }
        (100, 306) => {
            String::from("Mindcontrol only work on NPCs")
        }
        (100, 93_891_438) => {
            String::from("Location")
        }
        (100, 73) => {
            String::from("Only organization leaders can remove items from the contract.")
        }
        (100, 588) => {
            String::from("Backstab!")
        }
        (100, 437) => {
            let a = &arguments[0];
            format!("You received {a} xp.")
        }
        (100, 655) => {
            String::from(" /shop query <name> <minlevel> <maxlevel> <placement> <minprice> <maxprice> <category>\r\n")
        }
        (100, 31) => {
            String::from("Player is not in your organization.")
        }
        (100, 200) => {
            String::from("Target is in a district with higher suppression. Using rules of that district.")
        }
        (100, 71) => {
            String::from("You can't use this item now!")
        }
        (100, 360) => {
            String::from("Unable to execute nano program. Target not found.")
        }
        (100, 661) => {
            String::from("Only the leader of the organization owning this market can set the rent on the shop!")
        }
        (100, 63_220_709) => {
            String::from("Pet is too far away. It can't hear your command.")
        }
        (100, 384) => {
            String::from("PvP not allowed since you are neutral. Can only attack if attacked first.")
        }
        (100, 499) => {
            String::from("The quality of the reverse engineered items are poor.")
        }
        (100, 556) => {
            String::from("The target must be in your organization!")
        }
        (100, 224) => {
            String::from("This computer deck slot is not available.")
        }
        (100, 438) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            format!("{a} hit {b} for {c} points of {d} damage.")
        }
        (100, 644) => {
            String::from("Get the status of your shop:")
        }
        (100, 263) => {
            String::from("You can increase the necessary skills from the skill view (Button below with SKL on it). Later you can use nanoprograms, implants and other items to enhance your character.")
        }
        (100, 637) => {
            String::from("This is a private shop!")
        }
        (100, 691) => {
            let a = &arguments[0];
            format!("As a result of your death, you just lost some of your Alien Experience ({a}).")
        }
        (100, 428) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("{a} took {b} points of fall damage.")
        }
        (100, 682) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("Time until decay: {a}:{b}:{c} (owner inactivity)")
        }
        (100, 77) => {
            String::from("Special attack not possible. The target must be fighting someone else.")
        }
        (100, 508) => {
            let a = &arguments[0];
            format!("A character on your level has earned {a} as normal IP.")
        }
        (100, 679) => {
            let a = &arguments[0];
            format!("This shop is rented by: {a}")
        }
        (100, 26) => {
            String::from("You can only vote once!")
        }
        (100, 16) => {
            String::from("Disarming the trap failed!")
        }
        (100, 171) => {
            String::from("You must carry the item.")
        }
        (100, 364) => {
            String::from("Nano program executed successfully.")
        }
        (100, 368) => {
            String::from("You are not allowed to execute hostile nanoprogram on this target.")
        }
        (100, 414) => {
            String::from("You died of nano program damage!")
        }
        (100, 628) => {
            let a = &arguments[0];
            format!("The shop balance is {a} credits.")
        }
        (100, 594) => {
            let a = &arguments[0];
            format!("You need at least {a} remaining nano energy to execute this program.")
        }
        (100, 201) => {
            String::from("This weapon doesn't work against this target.")
        }
        (100, 241) => {
            String::from("Chest contains a Unique item, and can't be traded.")
        }
        (100, 622) => {
            String::from("Temporary items can't be used in a tradeskill process!")
        }
        (100, 156_067_838) => {
            String::from("Description")
        }
        (100, 249) => {
            String::from("Can't carry that.")
        }
        (100, 332) => {
            String::from("Action is not available.")
        }
        (100, 443) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Something hit {a} for {b} points of damage by damage shield.")
        }
        (100, 138) => {
            String::from("You found an item!")
        }
        (100, 650) => {
            String::from("Adjust prices:")
        }
        (100, 654) => {
            String::from("Query global ShopNet database:")
        }
        (100, 690) => {
            let a = &arguments[0];
            format!("You gained {a} new Alien Experience Points.")
        }
        (100, 485) => {
            let a = &arguments[0];
            format!("You cannot process the \"{a}\" as some of its charges has been used.")
        }
        (100, 91_170_781) => {
            String::from("You need to be in a team to use a team mission booth.")
        }
        (100, 119) => {
            String::from("You're unable to wear this item.")
        }
        (100, 266) => {
            String::from("The target must have the Shadowlands Expansion Pack!")
        }
        (100, 93) => {
            String::from("You need to be a team leader to perform this action!")
        }
        (100, 222) => {
            String::from("Target must be in a team in order to perform this action.")
        }
        (100, 593) => {
            String::from("Base abilities too low to increase skill more.")
        }
        (100, 473) => {
            let a = &arguments[0];
            format!("Character stored. {a} XP saved.")
        }
        (100, 535) => {
            String::from("You need to have a 2 hand edged weapon equipped.")
        }
        (100, 61) => {
            String::from("All items must be unequipped before performing a skill reset!")
        }
        (100, 50) => {
            String::from("Target is already in a trade.")
        }
        (100, 183) => {
            String::from("The target is outside special attack range!")
        }
        (100, 524) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("You hit {a} with nanobots for {b} points of {c} damage.")
        }
        (100, 665) => {
            String::from("This shop is only open for members of the organization owning the market!")
        }
        (100, 96_550_760) => {
            String::from("Team Search")
        }
        (100, 160) => {
            String::from("You can't split this item.")
        }
        (100, 353) => {
            String::from("Unable to execute nano program. You must stand still.")
        }
        (100, 102) => {
            String::from("Mission chance of token reward not upped due to your superior level.")
        }
        (100, 258) => {
            String::from("You can't fight at the moment!")
        }
        (100, 513) => {
            let a = &arguments[0];
            format!("All in all, you now have {a} IP points.")
        }
        (100, 309) => {
            String::from("Fear has no impact on players.")
        }
        (100, 81) => {
            String::from("It is no longer possible to change this option!")
        }
        (100, 216) => {
            String::from("Target can't be in a fight.")
        }
        (100, 694) => {
            String::from("The two items are the same item!")
        }
        (100, 680) => {
            let a = &arguments[0];
            format!("The current commission is: {a}%")
        }
        (100, 84) => {
            String::from("No Shadowknowledge is learned! You are not in the Shadowlands!")
        }
        (100, 626) => {
            String::from("You are currently unable to use perk special actions!")
        }
        (100, 301) => {
            String::from("Nanobots are busy. Aborting previous nano program.")
        }
        (100, 69) => {
            String::from("****SYSTEM MESSAGE: Anarchy Online has a special \"chain death\" avoidance system. If your character dies very quickly more than twice, the system assums that you have been chain killed")
        }
        (100, 458) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("You recouped {a} of your previously lost experience - remaining to be recouped: {b}.")
        }
        (100, 394) => {
            let a = &arguments[0];
            format!("{a} is full.")
        }
        (100, 41) => {
            String::from("You're not allowed to loot these remains!")
        }
        (100, 604) => {
            let a = &arguments[0];
            format!("Received mission reward: XP {a}")
        }
        (100, 76) => {
            String::from("You are unable to gain a new level with tradeskills. You need to get the last experience points in some other way.")
        }
        (100, 148) => {
            String::from("You caused trivial drops to be removed from the kill.")
        }
        (100, 261) => {
            String::from("You can't pick this lock!")
        }
        (100, 282) => {
            String::from("You can't open a chest while you're in a fight.")
        }
        (100, 464) => {
            String::from("You were successfully bound to this location.")
        }
        (100, 346_165) => {
            String::from("Name")
        }
        (100, 194) => {
            String::from("You can't attack while swimming.")
        }
        (100, 369) => {
            String::from("You are not allowed to execute friendly nanoprogram on enemy.")
        }
        (100, 467) => {
            String::from("Your summon target has not been bound while in this team.")
        }
        (100, 289) => {
            String::from("Your inventory and bank is full. Items are put where you are standing.")
        }
        (100, 574) => {
            let a = &arguments[0];
            format!("You need at least {a} in Multiple Melee to wield these two weapons.")
        }
        (100, 608) => {
            String::from("You have completed the second specialization!")
        }
        (100, 647) => {
            String::from(" /shop withdraw <amount>")
        }
        (100, 701) => {
            String::from("Unable to find source item?")
        }
        (100, 90) => {
            String::from("No room for more Perks!")
        }
        (100, 558) => {
            String::from("You can't do this while you are falling!")
        }
        (100, 122) => {
            String::from("You're unable to unequip this item.")
        }
        (100, 257) => {
            String::from("Arming trap failed.")
        }
        (100, 529) => {
            String::from("You need to have a melee weapon equipped.")
        }
        (100, 151) => {
            String::from("You can't trade more items in one go.")
        }
        (100, 25) => {
            String::from("Only one vote at a time!")
        }
        (100, 210) => {
            String::from("You must be behind the target!")
        }
        (100, 382) => {
            String::from("Not allowed to attack team-members.")
        }
        (100, 248) => {
            String::from("You have already got this unique item!")
        }
        (100, 176_308_692) => {
            String::from("Affirmative! The Attack has been initiated.")
        }
        (100, 450) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("{a} absorbed {b} points of {c} damage.")
        }
        (100, 597) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("{a} forces your NCU to run {b}...")
        }
        (100, 131) => {
            String::from("You failed the requirements to train this perk!")
        }
        (100, 337) => {
            String::from("No PvP grace period since you already are in a fight.")
        }
        (100, 518) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Your pet {a} was damaged by a toxic substance for {b} points of damage.")
        }
        (100, 418) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("{a} can loot these remains. (Loot order: {b})")
        }
        (100, 157) => {
            String::from("No room in your inventory.")
        }
        (100, 532) => {
            String::from("You need to have a SMG weapon equipped.")
        }
        (100, 226) => {
            String::from("You must carry the container in order to operate on the items inside it!")
        }
        (100, 243) => {
            String::from("You unlocked the item.")
        }
        (100, 390) => {
            String::from("Your breed can't have more professions!")
        }
        (100, 92) => {
            String::from("You need to be in a team to perform this action!")
        }
        (100, 3) => {
            String::from("No corpse in reclaim booth.")
        }
        (100, 29) => {
            String::from("Player is already in an organization.")
        }
        (100, 584) => {
            String::from("Fling Shot!")
        }
        (100, 39_801_003) => {
            String::from("Must not have perk:")
        }
        (100, 322) => {
            String::from("This mine is armed.  You might be able to disarm it using the proper tools!")
        }
        (100, 74) => {
            String::from("You are not allowed to execute a friendly nano program on this target here! PvP level restrictions apply on healing/friendly nano programs in Land Control areas.")
        }
        (100, 214) => {
            String::from("Must be in combat mode.")
        }
        (100, 211) => {
            String::from("You must be in front of the target!")
        }
        (100, 153) => {
            String::from("Chest contains a NoDrop item, and can't be dropped.")
        }
        (100, 287) => {
            String::from("Your inventory is full. Items are put into your bank.")
        }
        (100, 347) => {
            String::from("Trade cancelled.")
        }
        (100, 227) => {
            String::from("Item dropped on ground.")
        }
        (100, 109) => {
            String::from("Player can no longer afford the trade. Trade aborted.")
        }
        (100, 51) => {
            String::from("The maximum amount of items in your apartment has been reached! This item will be lost if left here.")
        }
        (100, 402) => {
            let a = &arguments[0];
            format!("You have been detected by {a}!")
        }
        (100, 425) => {
            let a = &arguments[0];
            format!("You took {a} points of fall damage.")
        }
        (100, 186) => {
            String::from("No target to assist.")
        }
        (100, 215) => {
            String::from("You can wield no weapons before using this item.")
        }
        (100, 454) => {
            let a = &arguments[0];
            format!("{a} tried to hit you, but missed!")
        }
        (100, 218) => {
            String::from("Wrong target selected.")
        }
        (100, 509) => {
            let a = &arguments[0];
            format!("Right now you have spent {a} IP in Map Navigation. You will not get these back.")
        }
        (100, 546) => {
            String::from("The target must be a controller!")
        }
        (100, 570) => {
            let a = &arguments[0];
            format!("Attacked by {a}!")
        }
        (100, 181) => {
            String::from("Accessing implants requires technical supervision.")
        }
        (100, 236) => {
            String::from("Wearing is disabled when you have items in your overflow!")
        }
        (100, 372) => {
            String::from("NCU error: This nano program can't automatically replace other program.")
        }
        (100, 589) => {
            let a = &arguments[0];
            format!("Maximum increase in skill is {a} per level!")
        }
        (100, 685) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("{a} bought {b} for {c} credits.")
        }
        (100, 687) => {
            let a = &arguments[0];
            format!("You have gained a new Alien Title: {a}!")
        }
        (100, 203) => {
            String::from("You can't reload the weapon with this ammunition type.")
        }
        (100, 209) => {
            String::from("You must not be in the Shadowlands in order to use this!")
        }
        (100, 179) => {
            String::from("You can't perform social animations while swimming!")
        }
        (100, 479) => {
            let a = &arguments[0];
            format!("Your attack shield blocked the attack! ({a}) left.")
        }
        (100, 562) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("{a} must not have a {b} running!")
        }
        (100, 333) => {
            String::from("Searching for hidden objects.")
        }
        (100, 681) => {
            let a = &arguments[0];
            format!("This shop is owned by: {a}")
        }
        (100, 625) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Price set on {a} to {b}!")
        }
        (100, 696) => {
            String::from("Source item is not owned by the char and does not have the stationary flags set.")
        }
        (100, 26713) => {
            String::from("any")
        }
        (100, 590) => {
            String::from("You have reached your max for this title.")
        }
        (100, 33) => {
            String::from("You need at least 2 free slots to receive mission reward. Mission completion is pending.")
        }
        (100, 275) => {
            String::from("This action requires that you are wielding Jobe-chipped Cyberdeck!")
        }
        (100, 319) => {
            String::from("A trap was sprung! You failed disarming the trap!.")
        }
        (100, 697) => {
            String::from("Stationary target item too far away!")
        }
        (100, 315) => {
            String::from("Can't use all ammo at once.")
        }
        (100, 627) => {
            let a = &arguments[0];
            format!("The shop contains {a} items.")
        }
        (100, 552) => {
            String::from("Your")
        }
        (100, 463) => {
            let a = &arguments[0];
            format!("This Shadowknowledge was added to the pool of unsaved experience points that you get back through gaining new experience points. Your current pool of unsaved experience is {a}.")
        }
        (100, 559) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("{a} must recently have {b} running!")
        }
        (100, 581) => {
            String::from("Full Auto!")
        }
        (100, 192) => {
            String::from("You can't attack this item.")
        }
        (100, 545) => {
            let a = &arguments[0];
            format!("Target breed must be {a}!")
        }
        (100, 64) => {
            String::from("You must empty your reclaim before you can save! Type \"/reclaim\" to access your items.")
        }
        (100, 527) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            format!("{a} was attacked with nanobots from {b} for {c} points of {d} damage.")
        }
        (100, 24) => {
            String::from("Your inventory is full. Can't complete trade!")
        }
        (100, 662) => {
            let a = &arguments[0];
            format!("Shop commission set to {a}%!")
        }
        (100, 387) => {
            String::from("You cannot attack this target, because you are in a mixed omni/clan team.")
        }
        (100, 706) => {
            String::from("No items found.")
        }
        (100, 295) => {
            String::from("You will not be rewarded for this mission due to a too high level character in the team.")
        }
        (100, 202) => {
            String::from("Already reloading. Please wait.")
        }
        (100, 299) => {
            String::from("You are not in an organization.")
        }
        (100, 44) => {
            String::from("You can't do this while fighting!")
        }
        (100, 715) => {
            String::from("The owner of the shop is accessing the shop. Please wait.")
        }
        (100, 292) => {
            String::from("The item was not accepted as a key for this door.")
        }
        (100, 486) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            let e = &arguments[4];
            format!("\"{a}\" is of a too low quality level. With \"{b}\" at quality of {c} , the \"{d}\" must be at least at quality {e}.")
        }
        (100, 540) => {
            String::from("You need to have a rifle weapon equipped.")
        }
        (100, 251) => {
            String::from("Can't add fixture to building.")
        }
        (100, 415) => {
            String::from("You died of fall damage!")
        }
        (100, 265) => {
            String::from("You must have the Notum Wars Booster Pack!")
        }
        (100, 170) => {
            String::from("Stationary item too far away.")
        }
        (100, 325) => {
            String::from("You are not a member of a team!")
        }
        (100, 46) => {
            String::from("Trade contains a NoDrop item and can't be completed.")
        }
        (100, 199) => {
            String::from("You have accepted your targets offer of surrender, and are thus not allowed to attack it again.")
        }
        (100, 57) => {
            String::from("You won a new PvP title!")
        }
        (100, 663) => {
            String::from("This is now a closed shop, only open for members of your organization!")
        }
        (100, 606) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("You recouped {a} of your previously lost Shadowknowledge - remaining to be recouped: {b}.")
        }
        (100, 48) => {
            String::from("You already have this unique item.")
        }
        (100, 238) => {
            String::from("Target slot is not empty.")
        }
        (100, 365) => {
            String::from("Nano program execution error. You fumbled.")
        }
        (100, 491) => {
            String::from("You can assemble these two items, but they will be of poor quality.")
        }
        (100, 125) => {
            String::from("Target resisted.")
        }
        (100, 97) => {
            String::from("Do you wish to permanently commit to your side? (You can no longer change side after reaching level 201!)")
        }
        (100, 21) => {
            String::from("Other player's inventory is full, can't complete trade!")
        }
        (100, 15) => {
            String::from("Your inventory is full!")
        }
        (100, 40) => {
            String::from("Somebody is already looting these remains.")
        }
        (100, 334) => {
            String::from("You start sneaking.")
        }
        (100, 698) => {
            String::from("Target item is not owned by the char and does not have the stationary flags set!")
        }
        (100, 232) => {
            String::from("Target already contains this unique item.")
        }
        (100, 636) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("ShopNet: Too many entries found ({a}). {b} entries displayed:")
        }
        (100, 331) => {
            String::from("You can't move.")
        }
        (100, 561) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("{a} must have a {b} running!")
        }
        (100, 512) => {
            String::from("Please note that there might be some very minor rounding errors, but they should not amount to more than 5-10 IP total. Please also note that some people have more IP than their normal total IP, because of special items yielding additional IP.")
        }
        (100, 447) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("{a}'s reflect shield hit {b} for {c} points of damage.")
        }
        (100, 176) => {
            String::from("Items can't be used directly from the corpse!")
        }
        (100, 79) => {
            String::from("Congratulations! You have received a new special attack: Backstab!")
        }
        (100, 116) => {
            String::from("Stuck-resolve mode entered; expect resolution in 30 seconds.")
        }
        (100, 58) => {
            String::from("You lost your PvP title!")
        }
        (100, 252) => {
            String::from("This item can't be lifted.")
        }
        (100, 538) => {
            String::from("You need to have a pistol weapon equipped.")
        }
        (100, 130) => {
            String::from("Stuck-resolve command not allowed for players that cannot move.")
        }
        (100, 307) => {
            String::from("Daze activated.")
        }
        (100, 526) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            let e = &arguments[4];
            format!("{a} was attacked with {b} from {c} for {d} points of {e} damage.")
        }
        (100, 411) => {
            String::from("You died of reflect shield damage!")
        }
        (100, 536) => {
            String::from("You need to have a 2 hand blunt weapon equipped.")
        }
        (100, 699) => {
            String::from("Unable to get the source item, at all!")
        }
        (100, 312) => {
            String::from("You need a target for this item!")
        }
        (100, 534) => {
            String::from("You need to have a 1 hand blunt weapon equipped.")
        }
        (100, 551) => {
            String::from("Target")
        }
        (100, 246) => {
            String::from("You can't take a fixture from a building owned by someone.")
        }
        (100, 279) => {
            String::from("Chest full.")
        }
        (100, 67) => {
            String::from("Warning - Complimentary insurance buffer has been terminated. From now on, all experience gained since last insurance payment will be lost upon death. Please, for the good of your future, remember to insure yourself regularly.")
        }
        (100, 327) => {
            String::from("/team loot")
        }
        (100, 348) => {
            String::from("You got a new mission.")
        }
        (100, 457) => {
            let a = &arguments[0];
            format!("This XP was added to the pool of unsaved experience points that you get back through gaining new experience points. Your current pool of unsaved experience is {a}.")
        }
        (100, 522) => {
            let a = &arguments[0];
            format!("You lost {a} points of Shadowknowledge.")
        }
        (100, 165) => {
            String::from("Inventory full. Please make room in your inventory first!")
        }
        (100, 237) => {
            String::from("You can't wear stacked items.")
        }
        (100, 363) => {
            String::from("Target out of range for nano program.")
        }
        (100, 525) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            format!("You hit {a} with {b} for {c} points of {d} damage.")
        }
        (100, 537) => {
            String::from("You need to have a piercing weapon equipped.")
        }
        (100, 413) => {
            String::from("You died of weapon damage!")
        }
        (100, 591) => {
            String::from("Absolute ability maximum for your breed has been reached!")
        }
        (100, 602) => {
            let a = &arguments[0];
            format!("Current team loot order is: {a}")
        }
        (100, 517) => {
            String::from("You need at least one free inventory slot! Please remove an item and try again.")
        }
        (100, 392) => {
            let a = &arguments[0];
            format!("New Level: {a}!")
        }
        (100, 470) => {
            String::from("You gain Shadowknowledge from killing monsters and performing tasks aligned with the two Unredeemed / Redeemed factions. If you kill an Unredeemed monster, you need Redeemed faction and vice versa. The higher the faction, the greater the Shadowknowledge.\" ")
        }
        (100, 641) => {
            String::from("Sell items to the shop that you want other players to be able to buy!")
        }
        (100, 722) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = {
                if a.to_string() == "1" {
                    " item "
                } else {
                    " items"
                }
            };
            let d = {
                if a.to_string() == "1" {
                    " credit "
                } else {
                    " credits"
                }
            };
            format!("(Found {a} {c} and {b} {d} in the shop.)")
        }
        (100, 135) => {
            String::from("You are too low level to forage in this area!")
        }
        (100, 159) => {
            String::from("Can only stack items of the same level and type.")
        }
        (100, 401) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("{a} was born on {b}")
        }
        (100, 472) => {
            let a = &arguments[0];
            format!("{a} credits were deducted from your account.")
        }
        (100, 533) => {
            String::from("You need to have a 1 hand edged weapon equipped.")
        }
        (100, 205) => {
            String::from("Reloading...")
        }
        (100, 17) => {
            String::from("All items have been reclaimed!")
        }
        (100, 308) => {
            String::from("You need a selected target for this program!")
        }
        (100, 326) => {
            String::from("Available team commands:")
        }
        (100, 377) => {
            String::from("Combat is not possible in this district.")
        }
        (100, 709) => {
            String::from("You have been selected as a member of the boarding team.  You have 5 minutes to enter the spaceship.  You can do this by walking into the teleport beam.")
        }
        (100, 115) => {
            String::from("Stuck-resolve command not allowed while one or more pet is affected by hostile nano programs.")
        }
        (100, 106) => {
            String::from("You have no service tower.")
        }
        (100, 565) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("You increased nano on {a} for {b} points.")
        }
        (100, 718) => {
            String::from("You must have the Alien Invasion expansion!")
        }
        (100, 406) => {
            let a = &arguments[0];
            format!("{a} skill available.")
        }
        (100, 554) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("{a} {b} base value is required to be at least {c}!")
        }
        (100, 635) => {
            String::from("Unable to execute nano program. You can't execute this nano on the target.")
        }
        (100, 566) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("You got nano from {a} for {b} points.")
        }
        (100, 235) => {
            String::from("Temporary items can't be moved into the bank!")
        }
        (100, 666) => {
            let a = &arguments[0];
            format!("You have been challenged to a fight from {a}. Do you accept?")
        }
        (100, 683) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("Locked down by Org leader: time until reset {a:02}:{b:02}:{c:02}")
        }
        (100, 127) => {
            String::from("You stop bluffing")
        }
        (100, 446) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Your reflect shield hit {a} for {b} points of damage.")
        }
        (100, 572) => {
            let a = &arguments[0];
            format!("One of your team members ({a}) is in the same organization as your target")
        }
        (100, 634) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("Item: {a} level: {b} price: {c}")
        }
        (100, 582) => {
            String::from("Aimed Shot!")
        }
        (100, 461) => {
            String::from("Neither you nor target can be fighting!")
        }
        (100, 87) => {
            String::from("You need the Shadowlands expansion pack to access Perks.")
        }
        (100, 349) => {
            String::from("Mission expired.")
        }
        (100, 658) => {
            String::from("Do you wish to reset ownership of this shop?")
        }
        (100, 436) => {
            let a = &arguments[0];
            format!("You lost {a} xp.")
        }
        (100, 576) => {
            let a = &arguments[0];
            format!("You were hit for {a} points of damage.")
        }
        (100, 410) => {
            String::from("You died of self termination!")
        }
        (100, 250) => {
            String::from("You can't drop this item here!")
        }
        (100, 167) => {
            String::from("Move item to inventory before dropping it on ground.")
        }
        (100, 383) => {
            String::from("PvP not allowed in this district.")
        }
        (100, 422) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("You hit {a} for {b} points of {c} damage.")
        }
        (100, 714) => {
            String::from("The shop was altered by the owner. Trade aborted.\r\n")
        }
        (100, 544) => {
            let a = &arguments[0];
            format!("Target sex must be {a}!")
        }
        (100, 405) => {
            let a = &arguments[0];
            format!("{a} tried to hide but failed.")
        }
        (100, 548) => {
            let a = &arguments[0];
            format!("Target side must not be {a}!")
        }
        (100, 175) => {
            String::from("Move item to inventory before using it.")
        }
        (100, 495) => {
            String::from("The quality of the new item is poor.")
        }
        (100, 503) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("{a} was unaffected by your {b}.")
        }
        (100, 152) => {
            String::from("Chest contains a NoDrop item, and can't be traded or sold.")
        }
        (100, 182) => {
            String::from("Special attack is unavailable.")
        }
        (100, 466) => {
            String::from("Your summon failed because the area is currently too crowded.")
        }
        (100, 317) => {
            String::from("This item can't be used directly. Use it on another item to boobytrap it.")
        }
        (100, 35) => {
            String::from("Player is already a team member.")
        }
        (100, 441) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Your damage shield hit {a} for {b} points of damage.")
        }
        (100, 710) => {
            String::from("You need the Alien Invasion expansion pack to access this perk.\r\n")
        }
        (100, 23_724_623) => {
            String::from("Join team request sent to ")
        }
        (100, 75) => {
            String::from("You are already running this action!")
        }
        (100, 98) => {
            String::from("You will not be able to gain ShadowKnowledge nor obtain ShadowLevels (levels above 200)! You can change this through the option->misc view.")
        }
        (100, 416) => {
            String::from("You died of liquid damage!")
        }
        (100, 612) => {
            String::from("You are now renting this shop terminal!")
        }
        (100, 13) => {
            String::from("You don't meet the criteria for joining this organization!")
        }
        (100, 667) => {
            String::from("Open shop (Shop open to everyone):")
        }
        (100, 455) => {
            String::from("Critical hit!")
        }
        (100, 669) => {
            String::from("Closed shop (Shop open only to org members):")
        }
        (100, 44_660_212) => {
            String::from("Flying must be allowed on the playfield.")
        }
        (100, 1) => {
            String::from("You can't attack this target.")
        }
        (100, 386) => {
            String::from("Can't attack. Target is still in PvP grace period.")
        }
        (100, 146) => {
            String::from("While crawling, you may only fight with ranged weapons!")
        }
        (100, 621) => {
            String::from("You can't delete an item that is being equipped!")
        }
        (100, 575) => {
            let a = &arguments[0];
            format!("You need at least {a} in Multiple Ranged to wield these two weapons.")
        }
        (100, 640) => {
            let a = &arguments[0];
            format!("This is your shop (max {a} items)!")
        }
        (100, 374) => {
            String::from("Nanoprogram did not activate. Not enough nanoenergy.")
        }
        (100, 686) => {
            String::from("The container must be empty before it can be sold!")
        }
        (100, 568) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("You were drained for {a} points of nano energy by {b}.")
        }
        (100, 603) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Received mission reward: XP {a} Cash {b}")
        }
        (100, 19) => {
            String::from("Not enough nano energy to execute nanoprogram.")
        }
        (100, 80) => {
            String::from("You only gain Shadowknowledge from unsided actions or opposing side kills!")
        }
        (100, 692) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Item at: {a} {b}.")
        }
        (100, 128) => {
            String::from("You can only bluff while walking!")
        }
        (100, 143) => {
            String::from("Item must be applied on a friendly target.")
        }
        (100, 283) => {
            String::from("You need at least 2 free inventory slots for mission items.")
        }
        (100, 37) => {
            String::from("You lacked the skill to perform the action.")
        }
        (100, 403) => {
            let a = &arguments[0];
            format!("{a} saw through your bluff!")
        }
        (100, 397) => {
            String::from("You cannot attack the tower -- the defense shield is enabled.  Disable it by using a defense shield disabler item near the tower.  (You don't have to do this if your organization has a controller tower.)")
        }
        (100, 530) => {
            String::from("You need to have a long range weapon equipped.")
        }
        (100, 351) => {
            String::from("Executing programs here is forbidden.")
        }
        (100, 677) => {
            let a = &arguments[0];
            format!("{a}'s Shop")
        }
        (100, 587) => {
            String::from("Sneak Attack!")
        }
        (100, 703) => {
            String::from("No combination found!")
        }
        (100, 185) => {
            String::from("NoDrop item can't be added to a container on ground!")
        }
        (100, 101) => {
            String::from("Your target didn't lose enough to grant any reward.")
        }
        (100, 164) => {
            String::from("You must be sitting in order to log out.")
        }
        (100, 189) => {
            String::from("You can't be polymorphed when crawling!")
        }
        (100, 68) => {
            String::from("You can't reset this skill! You need to have spent some IP on the skill first.")
        }
        (100, 318) => {
            String::from("You succeded disarming the trap!.")
        }
        (100, 385) => {
            String::from("PvP not allowed since your team is neutral. Can only attack if attacked first.")
        }
        (100, 481) => {
            let a = &arguments[0];
            format!("You drained {a} points of nano from the target.")
        }
        (100, 523) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("You were attacked with nanobots from {a} for {b} points of {c} damage.")
        }
        (100, 339) => {
            String::from("PvP disgrace period has ended.")
        }
        (100, 47) => {
            String::from("Your trade partner already has this unique item.")
        }
        (100, 112) => {
            String::from("Stuck-resolve command not allowed while in combat.")
        }
        (100, 268) => {
            String::from("You must be in the Shadowlands!")
        }
        (100, 629) => {
            String::from("This is a private shop!")
        }
        (100, 688) => {
            let a = &arguments[0];
            format!("You have gained enough Alien XP to get to another alien level but your level must be: {a} to obtain it.")
        }
        (100, 345) => {
            String::from("The door is now locked.")
        }
        (100, 22) => {
            String::from("No room in your inventory. Item dropped on ground.")
        }
        (100, 592) => {
            let a = &arguments[0];
            format!("Maximum increase in ability is {a} per level!")
        }
        (100, 290) => {
            String::from("Your inventory and bank is full. An item is put where you are standing.")
        }
        (100, 120) => {
            String::from("Social armor requires that no other armor is worn.")
        }
        (100, 569) => {
            let a = &arguments[0];
            format!("Attacking {a}...")
        }
        (100, 145) => {
            String::from("This item requires a fighting-target to be applied on.")
        }
        (100, 229) => {
            String::from("This item can only reside in your main inventory!")
        }
        (100, 476) => {
            let a = &arguments[0];
            format!("{a} cannot be summoned because it is immobile.")
        }
        (100, 163) => {
            String::from("You can't log out while in a fight.")
        }
        (100, 631) => {
            String::from("ShopNet: No entries found!")
        }
        (100, 671) => {
            String::from("Do you wish to transfer all credits from the shop?")
        }
        (100, 228) => {
            String::from("This item can't be traded!")
        }
        (100, 344) => {
            String::from("You need a free entry in your inventory for your access card before you can enter!")
        }
        (100, 134) => {
            String::from("You are too high level to forage in this area!")
        }
        (100, 338) => {
            String::from("PvP grace period has ended.")
        }
        (100, 197) => {
            String::from("Target is surrendering to another person.")
        }
        (100, 330) => {
            String::from("Wait for current program activation to finish.")
        }
        (100, 141) => {
            String::from("Spirits can only be used by Shades!")
        }
        (100, 478) => {
            let a = &arguments[0];
            format!("You block the attack from {a}, and perform an attack of opportunity!")
        }
        (100, 487) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            format!("It is theoretically possible to combine \"{a}\" with \"{b}\". But you need at least {c} in {d}.")
        }
        (100, 519) => {
            let a = &arguments[0];
            format!("You gained {a} points of Shadowknowledge as a side bonus.")
        }
        (100, 341) => {
            String::from("Looter is not present.")
        }
        (100, 563) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("You healed {a} for {b} points of health.")
        }
        (100, 113) => {
            String::from("Stuck-resolve command not allowed while affected by hostile nano programs.")
        }
        (100, 286) => {
            String::from("You are entering an overcrowded area. You will be moved back. Stop now. Please wait or find another destination.")
        }
        (100, 483) => {
            let a = &arguments[0];
            format!("{a} is open. Please close it first.")
        }
        (100, 231) => {
            String::from("Your deck must be empty before it can be unequipped!")
        }
        (100, 586) => {
            String::from("Brawl!")
        }
        (100, 705) => {
            String::from("Combine failed!")
        }
        (100, 743_316) => {
            let a = &arguments[0];
            let b = {
                if a.to_string() == "1" {
                    " credit was "
                } else {
                    " credits were"
                }
            };
            format!("{a} {b} deducted from your account.")
        }
        (100, 553) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("You have to kill {a} more {b} to fulfill one of your missions!")
        }
        (100, 399) => {
            String::from("Team member went link dead.")
        }
        (100, 434) => {
            let a = &arguments[0];
            format!("A player hit you for {a} points of damage.")
        }
        (100, 140) => {
            String::from("Shades can't use implants!")
        }
        (100, 583) => {
            String::from("Fast attack!")
        }
        (100, 264) => {
            String::from("The target must have the Notum Wars Booster Pack!")
        }
        (100, 719) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("\"{a}\" at level {b} with price {c}.")
        }
        (100, 60) => {
            String::from("You can't reset Map Navigation!")
        }
        (100, 605) => {
            let a = &arguments[0];
            format!("Received mission reward: Cash {a}")
        }
        (100, 204) => {
            let a = &arguments[0];
            format!("{a} is already fully loaded.")
        }
        (100, 417) => {
            let a = &arguments[0];
            format!("Your items will be reclaimed in {a} seconds, and made available in a reclaim booth near your resurrection site.")
        }
        (100, 269) => {
            String::from("This action requires that you have the fourth specialization completed!")
        }
        (100, 610) => {
            String::from("You have completed the fourth specialization!")
        }
        (100, 659) => {
            let a = &arguments[0];
            format!("This terminal can only be rented by members of {a}!")
        }
        (100, 477) => {
            let a = &arguments[0];
            format!("{a} blocked your attack, and performs an attack of opportunity!")
        }
        (100, 445) => {
            let a = &arguments[0];
            format!("Someone's reflect shield hit you for {a} points of damage.")
        }
        (100, 564) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("You got healed by {a} for {b} points of health.")
        }
        (100, 136) => {
            String::from("There are no forageable items in this area!")
        }
        (100, 412) => {
            String::from("You died of shield damage!")
        }
        (100, 304) => {
            String::from("You are not allowed to attack this target.")
        }
        (100, 324) => {
            String::from("Resurrection shock fills your body. Stats temporarily reduced.")
        }
        (100, 239) => {
            String::from("Wearing items is forbidden here.")
        }
        (100, 539) => {
            String::from("You need to have an assault rifle weapon equipped.")
        }
        (100, 664) => {
            String::from("This is now an open shop! (open for all)")
        }
        (100, 672) => {
            String::from("This shop has been unlocked!")
        }
        (100, 247_358_642) => {
            String::from("You are wearing items that can not be used with Grid Armor.")
        }
        (100, 30) => {
            String::from("You're not allowed to invite other players.")
        }
        (100, 233) => {
            String::from("Temporary items can't be moved into a bag!")
        }
        (100, 230) => {
            String::from("You can't loot NoDrop items!")
        }
        (100, 320) => {
            String::from("You succeeded disarming the mine!")
        }
        (100, 607) => {
            String::from("You have completed the first specialization!")
        }
        (100, 240_339_044) => {
            let a = &arguments[0];
            format!("Your Organization cannot initiate a laser strike at the moment. Please wait at least {a} seconds.")
        }
        (100, 27) => {
            String::from("Your vote has been registered!")
        }
        (100, 220) => {
            String::from("Must be inside a dungeon.")
        }
        (100, 12) => {
            String::from("Organization form changed!")
        }
        (100, 36) => {
            String::from("Player is already an organization member.")
        }
        (100, 187) => {
            String::from("Can't assist yourself.")
        }
        (100, 723) => {
            String::from("You can not do this with a vehicle equipped.")
        }
        (100, 162) => {
            String::from("Use a surgery clinic to remove implants!")
        }
        (100, 452) => {
            let a = &arguments[0];
            format!("You were damaged by a toxic substance for {a} points of damage.")
        }
        (100, 172) => {
            String::from("Both items must be in your inventory window.")
        }
        (100, 456) => {
            let a = &arguments[0];
            format!("{a} contain items. Remove the items first.")
        }
        (100, 468) => {
            String::from("Your summon target has not been bound on this playfield.")
        }
        (100, 196) => {
            String::from("Target is already dead.")
        }
        (100, 300) => {
            String::from("You are not allowed to teleport players to this playfield.")
        }
        (100, 59) => {
            String::from("You need a Reset Point in order to reset a skill!")
        }
        (100, 280) => {
            String::from("Unable to fit item in container.")
        }
        (100, 329) => {
            String::from("Only team leader can change loot order!")
        }
        (100, 429) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            format!("{a} was attacked with {b} for {c} points of {d} damage.")
        }
        (100, 91) => {
            String::from("You have gained a new Perk!")
        }
        (100, 335) => {
            String::from("Invalid petcommand.")
        }
        (100, 373) => {
            String::from("You feel ok again. All residues from being resurrected are gone.")
        }
        (100, 42) => {
            String::from("You're not a member of the looting team!")
        }
        (100, 78) => {
            String::from("Special attack not possible. You must be behind the target.")
        }
        (100, 124) => {
            String::from("Target is outside range.")
        }
        (100, 234) => {
            String::from("Bank is full.")
        }
        (100, 395) => {
            let a = &arguments[0];
            format!("You parried the attack from {a}!")
        }
        (100, 514) => {
            String::from("Have fun :-)")
        }
        (100, 86_189_811) => {
            String::from("Smooth Animations")
        }
        (100, 72) => {
            String::from("You can't reset skills while equipping items!")
        }
        (100, 188) => {
            String::from("Target is not in a fight.")
        }
        (100, 465) => {
            let a = &arguments[0];
            format!("You successfully bound {a} teammember(s) to this location.")
        }
        (100, 675) => {
            String::from("This shop has been locked. To transfer credits you must first remove all items that are currently for sale.")
        }
        (100, 579) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("{a} attacked by {b}!")
        }
        (100, 11) => {
            String::from("You can't promote someone to your own level in organization or higher!")
        }
        (100, 707) => {
            let a = &arguments[0];
            format!("Item deleted at pos {a}.")
        }
        (100, 496) => {
            String::from("The quality of the new item is ok.")
        }
        (100, 432) => {
            let a = &arguments[0];
            format!("You were hit for {a} points of damage.")
        }
        (100, 557) => {
            String::from("The target must be in your team!")
        }
        (100, 147) => {
            String::from("The team did too little damage to get any xp from this kill!")
        }
        (100, 505) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("{a} casts nanoprogram '{b}' on {c}.")
        }
        (100, 528) => {
            String::from("You need to have an unarmed combat weapon equipped.")
        }
        (100, 642) => {
            String::from("Name your shop:")
        }
        (100, 670) => {
            String::from(" /shop closed")
        }
        (100, 212) => {
            String::from("Target must not be in a team in order to perform this action.")
        }
        (100, 281) => {
            String::from("This item contains a unique item you've already got.")
        }
        (100, 45) => {
            String::from("You are too far away; please move closer!")
        }
        (100, 105) => {
            String::from("Target is not your pet.")
        }
        (100, 515) => {
            String::from("You can't increase a skill that is temporarily changed!")
        }
        (100, 291) => {
            String::from("You did too little damage on the target to get any XP for this kill!")
        }
        (100, 354) => {
            String::from("Wait for current nano program execution to finish.")
        }
        (100, 273) => {
            String::from("This action requires that you are wearing Grid Armor!")
        }
        (100, 630) => {
            String::from("This shop can now only be used by characters on this account!")
        }
        (100, 155) => {
            String::from("Can't trade this type of item from container!")
        }
        (100, 254) => {
            String::from("Repair roll failed.")
        }
        (100, 247) => {
            String::from("You're not allowed to use this item.")
        }
        (100, 396) => {
            let a = &arguments[0];
            format!("{a} parried your attack!")
        }
        (100, 107) => {
            String::from("You have no pet.")
        }
        (100, 184) => {
            String::from("You have no line of sight to the target!")
        }
        (100, 380) => {
            String::from("You cannot attack this tower since you are too far away in level to the tower.")
        }
        (100, 193) => {
            String::from("You can't attack yourself.")
        }
        (100, 104) => {
            String::from("Wait for current nano program execution to finish.")
        }
        (100, 99) => {
            String::from("You will now be able to gain ShadowKnowledge and obtain ShadowLevels (levels above 200)!")
        }
        (100, 96) => {
            String::from("You need to be in an area owned by your organization to perform this action!")
        }
        (100, 178) => {
            String::from("You were unable to attack the target!")
        }
        (100, 379) => {
            String::from("You cannot attack this player since you are too far away in level.")
        }
        (100, 225) => {
            String::from("Please wait for item to deactivate before attempting to wear another.")
        }
        (100, 498) => {
            String::from("It's not possible to reverse engineer this item.")
        }
        (100, 144) => {
            String::from("This item requires a hostile target to be applied on.")
        }
        (100, 180) => {
            String::from("The item is not owned by you. Move it into inventory first!")
        }
        (100, 541) => {
            String::from("You need to have a shotgun weapon equipped.")
        }
        (100, 543) => {
            let a = &arguments[0];
            format!("Target side must be {a}!")
        }
        (120, 16) => {
            String::from("<type7> pet")
        }
        (120, 12) => {
            String::from("psychosis pet")
        }
        (120, 13) => {
            String::from("quest pet")
        }
        (120, 15) => {
            String::from("service tower")
        }
        (120, 10) => {
            String::from("regular pet")
        }
        (120, 14) => {
            String::from("social pet")
        }
        (120, 17) => {
            String::from("mindcontrolled character")
        }
        (120, 189_936_355) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Can't have more than {a} {b}.")
        }
        (120, 11) => {
            String::from("healing pet")
        }
        (120, 210_158_307) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Must have at least {a} {b}.")
        }
        (500, 167_191_294) => {
            String::from("Target is busy talking to somebody else, please wait.")
        }
        (505, 112) => {
            String::from("<center>Right<br>Finger</center>")
        }
        (505, 114) => {
            String::from("<center>Left<br>Finger</center>")
        }
        (505, 201) => {
            String::from("Head")
        }
        (505, 202) => {
            String::from("Ears")
        }
        (505, 210) => {
            String::from("Legs")
        }
        (505, 204) => {
            String::from("Chest")
        }
        (505, 308) => {
            String::from("<center>Left<br>Hand</center>")
        }
        (505, 104) => {
            String::from("Chest")
        }
        (505, 212) => {
            String::from("Feet")
        }
        (505, 100) => {
            String::from("Neck")
        }
        (505, 113) => {
            String::from("Feet")
        }
        (505, 109) => {
            String::from("<center>Right<br>Wrist</center>")
        }
        (505, 300) => {
            String::from("<center>Hud<br>1</center>")
        }
        (505, 102) => {
            String::from("Back")
        }
        (505, 306) => {
            String::from("<center>Right<br>Hand</center>")
        }
        (505, 313) => {
            String::from("<center>Deck<br>5</center>")
        }
        (505, 314) => {
            String::from("<center>Deck<br>6</center>")
        }
        (505, 209) => {
            String::from("<center>Right<br>Hand</center>")
        }
        (505, 103) => {
            String::from("<center>Right<br>Shoulder</center>")
        }
        (505, 107) => {
            String::from("Hands")
        }
        (505, 108) => {
            String::from("<center>Left<br>Arm</center>")
        }
        (505, 205) => {
            String::from("<center>Left<br>Arm</center>")
        }
        (505, 301) => {
            String::from("<center>Hud<br>2</center>")
        }
        (505, 309) => {
            String::from("<center>Deck<br>1</center>")
        }
        (505, 304) => {
            String::from("<center>Utils<br>2</center>")
        }
        (505, 200) => {
            String::from("Eyes")
        }
        (505, 206) => {
            String::from("<center>Right<br>Wrist</center>")
        }
        (505, 312) => {
            String::from("<center>Deck<br>4</center>")
        }
        (505, 311) => {
            String::from("<center>Deck<br>3</center>")
        }
        (505, 303) => {
            String::from("<center>Utils<br>1</center>")
        }
        (505, 111) => {
            String::from("<center>Left<br>Wrist</center>")
        }
        (505, 211) => {
            String::from("<center>Left<br>Hand</center>")
        }
        (505, 310) => {
            String::from("<center>Deck<br>2</center>")
        }
        (505, 110) => {
            String::from("Legs")
        }
        (505, 208) => {
            String::from("<center>Left<br>Wrist</center>")
        }
        (505, 307) => {
            String::from("Deck")
        }
        (505, 105) => {
            String::from("<center>Left<br>Shoulder</center>")
        }
        (505, 207) => {
            String::from("Waist")
        }
        (505, 302) => {
            String::from("<center>Hud<br>3</center>")
        }
        (505, 305) => {
            String::from("<center>Utils<br>3</center>")
        }
        (505, 101) => {
            String::from("Head")
        }
        (505, 203) => {
            String::from("<center>Right<br>Arm</center>")
        }
        (505, 106) => {
            String::from("<center>Right<br>Arm</center>")
        }
        (2011, 9_225_622) => {
            String::from("Create Reference")
        }
        (2011, 80_039_191) => {
            String::from("Next View")
        }
        (2011, 107_305_172) => {
            String::from("Select Pet 5")
        }
        (2011, 135_484_617) => {
            String::from("Inventory")
        }
        (2011, 107_881_436) => {
            String::from("Brawl")
        }
        (2011, 122_133_972) => {
            String::from("Screenshot")
        }
        (2011, 237_598_444) => {
            String::from("Left (Global)")
        }
        (2011, 135_652) => {
            String::from("Strafe Right")
        }
        (2011, 225_400_244) => {
            let a = &arguments[0];
            format!("Bar Select {a:02}")
        }
        (2011, 191_457_204) => {
            String::from("Raid")
        }
        (2011, 176_632_971) => {
            String::from("Bow Special Attack")
        }
        (2011, 159_500_883) => {
            String::from("Skills")
        }
        (2011, 235_809_454) => {
            String::from("Mission")
        }
        (2011, 16_207_143) => {
            String::from("Previous View")
        }
        (2011, 41_269_893) => {
            String::from("Knowledge")
        }
        (2011, 210_759_381) => {
            let a = &arguments[0];
            format!("Active Bar {a:02}")
        }
        (2011, 100_299_476) => {
            String::from("Select Team Member 4")
        }
        (2011, 100_723_820) => {
            String::from("Forward (Global)")
        }
        (2011, 100_299_475) => {
            String::from("Select Team Member 3")
        }
        (2011, 132_243_097) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Bar {a:02}_{b:02}")
        }
        (2011, 100_299_477) => {
            String::from("Select Team Member 5")
        }
        (2011, 4_942_195) => {
            String::from("Tradeskill")
        }
        (2011, 113_913_492) => {
            String::from("Left")
        }
        (2011, 76_280_300) => {
            String::from("Right (Global)")
        }
        (2011, 113_909_504) => {
            String::from("Jump")
        }
        (2011, 222_147_604) => {
            String::from("Forward")
        }
        (2011, 185_201_280) => {
            String::from("Item Shop")
        }
        (2011, 88_798_180) => {
            String::from("Fling Shot")
        }
        (2011, 107_305_168) => {
            String::from("Select Pet 1")
        }
        (2011, 184_974_133) => {
            String::from("Use")
        }
        (2011, 107_305_173) => {
            String::from("Select Pet 6")
        }
        (2011, 191_445_954) => {
            String::from("Wear")
        }
        (2011, 114_722_808) => {
            String::from("Dimach")
        }
        (2011, 132_241_463) => {
            let a = &arguments[0];
            format!("Active Bar Row {a:02}")
        }
        (2011, 100_299_474) => {
            String::from("Select Team Member 2")
        }
        (2011, 79_101_620) => {
            String::from("Looking For Team")
        }
        (2011, 184_975_268) => {
            String::from("Sit")
        }
        (2011, 68_519_854) => {
            String::from("Controls")
        }
        (2011, 167_004_139) => {
            String::from("Sneak Attack")
        }
        (2011, 107_305_170) => {
            String::from("Select Pet 3")
        }
        (2011, 144_647_470) => {
            String::from("Auto Run")
        }
        (2011, 133_342_484) => {
            String::from("Reload")
        }
        (2011, 100_299_473) => {
            String::from("Select Team Member 1")
        }
        (2011, 175_126_324) => {
            String::from("Switch Fight Target")
        }
        (2011, 107_305_169) => {
            String::from("Select Pet 2")
        }
        (2011, 83_503_776) => {
            String::from("Planet Map")
        }
        (2011, 142_496_931) => {
            String::from("Options")
        }
        (2011, 246_891_888) => {
            String::from("Copy To Clipboard")
        }
        (2011, 116_463_611) => {
            String::from("Attack")
        }
        (2011, 127_033_122) => {
            String::from("Browser")
        }
        (2011, 124_841_956) => {
            String::from("Look At")
        }
        (2011, 122_042_382) => {
            String::from("Toggle 1st/3rd Person View")
        }
        (2011, 109_043_835) => {
            String::from("Sneak")
        }
        (2011, 146_800_388) => {
            String::from("Aimed Shot")
        }
        (2011, 153_934_508) => {
            String::from("Back (Global)")
        }
        (2011, 79_101_312) => {
            String::from("Map")
        }
        (2011, 107_305_171) => {
            String::from("Select Pet 4")
        }
        (2011, 17_073_428) => {
            String::from("Strafe Left")
        }
        (2011, 107_327_622) => {
            String::from("Select Self")
        }
        (2011, 107_897_508) => {
            String::from("Burst")
        }
        (2011, 59_445_843) => {
            String::from("Friends")
        }
        (2011, 113_869_531) => {
            String::from("Back")
        }
        (2011, 191_866_895) => {
            String::from("Programs")
        }
        (2011, 215_015_928) => {
            String::from("Research")
        }
        (2011, 216_033_467) => {
            String::from("Toggle Walk")
        }
        (2011, 44_310_413) => {
            String::from("Pickup Item")
        }
        (2011, 205_224_610) => {
            String::from("Shortcut Bar")
        }
        (2011, 191_449_917) => {
            String::from("Team")
        }
        (2011, 191_449_796) => {
            String::from("Stats")
        }
        (2011, 79_101_061) => {
            String::from("NCU")
        }
        (2011, 149_755_275) => {
            String::from("Fast Attack")
        }
        (2011, 212_380_052) => {
            String::from("Right")
        }
        (2011, 44_100_111) => {
            String::from("Full Auto")
        }
        (10005, 1) => {
            String::from("Are you sure you want to noobs?")
        }
        (10005, 2) => {
            String::from("If you leave Arete Landing, you can never return. Are you sure you are ready to leave?")
        }
        (10005, 3) => {
            String::from("Are you sure you want to steal the money?")
        }
        (1005, 103) => {
            String::from("Female")
        }
        (1005, 304) => {
            String::from("Utils2")
        }
        (1005, 603) => {
            String::from("Nanomage")
        }
        (1005, 402) => {
            String::from("Head")
        }
        (1005, 19_624_722) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Enables Special: {a} ({b})")
        }
        (1005, 38_097_604) => {
            String::from("Must be your own pet")
        }
        (1005, 213) => {
            String::from("RightFinger")
        }
        (1005, 98_550_394) => {
            String::from("Wield:\r\n")
        }
        (1005, 135_311_333) => {
            String::from("her.")
        }
        (1005, 235_574_452) => {
            let a = &arguments[0];
            format!("Must be a Unsupported NPCFamily {a}")
        }
        (1005, 209) => {
            String::from("LeftArm")
        }
        (1005, 27_694_167) => {
            String::from("Can not be flying.")
        }
        (1005, 219_463_474) => {
            let a = &arguments[0];
            format!("Must not have enough free NCU for: {a}")
        }
        (1005, 35_823_108) => {
            String::from("Target not ")
        }
        (1005, 97_101_338) => {
            String::from("UseOn:\r\n")
        }
        (1005, 309) => {
            String::from("Deck1")
        }
        (1005, 1021) => {
            String::from("On Secondary Item:")
        }
        (1005, 204) => {
            String::from("RightShoulder")
        }
        (1005, 147_226_634) => {
            String::from("Repair:\r\n")
        }
        (1005, 234_435_940) => {
            String::from("Must have 1 free inventory slot")
        }
        (1005, 89_409_589) => {
            String::from("NoName")
        }
        (1005, 142_943_468) => {
            String::from("it.")
        }
        (1005, 226_077_982) => {
            let a = &arguments[0];
            format!("Must not have equipped weapon of type: {a}")
        }
        (1005, 211_769_543) => {
            String::from("Must be flying.")
        }
        (1005, 1_947_524) => {
            String::from("XP gain enabled")
        }
        (1005, 302) => {
            String::from("Hud3")
        }
        (1005, 147_030_019) => {
            String::from("Remains")
        }
        (1005, 247_233_266) => {
            String::from("Must be a Control Tower")
        }
        (1005, 264_795_810) => {
            let a = &arguments[0];
            format!("Must have no other {a}.")
        }
        (1005, 305) => {
            String::from("Utils3")
        }
        (1005, 297_035) => {
            String::from("Bank")
        }
        (1005, 77_831_061) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("{a} temp set to {b}")
        }
        (1005, 206_171_452) => {
            String::from("not")
        }
        (1005, 68_619_838) => {
            String::from("Must not have any items equipped")
        }
        (1005, 30_326_789) => {
            String::from("Team Side: ")
        }
        (1005, 17_281_922) => {
            String::from("Must not be a player character")
        }
        (1005, 118_713_314) => {
            String::from("Can not be a monster.")
        }
        (1005, 206) => {
            String::from("LeftShoulder")
        }
        (1005, 5_661_794) => {
            String::from("or")
        }
        (1005, 226_089_886) => {
            let a = &arguments[0];
            format!("Must not have equipped right hand weapon of type: {a}")
        }
        (1005, 208) => {
            String::from("Hands")
        }
        (1005, 212) => {
            String::from("LeftWrist")
        }
        (1005, 508) => {
            String::from("Not wielding Basic Cyberdeck.")
        }
        (1005, 1_696_342) => {
            String::from("Not ")
        }
        (1005, 501) => {
            String::from("Wearing Grid Armor.")
        }
        (1005, 255_731_522) => {
            String::from("Must be a player character")
        }
        (1005, 600) => {
            String::from("Undefined breed")
        }
        (1005, 211_470_132) => {
            String::from("Must be polymorphed")
        }
        (1005, 161_024_999) => {
            let a = &arguments[0];
            format!("Base {a} below ")
        }
        (1005, 11_235_313) => {
            let a = &arguments[0];
            format!("Must not have nanoprogram: {a}")
        }
        (1005, 150_998_212) => {
            String::from(" and ")
        }
        (1005, 379_274) => {
            String::from("Use:\r\n")
        }
        (1005, 59_017_441) => {
            String::from("Map: ")
        }
        (1005, 146_655_790) => {
            let a = &arguments[0];
            format!("Must have expansionset: {a}")
        }
        (1005, 203) => {
            String::from("Back")
        }
        (1005, 311) => {
            String::from("Deck3")
        }
        (1005, 210) => {
            String::from("RightWrist")
        }
        (1005, 500) => {
            String::from("Not wearing Grid Armor.")
        }
        (1005, 5_871_653) => {
            let a = &arguments[0];
            format!("Scale {a}%")
        }
        (1005, 202) => {
            String::from("Head")
        }
        (1005, 43_904_176) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            format!("Scale at lvl {a} {b}% ({c} at lvl 200)")
        }
        (1005, 601) => {
            String::from("Solitus")
        }
        (1005, 90_585_156) => {
            String::from("and")
        }
        (1005, 261_696_229) => {
            let a = &arguments[0];
            format!("Must not have unique item: {a}")
        }
        (1005, 51_238_704) => {
            let a = &arguments[0];
            let b = &arguments[1];
            let c = &arguments[2];
            let d = &arguments[3];
            format!("{a} at level {b} {c} ({d} at lvl 200)")
        }
        (1005, 505) => {
            String::from("Wielding Jobe-chipped Cyberdeck.")
        }
        (1005, 313) => {
            String::from("Deck5")
        }
        (1005, 504) => {
            String::from("Not wielding Jobe-chipped Cyberdeck.")
        }
        (1005, 96_475_172) => {
            String::from("Monster type is Reet.")
        }
        (1005, 214_904_692) => {
            String::from("Must be in raid.")
        }
        (1005, 5_020_002) => {
            String::from("Error")
        }
        (1005, 127_210_309) => {
            String::from("Must Dual Wield Melee and distance weapon.")
        }
        (1005, 174_826_242) => {
            String::from("Must not be a Control Tower")
        }
        (1005, 1026) => {
            String::from("On User:")
        }
        (1005, 21_618_020) => {
            String::from("Must be in a transformed shape.")
        }
        (1005, 74_285_572) => {
            String::from("Must not have allied characters in combat")
        }
        (1005, 1110) => {
            String::from("On Fight Target:")
        }
        (1005, 28_755_821) => {
            String::from("Can not be in a team.")
        }
        (1005, 96_778_276) => {
            String::from("Monster type is Leet.")
        }
        (1005, 4_953_658) => {
            String::from("Drop:\r\n")
        }
        (1005, 179_552_215) => {
            String::from("Can not be in a fight.")
        }
        (1005, 237_116_148) => {
            let a = &arguments[0];
            format!("Must have {a} equipped")
        }
        (1005, 214_893_565) => {
            String::from("Must be in a team.")
        }
        (1005, 47_277_320) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Organization credits {a}{b}")
        }
        (1005, 211_711_630) => {
            let a = &arguments[0];
            format!("Must have equipped right hand weapon of type: {a}")
        }
        (1005, 249_068_204) => {
            String::from("Must have:\r\n")
        }
        (1005, 185_372_755) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Must have no more than {a} of pet type: {b}.")
        }
        (1005, 151_001_572) => {
            String::from("not")
        }
        (1005, 69_786_530) => {
            String::from("from")
        }
        (1005, 152_801_154) => {
            String::from("Paid Account")
        }
        (1005, 16_753_571) => {
            String::from("Cannot wield weapons.")
        }
        (1005, 93_771_044) => {
            let a = &arguments[0];
            format!("Must not have {a} equipped")
        }
        (1005, 511) => {
            String::from("Wielding Profession Nanodeck.")
        }
        (1005, 184_737_796) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Increase standing with {a} by {b}")
        }
        (1005, 212_249_301) => {
            String::from("While wielding Melee Weapons.")
        }
        (1005, 205) => {
            String::from("Chest")
        }
        (1005, 408) => {
            String::from("Waist")
        }
        (1005, 9_429_678) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("{a} temp set to {b}")
        }
        (1005, 101) => {
            String::from("Uni")
        }
        (1005, 315) => {
            String::from("Hud2")
        }
        (1005, 239_200_100) => {
            String::from("Must be sitting on ground")
        }
        (1005, 247_645_193) => {
            let a = &arguments[0];
            format!("Not affected by: {a}")
        }
        (1005, 16_669_669) => {
            String::from("Damage")
        }
        (1005, 102) => {
            String::from("Male")
        }
        (1005, 3_967_223) => {
            String::from("Must be in an organization\r\nMust have Booster Pack 1 - The Notum Wars")
        }
        (1005, 175_527_794) => {
            String::from("Free Account")
        }
        (1005, 185_789_767) => {
            String::from("Healing")
        }
        (1005, 267_085_425) => {
            String::from("Must be on Rubi-ka.")
        }
        (1005, 201) => {
            String::from("Neck")
        }
        (1005, 404) => {
            String::from("RightArm")
        }
        (1005, 604) => {
            String::from("Athrox")
        }
        (1005, 66_916_441) => {
            String::from("Outdoors only.")
        }
        (1005, 113_782_873) => {
            String::from("Indoors only.")
        }
        (1005, 104_594_638) => {
            String::from("First Specialization Completed.")
        }
        (1005, 126_687_412) => {
            String::from("Monster type is Pit Lizard.")
        }
        (1005, 59_039_217) => {
            String::from("None")
        }
        (1005, 188_255_332) => {
            String::from("\r\nTarget:  ")
        }
        (1005, 318_586) => {
            String::from("Get:\r\n")
        }
        (1005, 101_518_340) => {
            String::from("Organization contract")
        }
        (1005, 236_107_580) => {
            let a = &arguments[0];
            format!("Enables Special: {a}")
        }
        (1005, 102_366_414) => {
            String::from("Third Specialization Completed.")
        }
        (1005, 103_671_093) => {
            let a = &arguments[0];
            format!("Must have unique item: {a}")
        }
        (1005, 314) => {
            String::from("Deck6")
        }
        (1005, 310) => {
            String::from("Deck2")
        }
        (1005, 11_455_727) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Resist Nano: {a}% {b}")
        }
        (1005, 1019) => {
            String::from("On Self:")
        }
        (1005, 94_674_756) => {
            String::from("Self")
        }
        (1005, 214) => {
            String::from("Feet")
        }
        (1005, 16_258_836) => {
            String::from("Unable to locate any treatment item in root inventory.")
        }
        (1005, 187_755_044) => {
            String::from("Must not be a Psychosis Pet")
        }
        (1005, 413) => {
            String::from("Feet")
        }
        (1005, 411) => {
            String::from("Legs")
        }
        (1005, 602) => {
            String::from("Opifex")
        }
        (1005, 303) => {
            String::from("Utils1")
        }
        (1005, 13_169_918) => {
            String::from("Must be in an organization.")
        }
        (1005, 101_432_526) => {
            String::from("Fourth Specialization Completed.")
        }
        (1005, 407) => {
            String::from("RightWrist")
        }
        (1005, 312) => {
            String::from("Deck4")
        }
        (1005, 510) => {
            String::from("Not Wielding Profession Nanodeck.")
        }
        (1005, 6_142_042) => {
            String::from("Wear:\r\n")
        }
        (1005, 306) => {
            String::from("RightHand")
        }
        (1005, 214_489_095) => {
            String::from("Overflow")
        }
        (1005, 261_314_147) => {
            let a = &arguments[0];
            format!("Must have {a} free inventory slots")
        }
        (1005, 200) => {
            String::from("Default")
        }
        (1005, 109_081_314) => {
            let a = &arguments[0];
            format!("Must have enough free NCU for: {a}")
        }
        (1005, 162_952_846) => {
            let a = &arguments[0];
            format!("Must have equipped weapon of type: {a}")
        }
        (1005, 114_151_651) => {
            String::from("Must be in the shadowlands.")
        }
        (1005, 147_164_012) => {
            String::from(" not ")
        }
        (1005, 212_671_124) => {
            String::from("Complete mission")
        }
        (1005, 606) => {
            String::from("Monster")
        }
        (1005, 507) => {
            String::from("Wielding Augmented Cyberdeck.")
        }
        (1005, 218_850_008) => {
            String::from("Monster type is Sabretooth.")
        }
        (1005, 502) => {
            String::from("Not wielding Izgimmer-modified Cyberdeck.")
        }
        (1005, 58_139_847) => {
            String::from("Must be in a fight.")
        }
        (1005, 301) => {
            String::from("Hud1")
        }
        (1005, 240_038_980) => {
            String::from("Must be in combat mode.")
        }
        (1005, 8_253_517) => {
            let a = &arguments[0];
            format!("Upload nanoprogram {a}")
        }
        (1005, 3_038_100) => {
            String::from("Must have allied characters in combat")
        }
        (1005, 5_470_863) => {
            String::from("Macro")
        }
        (1005, 139_231_573) => {
            String::from("Must have normal shape.")
        }
        (1005, 410) => {
            String::from("RightHand")
        }
        (1005, 263_089_668) => {
            String::from("Must be able to teleport")
        }
        (1005, 76_477_381) => {
            String::from("Nanopoints -")
        }
        (1005, 98_551_738) => {
            String::from("Unwield:\r\n")
        }
        (1005, 206_123_628) => {
            String::from("Send mail")
        }
        (1005, 143_655_330) => {
            String::from(" or ")
        }
        (1005, 503) => {
            String::from("Wielding Izgimmer-modified Cyberdeck.")
        }
        (1005, 5_023_700) => {
            String::from("Must not be a Combat Pet")
        }
        (1005, 207_468_554) => {
            String::from("Unwear:\r\n")
        }
        (1005, 155_064_596) => {
            String::from("Must be a Robotic Pet")
        }
        (1005, 196_437_444) => {
            String::from("Must not be a Charmed Pet")
        }
        (1005, 26_616_357) => {
            String::from("Target ")
        }
        (1005, 42_637_588) => {
            String::from("Last Random Roll")
        }
        (1005, 307) => {
            String::from("Deck")
        }
        (1005, 76_347_700) => {
            let a = &arguments[0];
            format!("Stop combat ({a})")
        }
        (1005, 182_615_652) => {
            String::from("Delete mission")
        }
        (1005, 1_634_340) => {
            String::from("Fail mission")
        }
        (1005, 509) => {
            String::from("Wielding Basic Cyberdeck.")
        }
        (1005, 176_741_418) => {
            String::from("Target: ")
        }
        (1005, 247_145_937) => {
            String::from("Must have this Manta equipped")
        }
        (1005, 205_893_412) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("{a} temp set to {b}")
        }
        (1005, 13_293_284) => {
            String::from("Must not be a Robotic Pet")
        }
        (1005, 86_152_078) => {
            let a = &arguments[0];
            format!("Completed mission {a}")
        }
        (1005, 151_017_572) => {
            String::from("Must be a Combat Pet")
        }
        (1005, 97_989_742) => {
            String::from("Must be in your organization.")
        }
        (1005, 131_655_476) => {
            String::from("Must not be a Healing Pet")
        }
        (1005, 267_455_413) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Decrease standing with {a} by {b}")
        }
        (1005, 412) => {
            String::from("LeftHand")
        }
        (1005, 157_081_285) => {
            String::from("While not wielding melee weapons.")
        }
        (1005, 187_467_428) => {
            let a = &arguments[0];
            format!("Perk {a} cannot be locked")
        }
        (1005, 308) => {
            String::from("LeftHand")
        }
        (1005, 104_631_836) => {
            String::from("exactly")
        }
        (1005, 216_329_491) => {
            String::from("Must not have any other maps installed.")
        }
        (1005, 100) => {
            String::from("Undefined")
        }
        (1005, 409) => {
            String::from("LeftWrist")
        }
        (1005, 96_491_462) => {
            String::from("Monster type is Wolf.")
        }
        (1005, 103_152_846) => {
            String::from("Second Specialization Completed.")
        }
        (1005, 405) => {
            String::from("Chest")
        }
        (1005, 13_926_807) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("<font color=CCInfoHeader>Aligned to:</font>\r\n<font color=CCInfoText>Because of your standing with </font><font color=CCInfoHeader> {a} </font><font color=CCInfoText>, you are seen as </font><font color=CCInfoHeader> {b} </font><font color=CCInfoText>")
        }
        (1005, 164_230_325) => {
            String::from("him.")
        }
        (1005, 175_924_644) => {
            let a = &arguments[0];
            format!("Perk {a} must be locked")
        }
        (1005, 268_383_124) => {
            String::from("Must be a Psychosis Pet")
        }
        (1005, 300) => {
            String::from("Default")
        }
        (1005, 107_228_403) => {
            String::from("below")
        }
        (1005, 35_760_660) => {
            String::from("Must be a Charmed Pet")
        }
        (1005, 196_002_905) => {
            let a = &arguments[0];
            format!("Affected by: {a}")
        }
        (1005, 184_745_572) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Set standing with {a} to {b}")
        }
        (1005, 141_201_086) => {
            String::from("Without an organization.")
        }
        (1005, 607) => {
            String::from("human monster")
        }
        (1005, 215) => {
            String::from("LeftFinger")
        }
        (1005, 144_301_533) => {
            let a = &arguments[0];
            format!("Base {a} from ")
        }
        (1005, 241_969_284) => {
            String::from("Must be a Healing Pet")
        }
        (1005, 1018) => {
            String::from("On Target:")
        }
        (1005, 16_478_419) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Must have at least {a} of pet type: {b}.")
        }
        (1005, 400) => {
            String::from("Default")
        }
        (1005, 5_276_483) => {
            String::from("NPC character")
        }
        (1005, 207) => {
            String::from("RightArm")
        }
        (1005, 211) => {
            String::from("Legs")
        }
        (1005, 22852) => {
            String::from("Random Roll")
        }
        (1005, 98_142_478) => {
            let a = &arguments[0];
            let b = &arguments[1];
            format!("Modification to base of {a} skill by {b}%")
        }
        (1005, 401) => {
            String::from("Eyes")
        }
        (1005, 116_138_407) => {
            String::from("Must be standing")
        }
        (1005, 506) => {
            String::from("Not wielding Augmented Cyberdeck.")
        }
        (1005, 122_566_574) => {
            String::from("Visual Profession: ")
        }
        (1005, 230_543_061) => {
            let a = &arguments[0];
            format!("Scale {a}%")
        }
        (1005, 406) => {
            String::from("LeftArm")
        }
        (1005, 103_617_476) => {
            String::from("Can not have vehicle equipped.")
        }
        (1005, 605) => {
            String::from("Special")
        }
        (1005, 75_839_476) => {
            String::from("Nanopoints +")
        }
        (1005, 103_919_497) => {
            String::from("Outdoor only.")
        }
        (1005, 132_346_049) => {
            String::from("Must dual wield melee, distance and be able to attack with martial arts.")
        }
        (1005, 403) => {
            String::from("Ears")
        }
        (1005, 84_065_212) => {
            String::from("in duel")
        }
        (1005, 132_617_329) => {
            let a = &arguments[0];
            format!("Must have nanoprogram: {a}")
        }
        (1005, 247_564_329) => {
            String::from("Battle station only.")
        }
        (1005, 72_324_404) => {
            String::from("XP gain disabled")
        }
        (1005, 36_490_901) => {
            String::from("While wielding distance weapons.")
        }
        (1005, 1100) => {
            String::from("On Caster:")
        }
        (2015, 11) => {
            String::from("enemy healer")
        }
        (2015, 23) => {
            String::from("selected target")
        }
        (2015, 24) => {
            String::from("last follow target")
        }
        (2015, 5) => {
            String::from("transfer")
        }
        (2015, 12) => {
            String::from("friend attacker")
        }
        (2015, 13) => {
            String::from("command target")
        }
        (2015, 17) => {
            String::from("last opponent")
        }
        (2015, 8) => {
            String::from("attacker")
        }
        (2015, 7) => {
            String::from("person spotted")
        }
        (2015, 10) => {
            String::from("master")
        }
        (2015, 6) => {
            String::from("ground")
        }
        (2015, 2) => {
            String::from("user")
        }
        (2015, 19) => {
            String::from("person lost")
        }
        (2015, 4) => {
            String::from("item")
        }
        (2015, 20) => {
            String::from("pet")
        }
        (2015, 1) => {
            String::from("self")
        }
        (2015, 9) => {
            String::from("victim")
        }
        (2015, 15) => {
            String::from("scary enemy")
        }
        (2015, 18) => {
            String::from("person leaving")
        }
        (2015, 3) => {
            String::from("target")
        }
        (2015, 16) => {
            String::from("follow target")
        }
        (2015, 21) => {
            String::from("area")
        }
        (2015, 22) => {
            String::from("commander")
        }
        (2015, 14) => {
            String::from("fight target")
        }
        (503, 165_609_097) => {
            String::from("Is the proximity of identical tower types legal?")
        }
        (503, 256_387_540) => {
            String::from("War Checklist")
        }
        (503, 240_729_620) => {
            String::from("Disable Shield")
        }
        (503, 173_631_701) => {
            String::from("Is the distance to other towers sufficient?")
        }
        (503, 184_626_212) => {
            String::from("Am I powerful enough to build more towers?")
        }
        (503, 259_171_525) => {
            String::from("Am I building a unique controller?")
        }
        (503, 162_087_284) => {
            String::from("<font color=CCInfoHeadline>Drop the Defense Shield Neutralizer item here. If all checks are ok, you can disable the shield.</font>")
        }
        (503, 184_388_631) => {
            String::from("Is the area the property of my organization?")
        }
        (503, 117_792_508) => {
            String::from("Is the quality of the neutralizer sufficient?")
        }
        (503, 160_547_149) => {
            String::from("This is not a tower item.")
        }
        (503, 47_932_300) => {
            String::from("Is the tower within the area's level range?")
        }
        (503, 51_538_629) => {
            String::from("Does the current suppression gas allow me to attack?")
        }
        (503, 117_701_173) => {
            String::from("Is the area in a state of peace?")
        }
        (503, 243_560_868) => {
            String::from("Can my organization support more controllers?")
        }
        (503, 96_890_228) => {
            String::from("Is there a controller in this area?")
        }
        (503, 90_530_263) => {
            String::from("<font color=CCInfoHeadline>Drop the tower item you want to build. If all checks are ok, you can build it here.</font>")
        }
        (503, 137_212_273) => {
            String::from("Do I meet the skill requirements?")
        }
        (503, 219_475_444) => {
            String::from("Tower Build Menu")
        }
        (503, 17_289_613) => {
            String::from("Insert Tower Item")
        }
        (503, 250_180_887) => {
            String::from("Am I in an organization?")
        }
        (503, 234_933_476) => {
            String::from("Is the area approved for notum mining by the ICC?")
        }
        (503, 46_253_412) => {
            String::from("Is the total number of towers in the area legal?")
        }
        (503, 19_992_325) => {
            String::from("Am I building a tower unique to me?")
        }
        (503, 118_100_695) => {
            String::from("Am I standing on the ground?")
        }
        (503, 250_179_424) => {
            String::from("Do I have the Notum Wars expansion?")
        }
        (503, 47_516_468) => {
            String::from("Controller Build Menu")
        }
        (503, 112_132_436) => {
            String::from("Am I in a land control area?")
        }
        (503, 219_161_899) => {
            String::from("Check")
        }
        (503, 62_691_284) => {
            String::from("Is the controller's defence shield active?")
        }
        (503, 3_392_181) => {
            String::from("Is my level wihtin PvP range of the tower?")
        }
        (503, 85_094_203) => {
            String::from("Is my organization ranking high enough?")
        }
        (503, 21_475_490) => {
            String::from("Is the area controller-free?")
        }
        (503, 242_503_981) => {
            String::from("Insert Neutralizer Item")
        }
        (503, 219_181_716) => {
            String::from("Build")
        }
        (5000, 6_399_012) => {
            String::from("<font color=CCInfoHeader>You are about to report this bug:</font><br>\r\n<font color=CCInfoText>")
        }
        (5000, 40_655_268) => {
            String::from("The bug report has been sent; thank you for reporting it.")
        }
        (5000, 179_026_692) => {
            String::from("The bug report could not be sent.  Reason: Unknown.")
        }
        (5000, 16_883_060) => {
            String::from("The bug report was not sent.")
        }
        (2009, 631) => {
            String::from("Enforcer Taunt Procs Hatebringer")
        }
        (2009, 1019) => {
            String::from("Pet Warp")
        }
        (2009, 538) => {
            String::from("Add All Def. Perk Buff")
        }
        (2009, 489) => {
            String::from("Perk Seppuku Slash")
        }
        (2009, 752) => {
            String::from("Performed Hecatomb")
        }
        (2009, 270) => {
            String::from("Trader Team Heals 13")
        }
        (2009, 704) => {
            String::from("Road To Darkness")
        }
        (2009, 699) => {
            String::from("Silence Debuff")
        }
        (2009, 253) => {
            String::from("Fixer Suppressor Buff")
        }
        (2009, 221) => {
            String::from("Metaphysicist Mind Damage Nano Debuffs")
        }
        (2009, 303) => {
            String::from("Perk Concussive Shot")
        }
        (2009, 346) => {
            String::from("Perk Governance")
        }
        (2009, 82) => {
            String::from("General Knife Debuff")
        }
        (2009, 606) => {
            String::from("MonsterEffect4")
        }
        (2009, 739) => {
            String::from("Performed Ritual of Devotion")
        }
        (2009, 942) => {
            String::from("Derivate")
        }
        (2009, 358) => {
            String::from("Perk Breach Defenses")
        }
        (2009, 927) => {
            String::from("Mark of the Pious")
        }
        (2009, 822) => {
            String::from("Ancient Blessings")
        }
        (2009, 914) => {
            String::from("Vehicles")
        }
        (2009, 196) => {
            String::from("Chemistry/Pharm Buff")
        }
        (2009, 492) => {
            String::from("Perk Dimensional Fist")
        }
        (2009, 421) => {
            String::from("Perk Notum Overflow")
        }
        (2009, 68) => {
            String::from("General Field Quantum Physics Buff")
        }
        (2009, 590) => {
            String::from("Lifeblood")
        }
        (2009, 940) => {
            String::from("Notum Spring")
        }
        (2009, 94) => {
            String::from("General Melee AC Buff")
        }
        (2009, 260) => {
            String::from("Trader Team Heals 3")
        }
        (2009, 262) => {
            String::from("Trader Team Heals 5")
        }
        (2009, 313) => {
            String::from("Perk Bio Regrowth")
        }
        (2009, 448) => {
            String::from("Perk Blade of Night")
        }
        (2009, 665) => {
            String::from("Blade of Night")
        }
        (2009, 862) => {
            String::from("Nano Resist Debuff Proc")
        }
        (2009, 499) => {
            String::from("Perk Quark Containment Field")
        }
        (2009, 98) => {
            String::from("General Deflect Buff")
        }
        (2009, 844) => {
            String::from("Health Drain")
        }
        (2009, 39) => {
            String::from("General Bow Special Buff")
        }
        (2009, 688) => {
            String::from("Siphon Box")
        }
        (2009, 734) => {
            String::from("Quark Containment Field")
        }
        (2009, 398) => {
            String::from("Perk Taunt")
        }
        (2009, 360) => {
            String::from("Perk Exploration Teleportation 1")
        }
        (2009, 728) => {
            String::from("Shield of Light")
        }
        (2009, 615) => {
            String::from("Hecatomb Bleeding Wounds")
        }
        (2009, 52) => {
            String::from("General Dimach Debuff")
        }
        (2009, 200) => {
            String::from("Psychology Buff")
        }
        (2009, 532) => {
            String::from("Keeper Absorb Aura-Team")
        }
        (2009, 504) => {
            String::from("Perk Ignition Flare")
        }
        (2009, 651) => {
            String::from("Toxic Shock")
        }
        (2009, 225) => {
            String::from("Pet Short Term Damage Buffs")
        }
        (2009, 911) => {
            String::from("Dark Ruins Root and Snare")
        }
        (2009, 364) => {
            String::from("Perk Gutting Blow")
        }
        (2009, 58) => {
            String::from("General Strength Debuff")
        }
        (2009, 957) => {
            String::from("Alien Parasite")
        }
        (2009, 356) => {
            String::from("Perk Reconstruct DNA")
        }
        (2009, 66) => {
            String::from("General Fast Attack Buff")
        }
        (2009, 328) => {
            String::from("Perk Bureaucratic Shuffle")
        }
        (2009, 144) => {
            String::from("Major Evasion Buffs")
        }
        (2009, 47) => {
            String::from("General Chemistry Buff")
        }
        (2009, 425) => {
            String::from("Perk Dominator")
        }
        (2009, 288) => {
            String::from("Engineer Pet AOE Snare Buff")
        }
        (2009, 460) => {
            String::from("Perk Contained Burst")
        }
        (2009, 864) => {
            String::from("Magnifying Glass Buffs")
        }
        (2009, 95) => {
            String::from("General Nano Programming Buff")
        }
        (2009, 173) => {
            String::from("Field Quantum Physics Buff")
        }
        (2009, 182) => {
            String::from("Critical Increase Buff")
        }
        (2009, 744) => {
            String::from("Performed Devour Essence")
        }
        (2009, 440) => {
            String::from("Perk Power Combo")
        }
        (2009, 1016) => {
            String::from("Heal Pets")
        }
        (2009, 443) => {
            String::from("Perk Spirit Dissolution")
        }
        (2009, 430) => {
            String::from("Perk Impale")
        }
        (2009, 395) => {
            String::from("Perk Called Shot")
        }
        (2009, 528) => {
            String::from("Keeper Aura-Absorb/Reflect/AMS Buff")
        }
        (2009, 588) => {
            String::from("Nano Point Heals")
        }
        (2009, 545) => {
            String::from("Keeper Evade/Dodge/Duck Buff")
        }
        (2009, 573) => {
            String::from("MonsterWaveSpawn9")
        }
        (2009, 433) => {
            String::from("Perk Quick Shot")
        }
        (2009, 117) => {
            String::from("General SenseImp Debuff")
        }
        (2009, 724) => {
            String::from("Tunnel of Light")
        }
        (2009, 11) => {
            String::from("Halo Nano Debuff")
        }
        (2009, 1010) => {
            String::from("Root Removal Self")
        }
        (2009, 1015) => {
            String::from("Attack Pets")
        }
        (2009, 1044) => {
            String::from("AOE Nuke")
        }
        (2009, 92) => {
            String::from("General MatMet Debuff")
        }
        (2009, 17) => {
            String::from("General 1Hand Blunt Debuff")
        }
        (2009, 847) => {
            String::from("Skill Lock Modifier Debuff")
        }
        (2009, 19) => {
            String::from("General Aimed Shot Debuff")
        }
        (2009, 343) => {
            String::from("Perk Suppressive Primer")
        }
        (2009, 933) => {
            String::from("Bring The Pain")
        }
        (2009, 842) => {
            String::from("DeTaunt")
        }
        (2009, 523) => {
            String::from("Shade Init Debuff Proc")
        }
        (2009, 638) => {
            String::from("Atrophy")
        }
        (2009, 801) => {
            String::from("NT Area Nukes")
        }
        (2009, 414) => {
            String::from("Perk Strip Nano")
        }
        (2009, 503) => {
            String::from("Perk Sabotage Quark Field")
        }
        (2009, 571) => {
            String::from("MonsterWaveSpawn7")
        }
        (2009, 849) => {
            String::from("Incapacitate")
        }
        (2009, 785) => {
            String::from("Environmental Damage")
        }
        (2009, 711) => {
            String::from("Scream of Death")
        }
        (2009, 181) => {
            String::from("NF Range Buff")
        }
        (2009, 473) => {
            String::from("Perk Capture Spirit")
        }
        (2009, 727) => {
            String::from("Screen of Light")
        }
        (2009, 534) => {
            String::from("Keeper Reflect Aura-Team")
        }
        (2009, 134) => {
            String::from("General Weapon Smithing Buff")
        }
        (2009, 149) => {
            String::from("General Runspeed Buffs")
        }
        (2009, 87) => {
            String::from("General MatCrea Buff")
        }
        (2009, 77) => {
            String::from("General Grenade Buff")
        }
        (2009, 721) => {
            String::from("Hope Debuff")
        }
        (2009, 444) => {
            String::from("Perk Fade Armor")
        }
        (2009, 816) => {
            String::from("Pet Defensive Nanos")
        }
        (2009, 883) => {
            String::from("Fear - PVP")
        }
        (2009, 1012) => {
            String::from("Root Removal Team")
        }
        (2009, 853) => {
            String::from("Reanimated Cloak Debuffs")
        }
        (2009, 873) => {
            String::from("DBPF Teleport E")
        }
        (2009, 447) => {
            String::from("Perk Shadow Stab")
        }
        (2009, 49) => {
            String::from("General Cold AC Buff")
        }
        (2009, 514) => {
            String::from("Perk SPECIAL Acrobat")
        }
        (2009, 754) => {
            String::from("Performed Capture Essence")
        }
        (2009, 698) => {
            String::from("Silence")
        }
        (2009, 818) => {
            String::from("Cold Blooded")
        }
        (2009, 135) => {
            String::from("Trader Skill Transfer Target Debuff (Deprive)")
        }
        (2009, 1041) => {
            String::from("Nuke")
        }
        (2009, 165) => {
            String::from("SenseImp Buff")
        }
        (2009, 759) => {
            String::from("Performed Unsealed Pestilence")
        }
        (2009, 78) => {
            String::from("General Grenade Debuff")
        }
        (2009, 90) => {
            String::from("General MatLoc Debuff")
        }
        (2009, 491) => {
            String::from("Perk Etheral Touch")
        }
        (2009, 172) => {
            String::from("Electrical Engineering Buff")
        }
        (2009, 159) => {
            String::from("MatCrea Buff")
        }
        (2009, 422) => {
            String::from("Perk Stoneworks")
        }
        (2009, 556) => {
            String::from("Perk Nano Surgeon")
        }
        (2009, 56) => {
            String::from("General Sense Debuff")
        }
        (2009, 276) => {
            String::from("TowerSmokeBuffEffects")
        }
        (2009, 251) => {
            String::from("Adventurer Morph Buff")
        }
        (2009, 382) => {
            String::from("Perk Chaotic Energy")
        }
        (2009, 630) => {
            String::from("Enforcer Taunt Procs Wrathbringer")
        }
        (2009, 645) => {
            String::from("Called Shot Bleeding Wounds")
        }
        (2009, 702) => {
            String::from("Path of Darkness")
        }
        (2009, 817) => {
            String::from("Pet Damage Over Time Resist Nanos")
        }
        (2009, 684) => {
            String::from("Gadgeteer Pet Procs")
        }
        (2009, 128) => {
            String::from("General Cold AC Debuff")
        }
        (2009, 361) => {
            String::from("Perk Exploration Teleportation 2")
        }
        (2009, 513) => {
            String::from("Perk ECM 2")
        }
        (2009, 560) => {
            String::from("GM Nano buff")
        }
        (2009, 312) => {
            String::from("Perk Bio Rejuvenation")
        }
        (2009, 415) => {
            String::from("Perk Annihilate Notum Molecules")
        }
        (2009, 707) => {
            String::from("The Choice Debuff (Omni)")
        }
        (2009, 551) => {
            String::from("Perk Channeling Of Notum-Heal Stopper")
        }
        (2009, 142) => {
            String::from("Trader AC Transfer Caster Buff (Draw)")
        }
        (2009, 185) => {
            String::from("Doctor Short HP Buffs")
        }
        (2009, 379) => {
            String::from("Perk Reconstruction")
        }
        (2009, 394) => {
            String::from("Perk Find the Flaw")
        }
        (2009, 35) => {
            String::from("General BioMet Buff")
        }
        (2009, 438) => {
            String::from("Perk Power Shock")
        }
        (2009, 1005) => {
            String::from("AOE Snare")
        }
        (2009, 703) => {
            String::from("Path of Darkness Debuff")
        }
        (2009, 672) => {
            String::from("Bleeding Wounds")
        }
        (2009, 593) => {
            String::from("Ethereal Touch")
        }
        (2009, 807) => {
            String::from("Healing Construct Empowerment")
        }
        (2009, 766) => {
            String::from("Grove Healing Multiplier")
        }
        (2009, 1048) => {
            String::from("Proximity Range Debuff")
        }
        (2009, 9) => {
            String::from("DOT Agent Strain A")
        }
        (2009, 107) => {
            String::from("General Psychology Buff")
        }
        (2009, 86) => {
            String::from("General Martial Arts Debuff")
        }
        (2009, 486) => {
            String::from("Perk Deep Cuts")
        }
        (2009, 338) => {
            String::from("Perk Blessing Of Life")
        }
        (2009, 123) => {
            String::from("General Poison AC Debuff")
        }
        (2009, 248) => {
            String::from("Morph Heal")
        }
        (2009, 258) => {
            String::from("Trader Team Heals 1")
        }
        (2009, 627) => {
            String::from("Enforcer Taunt Procs")
        }
        (2009, 22) => {
            String::from("General 1H Edged Debuff")
        }
        (2009, 25) => {
            String::from("General 2H Edged Buff")
        }
        (2009, 1025) => {
            String::from("Nukes")
        }
        (2009, 281) => {
            String::from("Soldier Full Auto Buff")
        }
        (2009, 949) => {
            String::from("General Perception Buff")
        }
        (2009, 1051) => {
            String::from("Drain Heal")
        }
        (2009, 774) => {
            String::from("Affected by Deceptive Stance")
        }
        (2009, 240) => {
            String::from("Concentration Critical Line")
        }
        (2009, 1036) => {
            String::from("Summon Item")
        }
        (2009, 12) => {
            String::from("Heal Over Time")
        }
        (2009, 241) => {
            String::from("Sureshot Critical Line")
        }
        (2009, 632) => {
            String::from("Enforcer Taunt Procs Ragebringer")
        }
        (2009, 603) => {
            String::from("MonsterEffect1")
        }
        (2009, 934) => {
            String::from("Chemical Blindness")
        }
        (2009, 618) => {
            String::from("Perk Cleave")
        }
        (2009, 690) => {
            String::from("Install Explosive Device DoT")
        }
        (2009, 168) => {
            String::from("PsyMod Debuff")
        }
        (2009, 694) => {
            String::from("Shadowland Reflect Base")
        }
        (2009, 758) => {
            String::from("Performed Unsealed Blight")
        }
        (2009, 828) => {
            String::from("Health and Nano Over Time Drain")
        }
        (2009, 250) => {
            String::from("Pack Hunter Buff")
        }
        (2009, 230) => {
            String::from("Engineer Aura-Damage Shield Buff")
        }
        (2009, 349) => {
            String::from("Perk Reap Life")
        }
        (2009, 388) => {
            String::from("Perk Napalm Spray")
        }
        (2009, 782) => {
            String::from("WaitForAttackEffectNano2")
        }
        (2009, 126) => {
            String::from("General Tutoring Buff")
        }
        (2009, 1035) => {
            String::from("Spirit Drain")
        }
        (2009, 298) => {
            String::from("Perk Chemical Blindness")
        }
        (2009, 137) => {
            String::from("Trader Skill Transfer Caster Buff (Deprive)")
        }
        (2009, 325) => {
            String::from("Perk Big Smash")
        }
        (2009, 709) => {
            String::from("Slam of Darkness")
        }
        (2009, 53) => {
            String::from("General Agility Debuff")
        }
        (2009, 808) => {
            String::from("PH")
        }
        (2009, 812) => {
            String::from("Research Ability 1")
        }
        (2009, 779) => {
            String::from("Affected by Spirit of Purity")
        }
        (2009, 1023) => {
            String::from("Pet Proc (Line A)")
        }
        (2009, 275) => {
            String::from("UNUSED1")
        }
        (2009, 359) => {
            String::from("Perk Nano Heal")
        }
        (2009, 652) => {
            String::from("Toxic Shock Proc Effect")
        }
        (2009, 244) => {
            String::from("1HEdged Buff")
        }
        (2009, 215) => {
            String::from("Nano Drain - Line A")
        }
        (2009, 228) => {
            String::from("Engineer Aura-Armour")
        }
        (2009, 2) => {
            String::from("Reflect Shield")
        }
        (2009, 483) => {
            String::from("Perk Combust")
        }
        (2009, 537) => {
            String::from("Perk SPECIAL Assasin")
        }
        (2009, 786) => {
            String::from("Fixer Runspeed Base")
        }
        (2009, 938) => {
            String::from("Way of The Atrox")
        }
        (2009, 600) => {
            String::from("Ritual of Zeal")
        }
        (2009, 561) => {
            String::from("Perk Nano Surgeon")
        }
        (2009, 871) => {
            String::from("DBPF Teleport C")
        }
        (2009, 1027) => {
            String::from("Finishing Nukes")
        }
        (2009, 729) => {
            String::from("Shield of Light Buff")
        }
        (2009, 96) => {
            String::from("General Nano AC Buff")
        }
        (2009, 257) => {
            String::from("Fixer NCU Buff")
        }
        (2009, 452) => {
            String::from("Perk Leg Shot")
        }
        (2009, 799) => {
            String::from("Boss Root")
        }
        (2009, 333) => {
            String::from("Perk Elementary Teleportation 2")
        }
        (2009, 474) => {
            String::from("Perk Unsealed Contagation")
        }
        (2009, 125) => {
            String::from("General Treatment Buff")
        }
        (2009, 310) => {
            String::from("Perk Bio Shield")
        }
        (2009, 706) => {
            String::from("The Choice (Omni)")
        }
        (2009, 219) => {
            String::from("Absorb AC Buff")
        }
        (2009, 522) => {
            String::from("Shade HP/NP DoT Proc-Damage Inflict Segment")
        }
        (2009, 222) => {
            String::from("Controlled Destruction Buff")
        }
        (2009, 671) => {
            String::from("Knowledge Enhancer")
        }
        (2009, 693) => {
            String::from("Install Notum Depletion Device Countdown")
        }
        (2009, 408) => {
            String::from("Perk Enhanced Heal")
        }
        (2009, 1017) => {
            String::from("Support Pets")
        }
        (2009, 156) => {
            String::from("Strength Buff")
        }
        (2009, 340) => {
            String::from("Perk Draw Blood")
        }
        (2009, 273) => {
            String::from("Trader Team Heals 16")
        }
        (2009, 0) => {
            String::from("NO STACKING")
        }
        (2009, 622) => {
            String::from("Perk Pulverize")
        }
        (2009, 501) => {
            String::from("Perk Knowledge Enhancer")
        }
        (2009, 247) => {
            String::from("Kin of Tarasque")
        }
        (2009, 429) => {
            String::from("Perk Lacerate")
        }
        (2009, 498) => {
            String::from("Perk Chtonian Symbiosis")
        }
        (2009, 595) => {
            String::from("Nano Recharge")
        }
        (2009, 819) => {
            String::from("Singed Fists")
        }
        (2009, 926) => {
            String::from("Social Pets")
        }
        (2009, 352) => {
            String::from("Perk Quick Cut")
        }
        (2009, 239) => {
            String::from("Nano Shutdown Debuff")
        }
        (2009, 653) => {
            String::from("Dodge the Blame")
        }
        (2009, 952) => {
            String::from("Team Healing")
        }
        (2009, 552) => {
            String::from("Perk Theoretical Research")
        }
        (2009, 370) => {
            String::from("Perk Avalanche")
        }
        (2009, 290) => {
            String::from("Teporary Root/Snare Resistance Buff")
        }
        (2009, 122) => {
            String::from("General Nano AC Debuff")
        }
        (2009, 619) => {
            String::from("Perk Transfix")
        }
        (2009, 103) => {
            String::from("General Pistol Buff")
        }
        (2009, 856) => {
            String::from("Nano Drain - Line B")
        }
        (2009, 803) => {
            String::from("Scones")
        }
        (2009, 838) => {
            String::from("Dust Brigade Turrets I")
        }
        (2009, 136) => {
            String::from("Trader Skill Transfer Target Debuff (Ransack)")
        }
        (2009, 1034) => {
            String::from("Team Run Speed Buffs")
        }
        (2009, 294) => {
            String::from("BurntOutArmor Proc")
        }
        (2009, 305) => {
            String::from("Perk BattlegroupHeal 1")
        }
        (2009, 324) => {
            String::from("Perk Devastating Blow")
        }
        (2009, 434) => {
            String::from("Perk Double Shot")
        }
        (2009, 1061) => {
            String::from("Nano Over Time - Line B")
        }
        (2009, 623) => {
            String::from("Perk Hammer And Anvil")
        }
        (2009, 143) => {
            String::from("Trader AC Transfer Target Buff (Redeem)")
        }
        (2009, 637) => {
            String::from("MonsterEffect_MainLoop")
        }
        (2009, 482) => {
            String::from("Perk Dazzle with Lights")
        }
        (2009, 813) => {
            String::from("Research Ability 2")
        }
        (2009, 833) => {
            String::from("Bureaucrat Research Stun 1")
        }
        (2009, 834) => {
            String::from("Bureaucrat Research Stun 2")
        }
        (2009, 845) => {
            String::from("Damage Drain")
        }
        (2009, 931) => {
            String::from("Freak Shield")
        }
        (2009, 46) => {
            String::from("General Chemical AC Buff")
        }
        (2009, 55) => {
            String::from("General Psychic Debuff")
        }
        (2009, 453) => {
            String::from("Perk Easy Shot")
        }
        (2009, 207) => {
            String::from("Grenade Buffs")
        }
        (2009, 476) => {
            String::from("Perk Bane")
        }
        (2009, 594) => {
            String::from("Convulsive Tremor")
        }
        (2009, 148) => {
            String::from("NP Cost Buff")
        }
        (2009, 177) => {
            String::from("Computer Literacy Buff")
        }
        (2009, 38) => {
            String::from("General Bow Debuff")
        }
        (2009, 525) => {
            String::from("Keeper Reaper Proc-Damage Inflict Segment")
        }
        (2009, 943) => {
            String::from("Dizzying Heights")
        }
        (2009, 929) => {
            String::from("Loophole")
        }
        (2009, 204) => {
            String::from("Nano Resistance Buffs")
        }
        (2009, 548) => {
            String::from("Perk Aura Of Revival-Heal Stopper")
        }
        (2009, 297) => {
            String::from("Perk Dance O fFools")
        }
        (2009, 685) => {
            String::from("Groin Kick")
        }
        (2009, 647) => {
            String::from("Mark of Vengeance")
        }
        (2009, 790) => {
            String::from("Zix Line")
        }
        (2009, 673) => {
            String::from("Fixer Dodge Buff Line")
        }
        (2009, 138) => {
            String::from("Trader Skill Transfer Caster Buff (Ransack)")
        }
        (2009, 42) => {
            String::from("General Brawl Debuff")
        }
        (2009, 679) => {
            String::from("Freak Strength Self Stun")
        }
        (2009, 825) => {
            String::from("DOT Removal")
        }
        (2009, 1057) => {
            String::from("Charge")
        }
        (2009, 640) => {
            String::from("Trader Debuff AC Nanos")
        }
        (2009, 372) => {
            String::from("Perk Bearhug")
        }
        (2009, 410) => {
            String::from("Perk Team Heal")
        }
        (2009, 84) => {
            String::from("General SMG Debuff")
        }
        (2009, 840) => {
            String::from("Dust Brigade Turrets III")
        }
        (2009, 100) => {
            String::from("General Pharmaceutical Buff")
        }
        (2009, 186) => {
            String::from("Initiative Debuffs")
        }
        (2009, 958) => {
            String::from("Mind Control")
        }
        (2009, 648) => {
            String::from("Mark of Sufferance")
        }
        (2009, 435) => {
            String::from("Perk Deadeye")
        }
        (2009, 784) => {
            String::from("Dance of Fools")
        }
        (2009, 512) => {
            String::from("Perk ECM 1")
        }
        (2009, 457) => {
            String::from("Perk Neutronium Slug")
        }
        (2009, 717) => {
            String::from("Morning")
        }
        (2009, 64) => {
            String::from("General LR Energy Weapon Buff")
        }
        (2009, 941) => {
            String::from("Blinded by Delights")
        }
        (2009, 609) => {
            String::from("MonsterEffect7")
        }
        (2009, 508) => {
            String::from("Perk Devour Essence")
        }
        (2009, 284) => {
            String::from("Other Root/Snare Resist Buff")
        }
        (2009, 34) => {
            String::from("General Strength Buff")
        }
        (2009, 170) => {
            String::from("IntelligenceDebuff")
        }
        (2009, 13) => {
            String::from("AAO Debuffs")
        }
        (2009, 329) => {
            String::from("Perk Succumb")
        }
        (2009, 668) => {
            String::from("Guardian")
        }
        (2009, 132) => {
            String::from("General Projectile AC Debuff")
        }
        (2009, 1056) => {
            String::from("Heal Reactivity Multiplier Debuff")
        }
        (2009, 18) => {
            String::from("General Aimed Shot Buff")
        }
        (2009, 4) => {
            String::from("Damage Buffs - Line A")
        }
        (2009, 345) => {
            String::from("Perk Leadership")
        }
        (2009, 342) => {
            String::from("Perk Install Notum Depletion Device")
        }
        (2009, 710) => {
            String::from("Slam of Darkness Debuff")
        }
        (2009, 722) => {
            String::from("Life")
        }
        (2009, 37) => {
            String::from("General Bow Buff")
        }
        (2009, 209) => {
            String::from("Martial Arts Buff")
        }
        (2009, 272) => {
            String::from("Trader Team Heals 15")
        }
        (2009, 1043) => {
            String::from("Omega Nuke")
        }
        (2009, 890) => {
            String::from("Trader Shutdown Skill Buff")
        }
        (2009, 1058) => {
            String::from("Martial Artist Zazen Stance")
        }
        (2009, 102) => {
            String::from("General Piercing Debuff")
        }
        (2009, 494) => {
            String::from("Perk Convulsive Tremor")
        }
        (2009, 74) => {
            String::from("General Full Auto Debuff")
        }
        (2009, 158) => {
            String::from("MatMet Debuff")
        }
        (2009, 855) => {
            String::from("Max Nano Buffs")
        }
        (2009, 65) => {
            String::from("General LR Energy Weapon Debuff")
        }
        (2009, 735) => {
            String::from("Fury")
        }
        (2009, 843) => {
            String::from("Pet Heal Delta")
        }
        (2009, 152) => {
            String::from("Initiative Buffs")
        }
        (2009, 146) => {
            String::from("Root")
        }
        (2009, 36) => {
            String::from("General BioMet Debuff")
        }
        (2009, 584) => {
            String::from("Scale Repair")
        }
        (2009, 332) => {
            String::from("Perk Elementary Teleportation 1")
        }
        (2009, 500) => {
            String::from("Perk Accelerate Decaying Quarks")
        }
        (2009, 351) => {
            String::from("Perk Vital Shock")
        }
        (2009, 246) => {
            String::from("Controlled Rage Buff")
        }
        (2009, 643) => {
            String::from("Nano Resistance Debuff (Line A)")
        }
        (2009, 823) => {
            String::from("Augmented Mirror Shield Nano")
        }
        (2009, 936) => {
            String::from("Mongo Fury")
        }
        (2009, 682) => {
            String::from("Taunt Box")
        }
        (2009, 195) => {
            String::from("Agility Buff")
        }
        (2009, 14) => {
            String::from("Nano Over Time - Line A")
        }
        (2009, 302) => {
            String::from("Perk Toxic Shock")
        }
        (2009, 402) => {
            String::from("Perk Groin Kick")
        }
        (2009, 293) => {
            String::from("Beacon Warp")
        }
        (2009, 479) => {
            String::from("Perk Incapacitate")
        }
        (2009, 714) => {
            String::from("Gather Light")
        }
        (2009, 876) => {
            String::from("Magnifying Glass Attunement BX11")
        }
        (2009, 418) => {
            String::from("Perk Access Notum Source")
        }
        (2009, 193) => {
            String::from("Concealment Buff")
        }
        (2009, 663) => {
            String::from("Stop Notum Flow")
        }
        (2009, 691) => {
            String::from("Install Notum Depletion Device DoT")
        }
        (2009, 565) => {
            String::from("MonsterWaveSpawn1")
        }
        (2009, 872) => {
            String::from("DBPF Teleport D")
        }
        (2009, 639) => {
            String::from("Deep Cuts")
        }
        (2009, 650) => {
            String::from("Mark of the Unhallowed")
        }
        (2009, 697) => {
            String::from("Gather Darkness")
        }
        (2009, 238) => {
            String::from("Demotivational Speeches")
        }
        (2009, 614) => {
            String::from("Gore Bleeding Wounds")
        }
        (2009, 666) => {
            String::from("Violence")
        }
        (2009, 715) => {
            String::from("Rain of Light")
        }
        (2009, 1042) => {
            String::from("Alpha Nuke")
        }
        (2009, 141) => {
            String::from("Trader AC Transfer Caster Buff (Siphon)")
        }
        (2009, 598) => {
            String::from("Bonfire Recharger")
        }
        (2009, 511) => {
            String::from("Perk Ritual of Blood")
        }
        (2009, 922) => {
            String::from("Borrow Reflect")
        }
        (2009, 569) => {
            String::from("MonsterWaveSpawn5")
        }
        (2009, 768) => {
            String::from("Special Attack Absorber Base")
        }
        (2009, 510) => {
            String::from("Perk Devour Vitality")
        }
        (2009, 656) => {
            String::from("Troll Form")
        }
        (2009, 269) => {
            String::from("Trader Team Heals 12")
        }
        (2009, 611) => {
            String::from("Short Term XP Gain")
        }
        (2009, 725) => {
            String::from("Tunnel of Light Buff")
        }
        (2009, 423) => {
            String::from("Perk Cripple Psyche")
        }
        (2009, 45) => {
            String::from("General Burst Debuff")
        }
        (2009, 951) => {
            String::from("Single Target Healing")
        }
        (2009, 291) => {
            String::from("Mongo HoT Component")
        }
        (2009, 783) => {
            String::from("DuringFightNanoEffect2")
        }
        (2009, 1062) => {
            String::from("Nano Damage Multiplier Buffs")
        }
        (2009, 15) => {
            String::from("XP Bonus")
        }
        (2009, 81) => {
            String::from("General Knife Buff")
        }
        (2009, 279) => {
            String::from("Enforcer Melee Energy Buff")
        }
        (2009, 424) => {
            String::from("Perk Shatter Psyche")
        }
        (2009, 748) => {
            String::from("Performed Impale")
        }
        (2009, 478) => {
            String::from("Perk Chi Conductor")
        }
        (2009, 1047) => {
            String::from("Pet Debuff Cleanse")
        }
        (2009, 265) => {
            String::from("Trader Team Heals 8")
        }
        (2009, 654) => {
            String::from("Confound with Rules")
        }
        (2009, 220) => {
            String::from("Trader Team Skill Wrangler Buff")
        }
        (2009, 541) => {
            String::from("Perk Special Thief")
        }
        (2009, 151) => {
            String::from("HP Buff")
        }
        (2009, 464) => {
            String::from("Perk Vaccinate")
        }
        (2009, 480) => {
            String::from("Perk Flesh Quiver")
        }
        (2009, 495) => {
            String::from("Perk Symbiosis")
        }
        (2009, 563) => {
            String::from("General Dimach Buff")
        }
        (2009, 85) => {
            String::from("General Martial Arts Buff")
        }
        (2009, 875) => {
            String::from("DBPF Teleport X")
        }
        (2009, 676) => {
            String::from("Channel Rage")
        }
        (2009, 229) => {
            String::from("Engineer Aura-Damage Buff")
        }
        (2009, 449) => {
            String::from("Perk Shadow Killer")
        }
        (2009, 282) => {
            String::from("Complete Healing Line")
        }
        (2009, 755) => {
            String::from("Performed Capture Spirit")
        }
        (2009, 797) => {
            String::from("Link")
        }
        (2009, 761) => {
            String::from("Transition Of Ergo")
        }
        (2009, 870) => {
            String::from("DBPF Teleport B")
        }
        (2009, 334) => {
            String::from("Perk Elementary Teleportation 3")
        }
        (2009, 318) => {
            String::from("Perk Lay On Hands")
        }
        (2009, 1000) => {
            String::from("Nemesis Nano Programs")
        }
        (2009, 775) => {
            String::from("Deceptive Stance")
        }
        (2009, 115) => {
            String::from("General Riposte Debuff")
        }
        (2009, 157) => {
            String::from("MatMet Buff")
        }
        (2009, 535) => {
            String::from("Keeper Damage Aura-Team")
        }
        (2009, 740) => {
            String::from("Performed Ritual of Zeal")
        }
        (2009, 197) => {
            String::from("Evasion Debuffs")
        }
        (2009, 319) => {
            String::from("Perk Devotional Armor")
        }
        (2009, 830) => {
            String::from("True Profession")
        }
        (2009, 367) => {
            String::from("Perk Troll Form")
        }
        (2009, 169) => {
            String::from("Psychic Debuff")
        }
        (2009, 720) => {
            String::from("Hope Buff")
        }
        (2009, 802) => {
            String::from("AE Level Spawn")
        }
        (2009, 909) => {
            String::from("Endurance Skin")
        }
        (2009, 62) => {
            String::from("General Energy Melee Debuff")
        }
        (2009, 760) => {
            String::from("Performed Unsealed Contagion")
        }
        (2009, 1054) => {
            String::from("ICC Surveillance Software")
        }
        (2009, 781) => {
            String::from("Spirit of Purity")
        }
        (2009, 937) => {
            String::from("Wit of the Atrox")
        }
        (2009, 626) => {
            String::from("Pain Lance DoT")
        }
        (2009, 484) => {
            String::from("Perk Thermal Detonation")
        }
        (2009, 583) => {
            String::from("Devotional Armor")
        }
        (2009, 821) => {
            String::from("Shovel Buffs")
        }
        (2009, 641) => {
            String::from("Leg Shot")
        }
        (2009, 680) => {
            String::from("Agent Escape Nanos")
        }
        (2009, 860) => {
            String::from("Malpractice")
        }
        (2009, 477) => {
            String::from("Perk Dragonfire")
        }
        (2009, 1032) => {
            String::from("Emergency Grid")
        }
        (2009, 1049) => {
            String::from("Emergency Sneak")
        }
        (2009, 750) => {
            String::from("Performed Lacerate")
        }
        (2009, 580) => {
            String::from("Bio Regrowth")
        }
        (2009, 696) => {
            String::from("Obscure Vision")
        }
        (2009, 353) => {
            String::from("Perk Flay")
        }
        (2009, 283) => {
            String::from("Self Root/Snare Resist Buff")
        }
        (2009, 539) => {
            String::from("Keeper Str/Stam/Agi Buff")
        }
        (2009, 610) => {
            String::from("MonsterEffect8")
        }
        (2009, 726) => {
            String::from("The Choice (Clan)")
        }
        (2009, 145) => {
            String::from("Snare")
        }
        (2009, 924) => {
            String::from("Total Control")
        }
        (2009, 163) => {
            String::from("BioMet Buff")
        }
        (2009, 1013) => {
            String::from("Pet Root, Snare and Mezz Removal")
        }
        (2009, 363) => {
            String::from("Perk Bleeding Wounds")
        }
        (2009, 405) => {
            String::from("Perk Detonate StoneWorks")
        }
        (2009, 371) => {
            String::from("Perk Grasp")
        }
        (2009, 644) => {
            String::from("Debuff NanoAC Heavy")
        }
        (2009, 442) => {
            String::from("Perk Doom Touch")
        }
        (2009, 1021) => {
            String::from("Pet AOE Snare")
        }
        (2009, 636) => {
            String::from("Agent Proc Buff")
        }
        (2009, 559) => {
            String::from("Perk Enhanced Nano Damage")
        }
        (2009, 124) => {
            String::from("General Swim Buff")
        }
        (2009, 542) => {
            String::from("Perk SPECIAL Starfall")
        }
        (2009, 743) => {
            String::from("Performed Devour Vigor")
        }
        (2009, 468) => {
            String::from("Perk Team Hale and Hearty")
        }
        (2009, 649) => {
            String::from("Mark of the Unclean")
        }
        (2009, 767) => {
            String::from("Instinctive Control")
        }
        (2009, 339) => {
            String::from("Perk Lifeblood")
        }
        (2009, 385) => {
            String::from("Perk Laser Paint Target")
        }
        (2009, 765) => {
            String::from("Regain Nano")
        }
        (2009, 393) => {
            String::from("Perk Armor Piercing Shot")
        }
        (2009, 401) => {
            String::from("Perk Hatred")
        }
        (2009, 869) => {
            String::from("DBPF Teleport A")
        }
        (2009, 878) => {
            String::from("Magnifying Glass Attunement MVCN")
        }
        (2009, 1040) => {
            String::from("Nano Delta Debuff")
        }
        (2009, 932) => {
            String::from("Flim Focus")
        }
        (2009, 75) => {
            String::from("General Thrown Grappling Buff")
        }
        (2009, 131) => {
            String::from("General Melee AC Debuff")
        }
        (2009, 805) => {
            String::from("Batter Up")
        }
        (2009, 101) => {
            String::from("General Piercing Buff")
        }
        (2009, 1053) => {
            String::from("Skill Lock Modifier Debuff")
        }
        (2009, 730) => {
            String::from("Fortress of Light")
        }
        (2009, 390) => {
            String::from("Perk Mark of Sufferance")
        }
        (2009, 403) => {
            String::from("Perk Deconstruction")
        }
        (2009, 719) => {
            String::from("Hope")
        }
        (2009, 607) => {
            String::from("MonsterEffect5")
        }
        (2009, 99) => {
            String::from("General Deflect Debuff")
        }
        (2009, 184) => {
            String::from("Doctor HP Buffs")
        }
        (2009, 189) => {
            String::from("Rage")
        }
        (2009, 608) => {
            String::from("MonsterEffect6")
        }
        (2009, 1038) => {
            String::from("AOE Taunt DOT")
        }
        (2009, 826) => {
            String::from("Trader Nano Theft 1")
        }
        (2009, 88) => {
            String::from("General MatCrea Debuff")
        }
        (2009, 355) => {
            String::from("Perk Ribbon Flesh")
        }
        (2009, 323) => {
            String::from("Perk Bring The Pain")
        }
        (2009, 417) => {
            String::from("Perk Tap Notum Source")
        }
        (2009, 509) => {
            String::from("Perk Ritual of Spirit")
        }
        (2009, 129) => {
            String::from("General Energy AC Debuff")
        }
        (2009, 285) => {
            String::from("Pet Snare/Root Resistance Buff")
        }
        (2009, 692) => {
            String::from("Install Explosive Device Countdown")
        }
        (2009, 505) => {
            String::from("Perk Ritual of Devotion")
        }
        (2009, 266) => {
            String::from("Trader Team Heals 9")
        }
        (2009, 520) => {
            String::from("Shade Damage Proc-Damage Inflict Segment")
        }
        (2009, 48) => {
            String::from("General Climb Buff")
        }
        (2009, 554) => {
            String::from("Perk Special Forces")
        }
        (2009, 199) => {
            String::from("Pistol Buff")
        }
        (2009, 658) => {
            String::from("MP Damage Debuff Line A")
        }
        (2009, 427) => {
            String::from("Perk Double Stab")
        }
        (2009, 544) => {
            String::from("Perk Special Kungfu Master")
        }
        (2009, 677) => {
            String::from("Chaotic Modulation")
        }
        (2009, 263) => {
            String::from("Trader Team Heals 6")
        }
        (2009, 857) => {
            String::from("Notum Shield")
        }
        (2009, 683) => {
            String::from("Siphon Box")
        }
        (2009, 80) => {
            String::from("General Max Health Buff")
        }
        (2009, 866) => {
            String::from("Breathing Line 2")
        }
        (2009, 472) => {
            String::from("Perk Unsealed Pestilence")
        }
        (2009, 261) => {
            String::from("Trader Team Heals 4")
        }
        (2009, 794) => {
            String::from("AIPERK Assume Target")
        }
        (2009, 481) => {
            String::from("Perk Oboliterate")
        }
        (2009, 178) => {
            String::from("NP Buff")
        }
        (2009, 399) => {
            String::from("Perk Charge")
        }
        (2009, 224) => {
            String::from("Fortify")
        }
        (2009, 612) => {
            String::from("Double Stab Bleeding Wounds")
        }
        (2009, 521) => {
            String::from("Shade Proc Buff")
        }
        (2009, 858) => {
            String::from("Nano Burst (Cyberdeck Special)")
        }
        (2009, 930) => {
            String::from("Optimize Bot Protocol")
        }
        (2009, 599) => {
            String::from("Ritual of Devotion")
        }
        (2009, 61) => {
            String::from("General Energy Melee Buff")
        }
        (2009, 357) => {
            String::from("Perk Viral Wipe")
        }
        (2009, 865) => {
            String::from("Breathing Line 1")
        }
        (2009, 271) => {
            String::from("Trader Team Heals 14")
        }
        (2009, 1060) => {
            String::from("Trader AAD Drain")
        }
        (2009, 391) => {
            String::from("Perk Mark of the Unclean")
        }
        (2009, 456) => {
            String::from("Perk Solid Slug")
        }
        (2009, 629) => {
            String::from("Enforcer Taunt Procs Irebringer")
        }
        (2009, 686) => {
            String::from("Reconstruction")
        }
        (2009, 531) => {
            String::from("Keeper NP Heal Aura-Team")
        }
        (2009, 409) => {
            String::from("Perk Malicious Prohibition")
        }
        (2009, 576) => {
            String::from("Psy/Int Buff")
        }
        (2009, 161) => {
            String::from("MatLoc Buff")
        }
        (2009, 880) => {
            String::from("Alien Dropship Shield 1 inside west")
        }
        (2009, 233) => {
            String::from("Speech Line")
        }
        (2009, 347) => {
            String::from("Perk The Director")
        }
        (2009, 526) => {
            String::from("Keeper Proc Buff")
        }
        (2009, 861) => {
            String::from("Weapon Effect Add-On 2")
        }
        (2009, 562) => {
            String::from("UNUSED 2")
        }
        (2009, 26) => {
            String::from("General 2H Edged Debuff")
        }
        (2009, 660) => {
            String::from("Nano Shakes")
        }
        (2009, 1018) => {
            String::from("Pet Sacrifice")
        }
        (2009, 738) => {
            String::from("Shadowland Bind and Recall")
        }
        (2009, 314) => {
            String::from("Perk Chaotic Modulation")
        }
        (2009, 519) => {
            String::from("Fast Attack Buffs")
        }
        (2009, 763) => {
            String::from("Insurance Claim")
        }
        (2009, 60) => {
            String::from("General Electrical Engineering Buff")
        }
        (2009, 837) => {
            String::from("Affected by OFAB Debuff")
        }
        (2009, 256) => {
            String::from("Fear")
        }
        (2009, 350) => {
            String::from("Perk Bloodletting")
        }
        (2009, 742) => {
            String::from("Performed Ritual of Blood")
        }
        (2009, 798) => {
            String::from("No Terraform")
        }
        (2009, 180) => {
            String::from("Melee Weapon Buff Line")
        }
        (2009, 150) => {
            String::from("Runspeed Buffs")
        }
        (2009, 882) => {
            String::from("Alien Dropship Shield 3 inside north")
        }
        (2009, 380) => {
            String::from("Perk Taunt Box")
        }
        (2009, 113) => {
            String::from("General Rifle Debuff")
        }
        (2009, 286) => {
            String::from("Engineer Special Attack Absorber")
        }
        (2009, 1007) => {
            String::from("Snare Removal Self")
        }
        (2009, 577) => {
            String::from("Bio Shielding")
        }
        (2009, 296) => {
            String::from("Perk Limber")
        }
        (2009, 516) => {
            String::from("Perk SPECIAL persuader")
        }
        (2009, 1031) => {
            String::from("Team Grid")
        }
        (2009, 829) => {
            String::from("Health and Nano Over Time Transfer")
        }
        (2009, 881) => {
            String::from("Alien Dropship Shield 2 inside east")
        }
        (2009, 854) => {
            String::from("Aggressive Construct Empowerment")
        }
        (2009, 863) => {
            String::from("MP Attack Pet Damage Type")
        }
        (2009, 1039) => {
            String::from("Fixer Grid")
        }
        (2009, 527) => {
            String::from("Keeper Aura-HP and NP Heal")
        }
        (2009, 242) => {
            String::from("Executioner Buff")
        }
        (2009, 602) => {
            String::from("Ritual of Blood")
        }
        (2009, 232) => {
            String::from("Pet Taunt Buff")
        }
        (2009, 793) => {
            String::from("AIPERK Insight")
        }
        (2009, 796) => {
            String::from("Leet Empower")
        }
        (2009, 155) => {
            String::from("RiposteBuff")
        }
        (2009, 471) => {
            String::from("Perk Capture Essence")
        }
        (2009, 568) => {
            String::from("MonsterWaveSpawn4")
        }
        (2009, 836) => {
            String::from("AAO Buffs")
        }
        (2009, 642) => {
            String::from("Crush Bone")
        }
        (2009, 439) => {
            String::from("Perk Power Blast")
        }
        (2009, 792) => {
            String::from("AIPERK Silent Plague")
        }
        (2009, 574) => {
            String::from("MonsterWaveSpawn10")
        }
        (2009, 83) => {
            String::from("General SMG Buff")
        }
        (2009, 105) => {
            String::from("General Poison AC Buff")
        }
        (2009, 396) => {
            String::from("Perk Tremor Hand")
        }
        (2009, 558) => {
            String::from("Perk Grid NCU")
        }
        (2009, 712) => {
            String::from("Scream of Death Debuff")
        }
        (2009, 120) => {
            String::from("General Sneak Attack Buff")
        }
        (2009, 140) => {
            String::from("Trader AC Transfer Target Debuff (Draw)")
        }
        (2009, 236) => {
            String::from("Engineer Debuff Auras")
        }
        (2009, 624) => {
            String::from("Perk Overwhelming Might")
        }
        (2009, 317) => {
            String::from("Perk Death Strike")
        }
        (2009, 657) => {
            String::from("Disable Natural Healing")
        }
        (2009, 762) => {
            String::from("Insurance Agent")
        }
        (2009, 1029) => {
            String::from("Boss Buffs")
        }
        (2009, 32) => {
            String::from("General Sense Buff")
        }
        (2009, 1) => {
            String::from("Damage Shields")
        }
        (2009, 368) => {
            String::from("Perk Disable Natural Healing")
        }
        (2009, 176) => {
            String::from("Weapon Smithing Buff")
        }
        (2009, 374) => {
            String::from("Perk Removal 1")
        }
        (2009, 267) => {
            String::from("Trader Team Heals 10")
        }
        (2009, 910) => {
            String::from("PvP Enabled")
        }
        (2009, 147) => {
            String::from("Mezz")
        }
        (2009, 669) => {
            String::from("Total Mirror Shield")
        }
        (2009, 384) => {
            String::from("Perk NCU Booster")
        }
        (2009, 397) => {
            String::from("Perk Harmonize Body and Mind")
        }
        (2009, 1004) => {
            String::from("AOE Mezz")
        }
        (2009, 369) => {
            String::from("Perk Stonefist")
        }
        (2009, 753) => {
            String::from("Performed Capture Vigor")
        }
        (2009, 543) => {
            String::from("Perk Special Shadowsneak")
        }
        (2009, 959) => {
            String::from("Experience Constructs - XP Bonus")
        }
        (2009, 73) => {
            String::from("General Full Auto Buff")
        }
        (2009, 1022) => {
            String::from("Charm (Short)")
        }
        (2009, 76) => {
            String::from("General Thrown Grappling Debuff")
        }
        (2009, 106) => {
            String::from("General Projectile AC Buff")
        }
        (2009, 362) => {
            String::from("Perk Devour")
        }
        (2009, 28) => {
            String::from("General Assault Rifle Debuff")
        }
        (2009, 1002) => {
            String::from("AAD Buffs")
        }
        (2009, 119) => {
            String::from("General Shotgun Debuff")
        }
        (2009, 97) => {
            String::from("General NP Regeneration")
        }
        (2009, 104) => {
            String::from("General Pisto Debuff")
        }
        (2009, 162) => {
            String::from("MatLoc Debuff")
        }
        (2009, 164) => {
            String::from("BioMet Debuff")
        }
        (2009, 211) => {
            String::from("NP Cost Debuff")
        }
        (2009, 575) => {
            String::from("Battlegroup Heal")
        }
        (2009, 413) => {
            String::from("Perk Nano Shakes")
        }
        (2009, 579) => {
            String::from("Bio Rejuvenation")
        }
        (2009, 316) => {
            String::from("Perk Pinpoint Strike")
        }
        (2009, 681) => {
            String::from("Reconstruction")
        }
        (2009, 916) => {
            String::from("Prototype Nanoformula")
        }
        (2009, 578) => {
            String::from("Bio Cocoon")
        }
        (2009, 506) => {
            String::from("Perk Devour Vigor")
        }
        (2009, 795) => {
            String::from("Daring")
        }
        (2009, 54) => {
            String::from("General Intelligence Debuff")
        }
        (2009, 93) => {
            String::from("General Mechanical Engineering Buff")
        }
        (2009, 412) => {
            String::from("Perk Zap Nano")
        }
        (2009, 24) => {
            String::from("General 2H Blunt Debuff")
        }
        (2009, 213) => {
            String::from("Ranged Energy Weapon Buffs")
        }
        (2009, 900) => {
            String::from("Keeper Fear Immunity")
        }
        (2009, 454) => {
            String::from("Perk Reinforce Slugs")
        }
        (2009, 44) => {
            String::from("General Burst Buff")
        }
        (2009, 874) => {
            String::from("DBPF Teleport F")
        }
        (2009, 789) => {
            String::from("MINI DoT")
        }
        (2009, 20) => {
            String::from("General Air Transport Buff")
        }
        (2009, 557) => {
            String::from("Perk Heavy Ranged")
        }
        (2009, 1050) => {
            String::from("Heal Delta Debuff")
        }
        (2009, 392) => {
            String::from("Perk Mark of the Unhallowed")
        }
        (2009, 662) => {
            String::from("Blast Nano")
        }
        (2009, 387) => {
            String::from("Perk Triangulate Target")
        }
        (2009, 839) => {
            String::from("Dust Brigade Turrets II")
        }
        (2009, 1014) => {
            String::from("Pet Healing")
        }
        (2009, 628) => {
            String::from("Enforcer Taunt Procs Fearbringer")
        }
        (2009, 7) => {
            String::from("DOT - Line B")
        }
        (2009, 708) => {
            String::from("Blackfist")
        }
        (2009, 461) => {
            String::from("Perk Violence")
        }
        (2009, 859) => {
            String::from("Martial Artist HOT Line A")
        }
        (2009, 69) => {
            String::from("General Fire AC Buff")
        }
        (2009, 264) => {
            String::from("Trader Team Heals 7")
        }
        (2009, 811) => {
            String::from("Engineer Miniaturization")
        }
        (2009, 127) => {
            String::from("General Chemical AC Debuff")
        }
        (2009, 41) => {
            String::from("General Brawl Buff")
        }
        (2009, 218) => {
            String::from("False Profession")
        }
        (2009, 299) => {
            String::from("Perk Poison Sprinkle")
        }
        (2009, 770) => {
            String::from("Soldier Damage Base")
        }
        (2009, 1046) => {
            String::from("Food and Drink Buffs")
        }
        (2009, 515) => {
            String::from("Perk SPECIAL bureaucratic shuffle")
        }
        (2009, 331) => {
            String::from("Perk Evasive Stance")
        }
        (2009, 1024) => {
            String::from("Damage To Pet")
        }
        (2009, 490) => {
            String::from("Perk Exultation")
        }
        (2009, 30) => {
            String::from("General Intelligence Buff")
        }
        (2009, 426) => {
            String::from("Perk Stab")
        }
        (2009, 1011) => {
            String::from("Root Removal Other")
        }
        (2009, 549) => {
            String::from("Perk Commanding Presence")
        }
        (2009, 59) => {
            String::from("General Disarm Traps Buff")
        }
        (2009, 335) => {
            String::from("Perk Elementary Teleportation 4")
        }
        (2009, 815) => {
            String::from("Martial Artist Bow Buffs")
        }
        (2009, 326) => {
            String::from("Perk Followup Smash")
        }
        (2009, 278) => {
            String::from("Enforcer Piercing Buff")
        }
        (2009, 304) => {
            String::from("Perk Assasinate")
        }
        (2009, 121) => {
            String::from("General Sneak Attack Debuff")
        }
        (2009, 432) => {
            String::from("Perk Hecatomb")
        }
        (2009, 674) => {
            String::from("Hammer and Anvil")
        }
        (2009, 381) => {
            String::from("Perk Siphon Life")
        }
        (2009, 254) => {
            String::from("Chest Buff Line")
        }
        (2009, 378) => {
            String::from("Perk Great Purge")
        }
        (2009, 154) => {
            String::from("Brawl Buff")
        }
        (2009, 718) => {
            String::from("Morning Debuff")
        }
        (2009, 1003) => {
            String::from("Stun")
        }
        (2009, 201) => {
            String::from("Nano Delta Buffs")
        }
        (2009, 91) => {
            String::from("General MatMet Buff")
        }
        (2009, 809) => {
            String::from("Damage to Nano")
        }
        (2009, 749) => {
            String::from("Performed Double Stab")
        }
        (2009, 529) => {
            String::from("Keeper Aura-Damage/Snare Reduction Buff")
        }
        (2009, 533) => {
            String::from("Keeper AMS/DMS Aura-Team")
        }
        (2009, 944) => {
            String::from("Sprained Ankle")
        }
        (2009, 502) => {
            String::from("Perk Escape")
        }
        (2009, 63) => {
            String::from("General Energy AC Buff")
        }
        (2009, 167) => {
            String::from("PsyMod Buff")
        }
        (2009, 467) => {
            String::from("Perk Hale and Hearty")
        }
        (2009, 835) => {
            String::from("Nano Resist Buff ")
        }
        (2009, 212) => {
            String::from("Assault Rifle Buffs")
        }
        (2009, 116) => {
            String::from("General SenseImp Buff")
        }
        (2009, 458) => {
            String::from("Perk Field Bandage")
        }
        (2009, 586) => {
            String::from("Lick Wounds NA")
        }
        (2009, 179) => {
            String::from("1H Blunt Buff")
        }
        (2009, 540) => {
            String::from("Perk SPECIAL Tinkerer")
        }
        (2009, 237) => {
            String::from("Motivational Speech Nano Resist Buff")
        }
        (2009, 732) => {
            String::from("Misery Buff")
        }
        (2009, 10) => {
            String::from("DOT Nanotechnician Strain B")
        }
        (2009, 488) => {
            String::from("Perk Honoring The Ancients")
        }
        (2009, 431) => {
            String::from("Perk Gore")
        }
        (2009, 536) => {
            String::from("Keeper Snare Reduction Aura-Team")
        }
        (2009, 778) => {
            String::from("Affected by Spirit of Blessing")
        }
        (2009, 678) => {
            String::from("Freak Strength Stun")
        }
        (2009, 591) => {
            String::from("Draw Blood")
        }
        (2009, 791) => {
            String::from("AI AMSmodifier proc")
        }
        (2009, 5) => {
            String::from("Challenger")
        }
        (2009, 306) => {
            String::from("Perk BattlegroupHeal 2")
        }
        (2009, 315) => {
            String::from("Perk Soften Up")
        }
        (2009, 566) => {
            String::from("MonsterWaveSpawn2")
        }
        (2009, 109) => {
            String::from("General PsyMod Debuff")
        }
        (2009, 800) => {
            String::from("Cocoon")
        }
        (2009, 777) => {
            String::from("Short Term HP Buff")
        }
        (2009, 497) => {
            String::from("Perk Malevolent Symbiosis")
        }
        (2009, 320) => {
            String::from("Perk Curing Touch")
        }
        (2009, 428) => {
            String::from("Perk Perforate")
        }
        (2009, 756) => {
            String::from("Performed Capture Vitality")
        }
        (2009, 764) => {
            String::from("Affected by Insurance Claim")
        }
        (2009, 1028) => {
            String::from("Special Effect Nukes")
        }
        (2009, 110) => {
            String::from("General Radiation AC Buff")
        }
        (2009, 581) => {
            String::from("General Ranged Multiple Buff")
        }
        (2009, 596) => {
            String::from("Health Recharge")
        }
        (2009, 625) => {
            String::from("Perk Seismic Smash")
        }
        (2009, 925) => {
            String::from("Troll Form Run Debuff")
        }
        (2009, 366) => {
            String::from("Perk Invocation")
        }
        (2009, 634) => {
            String::from("Accelerate Decaying Quarks Debuff")
        }
        (2009, 330) => {
            String::from("Perk Confound With Rules")
        }
        (2009, 67) => {
            String::from("General Fast Attack Debuff")
        }
        (2009, 210) => {
            String::from("Nano Programming Buff")
        }
        (2009, 469) => {
            String::from("Perk Capture Vigor")
        }
        (2009, 689) => {
            String::from("Deconstruction")
        }
        (2009, 935) => {
            String::from("Poison Sprinkle")
        }
        (2009, 130) => {
            String::from("General Fire AC Debuff")
        }
        (2009, 231) => {
            String::from("Engineer Aura-Reflection Damage Buff")
        }
        (2009, 572) => {
            String::from("MonsterWaveSpawn8")
        }
        (2009, 33) => {
            String::from("General Stamina Buff")
        }
        (2009, 507) => {
            String::from("Perk Ritual of Zeal")
        }
        (2009, 354) => {
            String::from("Perk Flurry of Cuts")
        }
        (2009, 114) => {
            String::from("General Riposte Buff")
        }
        (2009, 280) => {
            String::from("Soldier Shotgun Buff")
        }
        (2009, 745) => {
            String::from("Performed Devour Vitality")
        }
        (2009, 292) => {
            String::from("Unhallowed Force Line")
        }
        (2009, 245) => {
            String::from("Multiwield Buff")
        }
        (2009, 400) => {
            String::from("Perk Headbutt")
        }
        (2009, 655) => {
            String::from("Succumb")
        }
        (2009, 613) => {
            String::from("Lacerate Bleeding Wounds")
        }
        (2009, 249) => {
            String::from("Pack Hunter Base")
        }
        (2009, 71) => {
            String::from("General Fling Shot Buff")
        }
        (2009, 1030) => {
            String::from("Self Grid")
        }
        (2009, 945) => {
            String::from("Feel")
        }
        (2009, 8) => {
            String::from("DOT Nanotechnician Strain A")
        }
        (2009, 72) => {
            String::from("General Fling Shot Debuff")
        }
        (2009, 190) => {
            String::from("First Aid And Treatment Buff")
        }
        (2009, 466) => {
            String::from("Perk Vaccinate 2")
        }
        (2009, 1059) => {
            String::from("Martial Artist HOT - Line B")
        }
        (2009, 635) => {
            String::from("Agent Damage Proc-DamageInflictSegment")
        }
        (2009, 550) => {
            String::from("Perk Directorship Buff")
        }
        (2009, 731) => {
            String::from("Fortress of Light Buff")
        }
        (2009, 21) => {
            String::from("General 1H Edged Buff")
        }
        (2009, 139) => {
            String::from("Trader AC Transfer Target Debuff (Siphon)")
        }
        (2009, 227) => {
            String::from("Engineer Auras")
        }
        (2009, 289) => {
            String::from("TemporalChaliceVisualEffectBuff")
        }
        (2009, 773) => {
            String::from("Agent Detaunt Proc-Detaunt Segment")
        }
        (2009, 620) => {
            String::from("Perk Pain Lance")
        }
        (2009, 23) => {
            String::from("General 2H Blunt Buff")
        }
        (2009, 736) => {
            String::from("Reinforced Slugs")
        }
        (2009, 868) => {
            String::from("Evasion Debuffs (Agent)")
        }
        (2009, 419) => {
            String::from("Perk Blast Nano")
        }
        (2009, 585) => {
            String::from("Slobber Wounds")
        }
        (2009, 255) => {
            String::from("Fixer Long HoT")
        }
        (2009, 420) => {
            String::from("Perk Stop Notum Flow")
        }
        (2009, 295) => {
            String::from("HellGun Dispel Proc")
        }
        (2009, 407) => {
            String::from("Perk Shutdown Removal 2")
        }
        (2009, 459) => {
            String::from("Perk Tracer")
        }
        (2009, 827) => {
            String::from("Trader Nano Theft 2")
        }
        (2009, 780) => {
            String::from("Spirit of Blessing")
        }
        (2009, 194) => {
            String::from("Rifle Buffs")
        }
        (2009, 411) => {
            String::from("Perk Treatment Transfer")
        }
        (2009, 831) => {
            String::from("Shield of the Obedient Servant")
        }
        (2009, 166) => {
            String::from("SenseImp Debuff")
        }
        (2009, 885) => {
            String::from("Fear - Cooldown")
        }
        (2009, 192) => {
            String::from("Sense Buff")
        }
        (2009, 616) => {
            String::from("MonsterEffect_Breakable")
        }
        (2009, 589) => {
            String::from("Blessing of Life")
        }
        (2009, 661) => {
            String::from("Tap Notum Source")
        }
        (2009, 274) => {
            String::from("Trader Team Heals 17")
        }
        (2009, 747) => {
            String::from("Performed Perforate")
        }
        (2009, 776) => {
            String::from("Affected by Consume the Soul")
        }
        (2009, 16) => {
            String::from("General 1Hand Blunt Buff")
        }
        (2009, 547) => {
            String::from("Dimach Buff")
        }
        (2009, 174) => {
            String::from("Mechanical Engineering Buff")
        }
        (2009, 475) => {
            String::from("Perk Capture Vitality")
        }
        (2009, 564) => {
            String::from("General Melee Multiple Buff")
        }
        (2009, 570) => {
            String::from("MonsterWaveSpawn6")
        }
        (2009, 524) => {
            String::from("Keeper Sanctifier Proc-Damage Inflict Segment")
        }
        (2009, 553) => {
            String::from("Perk Street Samurai")
        }
        (2009, 383) => {
            String::from("Perk Regain Nano")
        }
        (2009, 277) => {
            String::from("Drone Tower Buff")
        }
        (2009, 234) => {
            String::from("Motivational SpeechEffect")
        }
        (2009, 111) => {
            String::from("General HP Regeneration")
        }
        (2009, 203) => {
            String::from("Heal Delta Buff")
        }
        (2009, 70) => {
            String::from("General First Aid Buff")
        }
        (2009, 901) => {
            String::from("Fixer Fear Immunity")
        }
        (2009, 713) => {
            String::from("Lightstep")
        }
        (2009, 308) => {
            String::from("Perk BattlegroupHeal 3")
        }
        (2009, 810) => {
            String::from("Mesmerization Construct Empowerment")
        }
        (2009, 51) => {
            String::from("General Concealment Buff")
        }
        (2009, 214) => {
            String::from("Burst Buff")
        }
        (2009, 301) => {
            String::from("Perk Tranquilizer")
        }
        (2009, 307) => {
            String::from("Perk Viral Combination")
        }
        (2009, 31) => {
            String::from("General Psychic Buff")
        }
        (2009, 311) => {
            String::from("Perk Bio Cocoon")
        }
        (2009, 1055) => {
            String::from("Heal Reactivity Multiplier Buff")
        }
        (2009, 235) => {
            String::from("Disarm Trap Buff")
        }
        (2009, 582) => {
            String::from("DOT Strain C")
        }
        (2009, 470) => {
            String::from("Perk Unhealed Blight")
        }
        (2009, 659) => {
            String::from("MP Damage Debuff Line B")
        }
        (2009, 1009) => {
            String::from("Snare Removal Team")
        }
        (2009, 804) => {
            String::from("Privacy Shield")
        }
        (2009, 1052) => {
            String::from("Critical Decrease Buff")
        }
        (2009, 769) => {
            String::from("Total Focus")
        }
        (2009, 309) => {
            String::from("Perk BattlegroupHeal 4")
        }
        (2009, 617) => {
            String::from("MonsterEffect_DuringFight")
        }
        (2009, 406) => {
            String::from("Perk Shutdown Removal 1")
        }
        (2009, 404) => {
            String::from("Perk Encase in Stone")
        }
        (2009, 567) => {
            String::from("MonsterWaveSpawn3")
        }
        (2009, 723) => {
            String::from("Path of Light")
        }
        (2009, 820) => {
            String::from("AMS")
        }
        (2009, 365) => {
            String::from("Perk Heal")
        }
        (2009, 587) => {
            String::from("SL Nanopoint Drain")
        }
        (2009, 757) => {
            String::from("Affected by Taint Wounds")
        }
        (2009, 183) => {
            String::from("Interrupt Modifier")
        }
        (2009, 928) => {
            String::from("Focus")
        }
        (2009, 664) => {
            String::from("Notum Overflow")
        }
        (2009, 877) => {
            String::from("Magnifying Glass Attunement WQEL")
        }
        (2009, 327) => {
            String::from("Perk Blindside Blow")
        }
        (2009, 451) => {
            String::from("Perk Snipe Shot 2")
        }
        (2009, 375) => {
            String::from("Perk Removal 2")
        }
        (2009, 646) => {
            String::from("Energize")
        }
        (2009, 223) => {
            String::from("Polymorph")
        }
        (2009, 216) => {
            String::from("MP Pet Damage Buffs")
        }
        (2009, 198) => {
            String::from("Aimed Shot Buffs")
        }
        (2009, 386) => {
            String::from("Perk Weapon Bash")
        }
        (2009, 175) => {
            String::from("Pharmaceuticals Buff")
        }
        (2009, 716) => {
            String::from("Rain of Light Buff")
        }
        (2009, 946) => {
            String::from("Propaganda")
        }
        (2009, 493) => {
            String::from("Perk Disorient")
        }
        (2009, 737) => {
            String::from("Affected by Nano Heal")
        }
        (2009, 243) => {
            String::from("Damage Shield Upgrades")
        }
        (2009, 867) => {
            String::from("Breathing Line 3")
        }
        (2009, 852) => {
            String::from("Reanimated Cloak Blocker")
        }
        (2009, 771) => {
            String::from("Affected By Defensive Stance")
        }
        (2009, 463) => {
            String::from("Perk Cure")
        }
        (2009, 462) => {
            String::from("Perk Guardian")
        }
        (2009, 592) => {
            String::from("Heavy Weapons Buffs")
        }
        (2009, 416) => {
            String::from("Perk Fade Anger")
        }
        (2009, 601) => {
            String::from("Ritual of Spirit")
        }
        (2009, 322) => {
            String::from("Perk Crush Bone")
        }
        (2009, 465) => {
            String::from("Perk Cure 2")
        }
        (2009, 436) => {
            String::from("Perk Energize")
        }
        (2009, 187) => {
            String::from("MetaPhysicist Damage Debuff")
        }
        (2009, 675) => {
            String::from("Zap Nano")
        }
        (2009, 376) => {
            String::from("Perk Purge 1")
        }
        (2009, 879) => {
            String::from("Magnifying Glass Attunement ZLQ6")
        }
        (2009, 3) => {
            String::from("Armor Buff")
        }
        (2009, 518) => {
            String::from("Keeper Deflect/Riposte Buff")
        }
        (2009, 43) => {
            String::from("General Break Entry Buff")
        }
        (2009, 57) => {
            String::from("General Stamina Debuff")
        }
        (2009, 670) => {
            String::from("Dazzle with Lights")
        }
        (2009, 741) => {
            String::from("Performed Ritual of Spirit")
        }
        (2009, 50) => {
            String::from("General Computer Literacy Buff")
        }
        (2009, 259) => {
            String::from("Trader Team Heals 2")
        }
        (2009, 621) => {
            String::from("Perk Slice And Dice")
        }
        (2009, 133) => {
            String::from("General Radiation AC Debuff")
        }
        (2009, 887) => {
            String::from("Unremovable Snare")
        }
        (2009, 191) => {
            String::from("Perception Buffs")
        }
        (2009, 841) => {
            String::from("Adventurer Damage Modifier")
        }
        (2009, 287) => {
            String::from("Ransack/Deprive Resist Buff")
        }
        (2009, 208) => {
            String::from("Sneak Attack Buffs")
        }
        (2009, 373) => {
            String::from("Perk Grip of Colossus")
        }
        (2009, 160) => {
            String::from("MatCrea Debuff")
        }
        (2009, 217) => {
            String::from("MP Pet Initiative Buffs")
        }
        (2009, 206) => {
            String::from("Breaking & Entry/Disarm Traps Buff")
        }
        (2009, 939) => {
            String::from("Notum Domination")
        }
        (2009, 947) => {
            String::from("Treatment Transfer")
        }
        (2009, 605) => {
            String::from("MonsterEffect3")
        }
        (2009, 850) => {
            String::from("Pet Heal Delta")
        }
        (2009, 321) => {
            String::from("Perk Quick Bash")
        }
        (2009, 687) => {
            String::from("Taunt Box")
        }
        (2009, 348) => {
            String::from("Perk Balance Of Yin and Yang")
        }
        (2009, 1020) => {
            String::from("Pet Proc (Line B)")
        }
        (2009, 496) => {
            String::from("Perk Malicious Symbiosis")
        }
        (2009, 171) => {
            String::from("Break & Entry Buffs")
        }
        (2009, 832) => {
            String::from("NT Area Nukes 2")
        }
        (2009, 40) => {
            String::from("General Bow Special Debuff")
        }
        (2009, 118) => {
            String::from("General Shotgun Buff")
        }
        (2009, 27) => {
            String::from("General Assault Rifle Buff")
        }
        (2009, 268) => {
            String::from("Trader Team Heals 11")
        }
        (2009, 341) => {
            String::from("Perk Install Explosive Devices")
        }
        (2009, 695) => {
            String::from("Blackstep")
        }
        (2009, 108) => {
            String::from("General PsyMod Buff")
        }
        (2009, 956) => {
            String::from("Kyr'Ozch Gene Pool")
        }
        (2009, 597) => {
            String::from("Damage Change Buffs")
        }
        (2009, 441) => {
            String::from("Perk Atrophy")
        }
        (2009, 787) => {
            String::from("AIPERK Blur")
        }
        (2009, 1037) => {
            String::from("Taunt")
        }
        (2009, 252) => {
            String::from("Damage Buff - Line C")
        }
        (2009, 300) => {
            String::from("Perk Seal Wounds")
        }
        (2009, 377) => {
            String::from("Perk Purge 2")
        }
        (2009, 485) => {
            String::from("Perk Supernova")
        }
        (2009, 337) => {
            String::from("Perk Channel Rage")
        }
        (2009, 226) => {
            String::from("Elian Soul")
        }
        (2009, 1045) => {
            String::from("Resurrection Sickness Removal")
        }
        (2009, 487) => {
            String::from("Perk Blade Whirlwind")
        }
        (2009, 1008) => {
            String::from("Snare Removal Other")
        }
        (2009, 806) => {
            String::from("Armor Damage")
        }
        (2009, 79) => {
            String::from("General Ground Transport Buff")
        }
        (2009, 89) => {
            String::from("General MatLoc Buff")
        }
        (2009, 555) => {
            String::from("Perk SMG Mastery")
        }
        (2009, 705) => {
            String::from("Road To Darkness Debuff")
        }
        (2009, 700) => {
            String::from("Misery")
        }
        (2009, 733) => {
            String::from("Misery Debuff")
        }
        (2009, 517) => {
            String::from("Perk SPECIAL alchemist")
        }
        (2009, 530) => {
            String::from("Keeper Heal Aura-Team")
        }
        (2009, 455) => {
            String::from("Perk Jarring Burst")
        }
        (2009, 701) => {
            String::from("Death")
        }
        (2009, 389) => {
            String::from("Perk Mark of Vengeance")
        }
        (2009, 446) => {
            String::from("Perk Night Killer")
        }
        (2009, 112) => {
            String::from("General Rifle Buff")
        }
        (2009, 633) => {
            String::from("Enforcer Taunt Procs Dreadbringer")
        }
        (2009, 667) => {
            String::from("Violence Controller")
        }
        (2009, 824) => {
            String::from("Nullity Sphere Nano")
        }
        (2009, 6) => {
            String::from("DOT - Line A")
        }
        (2009, 546) => {
            String::from("Shade Piercing Buff")
        }
        (2009, 851) => {
            String::from("Reanimated Cloak Buffs")
        }
        (2009, 884) => {
            String::from("Knockback")
        }
        (2009, 344) => {
            String::from("Perk Thermal Primer")
        }
        (2009, 153) => {
            String::from("2HEdged Buff")
        }
        (2009, 604) => {
            String::from("MonsterEffect2")
        }
        (2009, 202) => {
            String::from("Charm Other")
        }
        (2009, 751) => {
            String::from("Performed Gore")
        }
        (2009, 445) => {
            String::from("Perk Shadow Bullet")
        }
        (2009, 450) => {
            String::from("Perk Snipe Shot 1")
        }
        (2009, 1033) => {
            String::from("Shadowlands Maps")
        }
        (2009, 814) => {
            String::from("Trader AAO Drain")
        }
        (2009, 437) => {
            String::from("Perk Power Volley")
        }
        (2009, 188) => {
            String::from("Mongo Buff")
        }
        (2009, 1026) => {
            String::from("Alpha Nukes")
        }
        (2009, 336) => {
            String::from("Perk ICC Node Teleportation")
        }
        (2009, 889) => {
            String::from("Trader Shutdown Skill Debuff")
        }
        (2009, 29) => {
            String::from("General Agility Buff")
        }
        (2009, 772) => {
            String::from("Defensive Stance")
        }
        (2009, 788) => {
            String::from("AIPERK Sacrifice")
        }
        (2009, 886) => {
            String::from("Reverse Knockback")
        }
        (2009, 1006) => {
            String::from("AOE Root")
        }
        (2009, 746) => {
            String::from("Performed Stab")
        }
        _ => panic!("Unknown MMDB entry"),
    }
}
