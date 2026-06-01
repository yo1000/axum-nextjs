SET search_path = eight;

-- userProfile

INSERT INTO users(
    "id", "username", "family_name", "given_name", "age", "gender", "profile"
) VALUES (
    '00000000-0000-7000-b000-100000000000',
    'admin',
    '',
    'Admin',
    0,
    1,
    ''
);

INSERT INTO users(
    "id", "username", "family_name", "given_name", "age", "gender", "profile"
) VALUES (
    '00000001-0000-7000-b000-100000000000',
    'squall',
    'Leonhart',
    'Squall',
    17,
    1,
    'Uses a sword called a gunblade. Special skill is Renzokuken, using the gunblade. Silent, and a bit cold.'
);

INSERT INTO users("id", "username", "family_name", "given_name", "age", "gender", "profile")
VALUES
    (
        '00000002-0000-7000-b000-100000000000',
        'zell',
        'Dincht',
        'Zell',
        17,
        1,
        'Loves the hot dogs sold in the Garden cafeteria. Uses close combat fighting skills to defeat enemies with punches and kicks.'
    );

INSERT INTO users("id", "username", "family_name", "given_name", "age", "gender", "profile")
VALUES
    (
        '00000003-0000-7000-b000-100000000000',
        'irvine',
        'Kinneas',
        'Irvine',
        17,
        1,
        'An expert marksman. Can use specialized bullets to attack enemies. Doesn''t perform very well under pressure.'
    );

INSERT INTO users("id", "username", "family_name", "given_name", "age", "gender", "profile")
VALUES
    (
        '00000004-0000-7000-b000-100000000000',
        'quistis',
        'Trepe',
        'Quistis',
        18,
        2,
        'Uses a whip. When in danger, uses monster skills she has learned. Her admirers in the Garden have formed a fan club.'
    );

INSERT INTO users("id", "username", "family_name", "given_name", "age", "gender", "profile")
VALUES
    (
        '00000005-0000-7000-b000-100000000000',
        'rinoa',
        'Heartilly',
        'Rinoa',
        17,
        2,
        'A member of the Timber resistance group Forest Owls. Uses her dog to make special attacks.'
    );

INSERT INTO users("id", "username", "family_name", "given_name", "age", "gender", "profile")
VALUES
    (
        '00000006-0000-7000-b000-100000000000',
        'selphie',
        'Tilmitt',
        'Selphie',
        17,
        2,
        'A transfer student to Balamb Garden. Uses giant nunchaku and special magic.'
    );

INSERT INTO users("id", "username", "family_name", "given_name", "age", "gender", "profile")
VALUES
    (
        '00000007-0000-7000-b000-100000000000',
        'seifer',
        'Almasy',
        'Seifer',
        18,
        1,
        'One of the top fighters of the Garden. His temperament kept him from becoming a SeeD member. Uses a gunblade.'
    );

INSERT INTO users("id", "username", "family_name", "given_name", "age", "gender", "profile")
VALUES
    (
        '00000008-0000-7000-b000-100000000000',
        'edea',
        'Kramer',
        'Edea',
        null,
        2,
        'A sorceress. As a sorceress, she is well-versed in magic. Uses a special magic attack called Sorcery.'
    );

INSERT INTO users("id", "username", "family_name", "given_name", "age", "gender", "profile")
VALUES
    (
        '00000009-0000-7000-b000-100000000000',
        'laguna',
        'Loire',
        'Laguna',
        44,
        1,
        'An energetic man with a big heart. Uses a machine gun. Fires bursts at enemies to help party members in danger.'
    );

INSERT INTO users("id", "username", "family_name", "given_name", "age", "gender", "profile")
VALUES
    (
        '00000010-0000-7000-b000-100000000000',
        'kiros',
        'Seagill',
        'Kiros',
        40,
        1,
        'Wears sharp blade called Katal on both arms. His fighting style is also unique, like his weapon.'
    );

INSERT INTO users("id", "username", "family_name", "given_name", "age", "gender", "profile")
VALUES
    (
        '00000011-0000-7000-b000-100000000000',
        'ward',
        'Zabac',
        'Ward',
        42,
        1,
        'A big, powerful man. Uses the Harpoon, an unusually heavy weapon, and attacks by throwing it at the enemy.'
    );


-- item

INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000000-0000-7000-b000-000000000000', 0, '', null, null);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000001-0000-7000-b000-000000000000', 1, 'Potion', 100, 50);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000002-0000-7000-b000-000000000000', 2, 'Potion+', null, 100);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000003-0000-7000-b000-000000000000', 3, 'Hi-Potion', 500, 250);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000004-0000-7000-b000-000000000000', 4, 'Hi-Potion+', null, 500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000005-0000-7000-b000-000000000000', 5, 'X-Potion', 5000, 2500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000006-0000-7000-b000-000000000000', 6, 'Mega-Potion', 10000, 5000);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000007-0000-7000-b000-000000000000', 7, 'Phoenix Down', 500, 250);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000008-0000-7000-b000-000000000000', 8, 'Mega Phoenix', 10000, 5000);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000009-0000-7000-b000-000000000000', 9, 'Elixir', 50000, 2500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000010-0000-7000-b000-000000000000', 10, 'Megalixir', null, 5000);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000011-0000-7000-b000-000000000000', 11, 'Antidote', 100, 50);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000012-0000-7000-b000-000000000000', 12, 'Soft', 100, 50);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000013-0000-7000-b000-000000000000', 13, 'Eye Drops', 100, 50);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000014-0000-7000-b000-000000000000', 14, 'Echo Screen', 100, 50);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000015-0000-7000-b000-000000000000', 15, 'Holy Water', 100, 50);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000016-0000-7000-b000-000000000000', 16, 'Remedy', 1000, 500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000017-0000-7000-b000-000000000000', 17, 'Remedy+', null, 1000);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000018-0000-7000-b000-000000000000', 18, 'Hero-trial', null, 5);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000019-0000-7000-b000-000000000000', 19, 'Hero', null, 5000);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000020-0000-7000-b000-000000000000', 20, 'Holy War-trial', null, 5);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000021-0000-7000-b000-000000000000', 21, 'Holy War', null, 10000);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000022-0000-7000-b000-000000000000', 22, 'Shell Stone', null, 5);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000023-0000-7000-b000-000000000000', 23, 'Protect Stone', null, 5);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000024-0000-7000-b000-000000000000', 24, 'Aura Stone', null, 5);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000025-0000-7000-b000-000000000000', 25, 'Death Stone', null, 5);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000026-0000-7000-b000-000000000000', 26, 'Holy Stone', null, 5);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000027-0000-7000-b000-000000000000', 27, 'Flare Stone', null, 5);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000028-0000-7000-b000-000000000000', 28, 'Meteor Stone', null, 5);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000029-0000-7000-b000-000000000000', 29, 'Ultima Stone', null, 5);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000030-0000-7000-b000-000000000000', 30, 'Gysahl Greens', null, 5);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000031-0000-7000-b000-000000000000', 31, 'Phoenix Pinion', null, 5);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000032-0000-7000-b000-000000000000', 32, 'Friendship', null, 5);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000033-0000-7000-b000-000000000000', 33, 'Tent', 1000, 250);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000034-0000-7000-b000-000000000000', 34, 'Pet House', 1000, 250);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000035-0000-7000-b000-000000000000', 35, 'Cottage', 1800, 450);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000036-0000-7000-b000-000000000000', 36, 'G-Potion', 200, 50);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000037-0000-7000-b000-000000000000', 37, 'G-Hi-Potion', 600, 150);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000038-0000-7000-b000-000000000000', 38, 'G-Mega-Potion', null, 250);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000039-0000-7000-b000-000000000000', 39, 'G-Returner', 500, 125);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000040-0000-7000-b000-000000000000', 40, 'Rename Card', null, 25);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000041-0000-7000-b000-000000000000', 41, 'Amnesia Greens', 1000, 250);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000042-0000-7000-b000-000000000000', 42, 'HP-J Scroll', 10000, 2500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000043-0000-7000-b000-000000000000', 43, 'Str-J Scroll', 10000, 2500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000044-0000-7000-b000-000000000000', 44, 'Vit-J Scroll', 10000, 2500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000045-0000-7000-b000-000000000000', 45, 'Mag-J Scroll', 10000, 2500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000046-0000-7000-b000-000000000000', 46, 'Spr-J Scroll', 10000, 2500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000047-0000-7000-b000-000000000000', 47, 'Spd-J Scroll', null, 12500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000048-0000-7000-b000-000000000000', 48, 'Luck-J Scroll', null, 12500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000049-0000-7000-b000-000000000000', 49, 'Aegis Shield', null, 12500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000050-0000-7000-b000-000000000000', 50, 'Elem Atk', null, 12500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000051-0000-7000-b000-000000000000', 51, 'Elem Guard', null, 12500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000052-0000-7000-b000-000000000000', 52, 'Status Atk', null, 12500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000053-0000-7000-b000-000000000000', 53, 'Status Guard', null, 12500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000054-0000-7000-b000-000000000000', 54, 'Rosetta Stone', null, 12500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000055-0000-7000-b000-000000000000', 55, 'Magic Scroll', 5000, 1250);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000056-0000-7000-b000-000000000000', 56, 'GF Scroll', 5000, 1250);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000057-0000-7000-b000-000000000000', 57, 'Draw Scroll', 5000, 1250);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000058-0000-7000-b000-000000000000', 58, 'Item Scroll', 5000, 1250);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000059-0000-7000-b000-000000000000', 59, 'Gambler Spirit', null, 1250);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000060-0000-7000-b000-000000000000', 60, 'Healing Ring', null, 2500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000061-0000-7000-b000-000000000000', 61, 'Phoenix Spirit', null, 2500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000062-0000-7000-b000-000000000000', 62, 'Med Kit', null, 2500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000063-0000-7000-b000-000000000000', 63, 'Bomb Spirit', null, 5000);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000064-0000-7000-b000-000000000000', 64, 'Hungry Cookpot', null, 5000);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000065-0000-7000-b000-000000000000', 65, 'Mog''s Amulet', null, 1250);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000066-0000-7000-b000-000000000000', 66, 'Steel Pipe', null, 75);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000067-0000-7000-b000-000000000000', 67, 'Star Fragment', null, 125);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000068-0000-7000-b000-000000000000', 68, 'Energy Crystal', null, 250);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000069-0000-7000-b000-000000000000', 69, 'Samantha Soul', null, 500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000070-0000-7000-b000-000000000000', 70, 'Healing Mail', null, 75);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000071-0000-7000-b000-000000000000', 71, 'Silver Mail', null, 125);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000072-0000-7000-b000-000000000000', 72, 'Gold Armor', null, 250);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000073-0000-7000-b000-000000000000', 73, 'Diamond Armor', null, 500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000074-0000-7000-b000-000000000000', 74, 'Regen Ring', null, 75);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000075-0000-7000-b000-000000000000', 75, 'Giant''s Ring', 20000, 5000);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000076-0000-7000-b000-000000000000', 76, 'Gaea''s Ring', null, 7500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000077-0000-7000-b000-000000000000', 77, 'Strength Love', null, 75);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000078-0000-7000-b000-000000000000', 78, 'Power Wrist', 20000, 5000);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000079-0000-7000-b000-000000000000', 79, 'Hyper Wrist', null, 7500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000080-0000-7000-b000-000000000000', 80, 'Turtle Shell', null, 75);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000081-0000-7000-b000-000000000000', 81, 'Orihalcon', 20000, 5000);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000082-0000-7000-b000-000000000000', 82, 'Adamantine', null, 7500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000083-0000-7000-b000-000000000000', 83, 'Rune Armlet', null, 75);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000084-0000-7000-b000-000000000000', 84, 'Force Armlet', 20000, 5000);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000085-0000-7000-b000-000000000000', 85, 'Magic Armlet', null, 7500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000086-0000-7000-b000-000000000000', 86, 'Circlet', null, 75);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000087-0000-7000-b000-000000000000', 87, 'Hypno Crown', 20000, 5000);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000088-0000-7000-b000-000000000000', 88, 'Royal Crown', null, 7500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000089-0000-7000-b000-000000000000', 89, 'Jet Engine', null, 5000);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000090-0000-7000-b000-000000000000', 90, 'Rocket Engine', null, 7500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000091-0000-7000-b000-000000000000', 91, 'Moon Curtain', null, 10000);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000092-0000-7000-b000-000000000000', 92, 'Steel Curtain', null, 10000);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000093-0000-7000-b000-000000000000', 93, 'Glow Curtain', null, 10000);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000094-0000-7000-b000-000000000000', 94, 'Accelerator', null, 12500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000095-0000-7000-b000-000000000000', 95, 'Monk''s Code', null, 12500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000096-0000-7000-b000-000000000000', 96, 'Knight''s Code', null, 10000);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000097-0000-7000-b000-000000000000', 97, 'Doc''s Code', null, 10000);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000098-0000-7000-b000-000000000000', 98, 'Hundred Needles', null, 10000);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000099-0000-7000-b000-000000000000', 99, 'Three Stars', null, 12500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000100-0000-7000-b000-000000000000', 100, 'Ribbon', null, 25000);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000101-0000-7000-b000-000000000000', 101, 'Normal Ammo', 20, 1);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000102-0000-7000-b000-000000000000', 102, 'Shotgun Ammo', 40, 2);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000103-0000-7000-b000-000000000000', 103, 'Dark Ammo', 300, 15);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000104-0000-7000-b000-000000000000', 104, 'Fire Ammo', 500, 25);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000105-0000-7000-b000-000000000000', 105, 'Demolition Ammo', 800, 40);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000106-0000-7000-b000-000000000000', 106, 'Fast Ammo', 100, 5);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000107-0000-7000-b000-000000000000', 107, 'AP Ammo', null, 50);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000108-0000-7000-b000-000000000000', 108, 'Pulse Ammo', null, 250);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000109-0000-7000-b000-000000000000', 109, 'M-Stone Piece', null, 5);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000110-0000-7000-b000-000000000000', 110, 'Magic Stone', null, 12);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000111-0000-7000-b000-000000000000', 111, 'Wizard Stone', null, 20);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000112-0000-7000-b000-000000000000', 112, 'Ochu Tentacle', null, 75);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000113-0000-7000-b000-000000000000', 113, 'Healing Water', null, 25);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000114-0000-7000-b000-000000000000', 114, 'Cockatrice Pinion', null, 50);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000115-0000-7000-b000-000000000000', 115, 'Zombie Powder', null, 50);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000116-0000-7000-b000-000000000000', 116, 'Lightweight', null, 50);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000117-0000-7000-b000-000000000000', 117, 'Sharp Spike', null, 50);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000118-0000-7000-b000-000000000000', 118, 'Screw', null, 25);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000119-0000-7000-b000-000000000000', 119, 'Saw Blade', null, 50);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000120-0000-7000-b000-000000000000', 120, 'Mesmerize Blade', null, 50);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000121-0000-7000-b000-000000000000', 121, 'Vampire Fang', null, 50);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000122-0000-7000-b000-000000000000', 122, 'Fury Fragment', null, 125);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000123-0000-7000-b000-000000000000', 123, 'Betrayal Sword', null, 50);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000124-0000-7000-b000-000000000000', 124, 'Sleep Powder', null, 50);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000125-0000-7000-b000-000000000000', 125, 'Life Ring', null, 50);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000126-0000-7000-b000-000000000000', 126, 'Dragon Fang', null, 50);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000127-0000-7000-b000-000000000000', 127, 'Spider Web', null, 50);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000128-0000-7000-b000-000000000000', 128, 'Coral Fragment', null, 75);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000129-0000-7000-b000-000000000000', 129, 'Curse Spike', null, 75);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000130-0000-7000-b000-000000000000', 130, 'Black Hole', null, 75);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000131-0000-7000-b000-000000000000', 131, 'Water Crystal', null, 75);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000132-0000-7000-b000-000000000000', 132, 'Missile', null, 75);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000133-0000-7000-b000-000000000000', 133, 'Mystery Fluid', null, 75);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000134-0000-7000-b000-000000000000', 134, 'Running Fire', null, 75);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000135-0000-7000-b000-000000000000', 135, 'Inferno Fang', null, 75);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000136-0000-7000-b000-000000000000', 136, 'Malboro Tentacle', null, 100);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000137-0000-7000-b000-000000000000', 137, 'Whisper', null, 100);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000138-0000-7000-b000-000000000000', 138, 'Laser Cannon', null, 125);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000139-0000-7000-b000-000000000000', 139, 'Barrier', null, 125);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000140-0000-7000-b000-000000000000', 140, 'Power Generator', null, 200);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000141-0000-7000-b000-000000000000', 141, 'Dark Matter', null, 250);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000142-0000-7000-b000-000000000000', 142, 'Bomb Fragment', null, 25);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000143-0000-7000-b000-000000000000', 143, 'Red Fang', null, 75);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000144-0000-7000-b000-000000000000', 144, 'Arctic Wind', null, 25);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000145-0000-7000-b000-000000000000', 145, 'North Wind', null, 75);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000146-0000-7000-b000-000000000000', 146, 'Dynamo Stone', null, 125);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000147-0000-7000-b000-000000000000', 147, 'Shear Feather', null, 125);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000148-0000-7000-b000-000000000000', 148, 'Venom Fang', null, 75);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000149-0000-7000-b000-000000000000', 149, 'Steel Orb', null, 75);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000150-0000-7000-b000-000000000000', 150, 'Moon Stone', null, 75);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000151-0000-7000-b000-000000000000', 151, 'Dino Bone', null, 75);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000152-0000-7000-b000-000000000000', 152, 'Windmill', null, 75);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000153-0000-7000-b000-000000000000', 153, 'Dragon Skin', null, 75);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000154-0000-7000-b000-000000000000', 154, 'Fish Fin', null, 25);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000155-0000-7000-b000-000000000000', 155, 'Dragon Fin', null, 25);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000156-0000-7000-b000-000000000000', 156, 'Silence Powder', null, 25);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000157-0000-7000-b000-000000000000', 157, 'Poison Powder', null, 25);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000158-0000-7000-b000-000000000000', 158, 'Dead Spirit', null, 75);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000159-0000-7000-b000-000000000000', 159, 'Chef''s Knife', null, 75);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000160-0000-7000-b000-000000000000', 160, 'Cactus Thorn', null, 75);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000161-0000-7000-b000-000000000000', 161, 'Shaman Stone', null, 1250);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000162-0000-7000-b000-000000000000', 162, 'Fuel', 3000, 750);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000163-0000-7000-b000-000000000000', 163, 'The Girl Next Door', null, 12500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000164-0000-7000-b000-000000000000', 164, 'Sorceress'' Letter', null, 125);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000165-0000-7000-b000-000000000000', 165, 'Chocobo''s Tag', null, 125);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000166-0000-7000-b000-000000000000', 166, 'Pet Nametag', null, 125);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000167-0000-7000-b000-000000000000', 167, 'Solomon Ring', null, 125);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000168-0000-7000-b000-000000000000', 168, 'Magical Lamp', null, 125);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000169-0000-7000-b000-000000000000', 169, 'HP Up', null, 250);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000170-0000-7000-b000-000000000000', 170, 'Str Up', null, 250);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000171-0000-7000-b000-000000000000', 171, 'Vit Up', null, 250);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000172-0000-7000-b000-000000000000', 172, 'Mag Up', null, 250);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000173-0000-7000-b000-000000000000', 173, 'Spr Up', null, 250);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000174-0000-7000-b000-000000000000', 174, 'Spd Up', null, 250);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000175-0000-7000-b000-000000000000', 175, 'Luck Up', null, 250);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000176-0000-7000-b000-000000000000', 176, 'LuvLuv G', null, 250);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000177-0000-7000-b000-000000000000', 177, 'Weapons Mon 1st', 50000, 25000);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000178-0000-7000-b000-000000000000', 178, 'Weapons Mon Mar', 1000, 500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000179-0000-7000-b000-000000000000', 179, 'Weapons Mon Apr', 1000, 500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000180-0000-7000-b000-000000000000', 180, 'Weapons Mon May', 1000, 500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000181-0000-7000-b000-000000000000', 181, 'Weapons Mon Jun', 1000, 500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000182-0000-7000-b000-000000000000', 182, 'Weapons Mon Jul', 1000, 500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000183-0000-7000-b000-000000000000', 183, 'Weapons Mon Aug', 1000, 500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000184-0000-7000-b000-000000000000', 184, 'Combat King 001', 1000, 500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000185-0000-7000-b000-000000000000', 185, 'Combat King 002', 1000, 500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000186-0000-7000-b000-000000000000', 186, 'Combat King 003', 1000, 500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000187-0000-7000-b000-000000000000', 187, 'Combat King 004', 1000, 500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000188-0000-7000-b000-000000000000', 188, 'Combat King 005', 30000, 15000);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000189-0000-7000-b000-000000000000', 189, 'Pet Pals Vol. 1', 1000, 500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000190-0000-7000-b000-000000000000', 190, 'Pet Pals Vol. 2', 1000, 500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000191-0000-7000-b000-000000000000', 191, 'Pet Pals Vol. 3', 1000, 500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000192-0000-7000-b000-000000000000', 192, 'Pet Pals Vol. 4', 1000, 500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000193-0000-7000-b000-000000000000', 193, 'Pet Pals Vol. 5', 1000, 500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000194-0000-7000-b000-000000000000', 194, 'Pet Pals Vol. 6', 1000, 500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000195-0000-7000-b000-000000000000', 195, 'Occult Fan I', 35000, 17500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000196-0000-7000-b000-000000000000', 196, 'Occult Fan II', 35000, 17500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000197-0000-7000-b000-000000000000', 197, 'Occult Fan III', null, 17500);
INSERT INTO items("id", "code", "name", "price", "sell_price") VALUES('00000198-0000-7000-b000-000000000000', 198, 'Occult Fan IV', null, 20000);


