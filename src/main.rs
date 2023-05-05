use std::{io, panic};
use std::io::prelude::*;
use std::collections::HashMap;
use clap::{Command, arg};

fn main() -> io::Result<()> {
    let matches = Command::new("i2map")
        .version("1.0")
        .author("TheAlan404")
        .about("Image/Video to Minecraft Map Format")
        .args(&[
            arg!(-w --width <WIDTH> "Display width in item frames"),
            arg!(-h --height <HEIGHT> "Display height in item frames"),
            arg!(-d --debug [dest] "Debug to destination"),
            arg!(-i --input <INPUT> "Specify the input"),
            arg!(-o --output <OUTPUT> "Specify the output"),
        ])
        .get_matches();

    let input_id = matches.get_one::<String>("input").unwrap();
    let output_id = matches.get_one::<String>("output").unwrap();
    
    match input_id {
        "stdin" => {
            fn start() -> io::Result<()> {
                
            }

            start
        },
        _ => panic!("Unsupported Input"),
    }

    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut handle = stdin.lock();

    let colors = get_colors();

    let mut cached: HashMap<[u8; 3], u8> = HashMap::new();

    loop {
        let mut buf: [u8;3] = [0, 0, 0];
        handle.read_exact(&mut buf)?;

        //println!("input: {:#?}", buf);

        let is_cached: Option<&u8> = cached.get(&buf);
        let v = match is_cached {
            Some(val) => *val,
            None => {
                let mut result: u8 = 0;
                let mut best_score = 195076;
                for (rgb, v) in &colors {
                    let distance = 
                        ((rgb[0] as i32 - buf[0] as i32)) ^ 2 +
                        ((rgb[1] as i32 - buf[1] as i32)) ^ 2 +
                        ((rgb[2] as i32 - buf[2] as i32)) ^ 2;
                    
                    if distance < best_score {
                        result = *v;
                        best_score = distance;
                    }
                };

                cached.insert(buf, result);

                result
            }
        };

        //println!("calculated: {:#?}", v);
        stdout.write(&mut [v])?;
    }
}