-- item_inventory

INSERT INTO item_inventories("id", "item_id", "quantity") VALUES('00000001-0000-7000-b000-000000000004', '00000001-0000-7000-b000-000000000000', 20);
INSERT INTO item_inventories("id", "item_id", "quantity") VALUES('00000002-0000-7000-b000-000000000004', '00000003-0000-7000-b000-000000000000', 20);
INSERT INTO item_inventories("id", "item_id", "quantity") VALUES('00000003-0000-7000-b000-000000000004', '00000007-0000-7000-b000-000000000000', 10);


-- weapon

INSERT INTO weapons("id", "name", "str", "hit") VALUES('00000000-0000-7000-b000-000000000001', 'Revolver', 11, 255);
INSERT INTO weapons("id", "name", "str", "hit") VALUES('00000001-0000-7000-b000-000000000001', 'Shear Trigger', 14, 255);
INSERT INTO weapons("id", "name", "str", "hit") VALUES('00000002-0000-7000-b000-000000000001', 'Cutting Trigger', 18, 255);
INSERT INTO weapons("id", "name", "str", "hit") VALUES('00000003-0000-7000-b000-000000000001', 'Flame Saber', 20, 255);
INSERT INTO weapons("id", "name", "str", "hit") VALUES('00000004-0000-7000-b000-000000000001', 'Twin Lance', 22, 255);
INSERT INTO weapons("id", "name", "str", "hit") VALUES('00000005-0000-7000-b000-000000000001', 'Punishment', 24, 255);
INSERT INTO weapons("id", "name", "str", "hit") VALUES('00000006-0000-7000-b000-000000000001', 'Lion Heart', 30, 255);
INSERT INTO weapons("id", "name", "str", "hit") VALUES('00000007-0000-7000-b000-000000000001', 'Metal Knuckle', 12, 98);
INSERT INTO weapons("id", "name", "str", "hit") VALUES('00000008-0000-7000-b000-000000000001', 'Maverick', 15, 99);
INSERT INTO weapons("id", "name", "str", "hit") VALUES('00000009-0000-7000-b000-000000000001', 'Gauntlet', 20, 101);
INSERT INTO weapons("id", "name", "str", "hit") VALUES('00000010-0000-7000-b000-000000000001', 'Ehrgeiz', 25, 103);
INSERT INTO weapons("id", "name", "str", "hit") VALUES('00000011-0000-7000-b000-000000000001', 'Valiant', 12, 105);
INSERT INTO weapons("id", "name", "str", "hit") VALUES('00000012-0000-7000-b000-000000000001', 'Ulysses', 15, 108);
INSERT INTO weapons("id", "name", "str", "hit") VALUES('00000013-0000-7000-b000-000000000001', 'Bismarck', 20, 110);
INSERT INTO weapons("id", "name", "str", "hit") VALUES('00000014-0000-7000-b000-000000000001', 'Exeter', 25, 115);
INSERT INTO weapons("id", "name", "str", "hit") VALUES('00000015-0000-7000-b000-000000000001', 'Chain Whip', 12, 103);
INSERT INTO weapons("id", "name", "str", "hit") VALUES('00000016-0000-7000-b000-000000000001', 'Slaying Tail', 15, 104);
INSERT INTO weapons("id", "name", "str", "hit") VALUES('00000017-0000-7000-b000-000000000001', 'Red Scorpion', 20, 105);
INSERT INTO weapons("id", "name", "str", "hit") VALUES('00000018-0000-7000-b000-000000000001', 'Save the Queen', 25, 107);
INSERT INTO weapons("id", "name", "str", "hit") VALUES('00000019-0000-7000-b000-000000000001', 'Pinwheel', 11, 99);
INSERT INTO weapons("id", "name", "str", "hit") VALUES('00000020-0000-7000-b000-000000000001', 'Valkyrie', 14, 101);
INSERT INTO weapons("id", "name", "str", "hit") VALUES('00000021-0000-7000-b000-000000000001', 'Rising Sun', 18, 103);
INSERT INTO weapons("id", "name", "str", "hit") VALUES('00000022-0000-7000-b000-000000000001', 'Cardinal', 24, 104);
INSERT INTO weapons("id", "name", "str", "hit") VALUES('00000023-0000-7000-b000-000000000001', 'Shooting Star', 28, 107);
INSERT INTO weapons("id", "name", "str", "hit") VALUES('00000024-0000-7000-b000-000000000001', 'Flail', 12, 98);
INSERT INTO weapons("id", "name", "str", "hit") VALUES('00000025-0000-7000-b000-000000000001', 'Morning Star', 15, 99);
INSERT INTO weapons("id", "name", "str", "hit") VALUES('00000026-0000-7000-b000-000000000001', 'Crescent Wish', 20, 100);
INSERT INTO weapons("id", "name", "str", "hit") VALUES('00000027-0000-7000-b000-000000000001', 'Strange Vision', 25, 255);
INSERT INTO weapons("id", "name", "str", "hit") VALUES('00000028-0000-7000-b000-000000000001', 'Hyperion', 12, 255);
INSERT INTO weapons("id", "name", "str", "hit") VALUES('00000029-0000-7000-b000-000000000001', 'None', 20, 97);
INSERT INTO weapons("id", "name", "str", "hit") VALUES('00000030-0000-7000-b000-000000000001', 'Machine Gun', 14, 103);
INSERT INTO weapons("id", "name", "str", "hit") VALUES('00000031-0000-7000-b000-000000000001', 'Katal', 13, 102);
INSERT INTO weapons("id", "name", "str", "hit") VALUES('00000032-0000-7000-b000-000000000001', 'Harpoon', 16, 98);


-- weapon_remodel

INSERT INTO weapon_remodels("id", "weapon_id", "price") VALUES('00000000-0000-7000-b000-000000000002', '00000000-0000-7000-b000-000000000001', 100);
INSERT INTO weapon_remodels("id", "weapon_id", "price") VALUES('00000001-0000-7000-b000-000000000002', '00000001-0000-7000-b000-000000000001', 200);
INSERT INTO weapon_remodels("id", "weapon_id", "price") VALUES('00000002-0000-7000-b000-000000000002', '00000002-0000-7000-b000-000000000001', 400);
INSERT INTO weapon_remodels("id", "weapon_id", "price") VALUES('00000003-0000-7000-b000-000000000002', '00000003-0000-7000-b000-000000000001', 600);
INSERT INTO weapon_remodels("id", "weapon_id", "price") VALUES('00000004-0000-7000-b000-000000000002', '00000004-0000-7000-b000-000000000001', 800);
INSERT INTO weapon_remodels("id", "weapon_id", "price") VALUES('00000005-0000-7000-b000-000000000002', '00000005-0000-7000-b000-000000000001', 1000);
INSERT INTO weapon_remodels("id", "weapon_id", "price") VALUES('00000006-0000-7000-b000-000000000002', '00000006-0000-7000-b000-000000000001', 2000);
INSERT INTO weapon_remodels("id", "weapon_id", "price") VALUES('00000007-0000-7000-b000-000000000002', '00000007-0000-7000-b000-000000000001', 100);
INSERT INTO weapon_remodels("id", "weapon_id", "price") VALUES('00000008-0000-7000-b000-000000000002', '00000008-0000-7000-b000-000000000001', 200);
INSERT INTO weapon_remodels("id", "weapon_id", "price") VALUES('00000009-0000-7000-b000-000000000002', '00000009-0000-7000-b000-000000000001', 400);
INSERT INTO weapon_remodels("id", "weapon_id", "price") VALUES('00000010-0000-7000-b000-000000000002', '00000010-0000-7000-b000-000000000001', 800);
INSERT INTO weapon_remodels("id", "weapon_id", "price") VALUES('00000011-0000-7000-b000-000000000002', '00000011-0000-7000-b000-000000000001', 100);
INSERT INTO weapon_remodels("id", "weapon_id", "price") VALUES('00000012-0000-7000-b000-000000000002', '00000012-0000-7000-b000-000000000001', 200);
INSERT INTO weapon_remodels("id", "weapon_id", "price") VALUES('00000013-0000-7000-b000-000000000002', '00000013-0000-7000-b000-000000000001', 400);
INSERT INTO weapon_remodels("id", "weapon_id", "price") VALUES('00000014-0000-7000-b000-000000000002', '00000014-0000-7000-b000-000000000001', 800);
INSERT INTO weapon_remodels("id", "weapon_id", "price") VALUES('00000015-0000-7000-b000-000000000002', '00000015-0000-7000-b000-000000000001', 100);
INSERT INTO weapon_remodels("id", "weapon_id", "price") VALUES('00000016-0000-7000-b000-000000000002', '00000016-0000-7000-b000-000000000001', 200);
INSERT INTO weapon_remodels("id", "weapon_id", "price") VALUES('00000017-0000-7000-b000-000000000002', '00000017-0000-7000-b000-000000000001', 400);
INSERT INTO weapon_remodels("id", "weapon_id", "price") VALUES('00000018-0000-7000-b000-000000000002', '00000018-0000-7000-b000-000000000001', 800);
INSERT INTO weapon_remodels("id", "weapon_id", "price") VALUES('00000019-0000-7000-b000-000000000002', '00000019-0000-7000-b000-000000000001', 100);
INSERT INTO weapon_remodels("id", "weapon_id", "price") VALUES('00000020-0000-7000-b000-000000000002', '00000020-0000-7000-b000-000000000001', 200);
INSERT INTO weapon_remodels("id", "weapon_id", "price") VALUES('00000021-0000-7000-b000-000000000002', '00000021-0000-7000-b000-000000000001', 400);
INSERT INTO weapon_remodels("id", "weapon_id", "price") VALUES('00000022-0000-7000-b000-000000000002', '00000022-0000-7000-b000-000000000001', 800);
INSERT INTO weapon_remodels("id", "weapon_id", "price") VALUES('00000023-0000-7000-b000-000000000002', '00000023-0000-7000-b000-000000000001', 1000);
INSERT INTO weapon_remodels("id", "weapon_id", "price") VALUES('00000024-0000-7000-b000-000000000002', '00000024-0000-7000-b000-000000000001', 100);
INSERT INTO weapon_remodels("id", "weapon_id", "price") VALUES('00000025-0000-7000-b000-000000000002', '00000025-0000-7000-b000-000000000001', 200);
INSERT INTO weapon_remodels("id", "weapon_id", "price") VALUES('00000026-0000-7000-b000-000000000002', '00000026-0000-7000-b000-000000000001', 400);
INSERT INTO weapon_remodels("id", "weapon_id", "price") VALUES('00000027-0000-7000-b000-000000000002', '00000027-0000-7000-b000-000000000001', 800);