fn get_colors() -> HashMap<[u8; 3], u8> {
    let mut colors = HashMap::new();

    colors.insert([89, 125, 39] as [u8;3], 0 as u8);
    colors.insert([109, 153, 48], 1);
    colors.insert([127, 178, 56], 2);
    colors.insert([67, 94, 29], 3);
    colors.insert([174, 164, 115], 4);
    colors.insert([213, 201, 140], 5);
    colors.insert([247, 233, 163], 6);
    colors.insert([130, 123, 86], 7);
    colors.insert([140, 140, 140], 8);
    colors.insert([171, 171, 171], 9);
    colors.insert([199, 199, 199], 10);
    colors.insert([105, 105, 105], 11);
    colors.insert([180, 0, 0], 12);
    colors.insert([220, 0, 0], 13);
    colors.insert([255, 0, 0], 14);
    colors.insert([135, 0, 0], 15);
    colors.insert([112, 112, 180], 16);
    colors.insert([138, 138, 220], 17);
    colors.insert([160, 160, 255], 18);
    colors.insert([84, 84, 135], 19);
    colors.insert([117, 117, 117], 20);
    colors.insert([144, 144, 144], 21);
    colors.insert([167, 167, 167], 22);
    colors.insert([88, 88, 88], 23);
    colors.insert([0, 87, 0], 24);
    colors.insert([0, 106, 0], 25);
    colors.insert([0, 124, 0], 26);
    colors.insert([0, 65, 0], 27);
    colors.insert([180, 180, 180], 28);
    colors.insert([220, 220, 220], 29);
    colors.insert([255, 255, 255], 30);
    colors.insert([135, 135, 135], 31);
    colors.insert([115, 118, 129], 32);
    colors.insert([141, 144, 158], 33);
    colors.insert([164, 168, 184], 34);
    colors.insert([86, 88, 97], 35);
    colors.insert([106, 76, 54], 36);
    colors.insert([130, 94, 66], 37);
    colors.insert([151, 109, 77], 38);
    colors.insert([79, 57, 40], 39);
    colors.insert([79, 79, 79], 40);
    colors.insert([96, 96, 96], 41);
    colors.insert([112, 112, 112], 42);
    colors.insert([59, 59, 59], 43);
    colors.insert([45, 45, 180], 44);
    colors.insert([55, 55, 220], 45);
    colors.insert([64, 64, 255], 46);
    colors.insert([33, 33, 135], 47);
    colors.insert([100, 84, 50], 48);
    colors.insert([123, 102, 62], 49);
    colors.insert([143, 119, 72], 50);
    colors.insert([75, 63, 38], 51);
    colors.insert([180, 177, 172], 52);
    colors.insert([220, 217, 211], 53);
    colors.insert([255, 252, 245], 54);
    colors.insert([135, 133, 129], 55);
    colors.insert([152, 89, 36], 56);
    colors.insert([186, 109, 44], 57);
    colors.insert([216, 127, 51], 58);
    colors.insert([114, 67, 27], 59);
    colors.insert([125, 53, 152], 60);
    colors.insert([153, 65, 186], 61);
    colors.insert([178, 76, 216], 62);
    colors.insert([94, 40, 114], 63);
    colors.insert([72, 108, 152], 64);
    colors.insert([88, 132, 186], 65);
    colors.insert([102, 153, 216], 66);
    colors.insert([54, 81, 114], 67);
    colors.insert([161, 161, 36], 68);
    colors.insert([197, 197, 44], 69);
    colors.insert([229, 229, 51], 70);
    colors.insert([121, 121, 27], 71);
    colors.insert([89, 144, 17], 72);
    colors.insert([109, 176, 21], 73);
    colors.insert([127, 204, 25], 74);
    colors.insert([67, 108, 13], 75);
    colors.insert([170, 89, 116], 76);
    colors.insert([208, 109, 142], 77);
    colors.insert([242, 127, 165], 78);
    colors.insert([128, 67, 87], 79);
    colors.insert([53, 53, 53], 80);
    colors.insert([65, 65, 65], 81);
    colors.insert([76, 76, 76], 82);
    colors.insert([40, 40, 40], 83);
    colors.insert([108, 108, 108], 84);
    colors.insert([132, 132, 132], 85);
    colors.insert([153, 153, 153], 86);
    colors.insert([81, 81, 81], 87);
    colors.insert([53, 89, 108], 88);
    colors.insert([65, 109, 132], 89);
    colors.insert([76, 127, 153], 90);
    colors.insert([40, 67, 81], 91);
    colors.insert([89, 44, 125], 92);
    colors.insert([109, 54, 153], 93);
    colors.insert([127, 63, 178], 94);
    colors.insert([67, 33, 94], 95);
    colors.insert([36, 53, 125], 96);
    colors.insert([44, 65, 153], 97);
    colors.insert([51, 76, 178], 98);
    colors.insert([27, 40, 94], 99);
    colors.insert([72, 53, 36], 100);
    colors.insert([88, 65, 44], 101);
    colors.insert([102, 76, 51], 102);
    colors.insert([54, 40, 27], 103);
    colors.insert([72, 89, 36], 104);
    colors.insert([88, 109, 44], 105);
    colors.insert([102, 127, 51], 106);
    colors.insert([54, 67, 27], 107);
    colors.insert([108, 36, 36], 108);
    colors.insert([132, 44, 44], 109);
    colors.insert([153, 51, 51], 110);
    colors.insert([81, 27, 27], 111);
    colors.insert([17, 17, 17], 112);
    colors.insert([21, 21, 21], 113);
    colors.insert([25, 25, 25], 114);
    colors.insert([13, 13, 13], 115);
    colors.insert([176, 168, 54], 116);
    colors.insert([215, 205, 66], 117);
    colors.insert([250, 238, 77], 118);
    colors.insert([132, 126, 40], 119);
    colors.insert([64, 154, 150], 120);
    colors.insert([79, 188, 183], 121);
    colors.insert([92, 219, 213], 122);
    colors.insert([48, 115, 112], 123);
    colors.insert([52, 90, 180], 124);
    colors.insert([63, 110, 220], 125);
    colors.insert([74, 128, 255], 126);
    colors.insert([39, 67, 135], 127);
    colors.insert([0, 153, 40], 128);
    colors.insert([0, 187, 50], 129);
    colors.insert([0, 217, 58], 130);
    colors.insert([0, 114, 30], 131);
    colors.insert([91, 60, 34], 132);
    colors.insert([111, 74, 42], 133);
    colors.insert([129, 86, 49], 134);
    colors.insert([68, 45, 25], 135);
    colors.insert([79, 1, 0], 136);
    colors.insert([96, 1, 0], 137);
    colors.insert([112, 2, 0], 138);
    colors.insert([59, 1, 0], 139);
    colors.insert([147, 124, 113], 140);
    colors.insert([180, 152, 138], 141);
    colors.insert([209, 177, 161], 142);
    colors.insert([110, 93, 85], 143);
    colors.insert([112, 57, 25], 144);
    colors.insert([137, 70, 31], 145);
    colors.insert([159, 82, 36], 146);
    colors.insert([84, 43, 19], 147);
    colors.insert([105, 61, 76], 148);
    colors.insert([128, 75, 93], 149);
    colors.insert([149, 87, 108], 150);
    colors.insert([78, 46, 57], 151);
    colors.insert([79, 76, 97], 152);
    colors.insert([96, 93, 119], 153);
    colors.insert([112, 108, 138], 154);
    colors.insert([59, 57, 73], 155);
    colors.insert([131, 93, 25], 156);
    colors.insert([160, 114, 31], 157);
    colors.insert([186, 133, 36], 158);
    colors.insert([98, 70, 19], 159);
    colors.insert([72, 82, 37], 160);
    colors.insert([88, 100, 45], 161);
    colors.insert([103, 117, 53], 162);
    colors.insert([54, 61, 28], 163);
    colors.insert([112, 54, 55], 164);
    colors.insert([138, 66, 67], 165);
    colors.insert([160, 77, 78], 166);
    colors.insert([84, 40, 41], 167);
    colors.insert([40, 28, 24], 168);
    colors.insert([49, 35, 30], 169);
    colors.insert([57, 41, 35], 170);
    colors.insert([30, 21, 18], 171);
    colors.insert([95, 75, 69], 172);
    colors.insert([116, 92, 84], 173);
    colors.insert([135, 107, 98], 174);
    colors.insert([71, 56, 51], 175);
    colors.insert([61, 64, 64], 176);
    colors.insert([75, 79, 79], 177);
    colors.insert([87, 92, 92], 178);
    colors.insert([46, 48, 48], 179);
    colors.insert([86, 51, 62], 180);
    colors.insert([105, 62, 75], 181);
    colors.insert([122, 73, 88], 182);
    colors.insert([64, 38, 46], 183);
    colors.insert([53, 43, 64], 184);
    colors.insert([65, 53, 79], 185);
    colors.insert([76, 62, 92], 186);
    colors.insert([40, 32, 48], 187);
    colors.insert([53, 35, 24], 188);
    colors.insert([65, 43, 30], 189);
    colors.insert([76, 50, 35], 190);
    colors.insert([40, 26, 18], 191);
    colors.insert([53, 57, 29], 192);
    colors.insert([65, 70, 36], 193);
    colors.insert([76, 82, 42], 194);
    colors.insert([40, 43, 22], 195);
    colors.insert([100, 42, 32], 196);
    colors.insert([122, 51, 39], 197);
    colors.insert([142, 60, 46], 198);
    colors.insert([75, 31, 24], 199);
    colors.insert([26, 15, 11], 200);
    colors.insert([31, 18, 13], 201);
    colors.insert([37, 22, 16], 202);
    colors.insert([19, 11, 8], 203);

    colors
}