-- weapon_material

-- Squall - Revolver
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000000-0000-7000-b000-000000000003', '00000000-0000-7000-b000-000000000001', '00000109-0000-7000-b000-000000000000', 6);
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000001-0000-7000-b000-000000000003', '00000000-0000-7000-b000-000000000001', '00000118-0000-7000-b000-000000000000', 2);

-- Squall - Shear Trigger
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000002-0000-7000-b000-000000000003', '00000001-0000-7000-b000-000000000001', '00000066-0000-7000-b000-000000000000', 1);
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000003-0000-7000-b000-000000000003', '00000001-0000-7000-b000-000000000001', '00000118-0000-7000-b000-000000000000', 4);

-- Squall - Cutting Trigger
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000004-0000-7000-b000-000000000003', '00000002-0000-7000-b000-000000000001', '00000120-0000-7000-b000-000000000000', 1);
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000005-0000-7000-b000-000000000003', '00000002-0000-7000-b000-000000000001', '00000118-0000-7000-b000-000000000000', 8);

-- Squall - Flame Saber
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000006-0000-7000-b000-000000000003', '00000003-0000-7000-b000-000000000001', '00000123-0000-7000-b000-000000000000', 1);
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000007-0000-7000-b000-000000000003', '00000003-0000-7000-b000-000000000001', '00000080-0000-7000-b000-000000000000', 1);
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000008-0000-7000-b000-000000000003', '00000003-0000-7000-b000-000000000001', '00000118-0000-7000-b000-000000000000', 4);

-- Squall - Twin Lance
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000009-0000-7000-b000-000000000003', '00000004-0000-7000-b000-000000000001', '00000151-0000-7000-b000-000000000000', 1);
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000010-0000-7000-b000-000000000003', '00000004-0000-7000-b000-000000000001', '00000143-0000-7000-b000-000000000000', 2);
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000011-0000-7000-b000-000000000003', '00000004-0000-7000-b000-000000000001', '00000118-0000-7000-b000-000000000000', 12);

-- Squall - Punishment
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000012-0000-7000-b000-000000000003', '00000005-0000-7000-b000-000000000001', '00000159-0000-7000-b000-000000000000', 1);
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000013-0000-7000-b000-000000000003', '00000005-0000-7000-b000-000000000001', '00000067-0000-7000-b000-000000000000', 2);
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000014-0000-7000-b000-000000000003', '00000005-0000-7000-b000-000000000001', '00000080-0000-7000-b000-000000000000', 1);
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000015-0000-7000-b000-000000000003', '00000005-0000-7000-b000-000000000001', '00000118-0000-7000-b000-000000000000', 8);

-- Squall - Lion Heart
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000016-0000-7000-b000-000000000003', '00000006-0000-7000-b000-000000000001', '00000082-0000-7000-b000-000000000000', 1);
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000017-0000-7000-b000-000000000003', '00000006-0000-7000-b000-000000000001', '00000126-0000-7000-b000-000000000000', 4);
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000018-0000-7000-b000-000000000003', '00000006-0000-7000-b000-000000000001', '00000108-0000-7000-b000-000000000000', 12);

-- Zell - Metal Knuckle
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000019-0000-7000-b000-000000000003', '00000007-0000-7000-b000-000000000001', '00000154-0000-7000-b000-000000000000', 1);
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000020-0000-7000-b000-000000000003', '00000007-0000-7000-b000-000000000001', '00000109-0000-7000-b000-000000000000', 4);

-- Zell - Maverick
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000021-0000-7000-b000-000000000003', '00000008-0000-7000-b000-000000000001', '00000155-0000-7000-b000-000000000000', 1);
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000022-0000-7000-b000-000000000003', '00000008-0000-7000-b000-000000000001', '00000127-0000-7000-b000-000000000000', 1);

-- Zell - Gauntlet
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000023-0000-7000-b000-000000000003', '00000009-0000-7000-b000-000000000001', '00000153-0000-7000-b000-000000000000', 1);
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000024-0000-7000-b000-000000000003', '00000009-0000-7000-b000-000000000001', '00000122-0000-7000-b000-000000000000', 1);

-- Zell - Ehrgeiz
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000025-0000-7000-b000-000000000003', '00000010-0000-7000-b000-000000000001', '00000082-0000-7000-b000-000000000000', 1);
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000026-0000-7000-b000-000000000003', '00000010-0000-7000-b000-000000000001', '00000153-0000-7000-b000-000000000000', 4);
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000027-0000-7000-b000-000000000003', '00000010-0000-7000-b000-000000000001', '00000122-0000-7000-b000-000000000000', 1);


-- Irvine - Valiant
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000028-0000-7000-b000-000000000003', '00000011-0000-7000-b000-000000000001', '00000066-0000-7000-b000-000000000000', 1);
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000029-0000-7000-b000-000000000003', '00000011-0000-7000-b000-000000000001', '00000118-0000-7000-b000-000000000000', 4);

-- Irvine - Ulysses
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000030-0000-7000-b000-000000000003', '00000012-0000-7000-b000-000000000001', '00000066-0000-7000-b000-000000000000', 1);
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000031-0000-7000-b000-000000000003', '00000012-0000-7000-b000-000000000001', '00000127-0000-7000-b000-000000000000', 1);
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000032-0000-7000-b000-000000000003', '00000012-0000-7000-b000-000000000001', '00000118-0000-7000-b000-000000000000', 2);

-- Irvine - Bismarck
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000033-0000-7000-b000-000000000003', '00000013-0000-7000-b000-000000000001', '00000066-0000-7000-b000-000000000000', 2);
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000034-0000-7000-b000-000000000003', '00000013-0000-7000-b000-000000000001', '00000146-0000-7000-b000-000000000000', 4);
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000035-0000-7000-b000-000000000003', '00000013-0000-7000-b000-000000000001', '00000118-0000-7000-b000-000000000000', 8);

-- Irvine - Exeter
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000036-0000-7000-b000-000000000003', '00000014-0000-7000-b000-000000000001', '00000151-0000-7000-b000-000000000000', 2);
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000037-0000-7000-b000-000000000003', '00000014-0000-7000-b000-000000000001', '00000150-0000-7000-b000-000000000000', 1);
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000038-0000-7000-b000-000000000003', '00000014-0000-7000-b000-000000000001', '00000067-0000-7000-b000-000000000000', 2);
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000039-0000-7000-b000-000000000003', '00000014-0000-7000-b000-000000000001', '00000118-0000-7000-b000-000000000000', 18);

-- Quistis - Chain Whip
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000040-0000-7000-b000-000000000003', '00000015-0000-7000-b000-000000000001', '00000109-0000-7000-b000-000000000000', 2);
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000041-0000-7000-b000-000000000003', '00000015-0000-7000-b000-000000000001', '00000127-0000-7000-b000-000000000000', 1);

-- Quistis - Slaying Tail
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000042-0000-7000-b000-000000000003', '00000016-0000-7000-b000-000000000001', '00000110-0000-7000-b000-000000000000', 2);
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000043-0000-7000-b000-000000000003', '00000016-0000-7000-b000-000000000001', '00000117-0000-7000-b000-000000000000', 1);

-- Quistis - Red Scorpion
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000044-0000-7000-b000-000000000003', '00000017-0000-7000-b000-000000000001', '00000112-0000-7000-b000-000000000000', 2);
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000045-0000-7000-b000-000000000003', '00000017-0000-7000-b000-000000000001', '00000153-0000-7000-b000-000000000000', 2);

-- Quistis - Save the Queen
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000046-0000-7000-b000-000000000003', '00000018-0000-7000-b000-000000000001', '00000136-0000-7000-b000-000000000000', 2);
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000047-0000-7000-b000-000000000003', '00000018-0000-7000-b000-000000000001', '00000117-0000-7000-b000-000000000000', 4);
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000048-0000-7000-b000-000000000003', '00000018-0000-7000-b000-000000000001', '00000068-0000-7000-b000-000000000000', 4);

-- Rinoa - Pinwheel
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000049-0000-7000-b000-000000000003', '00000019-0000-7000-b000-000000000001', '00000109-0000-7000-b000-000000000000', 3);

-- Rinoa - Valkyrie
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000050-0000-7000-b000-000000000003', '00000020-0000-7000-b000-000000000001', '00000147-0000-7000-b000-000000000000', 1);
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000051-0000-7000-b000-000000000003', '00000020-0000-7000-b000-000000000001', '00000110-0000-7000-b000-000000000000', 1);

-- Rinoa - Rising Sun
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000052-0000-7000-b000-000000000003', '00000021-0000-7000-b000-000000000001', '00000119-0000-7000-b000-000000000000', 1);
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000053-0000-7000-b000-000000000003', '00000021-0000-7000-b000-000000000001', '00000118-0000-7000-b000-000000000000', 8);

-- Rinoa - Cardinal
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000054-0000-7000-b000-000000000003', '00000022-0000-7000-b000-000000000001', '00000114-0000-7000-b000-000000000000', 1);
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000055-0000-7000-b000-000000000003', '00000022-0000-7000-b000-000000000001', '00000120-0000-7000-b000-000000000000', 1);
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000056-0000-7000-b000-000000000003', '00000022-0000-7000-b000-000000000001', '00000117-0000-7000-b000-000000000000', 1);

-- Rinoa - Shooting Star
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000057-0000-7000-b000-000000000003', '00000023-0000-7000-b000-000000000001', '00000152-0000-7000-b000-000000000000', 2);
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000058-0000-7000-b000-000000000003', '00000023-0000-7000-b000-000000000001', '00000082-0000-7000-b000-000000000000', 1);
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000059-0000-7000-b000-000000000003', '00000023-0000-7000-b000-000000000001', '00000084-0000-7000-b000-000000000000', 1);
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000060-0000-7000-b000-000000000003', '00000023-0000-7000-b000-000000000001', '00000068-0000-7000-b000-000000000000', 2);

-- Selphie - Flail
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000061-0000-7000-b000-000000000003', '00000024-0000-7000-b000-000000000001', '00000109-0000-7000-b000-000000000000', 2);
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000062-0000-7000-b000-000000000003', '00000024-0000-7000-b000-000000000001', '00000127-0000-7000-b000-000000000000', 1);

-- Selphie - Morning Star
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000063-0000-7000-b000-000000000003', '00000025-0000-7000-b000-000000000001', '00000149-0000-7000-b000-000000000000', 2);
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000064-0000-7000-b000-000000000003', '00000025-0000-7000-b000-000000000001', '00000117-0000-7000-b000-000000000000', 2);

-- Selphie - Crescent Wish
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000065-0000-7000-b000-000000000003', '00000026-0000-7000-b000-000000000001', '00000135-0000-7000-b000-000000000000', 1);
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000066-0000-7000-b000-000000000003', '00000026-0000-7000-b000-000000000001', '00000125-0000-7000-b000-000000000000', 1);
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000067-0000-7000-b000-000000000003', '00000026-0000-7000-b000-000000000001', '00000117-0000-7000-b000-000000000000', 4);

-- Selphie - Strange Vision
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000068-0000-7000-b000-000000000003', '00000027-0000-7000-b000-000000000001', '00000082-0000-7000-b000-000000000000', 1);
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000069-0000-7000-b000-000000000003', '00000027-0000-7000-b000-000000000001', '00000067-0000-7000-b000-000000000000', 3);
INSERT INTO weapon_materials("id", "weapon_id", "item_id", "quantity") VALUES('00000070-0000-7000-b000-000000000003', '00000027-0000-7000-b000-000000000001', '00000153-0000-7000-b000-000000000000', 2);


-- weapon_remodel_materials

-- Squall - Revolver
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000000-0000-7000-b000-000000000003', '00000000-0000-7000-b000-000000000002');
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000001-0000-7000-b000-000000000003', '00000000-0000-7000-b000-000000000002');

-- Squall - Shear Trigger
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000002-0000-7000-b000-000000000003', '00000001-0000-7000-b000-000000000002');
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000003-0000-7000-b000-000000000003', '00000001-0000-7000-b000-000000000002');

-- Squall - Cutting Trigger
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000004-0000-7000-b000-000000000003', '00000002-0000-7000-b000-000000000002');
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000005-0000-7000-b000-000000000003', '00000002-0000-7000-b000-000000000002');

-- Squall - Flame Saber
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000006-0000-7000-b000-000000000003', '00000003-0000-7000-b000-000000000002');
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000007-0000-7000-b000-000000000003', '00000003-0000-7000-b000-000000000002');
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000008-0000-7000-b000-000000000003', '00000003-0000-7000-b000-000000000002');

-- Squall - Twin Lance
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000009-0000-7000-b000-000000000003', '00000004-0000-7000-b000-000000000002');
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000010-0000-7000-b000-000000000003', '00000004-0000-7000-b000-000000000002');
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000011-0000-7000-b000-000000000003', '00000004-0000-7000-b000-000000000002');

-- Squall - Punishment
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000012-0000-7000-b000-000000000003', '00000005-0000-7000-b000-000000000002');
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000013-0000-7000-b000-000000000003', '00000005-0000-7000-b000-000000000002');
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000014-0000-7000-b000-000000000003', '00000005-0000-7000-b000-000000000002');
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000015-0000-7000-b000-000000000003', '00000005-0000-7000-b000-000000000002');

-- Squall - Lion Heart
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000016-0000-7000-b000-000000000003', '00000006-0000-7000-b000-000000000002');
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000017-0000-7000-b000-000000000003', '00000006-0000-7000-b000-000000000002');
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000018-0000-7000-b000-000000000003', '00000006-0000-7000-b000-000000000002');

-- Zell - Metal Knuckle
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000019-0000-7000-b000-000000000003', '00000007-0000-7000-b000-000000000002');
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000020-0000-7000-b000-000000000003', '00000007-0000-7000-b000-000000000002');

-- Zell - Maverick
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000021-0000-7000-b000-000000000003', '00000008-0000-7000-b000-000000000002');
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000022-0000-7000-b000-000000000003', '00000008-0000-7000-b000-000000000002');

-- Zell - Gauntlet
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000023-0000-7000-b000-000000000003', '00000009-0000-7000-b000-000000000002');
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000024-0000-7000-b000-000000000003', '00000009-0000-7000-b000-000000000002');

-- Zell - Ehrgeiz
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000025-0000-7000-b000-000000000003', '00000010-0000-7000-b000-000000000002');
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000026-0000-7000-b000-000000000003', '00000010-0000-7000-b000-000000000002');
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000027-0000-7000-b000-000000000003', '00000010-0000-7000-b000-000000000002');

-- Irvine - Valiant
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000028-0000-7000-b000-000000000003', '00000011-0000-7000-b000-000000000002');
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000029-0000-7000-b000-000000000003', '00000011-0000-7000-b000-000000000002');

-- Irvine - Ulysses
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000030-0000-7000-b000-000000000003', '00000012-0000-7000-b000-000000000002');
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000031-0000-7000-b000-000000000003', '00000012-0000-7000-b000-000000000002');
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000032-0000-7000-b000-000000000003', '00000012-0000-7000-b000-000000000002');

-- Irvine - Bismarck
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000033-0000-7000-b000-000000000003', '00000013-0000-7000-b000-000000000002');
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000034-0000-7000-b000-000000000003', '00000013-0000-7000-b000-000000000002');
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000035-0000-7000-b000-000000000003', '00000013-0000-7000-b000-000000000002');

-- Irvine - Exeter
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000036-0000-7000-b000-000000000003', '00000014-0000-7000-b000-000000000002');
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000037-0000-7000-b000-000000000003', '00000014-0000-7000-b000-000000000002');
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000038-0000-7000-b000-000000000003', '00000014-0000-7000-b000-000000000002');
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000039-0000-7000-b000-000000000003', '00000014-0000-7000-b000-000000000002');

-- Quistis - Chain Whip
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000040-0000-7000-b000-000000000003', '00000015-0000-7000-b000-000000000002');
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000041-0000-7000-b000-000000000003', '00000015-0000-7000-b000-000000000002');

-- Quistis - Slaying Tail
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000042-0000-7000-b000-000000000003', '00000016-0000-7000-b000-000000000002');
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000043-0000-7000-b000-000000000003', '00000016-0000-7000-b000-000000000002');

-- Quistis - Red Scorpion
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000044-0000-7000-b000-000000000003', '00000017-0000-7000-b000-000000000002');
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000045-0000-7000-b000-000000000003', '00000017-0000-7000-b000-000000000002');

-- Quistis - Save the Queen
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000046-0000-7000-b000-000000000003', '00000018-0000-7000-b000-000000000002');
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000047-0000-7000-b000-000000000003', '00000018-0000-7000-b000-000000000002');
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000048-0000-7000-b000-000000000003', '00000018-0000-7000-b000-000000000002');

-- Rinoa - Pinwheel
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000049-0000-7000-b000-000000000003', '00000019-0000-7000-b000-000000000002');

-- Rinoa - Valkyrie
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000050-0000-7000-b000-000000000003', '00000020-0000-7000-b000-000000000002');
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000051-0000-7000-b000-000000000003', '00000020-0000-7000-b000-000000000002');

-- Rinoa - Rising Sun
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000052-0000-7000-b000-000000000003', '00000021-0000-7000-b000-000000000002');
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000053-0000-7000-b000-000000000003', '00000021-0000-7000-b000-000000000002');

-- Rinoa - Cardinal
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000054-0000-7000-b000-000000000003', '00000022-0000-7000-b000-000000000002');
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000055-0000-7000-b000-000000000003', '00000022-0000-7000-b000-000000000002');
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000056-0000-7000-b000-000000000003', '00000022-0000-7000-b000-000000000002');

-- Rinoa - Shooting Star
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000057-0000-7000-b000-000000000003', '00000023-0000-7000-b000-000000000002');
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000058-0000-7000-b000-000000000003', '00000023-0000-7000-b000-000000000002');
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000059-0000-7000-b000-000000000003', '00000023-0000-7000-b000-000000000002');
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000060-0000-7000-b000-000000000003', '00000023-0000-7000-b000-000000000002');

-- Selphie - Flail
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000061-0000-7000-b000-000000000003', '00000024-0000-7000-b000-000000000002');
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000062-0000-7000-b000-000000000003', '00000024-0000-7000-b000-000000000002');

-- Selphie - Morning Star
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000063-0000-7000-b000-000000000003', '00000025-0000-7000-b000-000000000002');
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000064-0000-7000-b000-000000000003', '00000025-0000-7000-b000-000000000002');

-- Selphie - Crescent Wish
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000065-0000-7000-b000-000000000003', '00000026-0000-7000-b000-000000000002');
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000066-0000-7000-b000-000000000003', '00000026-0000-7000-b000-000000000002');
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000067-0000-7000-b000-000000000003', '00000026-0000-7000-b000-000000000002');

-- Selphie - Strange Vision
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000068-0000-7000-b000-000000000003', '00000027-0000-7000-b000-000000000002');
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000069-0000-7000-b000-000000000003', '00000027-0000-7000-b000-000000000002');
INSERT INTO weapon_remodel_materials("materials_id", "weapon_remodel_id") VALUES('00000070-0000-7000-b000-000000000003', '00000027-0000-7000-b000-000000000002');
